extern crate serial;
use serial::prelude::*;
use std::io::{BufRead, BufReader};
use std::io;
use std::str;
use std::time::Duration;
use thiserror::Error;
use std::process::Command;
use std::{thread, time};

use std::sync::mpsc::Sender;
use log::{debug, error};

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

#[derive(Clone)]
pub struct Arduino {
    pub name: String,
    pub serialport: String,
}

impl Arduino {
    pub fn new(arduino_name: String, raw_serial_port: Option<String>) -> Result<Self, String> {
        let serial_port = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0".to_string(),
        };
        Ok(Self {
            name: arduino_name,
            serialport: serial_port,
        })
    }


    pub fn read_channel_mock(&mut self, channel: Sender<String>) -> Result<String, BrainArduinoError> {
        debug!("...reading from Mocked Serial Port");
//        loop {
        let got = "SENSOR: button=0".to_string();
        thread::sleep(time::Duration::from_secs(1));
        match channel.send(got){
            Ok(c) => debug!("- Forwarded to brain: {:?} ", c),
            Err(_e) => (),
        };
        Ok("".to_string())
//        }
    }

    pub fn read_channel(&mut self, channel: Sender<String>) -> Result<String, BrainArduinoError> {
        info!("...reading from Serial Port {}", self.serialport);
        let mut port = serial::open(&self.serialport).unwrap();
        loop {
            let got = self.interact(&mut port).unwrap();
            if got != "" {
                if got.contains("ACTION: ") {
                    debug!("- Received Action message: {}", got);
                    channel.send(got).unwrap();
                } else if got.contains("SENSOR: ") {
                    debug!("- Received Sensor message: {}", got);
                    channel.send(got).unwrap();
                } else {
                    debug!("- Read ->{}<- from Serial Port", got);
                    break Ok(got)
                }
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
                    debug!("- Received Log message: {}", &res);
                    Ok("".to_string())
                } else {
                    Ok(res)
                }
            },
            Err(_e) => {
                    Ok("".to_string())
                },
        }
    }

    /// This one should avrdude to send a given file to the arduino
    pub fn install(&mut self, filename: &str) -> Result<(), BrainArduinoError> {
        // First check that avrdude is installed
        debug!("- Installing {} to arduino", filename);
        let mut _check_prog = match self.check_requirement("avrdude") {
            Ok(_v) => {
    // To test avrdude manually: sudo avrdude -c linuxgpio -p atmega328p -v 
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
                                error!("ERROR while installing {}!", filename);
                                return Err(BrainArduinoError::AvrdudeError)
                            },
                        }
                    },
                    _ => {
                        error!("ERROR while installing {}!", filename);
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
                        error!("{} is not installed!", prog);
                        Err(BrainArduinoError::ProgNotInstalledError(prog.to_string()))
                    },
                }
            },
            _ => {
                error!("{} is not installed!", prog);
                Err(BrainArduinoError::ProgNotInstalledError(prog.to_string()))
            },
        }
    }
}
