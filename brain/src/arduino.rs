extern crate serial;
use crate::log;
use serial::prelude::*;
use std::io::{BufRead, BufReader};
use std::io;
use std::str;
use std::time::Duration;
use thiserror::Error;
use std::process::Command;


#[derive(Error, Debug)]
pub enum BrainArduinoError {
    /// It used to represent an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    /// Now it's just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("{0} is NOT installed (or something went wrong while checking that it is)")]
    ProgNotInstalledError(String),

    #[error("AvrDude could not install the program to your Arduino!")]
    AvrdudeError,
}

pub struct Arduino<'a> {
    pub name: &'a str,
    pub serialport: &'a str,
}

impl Arduino<'_> {
    pub fn new(arduino_name: &'static str, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        let serial_port = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0",
        };
        Ok(Self {
            name: arduino_name,
            serialport: serial_port,
        })
    }

    pub fn read_channel(&mut self) -> Result<String, BrainArduinoError> {
        log(Some(&self.name), "D", &format!("Reading from Serial Port {}", self.serialport));
        let mut port = serial::open(self.serialport).unwrap();
        loop {
            let got = self.interact(&mut port).unwrap();
            if got != "" {
                log(Some(&self.name), "D", &format!("Read ->{}<- from Serial Port", got));
                break Ok(got)
            }
        }
    }

    pub fn interact<T: SerialPort>(&mut self, port: &mut T) -> io::Result<String> {
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        port.set_timeout(Duration::from_millis(100))?;

        let reader = BufReader::new(port);
        let mut lines = reader.lines();
        match lines.next().unwrap() {
            Ok(res) => {
                if res.contains("LOG:") {
                    log(Some(&self.name), "D", &format!("Got a Log message: {}", &res));
                    Ok("".to_string())
                } else {
                    //log(Some(&self.name), "D", &format!("Got a Result: ->{}<-", &res));
                    Ok(res)
                }
            },
            Err(_e) => {
                    //log(Some(&self.name), "D", &format!("Got an Error Reading from Port, {:?}", e));
                    Ok("".to_string())
                },
        }
    }

    /// This one should avrdude to send a given file to the arduino
    pub fn install(&mut self, filename: &str) -> Result<(), BrainArduinoError> {
        // First check that avrdude is installed
        log(Some(&self.name), "D", &format!("Installing {} to arduino", filename));
        let mut _check_prog = match self.check_requirement("avrdude") {
            Ok(_v) => {
    // This sudo cant be right
    // TODO: send a different error if the file is not there (unter anderem)
                let run = Command::new("sudo")
                        .arg("avrdude")
                        .arg("-c")
                        .arg("linuxgpio")
                        .arg("-p")
                        .arg("atmega328p")
                        .arg("-v")
                        .arg("-U")
                        .arg(format!("flash:w:{}:i", filename))
                        .output()
                        .expect("process failed to execute");
                match run.status.code() {
                    Some(code) => {
                        match code {
                            0 => return Ok(()),
                            _ => {
                                log(Some(&self.name), "E", &format!("ERROR while installing {}!", filename));
                                return Err(BrainArduinoError::AvrdudeError)
                            },
                        }
                    },
                    _ => {
                        log(Some(&self.name), "E", &format!("ERROR while installing {}!", filename));
                        return Err(BrainArduinoError::AvrdudeError)
                            },
                    };
                },
            Err(e) => return Err(e),
        };
    }

    /// Check that a given program is installed
    pub fn check_requirement(&mut self, prog: &str) -> Result<(), BrainArduinoError> {
        let check = Command::new("which")
                .arg(prog)
                .output()
                .expect("");
        match check.status.code() {
            Some(code) => {
                match code {
                    0 => Ok(()),
                    _ => {
                        log(Some(&self.name), "E", &format!("{} is not installed!", prog));
                        Err(BrainArduinoError::ProgNotInstalledError(prog.to_string()))
                    },
                }
            },
            _ => {
                log(Some(&self.name), "E", &format!("{} is not installed!", prog));
                Err(BrainArduinoError::ProgNotInstalledError(prog.to_string()))
                    },
        }
    }
}
