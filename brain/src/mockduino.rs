use std::{thread, time};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct Mockduino<'a> {
    pub connected: bool,
    pub msgbus_file: &'a str,
}

impl Mockduino<'_> {
    pub fn new(msg_filename: &'static str) -> Self {
        let mut connected = true;
        Self {
            connected: connected,
            msgbus_file: msg_filename,
        }
    }
    // TODO: how to make this a result of string and error?
    pub fn bootload(&mut self) -> Result<String, String> {
		thread::sleep(time::Duration::from_secs(4));
        let text = "New line 2".to_string();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.msgbus_file)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", text) {
            eprintln!("Couldn't write to file: {}", e);
        }
        Ok("Done".to_string())
    }
}
