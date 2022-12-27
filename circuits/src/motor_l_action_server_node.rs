use crate::comms::*;

use log::{debug, info};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub struct MotorLActionServerNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
    speed: i8,
}

impl<'a> MotorLActionServerNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        let node = match get_port(name, conns.clone()) {
            Ok(c) => MotorLActionServerNode {
                port_in: c,
                conns,
                speed: 0,
            },
            Err(_) => {
                // TODO: this should fail instead
                MotorLActionServerNode {
                    port_in: "",
                    conns,
                    speed: 0,
                }
            }
        };
        node
    }

    pub fn talk(&mut self) {
        let status_node = get_port("status", self.conns.clone()).unwrap();
        let comms = UDPComms::new(self.port_in.to_owned());

        let (tx, rx) = mpsc::channel();
        loop {
            let mut this_comms = comms.clone();
            let this_tx = tx.clone();
            let h = thread::spawn(move || {
                this_comms.get_data(this_tx);
            });

            let rcvd = rx.recv().unwrap();
            debug!("[motor_l] Received: {}", rcvd);
            if rcvd == "SET:fwd" {
                info!("[motor_l] Setting Left Motor Forwards");
                self.speed = 1;
                comms.send_to(&"SET:motorl:fwd".as_bytes().to_vec(), status_node);
            } else if rcvd == "SET:bwd" {
                info!("[motor_l] Setting Left Motor Backwards");
                self.speed = -1;
                comms.send_to(&"SET:motorl:bwd".as_bytes().to_vec(), status_node);
            } else if rcvd == "SET:STP" {
                info!("[motor_l] Setting Left Motor to Stop");
                self.speed = 0;
                comms.send_to(&"SET:motorl:stp".as_bytes().to_vec(), status_node);
            }
            h.join().unwrap();
        }
    }
}
