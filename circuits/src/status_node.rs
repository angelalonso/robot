use crate::comms::*;

//use crossbeam_channel::unbounded;
//use std::sync::mpsc;
//use std::thread;

pub struct StatusNode<'a> {
    port_in: &'a str,
    comms: UDPComms,
}

impl<'a> StatusNode<'a> {
    pub fn new(port_in: &'a str, port_out: &'a str) -> Self {
        let comms = UDPComms::new(port_in.to_owned(), port_out.to_owned());
        StatusNode { port_in, comms }
    }

    pub fn get_port(&self) -> &str {
        return &self.port_in;
    }

    fn take_action(&mut self, rx: String) {
        if rx == "GET:time" {
            let result = "time:xxxxxxx";
            self.comms.transmit(&result.as_bytes().to_vec());
        } else if rx == "ping" {
            let result = "pong";
            self.comms.transmit(&result.as_bytes().to_vec());
        } else {
            let result = "ack";
            self.comms.transmit(&result.as_bytes().to_vec());
        }
    }

    pub fn receive(&mut self) {
        loop {
            let rcvd = self.comms.listen();
            if rcvd != "" {
                println!("STATUS got {}", rcvd);
                self.take_action(rcvd);
            }
        }
    }
}
