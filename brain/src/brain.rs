extern crate regex;
use crate::arduino::Arduino;
use crate::leds::LEDs;
use crate::motors::Motors;
use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use std::io::prelude::*;

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
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ConfigInput {
    pub time: String,
    pub input_objs: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ConfigOutput {
    pub object: String,
    pub value: String,
    pub time: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RulesetEntry {
    id: String,
    condition: Vec<ConfigInput>,
    actionsloop: bool,
    actions: Vec<ConfigOutput>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ConfigEntry {
    id: String,
    condition: Vec<ConfigInput>,
    actionsloop: bool,
    triggercount: u32,
    actions: Vec<ConfigOutput>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ActionEntry {
    id: String,
    actions: Vec<ConfigOutput>
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
    mode: String,
    setup_file: String,
    start_time: u128,
    timestamp: f64,
    rec_file: String,
    config: Vec<ConfigEntry>,
    arduino: Arduino,
    motors: Motors,
    leds: LEDs,
    buffersets: Vec<Set>,
    metricsets: Vec<Set>,
}
static COUNTER: std::sync::atomic::AtomicUsize = AtomicUsize::new(1);
static MAX_BUFFERSIZE: u8 = 25;
static MAX_METRICSIZE: u8 = 25;
const OTHER_ACTIONS: &'static [&'static str] = &["load", "wait"];

impl Brain {
    pub fn new(brain_name: String, mut mode: String, setupfile: String) -> Result<Self, BrainDeadError> {
        // CATCH the record mode and clean up the original mode variable
        let special_mode = mode.split("_").collect::<Vec<_>>();
        let mut record_file = "".to_string();
        if special_mode.len() > 1 {
            match special_mode[1] {
                "record" => record_file = Brain::create_record_file(),
                _ => (),
            }
        }
        mode = special_mode[0].to_string();
        let st = SystemTime::now();
        let start_time = match st.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
        };
        let (first_action_set, _first_arduino_program, inputs, outputs) = match Brain::load_setup(setupfile.to_string()) {
            Ok((fas,fap,i,o)) => (fas,fap,i,o),
            Err(e) => {
                error!("There was an error!: {}", e);
                return Err(BrainDeadError::LoadSetupError)
            }

        };
        // TODO: either call everything config or actionset or actionrules
        let c = match Brain::load_action_rules(first_action_set) {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("There was an error!: {}", e);
                return Err(BrainDeadError::LoadActionRulesError)
            }

        };
        let mut a = Arduino::new("arduino".to_string(), Some("/dev/null".to_string())).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
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
        let mut bs = [].to_vec();
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
        for i in inputs {
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
        for o in outputs {
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
            bs.push(s_b);
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
            current_entry: s_e.clone(),
            max_size: MAX_METRICSIZE,
        };
        bs.push(s_b_o);
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
            mode: mode,
            setup_file: setupfile,
            start_time: start_time,
            timestamp: 0.0,
            rec_file: record_file,
            config: c,
            arduino: a,
            motors: m,
            leds: l,
            buffersets: bs,
            metricsets: ms,
        })
    }

    /// Load a robot setup yaml file and configures the system
    pub fn load_setup(setup_file: String) -> Result<(String, String, HashMap<String, HashMap<String, String>>, HashMap<String, HashMap<String, String>>), BrainDeadError> {
        #[derive(Deserialize)]
        struct Setup {
            start_actionset_file: String,
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
        return Ok((a.start_actionset_file, a.start_arduinohex_file, a.inputs, a.outputs))
        //return (a.start_actionset_file, a.start_arduinohex_file, [].to_vec(), [].to_vec())
    }

    /// Return current timestamp as millis
    pub fn get_current_time(&mut self) -> f64 {
        let now = SystemTime::now();
        let timestamp = match now.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis() as f64,
            Err(_e) => 0.0,
        };
        return timestamp;
    }

    /// Return difference between current timestamp and a given one, in millis
    pub fn get_timestamp_since(&mut self, start_timestamp: f64) -> f64 {
        let now = SystemTime::now();
        let timestamp = match now.duration_since(UNIX_EPOCH) {
            Ok(time) => (time.as_millis() as f64 - start_timestamp as f64) / 1000 as f64,
            Err(_e) => 0.0,
        };
        return timestamp;
    }

    /// Translates a message coming from the Arduino to actions
    pub fn use_arduino_msg(&mut self, timestamp: f64, raw_msg: String) {
        let msg_parts = raw_msg.split(": ").collect::<Vec<_>>();
        if self.rec_file != "".to_string() {
            self.record(timestamp, raw_msg.to_string());
        };
        match msg_parts[0] {
            // TODO: add other use cases
            "SENSOR" => {
                let sensors = msg_parts[1].split("|").collect::<Vec<_>>();
                for s in sensors {
                    let sensor = s.split("=").collect::<Vec<_>>();
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
            },
            _ => (),
        }
    }

    // ########## ACTIONS ########## //

    /// Log action buffers' content
    pub fn show_action_buffers(&mut self) {
        for b in &self.buffersets {
            //if b.object == "motor_r" {
            //    println!("- {} ACTIONS pending for {}", b.entries.len(), b.object);
            //    for (ix, action) in b.entries.clone().iter().enumerate() {
            //        println!(" #{} |data={}|time={}|", ix, action.data, action.time);
            //    }
            //}
            trace!("- {} ACTIONS pending for {}", b.entries.len(), b.object);
            for (ix, action) in b.entries.clone().iter().enumerate() {
                trace!(" #{} |data={}|time={}|", ix, action.data, action.time);
            }
        }
    }

    /// Load actions and rules using the same pattern.
    /// The actions differ from configs in that they will ALWAYS be loaded 
    pub fn load_action_rules(file: String) -> Result<Vec<ConfigEntry>, BrainDeadError> {
        let file_pointer = match File::open(file.clone()){
            Ok(fp) => fp,
            Err(_e) => {
                error!("File {} does not exist", file.clone());
                return Err(BrainDeadError::FileNotFoundError)
            }
        };
        let mut c: Vec<ConfigEntry> = [].to_vec();
        match serde_yaml::from_reader(file_pointer) {
            Ok(v) => return Ok(v),
            Err(e) => {
                if e.to_string().contains("missing field `triggercount`") {

                    let file_pointer = match File::open(file.clone()){
                        Ok(fp) => fp,
                        Err(_e) => {
                            error!("File {} does not exist", file.clone());
                            return Err(BrainDeadError::FileNotFoundError)
                        }
                    };
                    let a: Vec<RulesetEntry> = match serde_yaml::from_reader(file_pointer) {
                        Ok(a) => a,
                        Err(e) => {
                            error!("The file {}'s YAML is incorrect! - {}", file.clone(), e);
                            return Err(BrainDeadError::YamlError)
                        }
                    };
                    for i in a {
                        let c_elem = ConfigEntry {
                            id: i.id,
                            triggercount: 0,
                            condition: i.condition,
                            actionsloop: i.actionsloop,
                            actions: i.actions,
                        };
                        c.push(c_elem);
                    }
                    return Ok(c)
                } else if e.to_string().contains("missing field `condition`") {
                    let file_pointer = match File::open(file.clone()){
                        Ok(fp) => fp,
                        Err(_e) => {
                            error!("File {} does not exist", file.clone());
                            return Err(BrainDeadError::FileNotFoundError)
                        }
                    };
                    let a: Vec<RulesetEntry> = match serde_yaml::from_reader(file_pointer) {
                        Ok(a) => a,
                        Err(e) => {
                            error!("The file {}'s YAML is incorrect for a RuleSet! - {}", file.clone(), e);
                            return Err(BrainDeadError::YamlError)
                        }
                    };
                    for i in a {
                        let c_elem = ConfigEntry {
                            id: i.id,
                            triggercount: 0,
                            condition: [ConfigInput {
                                 time: "*".to_string(),
                                 input_objs: "".to_string(),
                            }].to_vec(),
                            actionsloop: false,
                            actions: i.actions,
                        };
                        c.push(c_elem);
                    }
                    return Ok(c)
                } else {
                    error!("The file {}'s YAML is incorrect! - {}", file, e);
                    return Err(BrainDeadError::YamlError)                    
                }
            },

        };
    }

    /// Turn a String containing an action into the related object
    pub fn get_action_from_string(&mut self, action: String) -> Result<ResultAction, String> {
        // Format would be motor_l=-60,time=2.6,source
        let format = action.split(",").collect::<Vec<_>>();
        //TODO: reproduce this error, then use BrainDeadError instead of unwrap
        let t = format[1].split("=").collect::<Vec<_>>()[1].parse::<f64>().unwrap();
        let data = format[0].split("=").collect::<Vec<_>>();
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
    pub fn are_actions_in_buffer(buffersets: Vec<Set>,rule: ConfigEntry) -> bool {
        let mut result = false;
        for buffer in buffersets {
            for existing in buffer.entries.clone() {
                if existing.belongsto == rule.id {
                    result = true;
                }
            }
            if buffer.current_entry.belongsto == rule.id {
                result = true;
            }
        }
        // NOTE: debug breakpoint
        //println!("{}", result);
        result
    }

    /// Adds action to the related actions buffer
    pub fn add_action(&mut self, action: String) {
        trace!("- Adding action {}", action);
        //TODO: once get_action_from_string has BrainDeadError, add a Result to this function
        let action_to_add = self.clone().get_action_from_string(action).unwrap();
        if OTHER_ACTIONS.iter().any(|&i| i==action_to_add.resource) {
            match self.buffersets.iter_mut().find(|x| *x.object == "other".to_string()) {
                Some(ob) => {
                    if ob.entries.len() >= ob.max_size.into() {
                        warn!("Buffer for {} is full! not adding new actions...", ob.object);
                    } else {
                        match action_to_add.resource.as_str() {
                            "load" => {
                                let a = TimedData {
                                    id: action_to_add.action.id,
                                    belongsto: action_to_add.action.belongsto,
                                    data: format!("{}_{}", action_to_add.resource, action_to_add.action.data),
                                    time: action_to_add.action.time,
                                };
                                ob.entries.push(a);
                            },
                            "wait" => {
                                let a = TimedData {
                                    id: action_to_add.action.id,
                                    belongsto: action_to_add.action.belongsto,
                                    data: format!("{}_{}secs", action_to_add.resource, action_to_add.action.time),
                                    time: action_to_add.action.time,
                                };
                                ob.entries.push(a);
                            },
                            _ => {
                                ob.entries.push(action_to_add.action);
                            },

                        }
                    };
                },
                None => (),
            };
        } else {
            match self.buffersets.iter_mut().find(|x| *x.object == *action_to_add.resource) {
                Some(ob) => {
                    if ob.entries.len() >= ob.max_size.into() {
                        warn!("Buffer for {} is full! not adding new actions...", ob.object);
                    } else {
                        match ob.object.as_str() {
                            "load" => {
                                let a = TimedData {
                                    id: action_to_add.action.id,
                                    belongsto: action_to_add.action.belongsto,
                                    data: format!("{}_{}", action_to_add.resource, action_to_add.action.data),
                                    time: action_to_add.action.time,
                                };
                                ob.entries.push(a);
                            },
                            _ => {
                                ob.entries.push(action_to_add.action);
                            },
                        }
                    };
                },
                None => (),
            };
        }
    }


    /// Performs the next action on each action buffer if the timestamp is right.
    /// Return the action(s) taken and it's related metric
    pub fn do_next_actions(&mut self, timestamp: f64) -> Result<(Vec<String>, Vec<String>), String>{
        let mut result = [].to_vec();
        let mut metrics = [].to_vec();
        //TODO: investigate:
        //     timestamp to decide on actions from the buffer is wrong, what timestamp are we
        //     using?
        //TODO: manage different types of actions
        for ob in self.buffersets.iter_mut() {
            match self.metricsets.iter_mut().find(|x| *x.object == *ob.object) {
                Some(om) => {
                    if timestamp >= om.last_change_timestamp {
                        if ob.entries.len() > 0 {
                            let a = &ob.entries.clone()[0];             
                            let time_passed = ((timestamp - ob.last_change_timestamp) as f64 * 1000 as f64).ceil() / 1000 as f64;
                            trace!("- {} > Time passed on current value - {:?}", om.object, time_passed);
                            if time_passed >= ob.current_entry.time {
                                ob.current_entry = a.clone();
                                ob.entries.retain(|x| *x != *a);
                                ob.last_change_timestamp = timestamp.clone();
                                debug!("- Buffer: {:#x?}", ob.entries);
                                // TODO: Avoid hardcoding this (use types of actions?)
                                if ob.object.starts_with("led") {
                                    // TODO: reproduce this error, then use BrainDeadError instead
                                    // of String in this function
                                    self.leds.set_led(om.object.clone(), a.data.parse::<u8>().unwrap() == 1);
                                } else if ob.object.starts_with("motor") {
                                    let action_vector = a.data.split("_").collect::<Vec<_>>();
                                    self.motors.set(ob.object.clone(), action_vector[0].to_string());
                                } else if ob.object.starts_with("other") {
                                    let other_action = a.data.split("_").collect::<Vec<_>>();
                                    if other_action[0] == "load" {
                                        let file_to_load = other_action[1..].join("_").to_string();
                                        //TODO: once the previous error allows for BrainDeadError,
                                        //use it here too
                                        self.config = Brain::load_action_rules(file_to_load).unwrap();
                                    }
                                }
                                //TODO: this info should come from the leds module itself
                                info!("- Just did {} -> {} (buffer)", om.object, a.data);
                                // TODO actually both the following could be one if we unified format
                                metrics.push(format!("{}__{}|{}", ob.object, a.data, a.id.to_string()));
                                result.push(format!("{}__{}__{:?}", ob.object, a.clone().data, a.clone().time));
                            }
                        }
                    }
                },
                None => (),
            };
        };
        if result.len() == 0 {result.push("".to_string())};
        if metrics.len() == 0 {metrics.push("".to_string())};
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
        match self.metricsets.iter_mut().find(|x| *x.object == *metric_decomp[0]) {
            Some(om) => {
                if om.entries.len() == 0 {
                    let new_m = TimedData {
                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
                        belongsto: source_id,
                        data: metric_decomp[1].to_string(),
                        time: timestamp.clone(), // here time means "since_timestamp"
                    };
                    om.entries.push(new_m);
                    om.last_change_timestamp = timestamp;
                } else {
                    if om.entries[0].data != metric_decomp[1].to_string() {
                        let new_m = TimedData {
                            id: COUNTER.fetch_add(1, Ordering::Relaxed),
                            belongsto: source_id,
                            data: metric_decomp[1].to_string(),
                            time: timestamp.clone(),
                        };
                        om.entries.insert(0, new_m);
                        om.last_change_timestamp = timestamp;
                    }
                }; 
                if om.entries.len() > om.max_size.into() {
                    om.entries.pop();
                };
            },
            None => (),
        };
    }

    pub fn does_condition_match(&mut self, rule: ConfigEntry, timestamp: f64) -> bool {
        let mut result = true;
        let checks = rule.condition[0].input_objs.split(",").collect::<Vec<_>>();
        for check in &checks {
            //TODO: reproduce this error, then get a BrainDeadError
            let re = regex::Regex::new(r"=|<=|>=|<|>").unwrap();
            let keyval = re.split(check).collect::<Vec<_>>();
            match self.metricsets.iter_mut().find(|x| *x.object == *keyval[0]) {
                Some(om) => {
                    //TODO do this differently for each type
                    match om.obj_type.as_str() {
                        // for binary and output we just try to match fully
                        "binary" | "output" => {
                            if om.entries.len() > 0 {
                                if om.entries[0].data != keyval[1] {
                                    result = false;
                                    return result
                                } else {
                                    if (timestamp - om.entries[0].time < rule.condition[0].time.parse::<f64>().unwrap()) && (om.entries[0].time != 0.0){
                                        result = false;
                                        return result
                                    };
                                };
                                // TODO: do we need to check timestamp here?
                            } else if keyval[1] != "0" {
                                result = false;
                                return result
                            }
                        },
                        "continuous" => {
                            let comparison = check.replace(keyval[0], "").replace(keyval[1], "");
                            let mut matched_metrics: Vec<TimedData> = [].to_vec();
                            if om.entries.len() > 0 {
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
                                if matched_metrics.len() > 0 {
                                    let acc_time = matched_metrics.clone().iter().map(|x| x.time).collect::<Vec<_>>().iter().cloned().fold(0./0., f64::min);
                                    if acc_time.to_string() == "NaN"{
                                        result = false;
                                        return result
                                    } else {
                                        if (timestamp - acc_time < rule.condition[0].time.parse::<f64>().unwrap()) && (acc_time != 0.0){
                                            result = false;
                                            return result
                                        }
                                    }
                                } else {
                                    // TODO: do we need to check timestamp here?
                                    result = false;
                                    return result
                                }
                                // put together all metrics that fit comparison
                                // add the timestamps
                                // compare to desired time
                                // ...and if it doesnt fit, remove
                            } else {
                                // TODO: do we need to check timestamp here?
                                result = false;
                                return result
                            }
                        },
                        &_ => {

                        },
                    };
                },
                None => (),
            };
        }
        return result
    }

    pub fn create_record_file() -> String {
        let path = "records";
        //TODO: find a way to reproduce this error, then add BrainDeadError to this function
        fs::create_dir_all(path).unwrap();
        let filename = path.to_owned() + "/last_run.yaml";
        //TODO: use BrainDeadError here when the functions allows for it
        File::create(filename.clone()).unwrap();
        filename.to_string()
    }

    pub fn record(&mut self, timestamp: f64, entry: String) {
        // TODO:
        // modify entry to something we can read as yaml
        //TODO: find a way to reproduce this error, then add BrainDeadError to this function
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.rec_file.clone())
            .unwrap();
        let log = format!("- time: {}\n  msg: \"{}\"", timestamp, entry);
        warn!("We are writing {} to {}", log, self.rec_file);
        //TODO: when the function allows for it, use BrainDeadError here too
        writeln!(&mut file, "{}", log).unwrap();
    }

    /// Just run the brain.
    /// - secs_to_run has to have decimals, so 4.0 is valid, but 4 is not
    /// - precission: how often we do stuff
    ///   - 1 is secs, 10 is decs of a sec, 100 is hundreds of a sec...
    pub fn run(&mut self, secs_to_run: Option<f64>, precission: u16, sender: Sender<String>) {
        // timestamps
        let start_timestamp = self.get_current_time();
        let mut ct: f64 = 0.0;
        // communication with arduino
        let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let msgs = s.clone();
        let mut arduino_clone = self.arduino.clone();
        let brain_clone = self.clone();
        let _handle = thread::spawn(move || {
                if brain_clone.mode != "dryrun" {
                    //TODO: find a way to reproduce this error, then add BrainDeadError to this function
                    arduino_clone.read_channel(msgs).unwrap();
            } else {
                    arduino_clone.read_channel_mock(msgs, brain_clone.setup_file.clone()).unwrap();
                };
            });
        let _msg = match r.try_recv() {
            Ok(m) => {
                self.use_arduino_msg(ct, m);
            },
            Err(_) => (),
        };
        loop {
            let mut new_metrics: Vec<String> = [].to_vec();
            let mut new_acts: Vec<String> = [].to_vec();
            // update current timestamp
            let ct_raw = self.get_timestamp_since(start_timestamp);
            let new_ct = (ct_raw * precission as f64).floor() / precission as f64;
            if new_ct > ct { 
                ct = new_ct;
                let _msg = match r.try_recv() {
                    Ok(m) => {
                        self.use_arduino_msg(ct, m);
                    },
                    Err(_) => (),
                };
                self.show_metrics();
                self.show_action_buffers();
                // GET ACTIONS
                match self.get_actions_from_rules(ct){
                    Ok(acts) => {
                        for objset in acts {
                            if OTHER_ACTIONS.iter().any(|&i| i==objset.object) {
                                match self.buffersets.iter_mut().find(|x| *x.object == "other".to_string()) {
                                    Some(ob) => {
                                        // We assume that if new actions are chosen, we can
                                        // overwrite whatever is on the buffer
                                        ob.entries = Vec::new();
                                    },
                                    None => (),
                                };
                            } else {
                                match self.buffersets.iter_mut().find(|x| *x.object == *objset.object) {
                                    Some(ob) => {
                                        // We assume that if new actions are chosen, we can
                                        // overwrite whatever is on the buffer
                                        ob.entries = Vec::new();
                                    },
                                    None => (),
                                };
                            };
                            for (ix, action) in objset.entries.clone().iter().enumerate() {
                                if ix == 0 {
                                    let (these_metrics, these_acts) = self.do_action(objset.clone(), action.clone(), ct).unwrap();
                                    for m_raw in these_metrics {
                                        new_metrics.push(m_raw);
                                    }
                                    for c_raw in these_acts {
                                        new_acts.push(c_raw);
                                    }
                                } else {
                                    let aux = format!("{}={},time={},{}", objset.object, action.data, action.time, action.id);
                                    self.add_action(aux);
                                }
                            }
                        }
                    },
                    Err(_e) => trace!("...no matching rules found"),
                };
                // DO ACTIONS
                // NOTE: debug entry point
                //println!("---------------------------------------------------- {}", ct);
                //TODO: when the function allows for it, use BrainDeadError here too
                let (these_metrics, these_acts) = self.do_next_actions(ct).unwrap();
                for m_raw in these_metrics {
                    new_metrics.push(m_raw);
                }
                for c_raw in these_acts {
                    if c_raw != "" {
                        new_acts.push(c_raw);
                    }
                }
                for m_raw in new_metrics.clone() {
                    let m = m_raw.split("|").collect::<Vec<_>>();
                    if m.len() > 1 {
                        self.add_metric(ct, m[0].to_string(), m[1].to_string());
                    }
                }
                // Send back the actions -> needed for tests
                //TODO: when the function allows for it, use BrainDeadError here too
                sender.send(format!("{:?}|{:?}", ct, new_acts)).unwrap();
            };
            // BREAK MECHANISM
            match secs_to_run {
                Some(s) => {
                    if ct >= s {
                        println!("Finished execution");
                        break
                    }
                },
                None => (),
            }
        }
    }

    /// Checks the input of the rules loaded and, if they fit, returns the actions to take
    pub fn get_actions_from_rules(&mut self, timestamp: f64) -> Result<Vec<Set>, BrainDeadError>{
        // NOTE: debug entry point
        //println!("---------------------------------------------------- {}", timestamp);
        //TODO try to clean this up
        let mut partial_rules: Vec<ConfigEntry> = self.config.clone();
        let mut action_vectors: Vec<Set> = [].to_vec();
        for bs in self.buffersets.clone() {
            let new_elem = Set {
                object: bs.object,
                obj_type: bs.obj_type,
                entries: [].to_vec(),
                last_change_timestamp: bs.last_change_timestamp,
                current_entry: bs.current_entry,
                max_size: bs.max_size,
            };
            action_vectors.push(new_elem);
        }
        for rule in partial_rules.clone() {
            // NEVER add something that is already on buffer
            if Brain::are_actions_in_buffer(self.buffersets.clone(), rule.clone()) {
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
                    if rule.actionsloop != true {
                        //if ! self.does_condition_match(rule.clone(), timestamp.clone()) {
                        //    partial_rules.retain(|x| *x != rule);
                        //}
                        let checks = rule.condition[0].input_objs.split(",").collect::<Vec<_>>();
                        if checks != [""].to_vec() && checks.len() != 0{
                            if ! self.does_condition_match(rule.clone(), timestamp.clone()) {
                                partial_rules.retain(|x| *x != rule);
                            }
                        } else {
                          partial_rules.retain(|x| *x != rule);
                        }
                    }
                } else {
                    let checks = rule.condition[0].input_objs.split(",").collect::<Vec<_>>();
                    if checks != [""].to_vec() && checks.len() != 0{
                        if ! self.does_condition_match(rule.clone(), timestamp.clone()) {
                            partial_rules.retain(|x| *x != rule);
                        }
                    }
                }
            }
        }
        // We want to count the amount of times the trigger was...triggered
        if partial_rules.len() > 0 {
            for rule in self.config.iter_mut() {
                match partial_rules.clone().iter_mut().find(|x| *x.id == *rule.id) {
                    Some(_) => {
                        rule.triggercount += 1;
                    },
                    //None => rule.triggercount = 0,
                    None => (),
                };
            }
            debug!("- Rules matching :");
            for (ix, rule) in partial_rules.clone().iter().enumerate() {
                debug!(" #{} {} input:", ix, rule.id);
                debug!("     input:");
                for ri in rule.condition.clone() {
                    debug!("      |{:?}|", ri);
                }
                debug!("     output:");
                // we store a vector of buffersets
                for action in rule.actions.clone() {
                    match action_vectors.iter_mut().find(|x| *x.object == *action.object) {
                        Some(bf) => {
                            let entry = TimedData {
                                id: COUNTER.fetch_add(1, Ordering::Relaxed),
                                belongsto: rule.id.clone(),
                                data: action.value,
                                time: action.time.parse::<f64>().unwrap(),
                            };
                            bf.entries.push(entry);
                        },
                        None => (),
                    }
                }
            }
        }
        // Clean up the vector from buffersets that are empty
        for s in action_vectors.clone() {
            if s.entries.len() == 0 {
                action_vectors.retain(|x| *x != s);
            }
        }
        // NOTE: debug entry point
        //println!("--  {}", timestamp);
        //for a in action_vectors.clone() {
        //    println!("{} - {:#x?}", a.object, a.entries);
        //}
        Ok(action_vectors)
    }

    pub fn do_action(&mut self, om: Set, a: TimedData, timestamp: f64) -> Result<(Vec<String>, Vec<String>), String>{
        let mut result = [].to_vec();
        let mut metrics = [].to_vec();
        match self.buffersets.iter_mut().find(|x| *x.object == *om.object) {
            Some(b) => {
                b.last_change_timestamp = timestamp.clone();
            },
            None => ()
        }
        if om.object.starts_with("led") {
            self.leds.set_led(om.object.clone(), a.data.parse::<u8>().unwrap() == 1);
        } else if om.object.starts_with("motor") {
            let action_vector = a.data.split("_").collect::<Vec<_>>();
            self.motors.set(om.object.clone(), action_vector[0].to_string());
        } else if om.object.starts_with("other") {
            let other_action = a.data.split("_").collect::<Vec<_>>();
            if other_action[0] == "load" {
                let file_to_load = other_action[1..].join("_").to_string();
                // TODO: find a way to reproduce this error, then add BrainDeadError to this function
                self.config = Brain::load_action_rules(file_to_load).unwrap();
            }
        }

        // TODO: should this be done on the do_next_action too?
        match self.buffersets.iter_mut().find(|x| *x.object == *om.object) {
            Some(bf) => {
                bf.current_entry = TimedData {
                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
                    belongsto: om.entries[0].clone().belongsto.clone(),
                    data: om.entries[0].clone().data,
                    time: om.entries[0].clone().time,
                };
            },
            None => (),
        }
        //TODO: this info should come from the leds module itself
        //TODO: all printlns should show a proper timestamp but self.timestamp is 0 here
        info!("- Just did {} -> {}", om.object, a.data);
        // TODO actually both the following could be one if we unified format
        metrics.push(format!("{}__{}|{}", om.object, a.data, a.belongsto.to_string()));
        result.push(format!("{}__{}__{:?}", om.object, a.clone().data, a.clone().time));
        return Ok((metrics, result))
    }

}
