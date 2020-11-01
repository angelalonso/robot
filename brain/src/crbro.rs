use crate::arduino::Arduino;
use crate::motors::Motors;
use crate::leds::LEDs;
use log::{debug, error, info, warn};
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

#[derive(Clone, Debug, PartialEq)]
pub struct Metrics {
    metrics: Vec<TimedData>,
    last_change_timestamp: f64,
    max_size: u8,
}

#[derive(Clone, Debug)]
pub struct ResultAction {
    resource: String,
    action: TimedData,
}

#[derive(Clone, Debug)]
pub struct ActionBuffer {
    buffer: Vec<TimedData>,
    last_change_timestamp: f64,
    max_size: u8,
}
// TODO use this as Metrics and ActionBuffer
#[derive(Clone, Debug)]
pub struct Buffer {
    entries: Vec<TimedData>,
    last_change_timestamp: f64,
    max_size: u8,
}

#[derive(Clone)]
pub struct Crbro {
    name: String,
    mode: String,
    start_time: u128,
    timestamp: f64,
    config: Vec<ConfigEntry>,
    arduino: Arduino,
    motors: Motors,
    leds: LEDs,
    buffer_led_y: ActionBuffer,
    metrics_led_y: Metrics,
}
// TODO: use timestamps starting at 0
// TODO: check why buffer for led_y is not working as expected
static COUNTER: std::sync::atomic::AtomicUsize = AtomicUsize::new(1);

impl Crbro {
    pub fn new(brain_name: String, mode: String, config_file: String) -> Result<Self, String> {
        let st = SystemTime::now();
        let start_time = match st.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
        };
        let mut a = Arduino::new("arduino".to_string(), Some("/dev/null".to_string())).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        let cfg_file_pointer = File::open(config_file).unwrap();
        let c: Vec<ConfigEntry> = serde_yaml::from_reader(cfg_file_pointer).unwrap();
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
        let b_ly = ActionBuffer {
            buffer: [].to_vec(),
            last_change_timestamp: 0.0,
            max_size: 10,
        };
        let m_ly_m = [TimedData {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            data: "0".to_string(),
            time: 0.0,
        },].to_vec();
        let m_ly = Metrics {
            metrics: m_ly_m,
            last_change_timestamp: 0.0,
            max_size: 8,
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
            buffer_led_y: b_ly,
            metrics_led_y: m_ly,
        })
    }
    pub fn do_io(&mut self) {
        loop {
            debug!("Reading from channel with Arduino");
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
                debug!("GOT {}", msg.clone().unwrap());
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
                debug!("add current metrics");
                self.add_current_metrics();
                debug!("Checking rules, adding actions");
                let _actions_from_config = match self.get_actions_from_rules(){
                    Ok(a) => {
                        if a.len() > 0 {
                            // Format would be motor_l=-60,time=2.6
                            // TODO: make this work for other outputs (motors...)
                            // if we got actions from a rule, previous actions get overriden
                            self.buffer_led_y.buffer = Vec::new();
                            let aux = format!("{}={},time={}", a[0].output[0].object, a[0].output[0].value, a[0].output[0].time);
                            debug!("{:#x?}", aux);
                            self.add_action(aux);
                        };
                    },
                    Err(_e) => debug!("No matching rules found"),
                };
                //info!("ACTIONS BUFFER - {:#x?}", self.buffer_led_y);
                //info!("ACTIONS BUFFER - {:#x?}", self.buffer_led_y.buffer.iter());
                debug!("Doing actions");
                'outer: loop {
                    self.timestamp = match ct.duration_since(UNIX_EPOCH) {
                        Ok(time) => (time.as_millis() as f64 - self.start_time as f64) / 1000 as f64,
                        Err(_e) => 0.0,
                    };
                    match self.do_next_actions() {
                        Ok(a) => {
                            info!("ACTION {:?} - {:?}", self.timestamp, a);
                            //latest_change = current_time as u128;
                            break 'outer;
                        },
                        Err(_e) => {
                            //latest_change = current_time as u128;
                            break 'outer;
                        },
                    };
                }
            }
        }
    }

    pub fn get_actions_from_rules(&mut self) -> Result<Vec<ConfigEntry>, BrainDeadError>{
        // Start with led_y
        let mut partial_rules: Vec<ConfigEntry> = [].to_vec();
        for rule in self.config.clone() {
            //info!("{:#x?}", rule);
            info!("LENGTH {:#x?}", self.metrics_led_y.metrics.len() as u16);
            if self.metrics_led_y.metrics.len() > 0 {
                if rule.input[0].led_y != "*" {
                    if self.metrics_led_y.metrics[0].data == rule.input[0].led_y {
                        if (self.metrics_led_y.metrics[0].time >= rule.input[0].time.parse::<f64>().unwrap()) || (self.metrics_led_y.metrics[0].time == 0.0){
                            partial_rules.push(rule.clone());
                        };
                    };
                } else {
                    if (self.metrics_led_y.metrics[0].time >= rule.input[0].time.parse::<f64>().unwrap()) || (self.metrics_led_y.metrics[0].time == 0.0){
                        partial_rules.push(rule.clone());
                    };
                };

            };
        };
        Ok(partial_rules)
    }

    pub fn add_current_metrics(&mut self) {
        // TODO: define how and when metrics are stored
        debug!("led_y metrics - {:#x?}", self.metrics_led_y);
    }

    pub fn add_metric(&mut self, metric: String) {
        debug!("Adding metric {}", metric);
        let metric_decomp = metric.split("__").collect::<Vec<_>>();
        match metric_decomp[0] {
            "led_y" => {
                if self.metrics_led_y.metrics.len() == 0 {
                    let new_m = TimedData {
                        id: COUNTER.fetch_add(1, Ordering::Relaxed),
                        data: metric_decomp[1].to_string(),
                        time: self.timestamp.clone(), // here time means "since_timestamp"
                    };
                    self.metrics_led_y.metrics.push(new_m);
                    self.metrics_led_y.last_change_timestamp = self.timestamp;
                } else {
                    if self.metrics_led_y.metrics[0].data != metric_decomp[1].to_string() {
                        let new_m = TimedData {
                            id: COUNTER.fetch_add(1, Ordering::Relaxed),
                            data: metric_decomp[1].to_string(),
                            time: self.timestamp.clone(),
                        };
                        self.metrics_led_y.metrics.insert(0, new_m);
                        self.metrics_led_y.last_change_timestamp = self.timestamp;
                    }
                }; 
                if self.metrics_led_y.metrics.len() > self.metrics_led_y.max_size.into() {
                    self.metrics_led_y.metrics.pop();
                };
            },
            _ => (),
        }
        debug!("{:?}", self.metrics_led_y.metrics[0].data);
    }

    pub fn get_action_from_string(&mut self, action: String) -> Result<ResultAction, String> {
        // Format would be motor_l=-60,time=2.6
        let format = action.split(",").collect::<Vec<_>>();
        let t = format[1].split("=").collect::<Vec<_>>()[1].parse::<f64>().unwrap();
        let data = format[0].split("=").collect::<Vec<_>>();
        match data[0] {
            "led_y" => {
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

    pub fn add_action(&mut self, action: String) {
        debug!("Adding action {}", action);
        let action_to_add = self.clone().get_action_from_string(action).unwrap();
        match action_to_add.resource.as_str() {
            "led_y" => {
                if self.buffer_led_y.buffer.len() >= self.buffer_led_y.max_size.into() {
                    warn!("Buffer for LED_y is full! not adding new actions...");
                } else {
                    self.buffer_led_y.buffer.push(action_to_add.action);
                };
            },
            _ => ()
        }
        debug!("RESOURCE - {:#x?}", action_to_add.resource);
    }

    pub fn do_next_actions(&mut self) -> Result<String, String>{
        if self.timestamp >= self.buffer_led_y.last_change_timestamp {
            if self.buffer_led_y.buffer.len() == 0 {
                self.buffer_led_y.last_change_timestamp = 0.0; // if a new action enters, we want it to run for as long as it's defined
                Err("No more actions to take".to_string())
            } else {
                let a = &self.buffer_led_y.buffer.clone()[0];
                let time_passed = (self.timestamp - self.buffer_led_y.last_change_timestamp) / 1000 as f64;
                if time_passed >= a.time {
                    self.buffer_led_y.buffer.retain(|x| *x != *a);
                    self.buffer_led_y.last_change_timestamp = self.timestamp.clone();
                    debug!("{:#x?}", self.buffer_led_y);
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
}
