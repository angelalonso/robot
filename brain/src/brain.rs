use crate::config::Config;
use crate::arduino::Arduino;
use crate::log;
use std::str;
use thiserror::Error;
use std::process;

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


pub struct Brain<'a> {
    pub name: &'a str,
    pub config: Config,
    pub arduino: Arduino<'a>,
    pub serialport: &'a str,
    pub timeout: u64,
    pub move_pair: (i16, i16),
}

impl Brain<'_> {
    pub fn new(brain_name: &'static str, config_file: String, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        let configdata = Config::new(config_file);
        let serial_port = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0",
        };
        let arduino_connector = Arduino::new("arduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        Ok(Self {
            name: brain_name,
            config: configdata,
            arduino: arduino_connector,
            serialport: serial_port,
            timeout: 4,
            move_pair: (0, 0),
        })
    }

    pub fn read(&mut self) {
        loop {
            self.show_move();
            let _received = match self.arduino.read_channel(){
                Ok(rcv) => {
                    let _taken_actions = match self.get_actions(&rcv){
                        Ok(acts) => println!("Taking action {:?}", acts.join(", ")),
                        Err(_) => log(Some(&self.name), "D", "No actions were found for trigger"),
                    };
                },
                Err(_) => log(Some(&self.name), "D", "Nothing read from Channel"),
            };
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

    pub fn show_move(&mut self) {
        log(Some(&self.name), "I", &format!("Moving L: {}, R: {}", self.move_pair.0, self.move_pair.1));
    }
}
