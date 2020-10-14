// This is the actual central block of our robot
// It is called Brain for obvious reasons, but is somehow divided on two:
//  - Cerebellum part: Manages any movement actions -> lives on its own module but depends on this
//  brain one
//  - Brain part: Manages any other actions, such as:
//    - installing a new .hex into arduino
use crate::arduino::Arduino;
use crate::config::Config;
use crate::cerebellum::Cerebellum;
use crate::log;
use crate::mover::Mover;
use std::process;
use std::str;
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

#[derive(Clone)]
pub struct Brain<'a> {
    pub name: &'a str,
    pub starttime: u128,
    pub config: Config,
    pub serialport: &'a str,
    pub timeout: u64,
    pub cerebellum: Cerebellum,
    pub arduino: Arduino<'a>,
    pub mover: Mover<'a>,
}

/// Initialize all the things
impl Brain<'static> {
    pub fn new(brain_name: &'static str, config_file: String, cerebellum_config_file: String, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        // This will be used to calculate our timestamps
        let st = SystemTime::now();
        let start_time = match st.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
        };
        // This loads the rules dictating what actions the brain takes
        let cfg = Config::new(config_file);
        // Instance of the Cerebellum Module
        let crb = Cerebellum::new(cerebellum_config_file);
        // Serial Port to communicate with Arduino
        let sp = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0",
        };
        // Instance of the Arduino Module
        let a = Arduino::new("arduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        // Instance of the Mover module
        let m = Mover::new().unwrap_or_else(|err| {
            eprintln!("Problem Initializing Mover: {}", err);
            process::exit(1);
        });
        Ok(Self {
            name: brain_name,
            starttime: start_time,
            config: cfg,
            serialport: sp,
            timeout: 4,
            cerebellum: crb,
            arduino: a,
            mover: m,
        })
    }

    pub fn get_input(mut self) {
        loop {
            let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let msgs = s.clone();
            let mut arduino = self.arduino.clone();
            let mut brain_clone = self.clone();
            let brain_name = self.name.clone();
            let _handle = thread::spawn(move || {
                let _received = match arduino.read_channel(msgs){
                    Ok(rcv) => {
                        match brain_clone.get_brain_actions(&rcv){
                            Ok(acts) => println!("Taking action {:?}", acts.join(", ")),
                            Err(_) => log(Some(&brain_name), "D", "No actions were found for trigger"),
                        };
                    },
                    Err(_) => log(Some(&brain_name), "D", "Nothing read from Channel"),
                };
            });
            loop {
                let msg = r.recv();
                let mut msg_actions = Vec::new();
                let mut msg_sensors = String::new();
                let actionmsg = msg.clone();
                let sensormsg = msg.clone();
                if actionmsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "ACTION".to_string() {
                    msg_actions.push(msg.unwrap().replace("ACTION: ", ""));
                } else if sensormsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "SENSOR".to_string() {
                    msg_sensors = msg.unwrap().replace("SENSOR: ", "");
                }
                self.do_brain_actions(msg_actions).unwrap();
                
                let crbllum_actions = self.cerebellum.manage_input(self.starttime, msg_sensors, self.mover.movement.clone()).unwrap();
                if crbllum_actions.len() > 0 {
                    self.mover.set_move(format!("{:?}_{:?}", crbllum_actions[0].action.motor_l, crbllum_actions[0].action.motor_r));
                }
            }
        }
    }
    ///------------------------------------------------------///
    ///  Brain
    ///------------------------------------------------------///
    /// Get the action that relates to the trigger received and call to apply it
    /// Hm...maybe this one and do_brain_actions should go together?
    pub fn get_brain_actions(&mut self, trigger: &str) -> Result<Vec<String>, BrainDeadError> {
        log(Some(&self.name), "D", &format!("Received {}", trigger));
        let actions = self.config.get_actions(&trigger);
        match actions {
            Ok(acts) => {
                match acts {
                    Some(a) => {
                        self.do_brain_actions(a.clone()).unwrap();
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
    pub fn do_brain_actions(&mut self, actions: Vec<String>) -> Result<(), BrainDeadError> {
        for action in &actions {
            let action_vec: Vec<&str> = action.split('_').collect();
            match action_vec[0] {
                "install" => self.arduino.install(&action_vec[1..].to_vec().join("_")).unwrap(),
                "move" => self.mover.set_move(action_vec[1..].to_vec().join("_")),
                _ => log(Some(&self.name), "D", "Relaxing here..."),
            };
        }
        Ok(())
    }
}
