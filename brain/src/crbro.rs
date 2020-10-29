use crate::arduino::Arduino;
use crate::motors::Motors;
use log::debug;
use std::process;
use thiserror::Error;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

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
    timer: f64,
}

#[derive(Clone)]
pub struct Crbro {
    name: String,
    mode: String,
    start_time: u128,
    arduino: Arduino,
    motors: Motors,
    actions_buffer_led_y: ActionBuffer,
    max_actions_buffer: u8,
    metrics_led_y: Vec<TimedData>,
    max_metrics_led_y: u8,
}

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
        let ab_ly = ActionBuffer {
            buffer: [].to_vec(),
            timer: 0.0,
        };
        Ok(Self {
            name: brain_name,
            mode: mode,
            start_time: start_time,
            arduino: a,
            motors: m,
            actions_buffer_led_y: ab_ly,
            max_actions_buffer: 10,
            metrics_led_y: [].to_vec(),
            max_metrics_led_y: 10,
        })
    }
    pub fn do_io(&mut self) {
        loop {
            debug!("Reading from channel with Arduino");
            let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let msgs = s.clone();
            let mut arduino = self.arduino.clone();
            let _brain_clone = self.clone();
            if self.mode != "dryrun" {
                let _handle = thread::spawn(move || {
                    arduino.read_channel(msgs).unwrap();
                });
            } else {
                let _handle = thread::spawn(move || {
                    arduino.read_channel_mock(msgs).unwrap();
                });
            };
            loop {
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
                let mut latest_change = self.start_time;
                'outer: loop {
                    let ct = SystemTime::now();
                    let current_time = match ct.duration_since(UNIX_EPOCH) {
                        Ok(time) => time.as_millis(),
                        Err(_e) => 0,
                    };
                    let timestamp: u128 = (current_time as u128 - latest_change as u128) as u128 / 1000 as u128;
                    match self.do_next_actions(timestamp) {
                        Ok(a) => {
                            println!("{:?} - {:?}", timestamp, a);
                            latest_change = current_time as u128;
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

                self.actions_buffer_led_y.buffer.push(action_to_add.action);
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

    pub fn do_next_actions(&mut self, timestamp: u128) -> Result<String, String>{
        println!("{:#x?}", self.actions_buffer_led_y);
        if timestamp as f64 >= self.actions_buffer_led_y.timer {
            if self.actions_buffer_led_y.buffer.len() == 0 {
                self.actions_buffer_led_y.timer = 0.0;
                Err("No more actions to take".to_string())
            } else {
                let a = &self.actions_buffer_led_y.buffer.clone()[0];
                self.actions_buffer_led_y.buffer.retain(|x| *x != *a);
                self.actions_buffer_led_y.timer = a.time as f64;
                println!("{:#x?}", self.actions_buffer_led_y);
                Ok(format!("done {:?}", a))
            }
        } else {
            Ok("done nothing".to_string())
        }
        

    }
}
