use crate::arduino::Arduino;
use crate::motors::Motors;
use log::{debug, error};
use std::process;
use thiserror::Error;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicUsize, Ordering};

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

#[derive(Clone, Debug)]
pub struct ActionBuffer {
    buffer: Vec<TimedData>,
    last_change_timestamp: f64,
    max_size: u8,
}

#[derive(Clone)]
pub struct Crbro {
    name: String,
    mode: String,
    start_time: u128,
    timestamp: f64,
    arduino: Arduino,
    motors: Motors,
    buffer_led_y: ActionBuffer,
    metrics_led_y: Vec<TimedData>,
    max_metrics_led_y: u8,
}

static COUNTER: std::sync::atomic::AtomicUsize = AtomicUsize::new(1);

impl Crbro {
    pub fn new(brain_name: String, mode: String) -> Result<Self, String> {
        let st = SystemTime::now();
        let start_time = match st.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
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
        };
        let m = Motors::new(mode.clone()).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Motors: {}", err);
            process::exit(1);
        });
        let b_ly = ActionBuffer {
            buffer: [].to_vec(),
            last_change_timestamp: 0.0,
            max_size: 10,
        };
        Ok(Self {
            name: brain_name,
            mode: mode,
            start_time: start_time,
            timestamp: 0.0,
            arduino: a,
            motors: m,
            buffer_led_y: b_ly,
            metrics_led_y: [].to_vec(),
            max_metrics_led_y: 10,
        })
    }
    pub fn do_io(&mut self) {
        loop {
            println!("_");
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
                println!(".");
                let msg = r.recv();
                debug!("GOT {}", msg.clone().unwrap());
                let actionmsg = msg.clone();
                let sensormsg = msg.clone();
                if actionmsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "ACTION".to_string() {
                    let msg_action = msg.unwrap().replace("ACTION: ", "");
                    self.add_action(msg_action);
                } else if sensormsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "SENSOR".to_string() {
                    let msg_sensor = msg.unwrap().replace("SENSOR: ", "");
                    self.add_metric(msg_sensor);
                }
                debug!("Checking rules, adding actions");
                debug!("Doing actions");
                'outer: loop {
                    let ct = SystemTime::now();
                    self.timestamp = match ct.duration_since(UNIX_EPOCH) {
                        Ok(time) => time.as_millis() as f64,
                        Err(_e) => 0.0,
                    };
                    match self.do_next_actions() {
                        Ok(a) => {
                            println!("{:?} - {:?}", self.timestamp, a);
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
                    error!("Buffer for LED_y reached its max size!");
                } else {
                    self.buffer_led_y.buffer.push(action_to_add.action);
                };
            },
            _ => ()
        }
        println!("{:#x?}", action_to_add.resource);
    }

    pub fn add_metric(&mut self, metric: String) {
        debug!("Adding metric {}", metric);
        // TODO: define how metrics are stored

    }

    pub fn check_rules(&mut self) {

    }

    pub fn do_next_actions(&mut self) -> Result<String, String>{
        // TODO: trigger the actual action
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
                    println!("{:#x?}", self.buffer_led_y);
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
