extern crate serial;
extern crate serde_derive;
extern crate serde_yaml;

use log::{debug, error};
use serial::prelude::*;
use std::io::{BufRead, BufReader};
use std::io;
use std::sync::mpsc::{SyncSender, Sender, Receiver};
use std::time::Duration;
use std::{thread};
use thiserror::Error;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

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
            //None => "/dev/ttyUSB0".to_string(),
            None => "/dev/ttyACM0".to_string(),
        };
        Ok(Self {
            name: arduino_name,
            serialport: serial_port,
        })
    }


    //pub fn read_channel(&mut self, channel: Sender<String>) -> Result<String, BrainArduinoError> {
    pub fn read_channel(&mut self, channel: SyncSender<String>) -> Result<String, BrainArduinoError> {
        println!("...reading Arduino messages from Serial Port {}", &self.serialport);
        let mut port = serial::open(&self.serialport).unwrap();
        loop {
            println!("read_channel");
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

        port.set_timeout(Duration::from_millis(1000))?;

        // NOTE: This is needed because we want to initiate conversation from the Brain
        //   The arduino program does not publish anything until Brain sends something (a 0) on the
        //   Serial port. We do this on every loop.
        // What we need to solve is the problem that the Arduino program boots and starts sending
        //   before our brain  program is ready.
        let buf: Vec<u8> = (0..1).collect();
        port.write_all(&buf[..])?;

        let reader = BufReader::new(port);
        let mut lines = reader.lines();
        match lines.next().unwrap() {
            Ok(res) => {
                if res.contains("LOG:") {
                    debug!("- Received Log message: {}", &res);
                    Ok("".to_string())
                } else {
                    debug!("- Received Other message: {}", &res);
                    Ok(res)
                }
            },
            Err(_e) => {
                    Ok("".to_string())
                },
        }
    }

}

pub fn use_received_msg(raw_msg: String) {
    println!("{:?}", raw_msg);
}

/// Return current timestamp as millis
pub fn get_current_time() -> f64 {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_millis() as f64,
        Err(_e) => 0.0,
    }
}

pub fn get_time_since(start_timestamp: f64) -> f64 {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(time) => (time.as_millis() as f64 - start_timestamp as f64) / 1000_f64,
        Err(_e) => 0.0,
    }
}

fn main() {
    env_logger::builder()
    .format_timestamp_millis()
    .init();

    let precission = 100;

    let a = Arduino::new("arduino".to_string(), Some("/dev/ttyACM0".to_string())).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Arduino: {}", err);
        process::exit(1);
    });

    let start_timestamp = get_current_time();

    let mut port = serial::open(&"/dev/ttyACM0".to_string()).unwrap();
    let mut ct: f64 = 0.0;
    let (s, r): (SyncSender<String>, Receiver<String>) = std::sync::mpsc::sync_channel(2);
    //let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
    let mut ac = a.clone();
    let mut ac_loop = a.clone();
    let msgs = s.clone();
    thread::spawn(move || {
        ac.read_channel(msgs).unwrap();
    });
    loop {
        let ct_raw = get_time_since(start_timestamp);
        let new_ct = (ct_raw * precission as f64).floor() / precission as f64;
        if new_ct > ct { 
            ac_loop.interact(&mut port).unwrap();
            ct = new_ct;
            if let Ok(m) = r.try_recv() { use_received_msg(m) }
        }
    }
}
