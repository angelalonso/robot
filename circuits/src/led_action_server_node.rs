use crate::comms::*;

use log::{debug, info};
// TODO: use a fake library for local tests
use rust_gpiozero::*;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub struct LedActionServerNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
    led: LED,
    led_on: bool,
}

impl<'a> LedActionServerNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        //TODO: pin comes from .env
        let led = LED::new(21);
        let node = match get_port(name, conns.clone()) {
            Ok(c) => LedActionServerNode {
                port_in: c,
                conns,
                led,
                led_on: false,
            },
            Err(_) => {
                // TODO: this should fail instead
                LedActionServerNode {
                    port_in: "",
                    conns,
                    led,
                    led_on: false,
                }
            }
        };
        node
    }

    pub fn talk(&mut self) {
        let status_node = get_port("status", self.conns.clone()).unwrap();
        let comms = UDPComms::new(self.port_in.to_owned());
        let _status: HashMap<String, String> = HashMap::new();
        let (tx, rx) = mpsc::channel();
        loop {
            //println!("led LOOP");
            let mut this_comms = comms.clone();
            let this_tx = tx.clone();
            let h = thread::spawn(move || {
                this_comms.get_data(this_tx);
            });

            let rcvd = rx.recv().unwrap();
            debug!("[led] Received: {}", rcvd);
            if rcvd == "SET:on" {
                info!("[led] Setting LED ON");
                self.led_on = true;
                self.led.on();
                comms.send_to(&"SET:led:on".as_bytes().to_vec(), status_node);
            } else if rcvd == "SET:off" {
                info!("[led] Setting LED OFF");
                self.led_on = false;
                self.led.off();
                comms.send_to(&"SET:led:off".as_bytes().to_vec(), status_node);
            } else if rcvd == "SET:switch" {
                if self.led_on == true {
                    info!("[led] Setting LED OFF");
                    self.led_on = false;
                    self.led.off();
                    comms.send_to(&"SET:led:off".as_bytes().to_vec(), status_node);
                } else {
                    info!("[led] Setting LED ON");
                    self.led_on = true;
                    self.led.on();
                    comms.send_to(&"SET:led:on".as_bytes().to_vec(), status_node);
                }
            }
            h.join().unwrap();
            //println!("led LOOP END");
        }
    }
}
