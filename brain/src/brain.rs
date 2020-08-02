use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io;
use std::str;
use std::{thread, time};
use crate::config::Config;

pub struct Brain<'a> {
    pub name: String,
    pub msgfile_in: &'a str,
    pub msgfile_out: &'a str,
    pub config: Config,
}

impl Brain<'_> {
    pub fn new(config_file: &'static str, msg_file_in: &'static str, msg_file_out: &'static str) -> Self {
        let configdata = Config::new(config_file);
        Self {
            name: "Main Brain".to_string(),
            msgfile_in: msg_file_in,
            msgfile_out: msg_file_out,
            config: configdata,
        }
    }
    // Read from a File like it's a message queue
    pub fn read_msg(&mut self) -> Result<String, io::Error> {
        let fileinfo = fs::metadata(self.msgfile_in)?;
        let modded = fileinfo.modified()?;
        let datasize = fileinfo.len();

        let mut exit = false;
        let change : u64;
        while !exit {
            let mod_fileinfo = fs::metadata(self.msgfile_in)?;
            let new_modded = mod_fileinfo.modified()?;

            if modded != new_modded {
                thread::sleep(time::Duration::from_millis(5));
                exit = true;
            }
        }
        let size_fileinfo = fs::metadata(self.msgfile_in)?;
        let new_datasize = size_fileinfo.len();
        change = new_datasize - datasize;
        let mut f = File::open(self.msgfile_in)?;
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
    pub fn read_msgs(&mut self) -> Result<(), Box<dyn Error>> {
        println!("{} is waiting for data...", self.name);
        loop {
            let results = self.read_msg();
            self.get_actions(&results.unwrap());
        }
        //Ok(())
    }
    
    // ------------------------------------------------------ //
    pub fn get_actions(&mut self, trigger: &str) {
        println!("{} - {:?}", self.name, trigger);
        let actions = self.config.get_actions(trigger);
        match actions {
            Some(a) => self.do_actions(a),
            None => println!("{} has nothing to do", self.name),
        }
    }
    pub fn do_actions(&mut self, actions: Vec<String>) {
        for action in &actions {
            let act: &str = &action[..];
            match act {
                "send_do_ping" => self.send("Do->Ping"),
                "send_do_pong" => self.send("Do->Pong"),
                _ => self.do_nothing(),
            };
        }
    }

    // ------------------------------------------------------ //
    pub fn do_nothing(&mut self) {
        println!("{} - Relaxing here...", self.name);
    }
    pub fn send(&mut self, message: &str) {
        let mut rng = rand::thread_rng();
		thread::sleep(time::Duration::from_millis(rng.gen_range(100, 4000)));
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.msgfile_out)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", message) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
