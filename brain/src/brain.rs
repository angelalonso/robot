// TODO: remove dependencies that are now commented (22.08.2020)
use crate::config::Config;
use crate::comm::Comm;
use crate::log;
use std::io;
use std::process::Command;
use std::str;
use thiserror::Error;

use tokio_util::codec::{Decoder, Encoder};
//use futures::stream::StreamExt;
use bytes::BytesMut;

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

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, _item: Self::Item, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
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

//    /// This is the loop that keeps calling to read from serial
//    #[tokio::main]
//    pub async fn read(&mut self) -> Result<(), BrainDeadError> {
//        log(Some(&self.name), "D", "Waiting for data...");
//        loop {
//            let results = self.read_one_from_serialport().await;
//            //println!("RECEIVED {:?}", results);
//            //TODO: does the following break working code?
//            let _taken_actions = match self.get_actions(&results.unwrap()){
//                Ok(_) => (),
//                Err(_) => log(Some(&self.name), "D", "No actions were found for trigger"),
//            };
//        }
//    }

    pub fn read_new(&mut self) {
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


//    /// Read one text from the serial port "give me one text"
//    // TODO: sort out that we only receive the first thing from Serial
//    //  - shall we use join thread somewhere?
//    //  - Shall we force it to read several times?
//    pub async fn read_one_from_serialport(&mut self) -> Result<String, BrainDeadError> {
//        log(Some(&self.name), "D", &format!("Reading from Serial Port {}", self.serialport));
//        //Err(BrainDeadError::EmptyError)
//        let settings = tokio_serial::SerialPortSettings::default();
//        let mut port = tokio_serial::Serial::from_path(self.serialport, &settings).unwrap();
//
//        #[cfg(unix)]
//        port.set_exclusive(false)
//            .expect("Unable to set serial port exclusive to false");
//
//        let mut reader = LineCodec.framed(port);
//
//        #[allow(clippy::never_loop)] while let Some(line_result) = reader.next().await {
//            let line = line_result.expect("Failed to read line");
//            //println!("{}", line);
//            return Ok(line)
//        }
//        Ok("".to_string())
//    }

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

    /// Do nothing, but take note that we have nothing to do
    pub fn do_nothing(&mut self) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "D", "Relaxing here...");
        Ok(())
    }

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

//    pub fn do_actions(&mut self, trigger: &str) -> Result<(), BrainDeadError> {
//        log(Some(&self.name), "D", &format!("Received {}", trigger));
//        let actions = self.config.get_actions(&trigger);
//        match actions {
//            Ok(acts) => {
//                match acts {
//                    Some(a) => {
//                        self.apply_actions(a).unwrap();
//                        Ok(())
//                    },
//                    None => {
//                        log(Some(&self.name), "D", "Nothing to do");
//                        Err(BrainDeadError::NoConfigFound)
//                    },
//                }
//            },
//            Err(_e) => {
//                log(Some(&self.name), "D", "Got an error while looking for actions");
//                Err(BrainDeadError::NoConfigFound)
//            },
//        }
//    }
}
