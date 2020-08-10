use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io;
use std::str;
use std::time::{Duration, Instant};
use std::{thread, time};
use crate::config::Config;
use crate::log;

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

    // TODO: how to make this a result of string and error?
    pub fn bootload(&mut self) -> Result<String, String> {
        // Simulate delay on booting the entity
        log(Some(&self.name), "I", &format!("Booting {}...", self.name));
        let mut rng = rand::thread_rng();
		thread::sleep(time::Duration::from_millis(rng.gen_range(100, 2000)));
        log(Some(&self.name), "I", "Booted");
        Ok("Booted".to_string())
    }

    // Read new data from a File like it's a message queue
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
    pub fn read_msgs(&mut self) -> Result<(), Box<dyn Error>> {
        log(Some(&self.name), "D", "Waiting for data...");
        loop {
            let results = self.read_msg(self.timeout);
            //self.get_actions(&results.unwrap());
            match &results {
                Ok(res) => self.get_actions(res),
                Err(e) => {
                    println!("Error Reading messages: {}", e);
                    Error(e)
                    //TODO: return a proper error here (?)
                },
            }
        }
    }

    pub fn get_actions(&mut self, trigger: &str) -> Result<(), io::Error> {
        log(Some(&self.name), "D", &format!("Received {}", trigger));
        let actions = self.config.get_actions(&trigger);
        match actions {
            //Some(a) => self.do_actions(a),
            Some(a) => self.apply_actions(a),
            None => {
                log(Some(&self.name), "D", "Nothing to do");
                Ok(())
            },
        };
        Ok(())
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

    pub fn apply_actions(&mut self, actions: Vec<String>) -> Result<(), io::Error> {
        for action in &actions {
            let action_vec: Vec<&str> = action.split('_').collect();
            match action_vec[0] {
                "send" => self.send(&action_vec[1..].to_vec().join("_")),
                "sendfileserial" => self.sendfileserial(&action_vec[1..].to_vec().join("_")),
                _ => self.do_nothing(),
            };
        }
        Ok(())
    }

    pub fn do_nothing(&mut self) -> Result<(), io::Error> {
        log(Some(&self.name), "D", "Relaxing here...");
        Ok(())
    }

    pub fn send(&mut self, message: &str) -> Result<(), io::Error> {
        log(Some(&self.name), "I", &format!("Sending {}", message));
        let mut rng = rand::thread_rng();
		thread::sleep(time::Duration::from_millis(rng.gen_range(100, 4000)));
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.msgfile_out)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", message) {
            return Err(Box::new)or(e)
        }
        Ok(())
    }

    pub fn sendfileserial(&mut self, filename: &str) -> Result<(), io::Error> {
        log(Some(&self.name), "D", &format!("Sending file {} through Serial Port {}", filename, self.serialport));
        Ok(())
    }
}
