use crate::comms::*;
use crate::udp_endpoint::*;

use crossbeam_channel::unbounded;
use std::sync::mpsc;
use std::thread;

pub struct TestNode<'a> {
    port_in: &'a str,
    port_out: &'a str,
    value: String,
    comms: UDPComms<'a>,
}

impl<'a> TestNode<'a> {
    pub fn new(port_in: &'a str, port_out: &'a str) -> Self {
        let mut comms = UDPComms::new(port_in.to_owned(), port_out.to_owned());
        TestNode {
            port_in,
            port_out,
            value: "".to_owned(),
            comms,
        }
    }

    pub fn get_port(&self) -> &str {
        return &self.port_in;
    }

    fn take_action(&mut self, rx: Vec<u8>) {
        let new_msg = format!("eeH {}", String::from_utf8_lossy(&rx));
        self.comms.transmit(&new_msg.as_bytes().to_vec());
    }

    pub fn receive(&mut self) {
        self.comms.run();
    }

    //TODO: every 5 seconds, send GET:time through the line, read the result and store it
}
