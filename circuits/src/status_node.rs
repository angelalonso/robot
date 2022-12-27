use crate::comms::*;

use log::debug;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub struct StatusNode<'a> {
    port_in: &'a str,
}

impl<'a> StatusNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        let node = match get_port(name, conns.clone()) {
            Ok(c) => StatusNode { port_in: c },
            Err(_) => {
                // TODO: this should fail instead
                StatusNode { port_in: "" }
            }
        };
        node
    }

    pub fn talk(&mut self) {
        let comms = UDPComms::new(self.port_in.to_owned());
        let mut status: HashMap<String, String> = HashMap::new();
        let (tx, rx) = mpsc::channel();
        loop {
            //debug!("status LOOP");
            let this_tx = tx.clone();
            let mut this_comms = comms.clone();
            let h = thread::spawn(move || {
                this_comms.get_data(this_tx);
            });
            let rcvd = rx.recv().unwrap();
            // IMPORTANT! We need a little bit of delay here
            std::thread::sleep(std::time::Duration::from_millis(1));
            debug!("[status] Received: {}", rcvd);
            if rcvd.contains("SET:") {
                let split1: Vec<&str> = rcvd.split(":").collect();
                status.insert(split1[1].to_owned(), split1[2].to_owned());
            } else if rcvd.contains("GET:") {
                let split1: Vec<&str> = rcvd.split("|").collect();
                let callback = split1[1];
                let split2: Vec<&str> = split1[0].split(":").collect();
                let key = split2[1];
                match status.get(key) {
                    Some(v) => {
                        //debug!("sending {} -> {}", v, callback);
                        comms.send_to(&v.as_bytes().to_vec(), callback);
                    }
                    None => (),
                };
            }
            h.join().unwrap();
            //debug!("status LOOP END");
        }
    }
}
