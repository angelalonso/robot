use crate::comms::*;

use log::debug;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub struct TestNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
}

impl<'a> TestNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        let node = match get_port(name, conns.clone()) {
            Ok(c) => TestNode { port_in: c, conns },
            Err(_) => {
                panic!(
                    "couldn't find a port to itself: {} (HINT: check name at main.rs)",
                    name
                );
            }
        };
        node
    }

    pub fn talk(&mut self) {
        let status_node = get_port("status", self.conns.clone()).unwrap();
        let led_node = get_port("led_action_server", self.conns.clone()).unwrap();
        let motor_l = get_port("motor_l_action_server", self.conns.clone()).unwrap();

        let comms = UDPComms::new(self.port_in.to_owned());
        let (tx, rx) = mpsc::channel();
        loop {
            //debug!("test LOOP");
            comms.send_to(&"SET:switch".as_bytes().to_vec(), led_node);
            //comms.send_to(&"SET:fwd".as_bytes().to_vec(), motor_l);
            std::thread::sleep(std::time::Duration::from_millis(50));
            comms.send_to(
                &format!("GET:led|{}", self.port_in).as_bytes().to_vec(),
                status_node,
            );
            let this_tx = tx.clone();
            let mut this_comms = comms.clone();
            let h = thread::spawn(move || {
                this_comms.get_data(this_tx);
            });
            let rcvd = rx.recv().unwrap();
            debug!("[test] Received: {}", rcvd);
            h.join().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
            //comms.send_to(&"SET:stp".as_bytes().to_vec(), motor_l);
            //std::thread::sleep(std::time::Duration::from_secs(1));
            //debug!("test LOOP END");
        }
    }
}
