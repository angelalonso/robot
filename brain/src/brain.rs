use crate::config::Config;
use crate::comm::Comm;
use crate::log;
use std::process::Command;
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

    #[error("{0} is NOT installed (or something went wrong while checking that it is)")]
    ProgNotInstalledError(String),

    #[error("AvrDude could not install the program to your Arduino!")]
    AvrdudeError,

    /// Represents the most basic error while sending a file (using avrdude)
    #[error("Something went wrong while using avrdude to send files")]
    SendFileError,

    #[error("Something went wrong while reading from the serial port")]
    ReadSerialError,

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { source: std::io::Error },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}


pub struct Brain<'a> {
    pub name: &'a str,
    pub config: Config,
    pub serialport: &'a str,
    pub timeout: u64,
}

impl Brain<'_> {
    pub fn new(brain_name: &'static str, config_file: &'static str, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        let configdata = Config::new(config_file);
        let serial_port = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0",
        };
        Ok(Self {
            name: brain_name,
            config: configdata,
            serialport: serial_port,
            timeout: 4,
        })
    }

    pub fn read(&mut self) {
        let mut comm = Comm::new("arduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Comm: {}", err);
            process::exit(1);
        });
        loop {
            let _received = match comm.read_channel(){
                Ok(rcv) => {
                    let _taken_actions = match self.get_actions(&rcv){
                        Ok(acts) => println!("Taking action {:?}", acts),
                        Err(_) => log(Some(&self.name), "D", "No actions were found for trigger"),
                        };
                    },
                Err(_) => log(Some(&self.name), "D", "Nothing read from Channel"),
            };
        }
    }

    /// Get the action that relates to the trigger received and call to apply it
    /// Hm...maybe this one and apply_actions should go together?
    pub fn get_actions(&mut self, trigger: &str) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "D", &format!("Received {}", trigger));
        let actions = self.config.get_actions(&trigger);
        match actions {
            Ok(acts) => {
                match acts {
                    Some(a) => {
                        self.apply_actions(a).unwrap();
                        Ok(())
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
                "sendfileserial" => self.install_to_arduino(&action_vec[1..].to_vec().join("_")).unwrap(),
                _ => self.do_nothing().unwrap(),
            };
        }
        Ok(())
    }

    // TODO this should go into comm
    /// This one should avrdude to send a given file to the arduino
    pub fn install_to_arduino(&mut self, filename: &str) -> Result<(), BrainDeadError> {
        // First check that avrdude is installed
        let mut _check_prog = match self.check_requirement("avrdude") {
            Ok(_v) => {
    // This sudo cant be right
    // TODO: send a different error if the file is not there (unter anderem)
                let status = Command::new("sudo")
                        .arg("avrdude")
                        .arg("-c")
                        .arg("linuxgpio")
                        .arg("-p")
                        .arg("atmega328p")
                        .arg("-v")
                        .arg("-U")
                        .arg(format!("flash:w:{}:i", filename))
                        .status()
                        .expect("process failed to execute");
                match status.code() {
                    Some(code) => {
                        match code {
                            0 => return Ok(()),
                            _ => {
                                log(Some(&self.name), "E", &format!("ERROR while installing {}!", filename));
                                return Err(BrainDeadError::AvrdudeError)
                            },
                        }
                    },
                    _ => {
                        log(Some(&self.name), "E", &format!("ERROR while installing {}!", filename));
                        return Err(BrainDeadError::AvrdudeError)
                            },
                    };
                },
            Err(e) => return Err(e),
        };
    }

    // TODO this should go into comm
    /// Do nothing, but take note that we have nothing to do
    pub fn do_nothing(&mut self) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "D", "Relaxing here...");
        Ok(())
    }

    // TODO this should go into comm
    /// Check that a given program is installed
    pub fn check_requirement(&mut self, prog: &str) -> Result<(), BrainDeadError> {
        let status = Command::new("which")
                .arg(prog)
                .status()
                .expect("");
        match status.code() {
            Some(code) => {
                match code {
                    0 => Ok(()),
                    _ => {
                        log(Some(&self.name), "E", &format!("{} is not installed!", prog));
                        Err(BrainDeadError::ProgNotInstalledError(prog.to_string()))
                    },
                }
            },
            _ => {
                log(Some(&self.name), "E", &format!("{} is not installed!", prog));
                Err(BrainDeadError::ProgNotInstalledError(prog.to_string()))
                    },
        }
    }
}
