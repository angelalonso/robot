use crate::log;
use std::process::Command;
use thiserror::Error;
use std::io;
use tokio_util::codec::{Decoder, Encoder};
use futures::stream::StreamExt;
use bytes::BytesMut;
use std::str;

#[derive(Error, Debug)]
pub enum BrainCommError {
    /// It used to represent an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    /// Now it's just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

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

pub struct Comm<'a> {
    pub name: &'a str,
    pub serialport: &'a str,
}

impl Comm<'_> {
    pub fn new(comm_name: &'static str, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        let serial_port = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0",
        };
        Ok(Self {
            name: comm_name,
            serialport: serial_port,
        })
    }

    /// This is the loop that keeps calling to read from serial
    #[tokio::main]
    pub async fn read_loop(&mut self) -> Result<(), BrainCommError> {
        log(Some(&self.name), "D", "Waiting for data...");
        loop {
            let results = self.read_one_from_serialport().await;
            //println!("RECEIVED {:?}", results);
            //TODO: does the following break working code?
            let _taken_actions = match self.get_actions(&results.unwrap()){
                Ok(_) => (),
                Err(_) => log(Some(&self.name), "D", "No actions were found for trigger"),
            };
        }
    }

    /// Read one text from the serial port "give me one text"
    // TODO: sort out that we only receive the first thing from Serial
    //  - shall we use join thread somewhere?
    //  - Shall we force it to read several times?
    pub async fn read_one_from_serialport(&mut self) -> Result<String, BrainCommError> {
        log(Some(&self.name), "D", &format!("Reading from Serial Port {}", self.serialport));
        //Err(BrainCommError::EmptyError)
        let settings = tokio_serial::SerialPortSettings::default();
        let mut port = tokio_serial::Serial::from_path(self.serialport, &settings).unwrap();

        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("Unable to set serial port exclusive to false");

        let mut reader = LineCodec.framed(port);

        #[allow(clippy::never_loop)] while let Some(line_result) = reader.next().await {
            let line = line_result.expect("Failed to read line");
            //println!("{}", line);
            return Ok(line)
        }
        Ok("".to_string())
    }

    /// This one should avrdude to send a given file to the arduino
    pub fn install_to_arduino(&mut self, filename: &str) -> Result<(), BrainCommError> {
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
                                return Err(BrainCommError::AvrdudeError)
                            },
                        }
                    },
                    _ => {
                        log(Some(&self.name), "E", &format!("ERROR while installing {}!", filename));
                        return Err(BrainCommError::AvrdudeError)
                            },
                    };
                },
            Err(e) => return Err(e),
        };
    }

    /// Check that a given program is installed
    pub fn check_requirement(&mut self, prog: &str) -> Result<(), BrainCommError> {
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
                        Err(BrainCommError::ProgNotInstalledError(prog.to_string()))
                    },
                }
            },
            _ => {
                log(Some(&self.name), "E", &format!("{} is not installed!", prog));
                Err(BrainCommError::ProgNotInstalledError(prog.to_string()))
                    },
        }
    }
}
