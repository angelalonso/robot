use crate::comms::*;

use crate::gpio_robot::*;
use log::{debug, info};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub struct MotorRActionServerNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
    motor: GPIOMotor,
    speed: i8,
}

impl<'a> MotorRActionServerNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        let motor = GPIOMotor::new(24, 23, 25);
        let node = match get_port(name, conns.clone()) {
            Ok(c) => MotorRActionServerNode {
                port_in: c,
                conns,
                motor,
                speed: 0,
            },
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
        let comms = UDPComms::new(self.port_in.to_owned());

        let (tx, rx) = mpsc::channel();
        loop {
            let mut this_comms = comms.clone();
            let this_tx = tx.clone();
            let h = thread::spawn(move || {
                this_comms.get_data(this_tx);
            });

            let rcvd = rx.recv().unwrap();
            debug!("[motor_r] Received: {}", rcvd);
            if rcvd == "SET:fwd" {
                info!("[motor_r] Setting Right Motor Forwards");
                self.speed = 1;
                self.motor.fwd();
                comms.send_to(&"SET:motorr:fwd".as_bytes().to_vec(), status_node);
            } else if rcvd == "SET:bwd" {
                info!("[motor_r] Setting Right Motor Backwards");
                self.speed = -1;
                self.motor.bwd();
                comms.send_to(&"SET:motorr:bwd".as_bytes().to_vec(), status_node);
            } else if rcvd == "SET:stp" {
                info!("[motor_r] Setting Right Motor to Stop");
                self.speed = 0;
                self.motor.stp();
                comms.send_to(&"SET:motorr:stp".as_bytes().to_vec(), status_node);
            } else if rcvd == "SET:switch" {
                if self.speed == 0 {
                    info!("[motor_r] Setting Right Motor Forwards");
                    self.speed = 1;
                    self.motor.fwd();
                    comms.send_to(&"SET:motorr:fwd".as_bytes().to_vec(), status_node);
                } else {
                    info!("[motor_r] Setting Right Motor to Stop");
                    self.speed = 0;
                    self.motor.stp();
                    comms.send_to(&"SET:motorr:stp".as_bytes().to_vec(), status_node);
                }
            }
            h.join().unwrap();
        }
    }
}
