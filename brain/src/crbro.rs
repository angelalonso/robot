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

#[derive(Clone)]
struct TimedData {
    data: String,
    time: u64,
}

#[derive(Clone)]
pub struct Crbro {
    name: String,
    mode: String,
    arduino: Arduino,
    motors: Motors,
    actions_buffer: Vec<TimedData>,
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
            actions_buffer: [].to_vec(),
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
            // TODO: from here, we should have a list of actions to add to the buffers
            // , either received as a message or matching from the configs
            loop {
                let msg = r.recv();
                debug!("GOT {}", msg.clone().unwrap());
                let mut msg_actions = Vec::new();
                let mut msg_sensors = String::new();
                let actionmsg = msg.clone();
                let sensormsg = msg.clone();
                // TODO: define how actions are stored
                // TODO: define how metrics are stored
                if actionmsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "ACTION".to_string() {
                    msg_actions.push(msg.unwrap().replace("ACTION: ", ""));
                } else if sensormsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "SENSOR".to_string() {
                    msg_sensors = msg.unwrap().replace("SENSOR: ", "");
                }
            }
            // TODO: here we should call for the actions to get done
        }
    }

    pub fn get_brain_actions(&mut self, trigger: &str) -> Result<Vec<String>, BrainDeadError> {
        debug!("Received {}", trigger);
        Err(BrainDeadError::NoConfigFound)
    }
}
