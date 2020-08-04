use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io;
use std::str;
use std::{thread, time};

pub struct Message {
    pub incoming: bool,
    pub message: String,
}

pub struct Messages {
    pub received: Vec<Message>,
    pub transmitted: Vec<Message>,
}

impl Message {
    pub fn new(inc: bool, msg: String) -> Self {
        Self {
            incoming: inc,
            message: msg,
        }
    }
}

impl Messages {
    pub fn new() -> Self {
        let rx = Vec::new();
        let tx= Vec::new();
        Self {
            received: rx,
            transmitted: tx,
        }
    }
    pub fn add(&mut self, incoming: bool, message: String) {
        if incoming {
            self.received.push(Message {incoming, message})
        } else {
            self.transmitted.push(Message {incoming, message})
        }
    }
    // TODO: result can be an error too, we just send empty though
    pub fn get_list(&mut self, vec_type: &str) -> Option<Vec<String>> {
        match vec_type {
            "received" => {
                let mut recvvec = Vec::new();
                for entry in &mut self.received {
                    recvvec.push(entry.message.to_string());
                }
                Some(recvvec)
            },
            "transmitted" => {
                let mut trnsvec = Vec::new();
                for entry in &mut self.transmitted {
                    trnsvec.push(entry.message.to_string());
                }
                Some(trnsvec)
            },
            &_ => {
                None
            },
        }
    }
    pub fn read_msg_mock(&mut self) -> Result<String, io::Error> {
        let filename = "from_mockduino.q";
        let metadata = fs::metadata(filename)?;
        let modded = metadata.modified()?;
        let datasize = metadata.len();

        let mut exit = false;
        let change : u64;
        while !exit {
            let mod_metadata = fs::metadata(filename)?;
            let new_modded = mod_metadata.modified()?;

            if modded != new_modded {
                thread::sleep(time::Duration::from_millis(5));
                exit = true;
            }
        }
        let size_metadata = fs::metadata(filename)?;
        let new_datasize = size_metadata.len();
        change = new_datasize - datasize;
        let mut f = File::open(filename)?;
        f.seek(SeekFrom::Start(datasize)).unwrap();
        let mut buf = vec![];
        let mut chunk = f.take(change);
        let _n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        Ok(s.to_string())
    }
    pub fn read_msgs_mock(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Waiting for data...");
        loop {
            let results = self.read_msg_mock();
            self.trigger_action(&results.unwrap());
        }
        //Ok(())
    }
    pub fn read_msgs(&mut self) -> String {
        String::from("Test")
    }
    pub fn trigger_action(&mut self, trigger: &str) {
        println!("Rx - {:?}", trigger);
    }
}
