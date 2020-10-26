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
pub struct Crbro {
    name: String,
    mode: String,
    arduino: Arduino,
    motors: Motors,
}

impl Crbro {
    pub fn new(brain_name: String, mode: String) -> Result<Self, String> {
        let mut a = Arduino::new("arduino".to_string(), Some("/dev/null".to_string())).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        if mode.clone() == "classic" {
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
        })
    }
    pub fn do_io(&mut self) {
        loop {
            debug!("Reading from channel with Arduino");
            let (s, _r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let msgs = s.clone();
            let mut arduino = self.arduino.clone();
            let mut brain_clone = self.clone();
            let _handle = thread::spawn(move || {
                // TODO: from here, we should have a list of actions to add to the buffers
                // , either received as a message or matching from the configs
                let _received = match arduino.read_channel(msgs){
                    Ok(rcv) => {
                        match brain_clone.get_brain_actions(&rcv){
                            Ok(acts) => debug!("Taking action {:?}", acts.join(", ")),
                            Err(_) => debug!("No actions were found for trigger"),
                        };
                    },
                    Err(_) => debug!("Nothing read from Channel"),
                };
            });
            // TODO: here we should call for the actions to get done
        }
    }

    pub fn get_brain_actions(&mut self, trigger: &str) -> Result<Vec<String>, BrainDeadError> {
        debug!("Received {}", trigger);
        Err(BrainDeadError::NoConfigFound)
    }
}
