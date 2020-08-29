// TODO : this maybe should be called arduino_comm
extern crate serial;
use crate::log;
use serial::prelude::*;
use std::io::{BufRead, BufReader};
use std::io;
use std::str;
use std::time::Duration;
use thiserror::Error;


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

    pub fn read_channel(&mut self) -> Result<String, BrainCommError> {
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

    fn interact<T: SerialPort>(&mut self, port: &mut T) -> io::Result<String> {
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        port.set_timeout(Duration::from_millis(500))?;

        let reader = BufReader::new(port);
        match reader.lines().next().unwrap() {
            Ok(res) => Ok(res),
            Err(_) => Ok("".to_string()),
        }
    }
}
