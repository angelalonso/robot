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
use crate::log;

pub struct Brain<'a> {
    pub name: &'a str,
    pub msgfile_in: &'a str,
    pub msgfile_out: &'a str,
    pub config: Config,
}

impl Brain<'_> {
    pub fn new(brain_name: &'static str, config_file: &'static str, msg_file_in: &'static str, msg_file_out: &'static str) -> Self {
        let configdata = Config::new(config_file);
        Self {
            name: brain_name,
            msgfile_in: msg_file_in,
            msgfile_out: msg_file_out,
            config: configdata,
        }
    }
    // ------------------------------------------------------ //
    // TODO: how to make this a result of string and error?
    pub fn bootload(&mut self) -> Result<String, String> {
        // Simulate delay on booting the entity
        log(Some(&self.name), "I", &format!("Booting {}...", self.name));
        let mut rng = rand::thread_rng();
		thread::sleep(time::Duration::from_millis(rng.gen_range(100, 2000)));
        log(Some(&self.name), "I", "Booted");
        Ok("Booted".to_string())
    }
    // ------------------------------------------------------ //
    // Read new data from a File like it's a message queue
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
    // ------------------------------------------------------ //
    // Loop through read_msg and apply related actions
    pub fn read_msgs(&mut self) -> Result<(), Box<dyn Error>> {
        log(Some(&self.name), "D", "Waiting for data...");
        loop {
            let results = self.read_msg();
            //self.get_actions(&results.unwrap());
            match &results {
                Ok(res) => self.get_actions(res),
                Err(e) => {
                    println!("Error Reading messages: {}", e);
                    //TODO: return a proper error here
                    break Ok(())},
            }
        }
    }
    // ------------------------------------------------------ //
    pub fn get_actions(&mut self, trigger: &str) {
        log(Some(&self.name), "I", trigger);
        let actions = self.config.get_actions(trigger);
        match actions {
            //Some(a) => self.do_actions(a),
            Some(a) => self.apply_actions(a),
            None => log(Some(&self.name), "D", "Nothing to do"),
        }
    }
    // ------------------------------------------------------ //
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
    // ------------------------------------------------------ //
    pub fn apply_actions(&mut self, actions: Vec<String>) {
        for action in &actions {
            let action_vec: Vec<&str> = action.split("_").collect();
            match action_vec[0] {
                // TODO: add sendfile_ action
                // TODO: normalize what comes after send_
                "send" => self.send(&action_vec[1..].to_vec().join("_")),
                _ => self.do_nothing(),
            }
        }
    }
    // ------------------------------------------------------ //
    pub fn do_nothing(&mut self) {
        log(Some(&self.name), "D", "Relaxing here...");
    }
    // ------------------------------------------------------ //
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
// ---------------------------------------------------------- //
#[cfg(test)]
mod brain_tests {
    use super::*;

    #[test]
    fn read_msg() {
        // I'll just test an error here
        let mut test = Brain::new("test", "testfiles/test.cfg.yaml", "testfiles/unexistingtest_from_mock.q", "testfiles/unexistingtest_to_mock.q");
        match test.read_msg() {
            Ok(read) => println!("{}", read),
            Err(e) => println!("{:?}", e),
        };
    }

    #[test]
    fn read_msgs() {
        // I'll just test an error here
        let mut test = Brain::new("test", "testfiles/test.cfg.yaml", "testfiles/unexistingtest_from_mock.q", "testfiles/unexistingtest_to_mock.q");
        match test.read_msgs() {
            Ok(read) => println!("{:?}", read),
            Err(e) => println!("{:?}", e),
        };
    }

    #[test]
    fn get_actions() {
        let mut test = Brain::new("test", "testfiles/test.cfg.yaml", "testfiles/test_from_mock.q", "testfiles/test_to_mock.q");
        test.get_actions("Ping\n");
    }

    #[test]
    fn apply_actions() {
        let mut test = Brain::new("test", "testfiles/test.cfg.yaml", "testfiles/test_from_mock.q", "testfiles/test_to_mock.q");
        test.apply_actions(Vec::from(["send_ping".to_string()]));

    }

    #[test]
    fn send() {
        let mut test = Brain::new("test", "testfiles/test.cfg.yaml", "testfiles/test_from_mock.q", "testfiles/test_to_mock.q");
        test.send("Do->Ping");

    }
}
