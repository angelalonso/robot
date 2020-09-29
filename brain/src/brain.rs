use crate::config::Config;
use crate::arduino::Arduino;
use crate::mover::Mover;
use crate::log;
use std::process;
use std::str;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainDeadError {
    /// It used to represent an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    /// Now it's just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("Config contains no related entries")]
    NoConfigFound,
}


#[derive(Clone)]
pub struct Brain<'a> {
    pub name: &'a str,
    pub config: Config,
    pub arduino: Arduino<'a>,
    pub serialport: &'a str,
    pub timeout: u64,
    pub mover: Mover<'a>,
}

impl Brain<'static> {
    pub fn new(brain_name: &'static str, config_file: String, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        let configdata = Config::new(config_file);
        let sp = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0",
        };
        let a = Arduino::new("arduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        let m = Mover::new().unwrap_or_else(|err| {
            eprintln!("Problem Initializing Mover: {}", err);
            process::exit(1);
        });
        Ok(Self {
            name: brain_name,
            config: configdata,
            arduino: a,
            serialport: sp,
            timeout: 4,
            mover: m,
        })
    }

    pub fn read(mut self) {
        loop {
            self.show_move();
            let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let msgs = s.clone();
            let mut arduino = self.arduino.clone();
            let mut avatar = self.clone();
            let this_name = self.name.clone();
            let _handle = thread::spawn(move || {
                let _received = match arduino.read_channel(msgs){
                    Ok(rcv) => {
                        let _taken_actions = match avatar.get_actions(&rcv){
                            Ok(acts) => println!("Taking action {:?}", acts.join(", ")),
                            Err(_) => log(Some(&this_name), "D", "No actions were found for trigger"),
                        };
                    },
                    Err(_) => log(Some(&this_name), "D", "Nothing read from Channel"),
                };
            });
            loop {
                let msg = r.recv();
                let mut msg_actions = Vec::new();
                msg_actions.push(msg.unwrap().replace("ACTION: ", "").replace("SENSOR: ", ""));
                self.apply_actions(msg_actions).unwrap();
                self.show_move();
            }
        }
    }

    /// Get the action that relates to the trigger received and call to apply it
    /// Hm...maybe this one and apply_actions should go together?
    pub fn get_actions(&mut self, trigger: &str) -> Result<Vec<String>, BrainDeadError> {
        log(Some(&self.name), "D", &format!("Received {}", trigger));
        let actions = self.config.get_actions(&trigger);
        match actions {
            Ok(acts) => {
                match acts {
                    Some(a) => {
                        self.apply_actions(a.clone()).unwrap();
                        Ok(a)
                    },
                    None => {
                        log(Some(&self.name), "D", "Nothing to do");
                        Err(BrainDeadError::NoConfigFound)
                    },
                }
            },
            Err(_e) => Err(BrainDeadError::NoConfigFound),
        }
    }

    /// Call the action needed
    /// , which, right now, is just installing a new hex into the arduino
    pub fn apply_actions(&mut self, actions: Vec<String>) -> Result<(), BrainDeadError> {
        for action in &actions {
            let action_vec: Vec<&str> = action.split('_').collect();
            match action_vec[0] {
                "install" => self.arduino.install(&action_vec[1..].to_vec().join("_")).unwrap(),
                "move" => self.mover.set_move(action_vec[1..].to_vec().join("_")),
                "data" => self.decide_on(action_vec[1..].to_vec().join("_")),
                _ => self.do_nothing().unwrap(),
            };
        }
        Ok(())
    }

    /// Do nothing, but take note that we have nothing to do
    pub fn do_nothing(&mut self) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "D", "Relaxing here...");
        Ok(())
    }


    /// Show current movement values at both engines
    pub fn show_move(&mut self) {
        log(Some(&self.name), "I", &format!("Moving : {}", self.mover.movement));
    }

    pub fn decide_on(&mut self, info: String) {
        println!("{}", info);
    }
}
