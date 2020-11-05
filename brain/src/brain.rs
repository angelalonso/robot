use crate::arduino::Arduino;
use crate::motors::Motors;
use crate::leds::LEDs;
use log::{debug, error, info, trace, warn};
use std::fs::File;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainDeadError {
    /// This is just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("Config contains no related entries")]
    NoConfigFound,

    #[error("Something went wrong while working with timestamps")]
    SystemTimeError,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigInput {
    pub time: String,
    pub led_y: String,
    pub led_r: String,
    pub motor_l: String,
    pub motor_r: String,
    pub tracker: String,
    pub distance: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigOutput {
    pub object: String,
    pub value: String,
    pub time: String,
    pub repeat: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigEntry {
    input: Vec<ConfigInput>,
    output: Vec<ConfigOutput>
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimedData {
    id: usize,
    data: String,
    time: f64,
}

#[derive(Clone, Debug)]
pub struct ResultAction {
    resource: String,
    action: TimedData,
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
    buffer_led_y: Buffer,
    metrics_led_y: Buffer,
    buffer_led_r: Buffer,
    metrics_led_r: Buffer,
    //buffer_led_g: Buffer,
    //metrics_led_g: Buffer,
    //buffer_led_b: Buffer,
    //metrics_led_b: Buffer,
}
static COUNTER: std::sync::atomic::AtomicUsize = AtomicUsize::new(1);

impl Brain {
    pub fn new(brain_name: String, mode: String, config_file: String) -> Result<Self, String> {
        let st = SystemTime::now();
        let start_time = match st.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
        };
        let cfg_file_pointer = File::open(config_file).unwrap();
        let c: Vec<ConfigEntry> = serde_yaml::from_reader(cfg_file_pointer).unwrap();
        let mut a = Arduino::new("arduino".to_string(), Some("/dev/null".to_string())).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        if mode.clone() != "dryrun" {
            a = Arduino::new("arduino".to_string(), None).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Arduino: {}", err);
                process::exit(1);
            });
        };
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
            data: "0".to_string(),
            time: 0.0,
        };
        let b_ly = Buffer {
            entries: [].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: b_ly_e,
            max_size: 10,
        };
        let m_ly_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            data: "0".to_string(),
            time: 0.0,
        };
        let m_ly = Buffer {
            entries: [m_ly_e.clone()].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: m_ly_e,
            max_size: 8,
        };
        // LED RED
        let b_lr_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            data: "0".to_string(),
            time: 0.0,
        };
        let b_lr = Buffer {
            entries: [].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: b_lr_e,
            max_size: 10,
        };
        let m_lr_e = TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            data: "0".to_string(),
            time: 0.0,
        };
        let m_lr = Buffer {
            entries: [m_lr_e.clone()].to_vec(),
            last_change_timestamp: 0.0,
            current_entry: m_lr_e,
            max_size: 8,
        };
        //// LED GREEN
        //let b_lg_e = TimedData {
        //    id: COUNTER.fetch_add(1, Ordering::Relaxed),
        //    data: "0".to_string(),
        //    time: 0.0,
        //};
        //let b_lg = Buffer {
        //    entries: [].to_vec(),
        //    last_change_timestamp: 0.0,
        //    current_entry: b_lg_e,
        //    max_size: 10,
        //};
        //let m_lg_e = TimedData {
        //    id: COUNTER.fetch_add(1, Ordering::Relaxed),
        //    data: "0".to_string(),
        //    time: 0.0,
        //};
        //let m_lg = Buffer {
        //    entries: [m_lg_e.clone()].to_vec(),
        //    last_change_timestamp: 0.0,
        //    current_entry: m_lg_e,
        //    max_size: 8,
        //};
        //// LED BLUE
        //let b_lb_e = TimedData {
        //    id: COUNTER.fetch_add(1, Ordering::Relaxed),
        //    data: "0".to_string(),
        //    time: 0.0,
        //};
        //let b_lb = Buffer {
        //    entries: [].to_vec(),
        //    last_change_timestamp: 0.0,
        //    current_entry: b_lb_e,
        //    max_size: 10,
        //};
        //let m_lb_e = TimedData {
        //    id: COUNTER.fetch_add(1, Ordering::Relaxed),
        //    data: "0".to_string(),
        //    time: 0.0,
        //};
        //let m_lb = Buffer {
        //    entries: [m_lb_e.clone()].to_vec(),
        //    last_change_timestamp: 0.0,
        //    current_entry: m_lb_e,
        //    max_size: 8,
        //};
        Ok(Self {
            name: brain_name,
            mode: mode,
            start_time: start_time,
            timestamp: 0.0,
            config: c,
            arduino: a,
            motors: m,
            leds: l,
            buffer_led_y: b_ly,
            metrics_led_y: m_ly,
            buffer_led_r: b_lr,
            metrics_led_r: m_lr,
            //buffer_led_g: b_lg,
            //metrics_led_g: m_lg,
            //buffer_led_b: b_lb,
            //metrics_led_b: m_lb,
        })
    }

    /// takes care of inputs and outputs to the brain
    ///  in a constant loop
    pub fn do_io(&mut self) {
        loop {
            debug!("...reading from channel with Arduino");
            let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let msgs = s.clone();
            let mut arduino = self.arduino.clone();
            let brain_clone = self.clone();
            let _handle = thread::spawn(move || {
                if brain_clone.mode != "dryrun" {
                    arduino.read_channel(msgs).unwrap();
            } else {
                    arduino.read_channel_mock(msgs).unwrap();
                };
            });
            loop {
                let ct = SystemTime::now();
                self.timestamp = match ct.duration_since(UNIX_EPOCH) {
                    Ok(time) => (time.as_millis() as f64 - self.start_time as f64) / 1000 as f64,
                    Err(_e) => 0.0,
                };
                let msg = r.recv();
                debug!("- Received {}", msg.clone().unwrap());
                let actionmsg = msg.clone();
                let sensormsg = msg.clone();
                if actionmsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "ACTION".to_string() {
                    let msg_action = msg.unwrap().replace("ACTION: ", "");
                    self.add_action(msg_action);
                } else if sensormsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "SENSOR".to_string() {
                    // NOTE: Sensor messages format go like "SENSOR: object_x__value"
                    let msg_sensor = msg.unwrap().replace("SENSOR: ", "");
                    self.add_metric(msg_sensor);
                }
                debug!("- Current timestamp: {}", self.timestamp);
                trace!("- Metrics - LED Y:");
                for (ix, action) in self.metrics_led_y.entries.clone().iter().enumerate() {
                    trace!(" #{} |data={}|time={}|", ix, action.data, action.time);
                }
                trace!("- Metrics - LED R:");
                for (ix, action) in self.metrics_led_r.entries.clone().iter().enumerate() {
                    trace!(" #{} |data={}|time={}|", ix, action.data, action.time);
                }
                //trace!("- Metrics - LED G:");
                //for (ix, action) in self.metrics_led_g.entries.clone().iter().enumerate() {
                //    trace!(" #{} |data={}|time={}|", ix, action.data, action.time);
                //}
                //trace!("- Metrics - LED B:");
                //for (ix, action) in self.metrics_led_b.entries.clone().iter().enumerate() {
                //    trace!(" #{} |data={}|time={}|", ix, action.data, action.time);
                //}
                trace!("...checking rules, adding actions");
                let _actions_from_config = match self.get_actions_from_rules(){
                    Ok(a) => {
                        if a.len() > 0 {
                            // Format would be motor_l=-60,time=2.6
                            //TODO: this should be done only it the action refers to the object
                            self.buffer_led_y.entries = Vec::new();
                            self.buffer_led_r.entries = Vec::new();
                            //self.buffer_led_g.entries = Vec::new();
                            //self.buffer_led_b.entries = Vec::new();
                            for action in a {
                                for o in action.output {
                                    let aux = format!("{}={},time={}", o.object, o.value, o.time);
                                    self.add_action(aux);
                                }
                            }
                        };
                    },
                    Err(_e) => debug!("...no matching rules found"),
                };
                debug!("...doing actions");
                'outer: loop {
                    self.timestamp = match ct.duration_since(UNIX_EPOCH) {
                        Ok(time) => (time.as_millis() as f64 - self.start_time as f64) / 1000 as f64,
                        Err(_e) => 0.0,
                    };
                    trace!("- Actions buffer - LED Y:");
                    trace!("  {:?}", self.buffer_led_y.entries);
                    trace!("- Actions buffer - LED R:");
                    trace!("  {:?}", self.buffer_led_r.entries);
                    //trace!("- Actions buffer - LED G:");
                    //trace!("  {:?}", self.buffer_led_g.entries);
                    //trace!("- Actions buffer - LED B:");
                    //trace!("  {:?}", self.buffer_led_b.entries);
                    match self.do_next_actions() {
                        Ok(a) => {
                            if a != "done nothing" {
                                debug!("- Action {:?} - {:?}", self.timestamp, a);
                            } else {
                                debug!("- Action {:?} - {:?}", self.timestamp, a);
                            }
                            break 'outer;
                        },
                        Err(_e) => {
                            break 'outer;
                        },
                    };
                }
            }
        }
    }

    /// adds metric to the related metrics buffer
    pub fn add_metric(&mut self, metric: String) {
        debug!("- Adding metric {}", metric);
        let metric_decomp = metric.split("__").collect::<Vec<_>>();
        match metric_decomp[0] {
            "led_y" => {
                if self.metrics_led_y.entries.len() == 0 {
                    let new_m = TimedData {
                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
                        data: metric_decomp[1].to_string(),
                        time: self.timestamp.clone(), // here time means "since_timestamp"
                    };
                    self.metrics_led_y.entries.push(new_m);
                    self.metrics_led_y.last_change_timestamp = self.timestamp;
                } else {
                    if self.metrics_led_y.entries[0].data != metric_decomp[1].to_string() {
                        let new_m = TimedData {
                            id: COUNTER.fetch_add(1, Ordering::Relaxed),
                            data: metric_decomp[1].to_string(),
                            time: self.timestamp.clone(),
                        };
                        self.metrics_led_y.entries.insert(0, new_m);
                        self.metrics_led_y.last_change_timestamp = self.timestamp;
                    }
                }; 
                if self.metrics_led_y.entries.len() > self.metrics_led_y.max_size.into() {
                    self.metrics_led_y.entries.pop();
                };
            },
            "led_r" => {
                if self.metrics_led_r.entries.len() == 0 {
                    let new_m = TimedData {
                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
                        data: metric_decomp[1].to_string(),
                        time: self.timestamp.clone(), // here time means "since_timestamp"
                    };
                    self.metrics_led_r.entries.push(new_m);
                    self.metrics_led_r.last_change_timestamp = self.timestamp;
                } else {
                    if self.metrics_led_r.entries[0].data != metric_decomp[1].to_string() {
                        let new_m = TimedData {
                            id: COUNTER.fetch_add(1, Ordering::Relaxed),
                            data: metric_decomp[1].to_string(),
                            time: self.timestamp.clone(),
                        };
                        self.metrics_led_r.entries.insert(0, new_m);
                        self.metrics_led_r.last_change_timestamp = self.timestamp;
                    }
                }; 
                if self.metrics_led_r.entries.len() > self.metrics_led_r.max_size.into() {
                    self.metrics_led_r.entries.pop();
                };
            },
            //"led_g" => {
            //    if self.metrics_led_g.entries.len() == 0 {
            //        let new_m = TimedData {
            //            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            //            data: metric_decomp[1].to_string(),
            //            time: self.timestamp.clone(), // here time means "since_timestamp"
            //        };
            //        self.metrics_led_g.entries.push(new_m);
            //        self.metrics_led_g.last_change_timestamp = self.timestamp;
            //    } else {
            //        if self.metrics_led_g.entries[0].data != metric_decomp[1].to_string() {
            //            let new_m = TimedData {
            //                id: COUNTER.fetch_add(1, Ordering::Relaxed),
            //                data: metric_decomp[1].to_string(),
            //                time: self.timestamp.clone(),
            //            };
            //            self.metrics_led_g.entries.insert(0, new_m);
            //            self.metrics_led_g.last_change_timestamp = self.timestamp;
            //        }
            //    }; 
            //    if self.metrics_led_g.entries.len() > self.metrics_led_g.max_size.into() {
            //        self.metrics_led_g.entries.pop();
            //    };
            //},
            //"led_b" => {
            //    if self.metrics_led_b.entries.len() == 0 {
            //        let new_m = TimedData {
            //            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            //            data: metric_decomp[1].to_string(),
            //            time: self.timestamp.clone(), // here time means "since_timestamp"
            //        };
            //        self.metrics_led_b.entries.push(new_m);
            //        self.metrics_led_b.last_change_timestamp = self.timestamp;
            //    } else {
            //        if self.metrics_led_b.entries[0].data != metric_decomp[1].to_string() {
            //            let new_m = TimedData {
            //                id: COUNTER.fetch_add(1, Ordering::Relaxed),
            //                data: metric_decomp[1].to_string(),
            //                time: self.timestamp.clone(),
            //            };
            //            self.metrics_led_b.entries.insert(0, new_m);
            //            self.metrics_led_b.last_change_timestamp = self.timestamp;
            //        }
            //    }; 
            //    if self.metrics_led_b.entries.len() > self.metrics_led_b.max_size.into() {
            //        self.metrics_led_b.entries.pop();
            //    };
            //},
            _ => (),
        }
    }

    /// Goes through all rules loaded from config, checks if they match what is currently on our
    /// metrics, and iif so, it returns them
    pub fn get_actions_from_rules(&mut self) -> Result<Vec<ConfigEntry>, BrainDeadError>{
        // Start with led_y
        let mut partial_rules: Vec<ConfigEntry> = [].to_vec();
        for rule in self.config.clone() {
            if self.metrics_led_y.entries.len() > 0 {
                if rule.input[0].led_y != "*" {
                    if self.metrics_led_y.entries[0].data == rule.input[0].led_y {
                        if (self.timestamp - self.metrics_led_y.entries[0].time >= rule.input[0].time.parse::<f64>().unwrap()) || (self.metrics_led_y.entries[0].time == 0.0){
                            // for ix, action in rule.output -> if same index on buffer_led_y has
                            // same action, then take note, anf if all of them are the same, dont
                            // add the rule
                            if ! self.are_actions_in_buffer(rule.clone()) {
                                partial_rules.push(rule.clone());
                            }
                        };
                    };
                } else {
                    if (self.timestamp - self.metrics_led_y.entries[0].time >= rule.input[0].time.parse::<f64>().unwrap()) || (self.metrics_led_y.entries[0].time == 0.0){
                        partial_rules.push(rule.clone());
                    };
                };

            };
        };
        // Then remove those that dont fit led_r
        //TODO: I think this should go the opposite way: dont add, remove!
        for rule in partial_rules.clone() {
            if self.metrics_led_r.entries.len() > 0 {
                if rule.input[0].led_r != "*" {
                    if self.metrics_led_r.entries[0].data == rule.input[0].led_r {
                        if (self.timestamp - self.metrics_led_r.entries[0].time >= rule.input[0].time.parse::<f64>().unwrap()) || (self.metrics_led_r.entries[0].time == 0.0){
                            // for ix, action in rule.output -> if same index on buffer_led_r has
                            // same action, then take note, anf if all of them are the same, dont
                            // add the rule
                            if ! self.are_actions_in_buffer(rule.clone()) {
                                partial_rules.push(rule.clone());
                            }
                        };
                    };
                } else {
                    if (self.timestamp - self.metrics_led_r.entries[0].time >= rule.input[0].time.parse::<f64>().unwrap()) || (self.metrics_led_r.entries[0].time == 0.0){
                        partial_rules.push(rule.clone());
                    };
                };

            };
        };
        if partial_rules.len() > 0 {
            trace!("- Rules matching :");
            for (ix, rule) in partial_rules.clone().iter().enumerate() {
                trace!(" #{} input:", ix);
                for ri in rule.input.clone() {
                    trace!("      |{:?}|", ri);
                }
                trace!("     output:");
                for ro in rule.output.clone() {
                    trace!("      |{:?}|", ro);
                }
            }
        }
        Ok(partial_rules)
    }

    /// turns a String containing an action into the related object
    pub fn get_action_from_string(&mut self, action: String) -> Result<ResultAction, String> {
        // Format would be motor_l=-60,time=2.6
        let format = action.split(",").collect::<Vec<_>>();
        let t = format[1].split("=").collect::<Vec<_>>()[1].parse::<f64>().unwrap();
        let data = format[0].split("=").collect::<Vec<_>>();
        match data[0] {
            // TODO: will this work??
            "led_y" | "led_r" => {
                let action_item = TimedData {
                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
                    data: data[1].to_string(),
                    time: t,
                };
                let result = ResultAction {
                    resource: data[0].to_string(),
                    action: action_item,
                };
                Ok(result)
            },
            _ => {
                let action_item = TimedData {
                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
                    data: data[1].to_string(),
                    time: t,
                };
                let result = ResultAction {
                    resource: data[0].to_string(),
                    action: action_item,
                };
                Ok(result)
            },
        }
    }

    /// Adds action to the related actions buffer
    pub fn add_action(&mut self, action: String) {
        debug!("- Adding action {}", action);
        let action_to_add = self.clone().get_action_from_string(action).unwrap();
        match action_to_add.resource.as_str() {
            "led_y" => {
                if self.buffer_led_y.entries.len() >= self.buffer_led_y.max_size.into() {
                    warn!("Buffer for LED_y is full! not adding new actions...");
                } else {
                    self.buffer_led_y.entries.push(action_to_add.action);
                };
            },
            "led_r" => {
                if self.buffer_led_r.entries.len() >= self.buffer_led_r.max_size.into() {
                    warn!("Buffer for LED_r is full! not adding new actions...");
                } else {
                    self.buffer_led_r.entries.push(action_to_add.action);
                };
            },
            _ => ()
        }
    }

    /// Checks the time passed for the current action and, when it goes over the time set, 
    /// it "moves" to the next one
    pub fn do_next_actions(&mut self) -> Result<String, String>{
        // TODO: add led_r here...but HOW?
        if self.timestamp >= self.metrics_led_y.last_change_timestamp {
            if self.buffer_led_y.entries.len() == 0 {
                Err("No more actions to take".to_string())
            } else {
                let a = &self.buffer_led_y.entries.clone()[0];
                let time_passed = self.timestamp - self.buffer_led_y.last_change_timestamp;
                debug!("- Time passed on current value - {:?}", time_passed);
                if time_passed >= self.buffer_led_y.current_entry.time {
                    self.buffer_led_y.current_entry = a.clone();
                    self.buffer_led_y.entries.retain(|x| *x != *a);
                    self.buffer_led_y.last_change_timestamp = self.timestamp.clone();
                    trace!("- Buffer: {:#x?}", self.buffer_led_y.entries);
                    info!("- Just did LED_Y -> {}", a.data);
                    self.leds.set_led_y(a.data.parse::<u8>().unwrap() == 1);
                    self.add_metric(format!("led_y__{}", a.data));
                    Ok(format!("done {:?}", a))

                } else {
                    Ok("done nothing".to_string())
                }
            }
        } else {
            Ok("done nothing".to_string())
        }
    }

    /// Returns whether a set of actions are already on the buffer, 
    /// to avoid constantly adding the same ones
    pub fn are_actions_in_buffer(&self, rule: ConfigEntry) -> bool {
        // first loop to fill up vectors of actions separately
        let mut rule_out_led_y = [].to_vec();
        let mut rule_out_led_r = [].to_vec();
        //let mut rule_out_led_g = [].to_vec();
        //let mut rule_out_led_b = [].to_vec();
        let mut rule_out_other = [].to_vec();
        for r in rule.output {
            match r.object.as_str() {
                "led_y" => rule_out_led_y.push(r),
                "led_r" => rule_out_led_r.push(r),
                //"led_g" => rule_out_led_g.push(r),
                //"led_b" => rule_out_led_b.push(r),
                _ => rule_out_other.push(r),
            }
        }
        let mut result = true;
        for (ix, r) in rule_out_led_y.iter().enumerate() {
            if self.buffer_led_y.entries.len() > ix {
                if format!("{}_{}", r.value, r.time) != format!("{}_{}", self.buffer_led_y.entries[ix].data, self.buffer_led_y.entries[ix].time) {
                   result = false; 
                }
            } else {
               result = false; 
            }
        }
        for (ix, r) in rule_out_led_r.iter().enumerate() {
            if self.buffer_led_r.entries.len() > ix {
                if format!("{}_{}", r.value, r.time) != format!("{}_{}", self.buffer_led_r.entries[ix].data, self.buffer_led_r.entries[ix].time) {
                   result = false; 
                }
            } else {
               result = false; 
            }
        }
        //for (ix, r) in rule_out_led_g.iter().enumerate() {
        //    if self.buffer_led_g.entries.len() > ix {
        //        if format!("{}_{}", r.value, r.time) != format!("{}_{}", self.buffer_led_g.entries[ix].data, self.buffer_led_g.entries[ix].time) {
        //           result = false; 
        //        }
        //    } else {
        //       result = false; 
        //    }
        //}
        //for (ix, r) in rule_out_led_b.iter().enumerate() {
        //    if self.buffer_led_b.entries.len() > ix {
        //        if format!("{}_{}", r.value, r.time) != format!("{}_{}", self.buffer_led_b.entries[ix].data, self.buffer_led_b.entries[ix].time) {
        //           result = false; 
        //        }
        //    } else {
        //       result = false; 
        //    }
        //}
        result
    }
}
