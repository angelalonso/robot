extern crate regex;
use crate::api::Api;
use crate::arduino::Arduino;
use crate::leds::LEDs;
use crate::motors::Motors;
use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{SyncSender, Sender, Receiver};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainDeadError {
    /// This is just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("Setup could not be loaded")]
    LoadSetupError,

    #[error("Action Rules could not be loaded")]
    LoadActionRulesError,

    #[error("YAML file does not contain the expected content")]
    YamlError,

    #[error("File does not exist")]
    FileNotFoundError,

    #[error("Couldn't translate String into action")]
    GetActionFromStringError,

    #[error("Couldn't add action")]
    AddActionError,

    #[error("Couldn't do action")]
    DoActionError,
}

#[derive(Deserialize)]
pub struct ConfigHashmap {
    #[allow(dead_code)]
    entries: HashMap<String, HashMap<String, String>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Condition {
    pub time: String,
    pub input_objs: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Action {
    pub object: String,
    pub value: String,
    pub time: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ActionRule {
    id: String,
    condition: Vec<Condition>,
    actionsloop: bool,
    triggercount: u32,
    actions: Vec<Action>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NoTriggerActionRule {
    id: String,
    condition: Vec<Condition>,
    actionsloop: bool,
    actions: Vec<Action>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NoConditionActionRule {
    id: String,
    actions: Vec<Action>
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimedData {
    id: usize,
    belongsto: String,
    data: String,
    time: f64,
}

#[derive(Clone, Debug)]
pub struct ResultAction {
    resource: String,
    action: TimedData,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Set {
    object: String,
    obj_type: String,
    entries: Vec<TimedData>,
    last_change_timestamp: f64,
    current_entry: TimedData,
    max_size: u8,
}

#[derive(Clone)]
pub struct Brain {
    name: String,
    pub mode: String,
    pub setup_file: String,
    start_time: u128,
    timestamp: f64,
    record_file: String,
    actionrules: Vec<ActionRule>,
    pub arduino: Arduino,
    pub api: Api,
    motors: Motors,
    leds: LEDs,
    actionbuffersets: Vec<Set>,
    metricsets: Vec<Set>,
}

static COUNTER: std::sync::atomic::AtomicUsize = AtomicUsize::new(1);
static MAX_BUFFERSIZE: u8 = 25;
static MAX_METRICSIZE: u8 = 25;
// this is a list of the actions that go in the same actionbufferset, called brilliantly "other"
const OTHER_ACTIONS: &[&str; 2] = &["load", "wait"];

impl Brain {
    pub fn new(brain_name: String, mut mode: String, setup_file: String) -> Result<Self, BrainDeadError> {
        // CATCH the record mode and clean up the original mode variable
        let special_mode = mode.split('_').collect::<Vec<_>>();
        let mut record_file = "".to_string();
        if special_mode.len() > 1 {
            if let "record" = special_mode[1] { record_file = Brain::create_record_file() }
        }
        mode = special_mode[0].to_string();
        let st = SystemTime::now();
        let start_time = match st.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
        };
        let (first_action_set, _first_arduino_program, inputs, outputs) = match Brain::load_setup(setup_file.to_string()) {
            Ok((fas,fap,i,o)) => (fas,fap,i,o),
            Err(e) => {
                error!("There was an error!: {}", e);
                return Err(BrainDeadError::LoadSetupError)
            }

        };
        let ar = match Brain::load_action_rules(first_action_set) {
            Ok(action_rules) => action_rules,
            Err(e) => {
                error!("There was an error!: {}", e);
                return Err(BrainDeadError::LoadActionRulesError)
            }
        };
        let mut a = Arduino::new("arduino".to_string(), Some("/dev/null".to_string())).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        let ap = Api::new().unwrap_or_else(|err| {
            eprintln!("Problem Initializing Aoi: {}", err);
            process::exit(1);
        });
        if mode.clone() != "dryrun" {
            a = Arduino::new("arduino".to_string(), None).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Arduino: {}", err);
                process::exit(1);
            });
            //NOTE: We want to avoid installing with avrdude from the raspberry for now
            //a.install(&first_arduino_program).unwrap();
        };
        let mut abs = [].to_vec();
        let mut ms = [].to_vec();
        let mut leds_map = HashMap::new();
        let mut motors_map = HashMap::new();
        // Generic empty element for the buffers
        let s_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        // INPUTS -> they only have metrics buffers
        //for i in inputs.entries {
        for i in inputs { // previous line when new_load_setup works
            let s = Set {
                object: i.0,
                obj_type: i.1["type"].clone(),
                entries: [].to_vec(),
                last_change_timestamp: 0.0,
                current_entry: s_e.clone(),
                max_size: MAX_METRICSIZE,
            };
            ms.push(s);
        }
        // OUTPUTS -> they have actions buffers and metrics buffers
        //for o in outputs.entries {
        for o in outputs { // previous line when new_load_setup works
            // This trick allows us to define configs for the output objects
            let mut name = o.0.clone();
            if o.0.starts_with("led_") {
                let led_key = o.0.clone();
                let led_val = o.1.clone();
                leds_map.insert(led_key, led_val);
                name = o.0.split("__").collect::<Vec<_>>()[0].to_string();
            } else if o.0.starts_with("motor_") {
                let motor_key = o.0.clone();
                let motor_val = o.1.clone();
                motors_map.insert(motor_key, motor_val);
                name = o.0.split("__").collect::<Vec<_>>()[0].to_string();
            }
            let s_b = Set {
                object: name.clone(),
                obj_type: "output".to_string(),
                entries: [].to_vec(),
                last_change_timestamp: 0.0,
                current_entry: s_e.clone(),
                max_size: MAX_BUFFERSIZE,
            };
            abs.push(s_b);
            let s_m = Set {
                object: name.clone(),
                obj_type: "output".to_string(),
                entries: [].to_vec(),
                last_change_timestamp: 0.0,
                current_entry: s_e.clone(),
                max_size: MAX_METRICSIZE,
            };
            ms.push(s_m);
        }
        // OTHER -> we need buffers and metrics for other stuff
        let s_b_o = Set {
            object: "other".to_string(),
            obj_type: "other".to_string(),
            entries: [].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: s_e.clone(),
            max_size: MAX_BUFFERSIZE,
        };
        let s_m_o = Set {
            object: "other".to_string(),
            obj_type: "other".to_string(),
            entries: [].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: s_e,
            max_size: MAX_METRICSIZE,
        };
        abs.push(s_b_o);
        ms.push(s_m_o);
        let m = Motors::new(mode.clone(), motors_map).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Motors: {}", err);
            process::exit(1);
        });
        let l = LEDs::new(mode.clone(), leds_map).unwrap_or_else(|err| {
            eprintln!("Problem Initializing LEDs: {}", err);
            process::exit(1);
        });
        Ok(Self {
            name: brain_name,
            mode,
            setup_file,
            start_time,
            timestamp: 0.0,
            record_file,
            actionrules: ar,
            arduino: a,
            api: ap,
            motors: m,
            leds: l,
            actionbuffersets: abs,
            metricsets: ms,
        })
    }

    /// Load a robot setup yaml file and configures the system
    pub fn load_setup(setup_file: String) -> Result<(String, String, HashMap<String, HashMap<String, String>>, HashMap<String, HashMap<String, String>>), BrainDeadError> {
        #[derive(Deserialize)]
        struct Setup {
            start_ruleset_file: String,
            start_arduinohex_file: String,
            inputs: HashMap<String, HashMap<String, String>>,
            outputs: HashMap<String, HashMap<String, String>>,
        }
        let file_pointer = match File::open(setup_file.clone()) {
            Ok(fp) => fp,
            Err(_) => {
                error!("File {} does not exist", setup_file.clone());
                return Err(BrainDeadError::FileNotFoundError)
            }
        };
        let a: Setup = match serde_yaml::from_reader(file_pointer) {
            Ok(ya) => ya,
            Err(e) => {
                error!("The file {}'s YAML is incorrect! - {}", setup_file.clone(), e);
                return Err(BrainDeadError::YamlError)
            }
        };
        return Ok((a.start_ruleset_file, a.start_arduinohex_file, a.inputs, a.outputs))
        //return (a.start_ruleset_file, a.start_arduinohex_file, [].to_vec(), [].to_vec())
    }

    /// Load a robot setup yaml file and configures the system
    pub fn new_load_setup(setup_file: String) -> Result<(String, String, ConfigHashmap, ConfigHashmap), BrainDeadError> {
        #[derive(Deserialize)]
        struct Setup {
            start_ruleset_file: String,
            start_arduinohex_file: String,
            inputs: ConfigHashmap,
            outputs: ConfigHashmap,
        }
        let file_pointer = match File::open(setup_file.clone()) {
            Ok(fp) => fp,
            Err(_) => {
                error!("File {} does not exist", setup_file);
                return Err(BrainDeadError::FileNotFoundError)
            }
        };
        let a: Setup = match serde_yaml::from_reader(file_pointer) {
            Ok(ya) => ya,
            Err(e) => {
                error!("The file {}'s YAML is incorrect! - {}", setup_file, e);
                return Err(BrainDeadError::YamlError)
            }
        };
        Ok((a.start_ruleset_file, a.start_arduinohex_file, a.inputs, a.outputs))
    }

    /// Return current timestamp as millis
    pub fn get_current_time(&mut self) -> f64 {
        let now = SystemTime::now();
        match now.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis() as f64,
            Err(_e) => 0.0,
        }
    }

    /// Return difference between current timestamp and a given one, in millis
    pub fn get_time_since(&mut self, start_timestamp: f64) -> f64 {
        let now = SystemTime::now();
        match now.duration_since(UNIX_EPOCH) {
            Ok(time) => (time.as_millis() as f64 - start_timestamp as f64) / 1000_f64,
            Err(_e) => 0.0,
        }
    }

    /// Translates a message coming from the Arduino to actions
    pub fn use_arduino_msg(&mut self, timestamp: f64, raw_msg: String) {
        let msg_parts = raw_msg.split(": ").collect::<Vec<_>>();
        if self.record_file != "" {
            self.record(timestamp, raw_msg.to_string());
        };
        // NOTE: we could have other types of messages but we don't need them just yet
        if let "SENSOR" = msg_parts[0] {
            let sensors = msg_parts[1].split('|').collect::<Vec<_>>();
            for s in sensors {
                let sensor = s.split('=').collect::<Vec<_>>();
                if sensor != [""] {
                    info!("Message from Arduino: {:?}", sensor);
                } else {
                    debug!("Message from Arduino: {:?}", sensor);
                }
                let sensor_id = "arduino".to_string();
                if sensor.len() > 1 {
                    self.add_metric(timestamp, format!("{}__{}", sensor[0], sensor[1]), sensor_id);
                } else {
                    trace!("{:?}", sensor);
                }
            }
        }
    }

    // ########## ACTIONS ########## //

    /// Log action buffers' content
    pub fn show_actionbuffers(&mut self) {
        for abs in &self.actionbuffersets {
            //if b.object == "motor_r" {
            //    println!("- {} ACTIONS pending for {}", b.entries.len(), b.object);
            //    for (ix, action) in b.entries.clone().iter().enumerate() {
            //        println!(" #{} |data={}|time={}|", ix, action.data, action.time);
            //    }
            //}
            trace!("- {} ACTIONS pending for {}", abs.entries.len(), abs.object);
            for (ix, action) in abs.entries.clone().iter().enumerate() {
                trace!(" #{} |data={}|time={}|", ix, action.data, action.time);
            }
        }
    }

    /// Load actions and rules using the same pattern.
    /// The actions differ from configs in that they will ALWAYS be loaded 
    pub fn load_action_rules(file: String) -> Result<Vec<ActionRule>, BrainDeadError> {
        let file_pointer = match File::open(file.clone()){
            Ok(fp) => fp,
            Err(_e) => {
                error!("File {} does not exist", file);
                return Err(BrainDeadError::FileNotFoundError)
            }
        };
        let mut c: Vec<ActionRule> = [].to_vec();
        match serde_yaml::from_reader(file_pointer) {
            Ok(v) => Ok(v),
            Err(e) => {
                if e.to_string().contains("missing field `triggercount`") {

                    let file_pointer = match File::open(file.clone()){
                        Ok(fp) => fp,
                        Err(_e) => {
                            error!("File {} does not exist", file);
                            return Err(BrainDeadError::FileNotFoundError)
                        }
                    };
                    let a: Vec<NoTriggerActionRule> = match serde_yaml::from_reader(file_pointer) {
                        Ok(a) => a,
                        Err(e) => {
                            error!("The file {}'s YAML is incorrect! - {}", file, e);
                            return Err(BrainDeadError::YamlError)
                        }
                    };
                    for i in a {
                        let c_elem = ActionRule {
                            id: i.id,
                            triggercount: 0,
                            condition: i.condition,
                            actionsloop: i.actionsloop,
                            actions: i.actions,
                        };
                        c.push(c_elem);
                    }
                    Ok(c)
                } else if e.to_string().contains("missing field `condition`") {
                    let file_pointer = match File::open(file.clone()){
                        Ok(fp) => fp,
                        Err(_e) => {
                            error!("File {} does not exist", file);
                            return Err(BrainDeadError::FileNotFoundError)
                        }
                    };
                    let a: Vec<NoConditionActionRule> = match serde_yaml::from_reader(file_pointer) {
                        Ok(a) => a,
                        Err(e) => {
                            error!("The file {}'s YAML is incorrect for a RuleSet! - {}", file, e);
                            return Err(BrainDeadError::YamlError)
                        }
                    };
                    for i in a {
                        let c_elem = ActionRule {
                            id: i.id,
                            triggercount: 0,
                            condition: [Condition {
                                 time: "*".to_string(),
                                 input_objs: "".to_string(),
                            }].to_vec(),
                            actionsloop: false,
                            actions: i.actions,
                        };
                        c.push(c_elem);
                    }
                    Ok(c)
                } else {
                    error!("The file {}'s YAML is incorrect! - {}", file, e);
                    Err(BrainDeadError::YamlError)                    
                }
            },
        }
    }

    /// Turn a String containing an action into the related object
    pub fn get_action_from_string(&mut self, action: String) -> Result<ResultAction, BrainDeadError> {
        let format = action.split(',').collect::<Vec<_>>();
        //TODO: reproduce this error, then use BrainDeadError instead of unwrap
        let t = format[1].split('=').collect::<Vec<_>>()[1].parse::<f64>().unwrap();
        let data = format[0].split('=').collect::<Vec<_>>();
        let mut source = "";
        if format.len() > 2 {
            source = format[2];
        }
        let action_item = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: source.to_string(),
            data: data[1].to_string(),
            time: t,
        };
        let result = ResultAction {
            resource: data[0].to_string(),
            action: action_item,
        };
        Ok(result)
    }

    /// Returns true if a given action is already in the related actions buffer
    pub fn are_actions_in_buffer(actionbuffersets: Vec<Set>,rule: ActionRule) -> bool {
        let mut result = false;
        for abs in actionbuffersets {
            for existing in abs.entries.clone() {
                if existing.belongsto == rule.id {
                    result = true;
                }
            }
            if abs.current_entry.belongsto == rule.id {
                result = true;
            }
        }
        result
    }

    /// Adds action to the related actions buffer
    pub fn add_action(&mut self, action: String) -> Result<String, BrainDeadError> {
        trace!("- Adding action {}", action);
        let action_to_add = match self.clone().get_action_from_string(action) {
            Ok(resultaction) => resultaction,
            Err(e) => {
                error!("There was an error!: {}", e);
                return Err(BrainDeadError::GetActionFromStringError)
            }
        };
        if OTHER_ACTIONS.iter().any(|&i| i==action_to_add.resource) {
            if let Some(abs) = self.actionbuffersets.iter_mut().find(|x| x.object == "other") {
                if abs.entries.len() >= abs.max_size.into() {
                    warn!("Buffer for {} is full! not adding new actions...", abs.object);
                } else {
                    match action_to_add.resource.as_str() {
                        "load" => {
                            let a = TimedData {
                                id: action_to_add.action.id,
                                belongsto: action_to_add.action.belongsto,
                                data: format!("{}_{}", action_to_add.resource, action_to_add.action.data),
                                time: action_to_add.action.time,
                            };
                            abs.entries.push(a);
                        },
                        "wait" => {
                            let a = TimedData {
                                id: action_to_add.action.id,
                                belongsto: action_to_add.action.belongsto,
                                data: format!("{}_{}secs", action_to_add.resource, action_to_add.action.time),
                                time: action_to_add.action.time,
                            };
                            abs.entries.push(a);
                        },
                        _ => {
                            abs.entries.push(action_to_add.action);
                        },
                    }
                };
            };
        } else if let Some(abs) = self.actionbuffersets.iter_mut().find(|x| *x.object == *action_to_add.resource) {
            if abs.entries.len() >= abs.max_size.into() {
                warn!("Buffer for {} is full! not adding new actions...", abs.object);
            } else {
                match abs.object.as_str() {
                    "load" => {
                        let a = TimedData {
                            id: action_to_add.action.id,
                            belongsto: action_to_add.action.belongsto,
                            data: format!("{}_{}", action_to_add.resource, action_to_add.action.data),
                            time: action_to_add.action.time,
                        };
                        abs.entries.push(a);
                    },
                    _ => {
                        abs.entries.push(action_to_add.action);
                    },
                }
            };
        }
        Ok("".to_string())
    }

    /// Starting from a list of actions received from the actionrules:
    /// - Empties each actionbufferset for objects involved
    /// - Performs the first action(s) for each object
    /// - Adds the other actions for each object to the actionbuffersets
    /// Return the action(s) taken and it's related metric
    pub fn do_actions_from_rules(&mut self, actions: Vec<Set>, ct: f64) -> Result<(Vec<String>, Vec<String>), BrainDeadError>{
        let mut new_metrics: Vec<String> = [].to_vec();
        let mut new_actions: Vec<String> = [].to_vec();
        for mut action in actions {
            // cleanup actionbufferset
            let mut action_object = action.object.clone();
            if OTHER_ACTIONS.iter().any(|&i| i == action.object) { action_object = "other".to_string() }
            if let Some(abs) = self.actionbuffersets.iter_mut().find(|x| x.object == action_object) { abs.entries = Vec::new() }
            // trigger first action
            if let Some(entry) = action.entries.first() {
                let (these_metrics, these_actions) = match self.do_action(action.clone(), entry.clone(), ct) {
                    Ok((met, act)) => (met, act),
                    Err(e) => {
                        error!("There was an error!: {}", e);
                        return Err(BrainDeadError::DoActionError)
                    }
                };
                for m_raw in these_metrics {
                    new_metrics.push(m_raw);
                }
                for c_raw in these_actions {
                    new_actions.push(c_raw);
                }
                action.entries.remove(0);
            }
            // add the rest to actionbufferset
            for entry in action.entries {
                let action_string = format!("{}={},time={},{}", action.object, entry.data, entry.time, entry.id);
                match self.add_action(action_string) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("There was an error!: {}", e);
                        return Err(BrainDeadError::AddActionError)
                    }
                };
            }
        }
        Ok((new_metrics, new_actions))
    }

    /// Performs the next action on each action buffer if the timestamp is right.
    /// Return the action(s) taken and it's related metric
    pub fn do_actions_from_actionbuffersets(&mut self, timestamp: f64) -> Result<(Vec<String>, Vec<String>), BrainDeadError>{
        let mut result = [].to_vec();
        let mut metrics = [].to_vec();
        for abs in self.actionbuffersets.iter_mut() {
            if let Some(om) = self.metricsets.iter_mut().find(|x| *x.object == *abs.object) {
                //if timestamp >= om.last_change_timestamp {
                //    if abs.entries.len() > 0 {
                if (timestamp >= om.last_change_timestamp) && (!abs.entries.is_empty()) {
                    let a = &abs.entries.clone()[0];             
                    let time_passed = ((timestamp - abs.last_change_timestamp) as f64 * 1000_f64).ceil() / 1000_f64;
                    trace!("- {} > Time passed on current value - {:?}", om.object, time_passed);
                    if time_passed >= abs.current_entry.time {
                        abs.current_entry = a.clone();
                        abs.entries.retain(|x| *x != *a);
                        abs.last_change_timestamp = timestamp;
                        debug!("- Buffer: {:#x?}", abs.entries);
                        // TODO: Avoid hardcoding this (use types of actions?)
                        if abs.object.starts_with("led") {
                            self.leds.set_led(om.object.clone(), a.data.parse::<u8>().unwrap() == 1);
                        } else if abs.object.starts_with("motor") {
                            let action_vector = a.data.split('_').collect::<Vec<_>>();
                            self.motors.set(abs.object.clone(), action_vector[0].to_string());
                        } else if abs.object.starts_with("other") {
                            let other_action = a.data.split('_').collect::<Vec<_>>();
                            if other_action[0] == "load" {
                                let file_to_load = other_action[1..].join("_").to_string();
                                self.actionrules = match Brain::load_action_rules(file_to_load) {
                                    Ok(action_rules) => action_rules,
                                    Err(e) => {
                                        error!("There was an error!: {}", e);
                                        return Err(BrainDeadError::LoadActionRulesError)
                                    }
                                };
                            }
                        }
                        info!("- Just did {} -> {} (from buffer)", om.object, a.data);
                        // TODO actually both the following could be one if we unified format
                        metrics.push(format!("{}__{}|{}", abs.object, a.data, a.id.to_string()));
                        result.push(format!("{}__{}__{:?}", abs.object, a.clone().data, a.clone().time));
                    }
                }
            }
        };
        if result.is_empty() {result.push("".to_string())};
        if metrics.is_empty() {metrics.push("".to_string())};
        Ok((metrics, result))
    }

    // ########## METRICS ########## //

    /// Log objects' metrics
    pub fn show_metrics(&mut self) {
        for m in self.metricsets.clone().iter() {
            //if m.object == "motor_r" {
            //    println!("- {} METRICS recorded for {}", m.entries.len(), m.object);
            //    for (ix, action) in m.entries.clone().iter().enumerate() {
            //        println!(" #{} |data={}|time={}|", ix, action.data, action.time);
            //    }
            //}
            trace!("- Metrics - {} : {} entries", m.object, m.entries.len());
            for (ix, action) in m.entries.clone().iter().enumerate() {
                trace!(" #{} |data={}|time={}|", ix, action.data, action.time);
            }
        }
    }

    /// Add metric to its related metric set
    pub fn add_metric(&mut self, timestamp: f64,metric: String, source_id: String) {
        // NOTE: debug entry point
        //println!("- Adding metric {} from {}", metric, source_id);
        let metric_decomp = metric.split("__").collect::<Vec<_>>();
        if let Some(om) = self.metricsets.iter_mut().find(|x| *x.object == *metric_decomp[0]) {
            if om.entries.is_empty() {
                let new_m = TimedData {
                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
                    belongsto: source_id,
                    data: metric_decomp[1].to_string(),
                    time: timestamp, 
                };
                om.entries.push(new_m);
                om.last_change_timestamp = timestamp;
            } else if om.entries[0].data != metric_decomp[1] {
                let new_m = TimedData {
                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
                    belongsto: source_id,
                    data: metric_decomp[1].to_string(),
                    time: timestamp,
                };
                om.entries.insert(0, new_m);
                om.last_change_timestamp = timestamp;
            }; 
            if om.entries.len() > om.max_size.into() {
                om.entries.pop();
            };
        }
    }

    pub fn does_condition_match(&mut self, rule: ActionRule, timestamp: f64) -> bool {
        let mut result = true;
        let checks = rule.condition[0].input_objs.split(',').collect::<Vec<_>>();
        for check in &checks {
            let re = regex::Regex::new(r"=|<=|>=|<|>").unwrap();
            let keyval = re.split(check).collect::<Vec<_>>();
            if let Some(om) = self.metricsets.iter_mut().find(|x| *x.object == *keyval[0]) {
                match om.obj_type.as_str() {
                    // for binary and output we just try to match fully
                    "binary" | "output" => {
                        if !om.entries.is_empty() {
                            if om.entries[0].data != keyval[1] || 
                                (timestamp - om.entries[0].time < rule.condition[0].time.parse::<f64>().unwrap()) && (om.entries[0].time != 0.0) {
                                result = false;
                                return result
                            };
                        } else if keyval[1] != "0" {
                            result = false;
                            return result
                        }
                    },
                    "continuous" => {
                        let comparison = check.replace(keyval[0], "").replace(keyval[1], "");
                        let mut matched_metrics: Vec<TimedData> = [].to_vec();
                        if !om.entries.is_empty() {
                            match comparison.as_str() {
                                "=" => {
                                    for m in om.entries.clone() {
                                        if m.data.parse::<u16>().unwrap() == keyval[1].parse::<u16>().unwrap() {
                                            matched_metrics.push(m);
                                        } else {
                                            break;
                                        }
                                    }
                                },
                                "<=" => {
                                    for m in om.entries.clone() {
                                        if m.data.parse::<u16>().unwrap() <= keyval[1].parse::<u16>().unwrap() {
                                            matched_metrics.push(m);
                                        } else {
                                            break;
                                        }
                                    }
                                },
                                ">=" => {
                                    for m in om.entries.clone() {
                                        if m.data.parse::<u16>().unwrap() >= keyval[1].parse::<u16>().unwrap() {
                                            matched_metrics.push(m);
                                        } else {
                                            break;
                                        }
                                    }
                                },
                                "<" => {
                                    for m in om.entries.clone() {
                                        if m.data.parse::<u16>().unwrap() < keyval[1].parse::<u16>().unwrap() {
                                            matched_metrics.push(m);
                                        } else {
                                            break;
                                        }
                                    }
                                },
                                ">" => {
                                    for m in om.entries.clone() {
                                        if m.data.parse::<u16>().unwrap() > keyval[1].parse::<u16>().unwrap() {
                                            matched_metrics.push(m);
                                        } else {
                                            break;
                                        }
                                    }
                                },
                                &_ => {},
                            }
                            if !matched_metrics.is_empty() {
                                // This is a trick meant to get NaN as the initial value, that's why we want to ignore
                                // https://www.reddit.com/r/rust/comments/3fg0xr/how_do_i_find_the_max_value_in_a_vecf64/
                                #[allow(clippy::zero_divided_by_zero)]
                                #[allow(clippy::eq_op)]
                                let acc_time = matched_metrics.clone().iter().map(|x| x.time).collect::<Vec<_>>().iter().cloned().fold(0./0., f64::min);
                                if acc_time.to_string() == "NaN" || 
                                    (timestamp - acc_time < rule.condition[0].time.parse::<f64>().unwrap()) && (acc_time != 0.0) {
                                    result = false;
                                    return result
                                }
                            } else {
                                result = false;
                                return result
                            }
                            // put together all metrics that fit comparison
                            // add the timestamps
                            // compare to desired time
                            // ...and if it doesnt fit, remove
                        } else {
                            result = false;
                            return result
                        }
                    },
                    &_ => {
                    },
                };
            }
        }
        result
    }

    pub fn create_record_file() -> String {
        let path = "records";
        fs::create_dir_all(path).unwrap();
        let filename = path.to_owned() + "/last_run.yaml";
        File::create(filename.clone()).unwrap();
        filename
    }

    pub fn record(&mut self, timestamp: f64, entry: String) {
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.record_file.clone())
            .unwrap();
        let log = format!("- time: {}\n  msg: \"{}\"", timestamp, entry);
        warn!("We are writing {} to {}", log, self.record_file);
        writeln!(&mut file, "{}", log).unwrap();
    }

    /// Checks the input of the rules loaded and, if they fit, returns the actions to take
    pub fn get_actions_from_rules(&mut self, timestamp: f64) -> Result<Vec<Set>, BrainDeadError>{
        let mut partial_rules: Vec<ActionRule> = self.actionrules.clone();
        let mut action_vectors: Vec<Set> = [].to_vec();
        for abs in self.actionbuffersets.clone() {
            let new_elem = Set {
                object: abs.object,
                obj_type: abs.obj_type,
                entries: [].to_vec(),
                last_change_timestamp: abs.last_change_timestamp,
                current_entry: abs.current_entry,
                max_size: abs.max_size,
            };
            action_vectors.push(new_elem);
        }
        for rule in partial_rules.clone() {
            // NEVER add something that is already on buffer
            if Brain::are_actions_in_buffer(self.actionbuffersets.clone(), rule.clone()) {
                partial_rules.retain(|x| *x != rule);
            } else {
                // triggercount > 0?
                //  y -> loop ==true?
                //       y -> add, adjust triggercount for self to +1
                //       n -> conditions == ""?
                //            y -> remove
                //            n -> do all conds match?
                //                 y -> add, adjust triggercount for self to +1
                //                 n -> remove
                //  n -> conditions == ""?
                //       y -> add, adjust triggercount for self to +1
                //       n -> do all conds match?
                //            y -> add, adjust triggercount for self to +1
                //            n -> remove
                if rule.triggercount > 0 {
                    if !rule.actionsloop {
                        //if ! self.does_condition_match(rule.clone(), timestamp.clone()) {
                        //    partial_rules.retain(|x| *x != rule);
                        //}
                        let checks = rule.condition[0].input_objs.split(',').collect::<Vec<_>>();
                        if checks != [""].to_vec() && !checks.is_empty() {
                            if ! self.does_condition_match(rule.clone(), timestamp) {
                                partial_rules.retain(|x| *x != rule);
                            }
                        } else {
                          partial_rules.retain(|x| *x != rule);
                        }
                    }
                } else {
                    let checks = rule.condition[0].input_objs.split(',').collect::<Vec<_>>();
                    if (checks != [""].to_vec()) && (!checks.is_empty()) && (! self.does_condition_match(rule.clone(), timestamp)) {
                        partial_rules.retain(|x| *x != rule);
                    }
                }
            }
        }
        // We want to count the amount of times the trigger was...triggered
        if !partial_rules.is_empty() {
            for rule in self.actionrules.iter_mut() {
                if partial_rules.clone().iter_mut().any(|x| x.id == *rule.id) {
                    rule.triggercount += 1;
                };
            }
            debug!("- Rules matching :");
            for (ix, rule) in partial_rules.iter().enumerate() {
                debug!(" #{} {} input:", ix, rule.id);
                debug!("     input:");
                for ri in rule.condition.clone() {
                    debug!("      |{:?}|", ri);
                }
                debug!("     output:");
                // we store a vector of actionbuffersets
                for action in rule.actions.clone() {
                    if let Some(bf) = action_vectors.iter_mut().find(|x| *x.object == *action.object) {
                        let entry = TimedData {
                            id: COUNTER.fetch_add(1, Ordering::Relaxed),
                            belongsto: rule.id.clone(),
                            data: action.value,
                            time: action.time.parse::<f64>().unwrap(),
                        };
                        bf.entries.push(entry);
                    }
                }
            }
        }
        // Clean up the vector from actionbuffersets that are empty
        for s in action_vectors.clone() {
            if s.entries.is_empty() {
                action_vectors.retain(|x| *x != s);
            }
        }
        Ok(action_vectors)
    }

    pub fn do_action(&mut self, om: Set, a: TimedData, timestamp: f64) -> Result<(Vec<String>, Vec<String>), BrainDeadError>{
        let mut result = [].to_vec();
        let mut metrics = [].to_vec();
        if let Some(abs) = self.actionbuffersets.iter_mut().find(|x| *x.object == *om.object) {
            abs.last_change_timestamp = timestamp;
        }
        if om.object.starts_with("led") {
            self.leds.set_led(om.object.clone(), a.data.parse::<u8>().unwrap() == 1);
        } else if om.object.starts_with("motor") {
            let action_vector = a.data.split('_').collect::<Vec<_>>();
            self.motors.set(om.object.clone(), action_vector[0].to_string());
        } else if om.object.starts_with("other") {
            let other_action = a.data.split('_').collect::<Vec<_>>();
            if other_action[0] == "load" {
                let file_to_load = other_action[1..].join("_");
                self.actionrules = match Brain::load_action_rules(file_to_load) {
                    Ok(action_rules) => action_rules,
                    Err(e) => {
                        error!("There was an error!: {}", e);
                        return Err(BrainDeadError::LoadActionRulesError)
                    }
                };
            }
        }
        if let Some(abs) = self.actionbuffersets.iter_mut().find(|x| *x.object == *om.object) {
            abs.current_entry = TimedData {
                id: COUNTER.fetch_add(1, Ordering::Relaxed),
                belongsto: om.entries[0].clone().belongsto,
                data: om.entries[0].clone().data,
                time: om.entries[0].clone().time,
            };
        }
        //TODO: this info should come from the leds module itself
        info!("- Just did {} -> {}", om.object, a.data);
        // TODO actually both the following could be one if we unified format
        metrics.push(format!("{}__{}|{}", om.object, a.data, a.belongsto.to_string()));
        result.push(format!("{}__{}__{:?}", om.object, a.clone().data, a.time));
        Ok((metrics, result))
    }

    /// Just run the brain.
    /// - secs_to_run has to have decimals, 4.0 is valid, 4 is not
    /// - precission: how often we do stuff
    ///   - 1 is secs, 10 is decs of a sec, 100 is hundreds of a sec...
    pub fn run(&mut self, secs_to_run: Option<f64>, precission: u16, sender: Sender<String>) {
        // timestamps
        let start_timestamp = self.get_current_time();
        let mut ct: f64 = 0.0;
        // communication with arduino
        let (s, r): (SyncSender<String>, Receiver<String>) = std::sync::mpsc::sync_channel(2);
        let msgs = s.clone();
        let msgs_api = s.clone();
        let mut arduino_clone = self.arduino.clone();
        let mut api_clone_runner = self.api.clone();
        thread::spawn(move || {
            api_clone_runner.run(msgs_api);
        });
        let brain_clone = self.clone();
        thread::spawn(move || {
            if brain_clone.mode != "dryrun" {
                //TODO: find a way to reproduce this error, then add BrainDeadError to this function
                arduino_clone.read_channel(msgs).unwrap();
            } else {
                arduino_clone.read_channel_mock(msgs, brain_clone.setup_file.clone()).unwrap();
            };
        });
        if let Ok(m) = r.try_recv() { self.use_arduino_msg(ct, m) }
        loop {
            let mut new_metrics: Vec<String> = [].to_vec();
            let mut new_actions: Vec<String> = [].to_vec();
            // update current timestamp
            let ct_raw = self.get_time_since(start_timestamp);
            let new_ct = (ct_raw * precission as f64).floor() / precission as f64;
            if new_ct > ct { 
                ct = new_ct;
                if let Ok(m) = r.try_recv() { self.use_arduino_msg(ct, m) }
                self.show_metrics();
                self.show_actionbuffers();
                // get actions
                // TODO: this is taking a while. Troubleshoot to see which one exactly and to try
                // and speed this up
                match self.get_actions_from_rules(ct){
                    Ok(actions) => {
                        // do first action from rules, add the rest to the actionbuffersets
                        let (these_metrics, these_actions) = self.do_actions_from_rules(actions.clone(), ct).unwrap();
                        for m_raw in these_metrics {
                            new_metrics.push(m_raw);
                        }
                        for c_raw in these_actions {
                            new_actions.push(c_raw);
                        }
                    },
                    Err(_e) => trace!("...no matching rules found"),
                };
                // do action(s) from the actionbuffersets that match the ct
                let (these_metrics, these_actions) = self.do_actions_from_actionbuffersets(ct).unwrap();
                for m_raw in these_metrics {
                    new_metrics.push(m_raw);
                }
                for c_raw in these_actions {
                    if c_raw != "" {
                        new_actions.push(c_raw);
                    }
                }
                // add metrics to metricsets
                for m_raw in new_metrics.clone() {
                    let m = m_raw.split('|').collect::<Vec<_>>();
                    if m.len() > 1 {
                        self.add_metric(ct, m[0].to_string(), m[1].to_string());
                    }
                }
                // Send back the actions -> needed for tests
                sender.send(format!("{:?}|{:?}", ct, new_actions)).unwrap();
            };
            // break mechanism
            if let Some(s) = secs_to_run {
                if ct >= s {
                    println!("Finished execution");
                    break
                }                
            }
        }
    }

}
