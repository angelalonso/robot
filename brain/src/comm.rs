use crate::log;
use std::process::Command;
use thiserror::Error;
use std::io;
use tokio_util::codec::{Decoder, Encoder};

use futures::stream::StreamExt;
use bytes::BytesMut;
use std::str;

use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use std::thread;
//use futures::executor;
use tokio::runtime::Runtime;

use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate serial;

use serial::prelude::*;
use std::io::prelude::*;

use std::env;
use std::time::Duration;


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

    ///// This is the loop that keeps calling to read from serial
    //#[tokio::main]
    //pub async fn read_loop(&mut self) -> Result<(), BrainCommError> {
    //    log(Some(&self.name), "D", "Waiting for data...");
    //    loop {
    //        let results = self.read_one_from_serialport().await;
    //        //println!("RECEIVED {:?}", results);
    //        //TODO: does the following break working code?
    //        let _taken_actions = match self.get_actions(&results.unwrap()){
    //            Ok(_) => (),
    //            Err(_) => log(Some(&self.name), "D", "No actions were found for trigger"),
    //        };
    //    }
    //}

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

    /// --------------------------- TO BE DELETED
    pub fn read_channel_stdio(&mut self) -> Receiver<String> {
        let (tx, rx) = mpsc::channel::<String>();
        thread::spawn(move || loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            tx.send(buffer).unwrap();
        });
        rx
    }
    /// --------------------------------------------------
    #[tokio::main]
    pub async fn read_one_from_serial(&mut self) -> Result<String, BrainCommError> {
        log(Some(&self.name), "D", &format!("Reading from Serial Port {}", self.serialport));
        //Err(BrainDeadError::EmptyError)
        let settings = tokio_serial::SerialPortSettings::default();
        let mut port = tokio_serial::Serial::from_path(self.serialport, &settings).unwrap();

        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("Unable to set serial port exclusive to false");

        let mut reader = LineCodec.framed(port);

        #[allow(clippy::never_loop)] while let Some(line_result) = reader.next().await {
            let line = line_result.expect("Failed to read line");
            return Ok(line)
        }
        Ok("".to_string())
    }
    pub fn read_one_from_serial_new(&mut self) -> Result<String, BrainCommError> {
        log(Some(&self.name), "D", &format!("Reading from Serial Port {}", self.serialport));
        let f = File::open(self.serialport).expect("oh no");
        let mut f = BufReader::new(f);
        let mut read_buffer: Vec<u8> = Vec::new();
        f.read_until(b'V', &mut read_buffer).expect("reading from cursor won't fail");
        let mut string_list : Vec<String> = vec![];
        for line in f.lines() {
            match line {
                Ok(content) => string_list.push(content),
                Err(_) => (),
            };
        }
        Ok(string_list.join(" "))
    }
    fn read_until<R: BufRead>(mut read: R, out: &mut Vec<u8>, pair: (u8, u8)) -> Result<usize, BrainCommError> {
        let mut bytes_read = 0;
        let mut got_possible_terminator = false;
        
        loop {
            let buf = read.fill_buf()?;
            if buf.len() == 0 { return Ok(bytes_read); } // EOF
            
            let mut seen = 0;
            
            for byte in buf.iter().copied() {
                seen += 1;
                if got_possible_terminator && byte == pair.1 {
                    out.pop(); // remove first half of terminator
                    read.consume(seen);
                    return Ok(bytes_read + seen - 2);
                }
                out.push(byte);
                got_possible_terminator = byte == pair.0;
            }
            let len = buf.len();
            read.consume(len);
            bytes_read += len;
        }
    }

    pub fn read_channel(&mut self) {
        log(Some(&self.name), "D", &format!("Reading from Serial Port {}", self.serialport));
        let mut port = serial::open(self.serialport).unwrap();
        self.interact(&mut port).unwrap();
    }
    /// ---------------------------
    fn interact<T: SerialPort>(&mut self, port: &mut T) -> io::Result<()> {
        log(Some(&self.name), "D", &format!("Running Interact..."));
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        port.set_timeout(Duration::from_millis(1000))?;

        let reader = BufReader::new(port);
        for line in reader.lines() {
            if line.is_ok() {
            println!("{:?}",  line.unwrap_or("Reading failed".into()));
            }
        }
        log(Some(&self.name), "D", &format!("Running INteract...DONE"));
        Ok(())
    }
}
