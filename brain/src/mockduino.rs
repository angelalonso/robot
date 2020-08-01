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

pub struct Mockduino<'a> {
    pub connected: bool,
    pub msgfile_in: &'a str,
    pub msgfile_out: &'a str,
    pub config: Config,
}

impl Mockduino<'_> {
    pub fn new(config_file: &'static str, msg_file_in: &'static str, msg_file_out: &'static str) -> Self {
        let connected = true;
        let mut configdata = Config::new(config_file);
        configdata.print();
        Self {
            connected: connected,
            msgfile_in: msg_file_in,
            msgfile_out: msg_file_out,
            config: configdata,
        }
    }
    // TODO: how to make this a result of string and error?
    pub fn bootload(&mut self) -> Result<String, String> {
		thread::sleep(time::Duration::from_secs(4));
        let text = "New line 2".to_string();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.msgfile_in)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", text) {
            eprintln!("Couldn't write to file: {}", e);
        }
        Ok("Booted".to_string())
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
        println!("Mockduino is waiting for data...");
        loop {
            let results = self.read_msg();
            self.get_actions(&results.unwrap());
        }
        //Ok(())
    }
    
    // ------------------------------------------------------ //
    pub fn get_actions(&mut self, trigger: &str) {
        println!("Tx - {:?}", trigger);
        let mut actions = self.config.get_actions(trigger);
        match actions {
            Some(a) => self.do_actions(a),
            None => println!("Nothing to do"),
        }
    }
    pub fn do_actions(&mut self, actions: Vec<String>) {
        for action in &actions {
            let act: &str = &action[..];
            match act {
                "do_beep" => self.do_beep(),
                _ => self.do_nothing(),
            };
        }
    }

    // ------------------------------------------------------ //
    pub fn do_beep(&mut self) {
        let filename = "from_mockduino.q";
        let text = "Result->Beep".to_string();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", text) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    pub fn do_nothing(&mut self) {
        println!("Relaxing here...");
    }
}
