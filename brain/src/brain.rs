
use rand::Rng;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io;
use std::str;
use std::time::Instant;
use std::{thread, time};
use crate::config::Config;
use crate::log;
use thiserror::Error;

use tokio_util::codec::{Decoder, Encoder};
use futures::stream::StreamExt;
use bytes::BytesMut;

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
#[derive(Error, Debug)]
pub enum BrainDeadError {
    /// It used to represent an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    /// Now it's just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    /// Represents the most basic error while sending a file (using avrdude)
    #[error("Something went wrong while using avrdude to send files")]
    SendFileError,

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { source: std::io::Error },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub struct Brain<'a> {
    pub name: &'a str,
    pub msgfile_in: &'a str,
    pub msgfile_out: &'a str,
    pub config: Config,
    pub serialport: &'a str,
    pub timeout: u64,
}

impl Brain<'_> {
    pub fn new(brain_name: &'static str, config_file: &'static str, msg_file_in: &'static str, msg_file_out: &'static str, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        let configdata = Config::new(config_file);
        let serial_port = match raw_serial_port {
            Some(port) => port,
            None => "/dev/tty7",
        };
        Ok(Self {
            name: brain_name,
            msgfile_in: msg_file_in,
            msgfile_out: msg_file_out,
            config: configdata,
            serialport: serial_port,
            timeout: 4,
        })
    }

    /// Future me: one day new_serial will replace new
    pub fn new_serial(brain_name: &'static str, config_file: &'static str, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        let configdata = Config::new(config_file);
        let serial_port = match raw_serial_port {
            Some(port) => port,
            None => "/dev/tty7",
        };
        Ok(Self {
            name: brain_name,
            msgfile_in: "none.q", // to be removed
            msgfile_out: "none.q", // to be removed
            config: configdata,
            serialport: serial_port,
            timeout: 4,
        })
    }

    // TODO: how to make this a result of string and error?
    /// aaaah nevermind, we will get rid of this one day
    pub fn bootload(&mut self) -> Result<String, String> {
        // Simulate delay on booting the entity
        log(Some(&self.name), "I", &format!("Booting {}...", self.name));
        let mut rng = rand::thread_rng();
		thread::sleep(time::Duration::from_millis(rng.gen_range(100, 2000)));
        log(Some(&self.name), "I", "Booted");
        Ok("Booted".to_string())
    }

    // Read new data from a File like it's a message queue
    /// Pffff "like it's a message queue" he says...
    pub fn read_msg(&mut self, timeout: u64) -> Result<String, io::Error> {
        // 3 metadatas:
        // base when we start
        // mod when the modified time changes
        // size when we finally get the new size
        let base_info = fs::metadata(self.msgfile_in)?;
        let modded = base_info.modified()?;
        let size = base_info.len();
        let basetime = Instant::now();

        let mut exit = false;
        let change : u64;
        while !exit {
            let mod_info = fs::metadata(self.msgfile_in)?;
            let new_modded = mod_info.modified()?;

            if modded != new_modded {
                thread::sleep(time::Duration::from_millis(5));
                exit = true;
            } else {
                let time_passed = basetime.elapsed().as_secs();
                if time_passed >= timeout {
                    exit = true
                }
            }
        }
        let size_info = fs::metadata(self.msgfile_in)?;
        let new_size = size_info.len();
        change = new_size - size;
        let mut f = File::open(self.msgfile_in)?;
        f.seek(SeekFrom::Start(size)).unwrap();
        let mut buf = vec![];
        let mut chunk = f.take(change);
        let _n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        Ok(s.to_string())
    }

    // Loop through read_msg and apply related actions
    /// Now this one here is the first one that might still make it to the final version
    pub fn read_msgs(&mut self) {
        log(Some(&self.name), "D", "Waiting for data...");
        loop {
            let results = self.read_msg(self.timeout);
            self.get_actions(&results.unwrap());
        }
    }

    /// OK this one might be useful too
    pub fn get_actions(&mut self, trigger: &str) {
        log(Some(&self.name), "D", &format!("Received {}", trigger));
        let actions = self.config.get_actions(&trigger);
        match actions {
            Some(a) => self.apply_actions(a),
            None => {
                log(Some(&self.name), "D", "Nothing to do");
            },
        };
    }

    pub fn do_actions(&mut self, actions: Vec<String>) {
        for action in &actions {
            let act: &str = &action[..];
            match act {
                "do_ping" => self.send("Result->Ping"),
                "do_pong" => self.send("Result->Pong"),
                "send_do_ping" => self.send("Do->Ping"),
                "send_do_pong" => self.send("Do->Pong"),
                _ => self.do_nothing(),
            };
        }
    }

    pub fn apply_actions(&mut self, actions: Vec<String>) {
        for action in &actions {
            let action_vec: Vec<&str> = action.split('_').collect();
            match action_vec[0] {
                "send" => self.send(&action_vec[1..].to_vec().join("_")),
                "sendfileserial" => self.sendfileserial(&action_vec[1..].to_vec().join("_")),
                _ => self.do_nothing(),
            };
        }
    }

    pub fn do_nothing(&mut self) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "D", "Relaxing here...");
        Ok(())
    }

    pub fn send(&mut self, message: &str) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "I", &format!("Sending {}", message));
        let mut rng = rand::thread_rng();
		thread::sleep(time::Duration::from_millis(rng.gen_range(100, 4000)));
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.msgfile_out)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", message) {
            //return Err(Box::new(e))
            return Err(BrainDeadError::IOError(e))
        }
        Ok(())
    }

    /// We actually need to read Serial,
    /// names apart, is this one the right one? or is it the next one?
    #[tokio::main]
    pub async fn readserial(&mut self, filename: &str) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "D", &format!("Sending file {} through Serial Port {}", filename, self.serialport));
        //Err(BrainDeadError::EmptyError)
        let settings = tokio_serial::SerialPortSettings::default();
        let mut port = tokio_serial::Serial::from_path(self.serialport, &settings).unwrap();

        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("Unable to set serial port exclusive to false");

        let mut reader = LineCodec.framed(port);

        while let Some(line_result) = reader.next().await {
            let line = line_result.expect("Failed to read line");
            println!("{}", line);
        }
        Ok(())
    }

    /// We actually need to read Serial,
    /// names apart, is this one the right one? or is it the previous one?
    #[tokio::main]
    pub async fn sendfileserial(&mut self, filename: &str) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "D", &format!("Sending file {} through Serial Port {}", filename, self.serialport));
        //Err(BrainDeadError::EmptyError)
        let settings = tokio_serial::SerialPortSettings::default();
        let mut port = tokio_serial::Serial::from_path(self.serialport, &settings).unwrap();

        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("Unable to set serial port exclusive to false");

        let mut reader = LineCodec.framed(port);

        while let Some(line_result) = reader.next().await {
            let line = line_result.expect("Failed to read line");
            println!("{}", line);
        }
        Ok(())
    }

    /// This one should avrdude to send a given file to the arduino
    pub fn sendfile_serial(&mut self, filename: &str) -> Result<(), BrainDeadError> {
        Err(BrainDeadError::SendFileError)
    }
}
