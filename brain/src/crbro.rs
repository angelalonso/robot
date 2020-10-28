use crate::arduino::Arduino;
use crate::motors::Motors;
use log::debug;
use std::process;
use thiserror::Error;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

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

#[derive(Clone, Debug)]
pub struct TimedData {
    data: String,
    time: f64,
}

#[derive(Clone, Debug)]
pub struct ResultAction {
    resource: String,
    action: TimedData,
}

#[derive(Clone)]
pub struct Crbro {
    name: String,
    mode: String,
    arduino: Arduino,
    motors: Motors,
    actions_buffer_led_y: Vec<TimedData>,
    max_actions_buffer: u8,
    metrics_led_y: Vec<TimedData>,
    max_metrics_led_y: u8,
}

impl Crbro {
    pub fn new(brain_name: String, mode: String) -> Result<Self, String> {
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
        Ok(Self {
            name: brain_name,
            mode: mode,
            arduino: a,
            motors: m,
            actions_buffer_led_y: [].to_vec(),
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
                self.actions_buffer_led_y.push(action_to_add.action);
            },
            _ => ()
        }
        println!("{:#x?}", action_to_add.resource);
        println!("{:#x?}", self.actions_buffer_led_y);
    }

    pub fn add_metric(&mut self, metric: String) {
        debug!("Adding metric {}", metric);
        // TODO: define how metrics are stored

    }

    pub fn check_rules(&mut self) {

    }

    pub fn run_actions(&mut self) {

    }
}
