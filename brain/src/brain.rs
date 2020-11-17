use crate::arduino::Arduino;
use crate::motors::Motors;
use crate::leds::LEDs;
use log::{debug, error, info, trace, warn};
use std::fs::File;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use std::thread;

#[derive(Error, Debug)]
pub enum BrainDeadError {
    /// This is just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("Config contains no related entries")]
    NoConfigFound,

    #[error("Config contains no related entries")]
    YamlError,

    #[error("Something went wrong while working with timestamps")]
    SystemTimeError,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ConfigInput {
    pub time: String,
    pub input_objs: String, //TODO: use this instead of one entry for each element
    pub led_y: String,
    pub led_r: String,
    pub led_g: String,
    pub led_b: String,
    pub button: String,
    pub motor_l: String,
    pub motor_r: String,
    pub tracker: String,
    pub distance: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ConfigOutput {
    pub object: String,
    pub value: String,
    pub time: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ConfigEntry {
    id: String,
    repeat: bool,
    input: Vec<ConfigInput>,
    output: Vec<ConfigOutput>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ActionEntry {
    id: String,
    output: Vec<ConfigOutput>
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
    entries: Vec<TimedData>,
    last_change_timestamp: f64,
    current_entry: TimedData,
    max_size: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Buffer {
    entries: Vec<TimedData>,
    last_change_timestamp: f64,
    current_entry: TimedData,
    max_size: u8,
}

#[derive(Clone)]
pub struct Brain {
    name: String,
    mode: String,
    start_time: u128,
    timestamp: f64,
    config: Vec<ConfigEntry>,
    arduino: Arduino,
    motors: Motors,
    leds: LEDs,
    buffersets: Vec<Set>,
    metricsets: Vec<Set>,
    buffer_led_y: Buffer,
    metrics_led_y: Buffer,
    buffer_led_r: Buffer,
    metrics_led_r: Buffer,
    buffer_led_g: Buffer,
    metrics_led_g: Buffer,
    buffer_led_b: Buffer,
    metrics_led_b: Buffer,
    metrics_button: Buffer,
    buffer_other: Buffer,
    metrics_other: Buffer,
}
static COUNTER: std::sync::atomic::AtomicUsize = AtomicUsize::new(1);
static MAX_BUFFERSIZE: u8 = 25;
static MAX_METRICSIZE: u8 = 25;

impl Brain {
    pub fn new(brain_name: String, mode: String, setup_file: String) -> Result<Self, String> {
        let st = SystemTime::now();
        let start_time = match st.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
        };
        let (first_action_set, first_arduino_program, inputs, outputs) = Brain::load_setup(setup_file.to_string());
        let c = Brain::load_action_rules(first_action_set).unwrap();
        let mut a = Arduino::new("arduino".to_string(), Some("/dev/null".to_string())).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        if mode.clone() != "dryrun" {
            a = Arduino::new("arduino".to_string(), None).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Arduino: {}", err);
                process::exit(1);
            });
            a.install(&first_arduino_program).unwrap();
        };
        let mut bs = [].to_vec();
        let mut ms = [].to_vec();
        // INPUTS -> they only have metrics buffers
        for i in inputs {
            let s_e = TimedData {
                id: COUNTER.fetch_add(1, Ordering::Relaxed),
                belongsto: "".to_string(),
                data: "0".to_string(),
                time: 0.0,
            };
            let s = Set {
                object: i,
                entries: [].to_vec(),
                last_change_timestamp: 0.0,
                current_entry: s_e,
                max_size: MAX_METRICSIZE,
            };
            ms.push(s);
        }
        // OUTPUTS -> they have actions buffers and metrics buffers
        for o in outputs {
            let s_e = TimedData {
                id: COUNTER.fetch_add(1, Ordering::Relaxed),
                belongsto: "".to_string(),
                data: "0".to_string(),
                time: 0.0,
            };
            let s_b = Set {
                object: o.clone(),
                entries: [].to_vec(),
                last_change_timestamp: 0.0,
                current_entry: s_e.clone(),
                max_size: MAX_BUFFERSIZE,
            };
            bs.push(s_b);
            let s_m = Set {
                object: o.clone(),
                entries: [].to_vec(),
                last_change_timestamp: 0.0,
                current_entry: s_e.clone(),
                max_size: MAX_METRICSIZE,
            };
            ms.push(s_m);
        }
        // OLD
        let m = Motors::new(mode.clone()).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Motors: {}", err);
            process::exit(1);
        });
        let l = LEDs::new(mode.clone()).unwrap_or_else(|err| {
            eprintln!("Problem Initializing LEDs: {}", err);
            process::exit(1);
        });
        // LED YELLOW
        let b_ly_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        let b_ly = Buffer {
            entries: [].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: b_ly_e,
            max_size: 100,
        };
        let m_ly_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        let m_ly = Buffer {
            entries: [m_ly_e.clone()].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: m_ly_e,
            max_size: 80,
        };
        // LED RED
        let b_lr_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        let b_lr = Buffer {
            entries: [].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: b_lr_e,
            max_size: 100,
        };
        let m_lr_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        let m_lr = Buffer {
            entries: [m_lr_e.clone()].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: m_lr_e,
            max_size: 80,
        };
        // LED GREEN
        let b_lg_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        let b_lg = Buffer {
            entries: [].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: b_lg_e,
            max_size: 100,
        };
        let m_lg_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        let m_lg = Buffer {
            entries: [m_lg_e.clone()].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: m_lg_e,
            max_size: 80,
        };
        // LED BLUE
        let b_lb_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        let b_lb = Buffer {
            entries: [].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: b_lb_e,
            max_size: 100,
        };
        let m_lb_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        let m_lb = Buffer {
            entries: [m_lb_e.clone()].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: m_lb_e,
            max_size: 80,
        };
        // BUTTON
        let m_bt_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "0".to_string(),
            time: 0.0,
        };
        let m_bt = Buffer {
            entries: [m_bt_e.clone()].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: m_bt_e,
            max_size: 80,
        };
        // OTHER
        let b_o_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "".to_string(),
            time: 0.0,
        };
        let b_o = Buffer {
            entries: [].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: b_o_e,
            max_size: 10,
        };
        let m_o_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            belongsto: "".to_string(),
            data: "".to_string(),
            time: 0.0,
        };
        let m_o = Buffer {
            entries: [m_o_e.clone()].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: m_o_e,
            max_size: 80,
        };
        Ok(Self {
            name: brain_name,
            mode: mode,
            start_time: start_time,
            timestamp: 0.0,
            config: c,
            arduino: a,
            motors: m,
            leds: l,
            buffersets: bs,
            metricsets: ms,
            buffer_led_y: b_ly,
            metrics_led_y: m_ly,
            buffer_led_r: b_lr,
            metrics_led_r: m_lr,
            buffer_led_g: b_lg,
            metrics_led_g: m_lg,
            buffer_led_b: b_lb,
            metrics_led_b: m_lb,
            metrics_button: m_bt,
            buffer_other: b_o,
            metrics_other: m_o,
        })
    }

    pub fn get_current_time(&mut self) -> f64 {
        let now = SystemTime::now();
        let timestamp = match now.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis() as f64,
            Err(_e) => 0.0,
        };
        return timestamp;
    }

    pub fn get_timestamp_since(&mut self, start_timestamp: f64) -> f64 {
        let now = SystemTime::now();
        let timestamp = match now.duration_since(UNIX_EPOCH) {
            Ok(time) => (time.as_millis() as f64 - start_timestamp as f64) / 1000 as f64,
            Err(_e) => 0.0,
        };
        return timestamp;
    }

    ///// adds metric to the related metrics buffer
    //pub fn add_metric(&mut self, metric: String, source_id: String) {
    //    trace!("- Adding metric {}", metric);
    //    let metric_decomp = metric.split("__").collect::<Vec<_>>();
    //    match metric_decomp[0] {
    //        "led_y" => {
    //            if self.metrics_led_y.entries.len() == 0 {
    //                let new_m = TimedData {
    //                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                    belongsto: source_id,
    //                    data: metric_decomp[1].to_string(),
    //                    time: self.timestamp.clone(), // here time means "since_timestamp"
    //                };
    //                self.metrics_led_y.entries.push(new_m);
    //                self.metrics_led_y.last_change_timestamp = self.timestamp;
    //            } else {
    //                if self.metrics_led_y.entries[0].data != metric_decomp[1].to_string() {
    //                    let new_m = TimedData {
    //                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                        belongsto: source_id,
    //                        data: metric_decomp[1].to_string(),
    //                        time: self.timestamp.clone(),
    //                    };
    //                    self.metrics_led_y.entries.insert(0, new_m);
    //                    self.metrics_led_y.last_change_timestamp = self.timestamp;
    //                }
    //            }; 
    //            if self.metrics_led_y.entries.len() > self.metrics_led_y.max_size.into() {
    //                self.metrics_led_y.entries.pop();
    //            };
    //        },
    //        "led_r" => {
    //            if self.metrics_led_r.entries.len() == 0 {
    //                let new_m = TimedData {
    //                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                    belongsto: source_id,
    //                    data: metric_decomp[1].to_string(),
    //                    time: self.timestamp.clone(), // here time means "since_timestamp"
    //                };
    //                self.metrics_led_r.entries.push(new_m);
    //                self.metrics_led_r.last_change_timestamp = self.timestamp;
    //            } else {
    //                if self.metrics_led_r.entries[0].data != metric_decomp[1].to_string() {
    //                    let new_m = TimedData {
    //                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                        belongsto: source_id,
    //                        data: metric_decomp[1].to_string(),
    //                        time: self.timestamp.clone(),
    //                    };
    //                    self.metrics_led_r.entries.insert(0, new_m);
    //                    self.metrics_led_r.last_change_timestamp = self.timestamp;
    //                }
    //            }; 
    //            if self.metrics_led_r.entries.len() > self.metrics_led_r.max_size.into() {
    //                self.metrics_led_r.entries.pop();
    //            };
    //        },
    //        "led_g" => {
    //            if self.metrics_led_g.entries.len() == 0 {
    //                let new_m = TimedData {
    //                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                    belongsto: source_id,
    //                    data: metric_decomp[1].to_string(),
    //                    time: self.timestamp.clone(), // here time means "since_timestamp"
    //                };
    //                self.metrics_led_g.entries.push(new_m);
    //                self.metrics_led_g.last_change_timestamp = self.timestamp;
    //            } else {
    //                if self.metrics_led_g.entries[0].data != metric_decomp[1].to_string() {
    //                    let new_m = TimedData {
    //                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                        belongsto: source_id,
    //                        data: metric_decomp[1].to_string(),
    //                        time: self.timestamp.clone(),
    //                    };
    //                    self.metrics_led_g.entries.insert(0, new_m);
    //                    self.metrics_led_g.last_change_timestamp = self.timestamp;
    //                }
    //            }; 
    //            if self.metrics_led_g.entries.len() > self.metrics_led_g.max_size.into() {
    //                self.metrics_led_g.entries.pop();
    //            };
    //        },
    //        "led_b" => {
    //            if self.metrics_led_b.entries.len() == 0 {
    //                let new_m = TimedData {
    //                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                    belongsto: source_id,
    //                    data: metric_decomp[1].to_string(),
    //                    time: self.timestamp.clone(), // here time means "since_timestamp"
    //                };
    //                self.metrics_led_b.entries.push(new_m);
    //                self.metrics_led_b.last_change_timestamp = self.timestamp;
    //            } else {
    //                if self.metrics_led_b.entries[0].data != metric_decomp[1].to_string() {
    //                    let new_m = TimedData {
    //                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                        belongsto: source_id,
    //                        data: metric_decomp[1].to_string(),
    //                        time: self.timestamp.clone(),
    //                    };
    //                    self.metrics_led_b.entries.insert(0, new_m);
    //                    self.metrics_led_b.last_change_timestamp = self.timestamp;
    //                }
    //            }; 
    //            if self.metrics_led_b.entries.len() > self.metrics_led_b.max_size.into() {
    //                self.metrics_led_b.entries.pop();
    //            };
    //        },
    //        "button" => {
    //            if self.metrics_button.entries.len() == 0 {
    //                let new_m = TimedData {
    //                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                    belongsto: source_id,
    //                    data: metric_decomp[1].to_string(),
    //                    time: self.timestamp.clone(), // here time means "since_timestamp"
    //                };
    //                self.metrics_button.entries.push(new_m);
    //                self.metrics_button.last_change_timestamp = self.timestamp;
    //            } else {
    //                if self.metrics_button.entries[0].data != metric_decomp[1].to_string() {
    //                    let new_m = TimedData {
    //                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
    //                        belongsto: source_id,
    //                        data: metric_decomp[1].to_string(),
    //                        time: self.timestamp.clone(),
    //                    };
    //                    self.metrics_button.entries.insert(0, new_m);
    //                    self.metrics_button.last_change_timestamp = self.timestamp;
    //                }
    //            }; 
    //            if self.metrics_button.entries.len() > self.metrics_button.max_size.into() {
    //                self.metrics_button.entries.pop();
    //            };
    //        },
    //        _ => (),
    //    }
    //}

    ///// Adds action to the related actions buffer
    //pub fn add_action(&mut self, action: String) {
    //    trace!("- Adding action {}", action);
    //    let action_to_add = self.clone().get_action_from_string(action).unwrap();
    //    match action_to_add.resource.as_str() {
    //        "led_y" => {
    //            if self.buffer_led_y.entries.len() >= self.buffer_led_y.max_size.into() {
    //                warn!("Buffer for LED_Y is full! not adding new actions...");
    //            } else {
    //                self.buffer_led_y.entries.push(action_to_add.action);
    //            };
    //        },
    //        "led_r" => {
    //            if self.buffer_led_r.entries.len() >= self.buffer_led_r.max_size.into() {
    //                warn!("Buffer for LED_R is full! not adding new actions...");
    //            } else {
    //                self.buffer_led_r.entries.push(action_to_add.action);
    //            };
    //        },
    //        "led_g" => {
    //            if self.buffer_led_g.entries.len() >= self.buffer_led_g.max_size.into() {
    //                warn!("Buffer for LED_G is full! not adding new actions...");
    //            } else {
    //                self.buffer_led_g.entries.push(action_to_add.action);
    //            };
    //        },
    //        "led_b" => {
    //            if self.buffer_led_b.entries.len() >= self.buffer_led_b.max_size.into() {
    //                warn!("Buffer for LED_B is full! not adding new actions...");
    //            } else {
    //                self.buffer_led_b.entries.push(action_to_add.action);
    //            };
    //        },
    //        "wait" => {
    //            if self.buffer_other.entries.len() >= self.buffer_other.max_size.into() {
    //                warn!("Buffer for OTHER ACTIONS is full! not adding new actions...");
    //            } else {
    //                self.buffer_other.entries.push(action_to_add.action);
    //            };
    //        },
    //        "load" => {
    //            if self.buffer_other.entries.len() >= self.buffer_other.max_size.into() {
    //                warn!("Buffer for OTHER ACTIONS is full! not adding new actions...");
    //            } else {
    //                let a = TimedData {
    //                    id: action_to_add.action.id,
    //                    belongsto: action_to_add.action.belongsto,
    //                    data: format!("{}_{}", action_to_add.resource, action_to_add.action.data),
    //                    time: action_to_add.action.time,
    //                };
    //                self.buffer_other.entries.push(a);
    //            };
    //        },
    //        _ => ()
    //    }
    //}

    ///// Goes through all rules loaded from config, checks if they match what is currently on our
    ///// metrics, and iif so, it returns them
    //pub fn get_actions_from_rules(&mut self, timestamp: f64) -> Result<Vec<ConfigEntry>, BrainDeadError>{
    //    // Start with led_y
    //    let mut partial_rules: Vec<ConfigEntry> = [].to_vec();
    //    for rule in self.config.clone() {
    //        //if rule.id.split("_").collect::<Vec<_>>()[0] == "do-once" {
    //        if rule.repeat == false {
    //             partial_rules.push(rule.clone());
    //        } else if rule.id.split("_").collect::<Vec<_>>()[0] != "done" {
    //            if self.metrics_led_y.entries.len() > 0 {
    //                if rule.input[0].led_y != "*" {
    //                    if self.metrics_led_y.entries[0].data == rule.input[0].led_y {
    //                        if timestamp - self.metrics_led_y.entries[0].time >= rule.input[0].time.parse::<f64>().unwrap(){
    //                            if ! self.are_actions_in_buffer(rule.clone()) {
    //                                partial_rules.push(rule.clone());
    //                            }
    //                        };
    //                    };
    //                } else {
    //                    partial_rules.push(rule.clone());
    //                };
    //            };
    //        };
    //    };
    //    // Then remove those that dont fit 
    //    for rule in partial_rules.clone() {
    //        if self.metrics_led_r.entries.len() > 0 {
    //            if rule.input[0].led_r != "*" {
    //                if self.metrics_led_r.entries[0].data != rule.input[0].led_r {
    //                    partial_rules.retain(|x| *x != rule);
    //                } else {
    //                    if (timestamp - self.metrics_led_r.entries[0].time < rule.input[0].time.parse::<f64>().unwrap()) && (self.metrics_led_y.entries[0].time != 0.0){
    //                        partial_rules.retain(|x| *x != rule);
    //                    };
    //                };
    //            };
    //        };
    //        if self.metrics_led_g.entries.len() > 0 {
    //            if rule.input[0].led_g != "*" {
    //                if self.metrics_led_g.entries[0].data != rule.input[0].led_g {
    //                    partial_rules.retain(|x| *x != rule);
    //                } else {
    //                    if (timestamp - self.metrics_led_g.entries[0].time < rule.input[0].time.parse::<f64>().unwrap()) && (self.metrics_led_y.entries[0].time != 0.0){
    //                        partial_rules.retain(|x| *x != rule);
    //                    };
    //                };
    //            };
    //        };
    //        if self.metrics_led_b.entries.len() > 0 {
    //            if rule.input[0].led_b != "*" {
    //                if self.metrics_led_b.entries[0].data != rule.input[0].led_b {
    //                    partial_rules.retain(|x| *x != rule);
    //                } else {
    //                    if (timestamp - self.metrics_led_b.entries[0].time < rule.input[0].time.parse::<f64>().unwrap()) && (self.metrics_led_y.entries[0].time != 0.0){
    //                        partial_rules.retain(|x| *x != rule);
    //                    };
    //                };
    //            };
    //        };
    //        if self.metrics_button.entries.len() > 0 {
    //            if rule.input[0].button != "*" {
    //                if self.metrics_button.entries[0].data != rule.input[0].button {
    //                    partial_rules.retain(|x| *x != rule);
    //                } else {
    //                    if (timestamp - self.metrics_button.entries[0].time < rule.input[0].time.parse::<f64>().unwrap()) && (self.metrics_led_y.entries[0].time != 0.0){
    //                        partial_rules.retain(|x| *x != rule);
    //                    };
    //                };
    //            };
    //        };
    //    };
    //    if partial_rules.len() > 0 {
    //        debug!("- Rules matching :");
    //        for (ix, rule) in partial_rules.clone().iter().enumerate() {
    //            debug!(" #{} input:", ix);
    //            for ri in rule.input.clone() {
    //                debug!("      |{:?}|", ri);
    //            }
    //            debug!("     output:");
    //            for ro in rule.output.clone() {
    //                debug!("      |{:?}|", ro);
    //            }
    //        }
    //    }
    //    Ok(partial_rules)
    //}

    ///// Checks the time passed for the current action and, when it goes over the time set, 
    ///// it "moves" to the next one
    //pub fn do_next_actions(&mut self, timestamp: f64) -> Result<Vec<String>, String>{
    //    let mut result = [].to_vec();
    //    if timestamp >= self.metrics_led_y.last_change_timestamp {
    //        if self.buffer_led_y.entries.len() > 0 {
    //            let a = &self.buffer_led_y.entries.clone()[0];
    //            let time_passed = ((timestamp - self.buffer_led_y.last_change_timestamp) as f64 * 1000 as f64).ceil() / 1000 as f64;
    //            trace!("- Time passed on current value - {:?}", time_passed);
    //            if time_passed >= self.buffer_led_y.current_entry.time {
    //                self.buffer_led_y.current_entry = a.clone();
    //                self.buffer_led_y.entries.retain(|x| *x != *a);
    //                self.buffer_led_y.last_change_timestamp = timestamp.clone();
    //                debug!("- Buffer: {:#x?}", self.buffer_led_y.entries);
    //                info!("- Just did LED_Y -> {}", a.data);
    //                self.leds.set_led_y(a.data.parse::<u8>().unwrap() == 1);
    //                self.add_metric(format!("led_y__{}", a.data), a.id.to_string());
    //                self.new_add_metric(format!("led_y__{}", a.data), a.id.to_string());
    //                result.push(format!("led_y__{}__{:?}", a.clone().data, a.clone().time));
    //            }
    //        }
    //    };
    //    if timestamp >= self.metrics_led_r.last_change_timestamp {
    //        if self.buffer_led_r.entries.len() > 0 {
    //            let a = &self.buffer_led_r.entries.clone()[0];
    //            let time_passed = ((timestamp - self.buffer_led_r.last_change_timestamp) as f64 * 1000 as f64).ceil() / 1000 as f64;
    //            trace!("- Time passed on current value - {:?}", time_passed);
    //            if time_passed >= self.buffer_led_r.current_entry.time {
    //                self.buffer_led_r.current_entry = a.clone();
    //                self.buffer_led_r.entries.retain(|x| *x != *a);
    //                self.buffer_led_r.last_change_timestamp = timestamp.clone();
    //                debug!("- Buffer: {:#x?}", self.buffer_led_r.entries);
    //                info!("- Just did LED_R -> {}", a.data);
    //                self.leds.set_led_r(a.data.parse::<u8>().unwrap() == 1);
    //                self.add_metric(format!("led_r__{}", a.data), a.id.to_string());
    //                self.new_add_metric(format!("led_r__{}", a.data), a.id.to_string());
    //                result.push(format!("led_r__{}__{:?}", a.clone().data, a.clone().time));
    //            }
    //        }
    //    };
    //    if timestamp >= self.metrics_led_g.last_change_timestamp {
    //        if self.buffer_led_g.entries.len() > 0 {
    //            let a = &self.buffer_led_g.entries.clone()[0];
    //            let time_passed = ((timestamp - self.buffer_led_g.last_change_timestamp) as f64 * 1000 as f64).ceil() / 1000 as f64;
    //            trace!("- Time passed on current value - {:?}", time_passed);
    //            if time_passed >= self.buffer_led_g.current_entry.time {
    //                self.buffer_led_g.current_entry = a.clone();
    //                self.buffer_led_g.entries.retain(|x| *x != *a);
    //                self.buffer_led_g.last_change_timestamp = timestamp.clone();
    //                debug!("- Buffer: {:#x?}", self.buffer_led_g.entries);
    //                info!("- Just did LED_G -> {}", a.data);
    //                self.leds.set_led_g(a.data.parse::<u8>().unwrap() == 1);
    //                self.add_metric(format!("led_g__{}", a.data), a.id.to_string());
    //                self.new_add_metric(format!("led_g__{}", a.data), a.id.to_string());
    //                result.push(format!("led_g__{}__{:?}", a.clone().data, a.clone().time));
    //            }
    //        }
    //    };
    //    if timestamp >= self.metrics_led_b.last_change_timestamp {
    //        if self.buffer_led_b.entries.len() > 0 {
    //            let a = &self.buffer_led_b.entries.clone()[0];
    //            let time_passed = ((timestamp - self.buffer_led_b.last_change_timestamp) as f64 * 1000 as f64).ceil() / 1000 as f64;
    //            trace!("- Time passed on current value - {:?}", time_passed);
    //            if time_passed >= self.buffer_led_b.current_entry.time {
    //                self.buffer_led_b.current_entry = a.clone();
    //                self.buffer_led_b.entries.retain(|x| *x != *a);
    //                self.buffer_led_b.last_change_timestamp = timestamp.clone();
    //                debug!("- Buffer: {:#x?}", self.buffer_led_b.entries);
    //                info!("- Just did LED_B -> {}", a.data);
    //                self.leds.set_led_b(a.data.parse::<u8>().unwrap() == 1);
    //                self.add_metric(format!("led_b__{}", a.data), a.id.to_string());
    //                self.new_add_metric(format!("led_b__{}", a.data), a.id.to_string());
    //                result.push(format!("led_b__{}__{:?}", a.clone().data, a.clone().time));
    //            }
    //        }
    //    };
    //    if timestamp >= self.metrics_other.last_change_timestamp {
    //        if self.buffer_other.entries.len() > 0 {
    //            let a = &self.buffer_other.entries.clone()[0];
    //            let time_passed = ((timestamp - self.buffer_other.last_change_timestamp) as f64 * 1000 as f64).ceil() / 1000 as f64;
    //            trace!("- Time passed on current value - {:?}", time_passed);
    //            if time_passed >= self.buffer_other.current_entry.time {
    //                self.buffer_other.current_entry = a.clone();
    //                self.buffer_other.entries.retain(|x| *x != *a);
    //                self.buffer_other.last_change_timestamp = timestamp.clone();
    //                debug!("- Buffer: {:#x?}", self.buffer_led_y.entries);
    //                info!("- Just did OTHER -> {}", a.data);
    //                if a.data.split("_").collect::<Vec<_>>()[0] == "load" {
    //                    self.config = Brain::load_action_rules(format!("./actions/{}", a.data.split("_").collect::<Vec<_>>()[1])).unwrap();
    //                    self.empty_buffers();
    //                };
    //                self.add_metric(format!("other__{}", a.data), a.id.to_string());
    //                self.new_add_metric(format!("other__{}", a.data), a.id.to_string());
    //                result.push(format!("other__{}__{:?}", a.clone().data, a.clone().time));
    //            }
    //        }
    //    };
    //    if result.len() == 0 {result.push("".to_string())};
    //    Ok(result)
    //}

    /// turns a String containing an action into the related object
    pub fn get_action_from_string(&mut self, action: String) -> Result<ResultAction, String> {
        // Format would be motor_l=-60,time=2.6,source
        let format = action.split(",").collect::<Vec<_>>();
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

    pub fn show_buffers(&mut self) {
        debug!("{} pending for LED_Y", self.buffer_led_y.entries.len());
        debug!("{} pending for LED_R", self.buffer_led_r.entries.len());
        debug!("{} pending for LED_G", self.buffer_led_g.entries.len());
        debug!("{} pending for LED_B", self.buffer_led_b.entries.len());
        debug!("{} pending for OTHER", self.buffer_other.entries.len());
        trace!("- Actions buffer - LED Y:");
        trace!("  {:?}", self.buffer_led_y.entries);
        trace!("- Actions buffer - LED R:");
        trace!("  {:?}", self.buffer_led_r.entries);
        trace!("- Actions buffer - LED G:");
        trace!("  {:?}", self.buffer_led_g.entries);
        trace!("- Actions buffer - LED B:");
        trace!("  {:?}", self.buffer_led_b.entries);
        trace!("- Actions buffer - OTHER:");
        trace!("  {:?}", self.buffer_other.entries);
        for b in &self.buffersets {
            info!("{} pending for {}", b.entries.len(), b.object);
            trace!("- Actions buffer - {}:", b.object);
            for (ix, action) in b.entries.clone().iter().enumerate() {
                trace!(" #{} |data={}|time={}|", ix, action.data, action.time);
            }
        }
    }

    pub fn show_metrics(&mut self) {
        debug!("- Metrics - LED Y:");
        for (ix, action) in self.metrics_led_y.entries.clone().iter().enumerate() {
            debug!(" #{} |data={}|time={}|", ix, action.data, action.time);
        }
        debug!("- Metrics - LED R:");
        for (ix, action) in self.metrics_led_r.entries.clone().iter().enumerate() {
            debug!(" #{} |data={}|time={}|", ix, action.data, action.time);
        }
        debug!("- Metrics - LED G:");
        for (ix, action) in self.metrics_led_g.entries.clone().iter().enumerate() {
            debug!(" #{} |data={}|time={}|", ix, action.data, action.time);
        }
        debug!("- Metrics - LED B:");
        for (ix, action) in self.metrics_led_b.entries.clone().iter().enumerate() {
            debug!(" #{} |data={}|time={}|", ix, action.data, action.time);
        }
        debug!("- Metrics - BUTTON:");
        for (ix, action) in self.metrics_button.entries.clone().iter().enumerate() {
            debug!(" #{} |data={}|time={}|", ix, action.data, action.time);
        }
        // TODO: remove the above and use only these
        for m in self.metricsets.clone().iter() {
            debug!("- Metrics - {}", m.object);
            for (ix, action) in m.entries.clone().iter().enumerate() {
                debug!(" #{} |data={}|time={}|", ix, action.data, action.time);
            }
        }
    }

    /// The whole point of this function is being able to load actions and configs through the same
    /// pattern. The actions differ from configs in that they will ALWAYS be loaded (but maybe only
    /// once).
    /// We give the id a special value to use later.
    pub fn load_action_rules(file: String) -> Result<Vec<ConfigEntry>, BrainDeadError> {
        let file_pointer = File::open(file.clone()).unwrap();
        let mut c: Vec<ConfigEntry> = [].to_vec();
        match serde_yaml::from_reader(file_pointer) {
            Ok(v) => return Ok(v),
            Err(e) => {
                if e.to_string().contains("missing field `input`") {
                    let file_pointer = File::open(file.clone()).unwrap();
                    let a: Vec<ActionEntry> = serde_yaml::from_reader(file_pointer).unwrap();
                    for i in a {
                        let c_elem = ConfigEntry {
                            //id: format!("do-once_{}", i.id),
                            id: i.id,
                            repeat: false,
                            input: [ConfigInput {
                                 time: "*".to_string(),
                                 input_objs: "".to_string(),
                                 led_y: "*".to_string(),
                                 led_r: "*".to_string(),
                                 led_g: "*".to_string(),
                                 led_b: "*".to_string(),
                                 button: "*".to_string(),
                                 motor_l: "*".to_string(),
                                 motor_r: "*".to_string(),
                                 tracker: "*".to_string(),
                                 distance: "*".to_string(),
                            }].to_vec(),
                            output: i.output,
                        };
                        c.push(c_elem);
                    }
                    return Ok(c)
                } else {
                    error!("The file {} is incorrect! - {}", file, e);
                    return Err(BrainDeadError::YamlError)                    
                }
            },

        };
    }

    /// Returns whether a set of actions are already on the buffer, 
    /// to avoid constantly adding the same ones
    pub fn are_actions_in_buffer(&self, rule: ConfigEntry) -> bool {
        let mut result = false;
        for existing in self.buffer_led_y.entries.clone() {
            if existing.belongsto == rule.id {
                result = true;
            }
        }
        for existing in self.buffer_led_r.entries.clone() {
            if existing.belongsto == rule.id {
                result = true;
            }
        }
        for existing in self.buffer_led_g.entries.clone() {
            if existing.belongsto == rule.id {
                result = true;
            }
        }
        for existing in self.buffer_led_b.entries.clone() {
            if existing.belongsto == rule.id {
                result = true;
            }
        }
        for existing in self.buffer_other.entries.clone() {
            if existing.belongsto == rule.id {
                result = true;
            }
        }
        result
    }

    pub fn empty_buffers(&mut self) {
        self.buffer_led_y.entries = Vec::new();
        self.buffer_led_r.entries = Vec::new();
        self.buffer_led_g.entries = Vec::new();
        self.buffer_led_b.entries = Vec::new();
        self.buffer_other.entries = Vec::new();
    }

    pub fn load_setup(setup_file: String) -> (String, String, Vec<String>, Vec<String>) {
        #[derive(Deserialize)]
        struct Setup {
            start_actionset_file: String,
            start_arduinohex_file: String,
            inputs: Vec<String>,
            outputs: Vec<String>,
        }
        let file_pointer = File::open(setup_file).unwrap();
        let a: Setup = serde_yaml::from_reader(file_pointer).unwrap();
        return (a.start_actionset_file, a.start_arduinohex_file, a.inputs, a.outputs)
    }

    pub fn use_arduino_msg(&mut self, raw_msg: String) {
        let msg_parts = raw_msg.split(": ").collect::<Vec<_>>();
        match msg_parts[0] {
            "SENSOR" => {
                let sensor = msg_parts[1].split("=").collect::<Vec<_>>();
                let sensor_id = "arduino".to_string();
                if sensor.len() > 1 {
                    //self.add_metric(format!("{}__{}", sensor[0], sensor[1]), sensor_id.clone());
                    self.new_add_metric(format!("{}__{}", sensor[0], sensor[1]), sensor_id);
                } else {
                    trace!("{:?}", sensor);
                }

            },
            _ => (),
        }
    }

    ///
    /// - secs_to_run has to have decimals, so 4.0 is valid, but 4 is not
    /// - precission: how often we do stuff
    ///   - 1 is secs, 10 is decs of a sec, 100 is hundreds of a sec...
    pub fn run(&mut self, secs_to_run: Option<f64>, precission: u16, sender: Sender<String>) {
        let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let start_timestamp = self.get_current_time();
        let mut ct: f64 = 0.0;

        let msgs = s.clone();
        let mut arduino_clone = self.arduino.clone();
        let brain_clone = self.clone();
        let _handle = thread::spawn(move || {
                if brain_clone.mode != "dryrun" {
                    arduino_clone.read_channel(msgs).unwrap();
            } else {
                    arduino_clone.read_channel_mock(msgs).unwrap();
                };
            });
        let _msg = match r.try_recv() {
            Ok(m) => {
                self.use_arduino_msg(m);
            },
            Err(_) => (),
        };
        loop {
            let ct_raw = self.get_timestamp_since(start_timestamp);
            // CONTROL RUNNING
            let new_ct = (ct_raw * precission as f64).floor() / precission as f64;
            if new_ct > ct { 
                ct = new_ct;
                // GET MESSAGES AND UPDATE METRICS
                //let msgs = s.clone();
                //let mut arduino_clone = self.arduino.clone();
                //let brain_clone = self.clone();
                //let _handle = thread::spawn(move || {
                //        if brain_clone.mode != "dryrun" {
                //            arduino_clone.read_channel(msgs).unwrap();
                //    } else {
                //            arduino_clone.read_channel_mock(msgs).unwrap();
                //        };
                //    });
                let _msg = match r.try_recv() {
                    Ok(m) => {
                        self.use_arduino_msg(m);
                    },
                    Err(_) => (),
                };
                self.show_metrics();
                self.show_buffers();
                // GET ACTIONS
                //match self.get_actions_from_rules(ct){
                match self.new_get_actions_from_rules(ct){
                    Ok(a) => {
                        if a.len() > 0 {
                            // Format would be motor_l=-60,time=2.6
                            // first the actions marked as do_once will be marked as done at the
                            // config
                            for mut rule in self.config.iter_mut() {
                                if rule.repeat == false {
                                //if rule.id.split("_").collect::<Vec<_>>()[0] == "do-once" {
                                    for action in a.clone() {
                                        if (action.id == rule.id) && (!action.id.starts_with("done_")) {
                                            //rule.id = action.id.to_string().replace("do-once", "done");
                                            
                                            rule.id = "done_".to_owned();
                                            rule.id.push_str(&action.id.to_string());
                                            debug!("Action '{}' will only be done once", action.id);
                                        }
                                    }
                                }
                            }
                            // then a round to check which objects we are adding new actions to
                            for action in a.clone() {
                                for o in action.output {
                                    match self.buffersets.iter_mut().find(|x| *x.object == *o.object) {
                                        Some(ob) => {
                                            ob.entries = Vec::new();
                                        },
                                        None => (),
                                    };
                                    //match o.object.as_str() {
                                    //    "led_y" => self.buffer_led_y.entries = Vec::new(),
                                    //    "led_r" => self.buffer_led_r.entries = Vec::new(),
                                    //    "led_g" => self.buffer_led_g.entries = Vec::new(),
                                    //    "led_b" => self.buffer_led_b.entries = Vec::new(),
                                    //    _ => (),
                                    //}
                                }
                            }
                            // then do the actions
                            for action in a {
                                for o in action.output {
                                    let aux = format!("{}={},time={},{}", o.object, o.value, o.time, action.id);
                                    self.new_add_action(aux);
                                }
                            }
                        };
                    },
                    Err(_e) => trace!("...no matching rules found"),
                };
                // DO ACTIONS
                //let acts = self.do_next_actions(ct).unwrap();
                let (new_metrics, acts) = self.new_do_next_actions(ct).unwrap();
                for m_raw in new_metrics {
                    let m = m_raw.split("|").collect::<Vec<_>>();
                    if m.len() > 1 {
                        self.new_add_metric(m[0].to_string(), m[1].to_string());
                    }
                }
                sender.send(format!("{:?}|{:?}", ct, acts)).unwrap();
            };
            // BREAK MECHANISM
            match secs_to_run {
                Some(s) => {
                    if ct >= s {break}
                },
                None => (),
            }
        }
    }

    pub fn new_get_actions_from_rules(&mut self, timestamp: f64) -> Result<Vec<ConfigEntry>, BrainDeadError>{
        // Start with led_y
        let mut partial_rules: Vec<ConfigEntry> = self.config.clone();
        for rule in self.config.clone() {
            let checks = rule.input[0].input_objs.split(",").collect::<Vec<_>>();
            for check in &checks {
                let keyval = check.split("=").collect::<Vec<_>>();
                match self.metricsets.iter_mut().find(|x| *x.object == *keyval[0]) {
                    Some(om) => {
                        if om.entries.len() > 0 {
                            if om.entries[0].data != keyval[1] {
                                partial_rules.retain(|x| *x != rule);
                            } else {
                                if (timestamp - om.entries[0].time < rule.input[0].time.parse::<f64>().unwrap()) && (om.entries[0].time != 0.0){
                                    partial_rules.retain(|x| *x != rule);
                                };
                            };

                        }
                    },
                    None => (),
                };
            }
        };
        if partial_rules.len() > 0 {
            debug!("- Rules matching :");
            for (ix, rule) in partial_rules.clone().iter().enumerate() {
                debug!(" #{} input:", ix);
                for ri in rule.input.clone() {
                    debug!("      |{:?}|", ri);
                }
                debug!("     output:");
                for ro in rule.output.clone() {
                    debug!("      |{:?}|", ro);
                }
            }
        }
        Ok(partial_rules)
    }

    pub fn new_add_metric(&mut self, metric: String, source_id: String) {
        trace!("- Adding metric {}", metric);
        let metric_decomp = metric.split("__").collect::<Vec<_>>();
        match self.metricsets.iter_mut().find(|x| *x.object == *metric_decomp[0]) {
            Some(om) => {
                if om.entries.len() == 0 {
                    let new_m = TimedData {
                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
                        belongsto: source_id,
                        data: metric_decomp[1].to_string(),
                        time: self.timestamp.clone(), // here time means "since_timestamp"
                    };
                    om.entries.push(new_m);
                    om.last_change_timestamp = self.timestamp;
                } else {
                    if om.entries[0].data != metric_decomp[1].to_string() {
                        let new_m = TimedData {
                            id: COUNTER.fetch_add(1, Ordering::Relaxed),
                            belongsto: source_id,
                            data: metric_decomp[1].to_string(),
                            time: self.timestamp.clone(),
                        };
                        om.entries.insert(0, new_m);
                        om.last_change_timestamp = self.timestamp;
                    }
                }; 
                if om.entries.len() > om.max_size.into() {
                    om.entries.pop();
                };
            },
            None => (),
        };
    }

    pub fn new_add_action(&mut self, action: String) {
        trace!("- Adding action {}", action);
        let action_to_add = self.clone().get_action_from_string(action).unwrap();
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

    pub fn new_do_next_actions(&mut self, timestamp: f64) -> Result<(Vec<String>, Vec<String>), String>{
        let mut result = [].to_vec();
        let mut metrics = [].to_vec();
        //TODO: manage different types of actions
        for ob in self.buffersets.iter_mut() {
            match self.metricsets.iter_mut().find(|x| *x.object == *ob.object) {
                Some(om) => {
                    if timestamp >= om.last_change_timestamp {
                        if ob.entries.len() > 0 {
                            let a = &ob.entries.clone()[0];             
                            let time_passed = ((timestamp - ob.last_change_timestamp) as f64 * 1000 as f64).ceil() / 1000 as f64;
                            trace!("- Time passed on current value - {:?}", time_passed);
                            if time_passed >= ob.current_entry.time {
                                ob.current_entry = a.clone();
                                ob.entries.retain(|x| *x != *a);
                                ob.last_change_timestamp = timestamp.clone();
                                debug!("- Buffer: {:#x?}", ob.entries);
                                info!("- Just did {} -> {}", om.object, a.data);
                                // TODO: correct LEDS, maybe create something
                                //self.leds.set_led_y(a.data.parse::<u8>().unwrap() == 1);
                                //self.new_add_metric(format!("led_y__{}", a.data), a.id.to_string());
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
}
