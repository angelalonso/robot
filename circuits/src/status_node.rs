use crate::comms::*;
use crate::udp_endpoint::*;

use crossbeam_channel::unbounded;
use std::sync::mpsc;
use std::thread;

pub struct StatusNode<'a> {
    port_in: &'a str,
    port_out: &'a str,
    value: String,
    comms: UDPComms<'a>,
}

impl<'a> StatusNode<'a> {
    pub fn new(port_in: &'a str, port_out: &'a str) -> Self {
        let mut comms = UDPComms::new(port_in.to_owned(), port_out.to_owned());
        StatusNode {
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
        let new_msg = format!("hey {}", String::from_utf8_lossy(&rx));
        self.comms.transmit(&new_msg.as_bytes().to_vec());
    }

    pub fn receive(&mut self) {
        self.comms.run();
    }

    //TODO: next step: save timestamp in seconds, send it as time:xxx when GET:time is received

    //    pub fn listen(&mut self) {
    //        // Connection In
    //        let mut conn_in = UDPConn::new(self.port_in.to_owned(), "9002".to_owned());
    //
    //        let (tx_in, rx_in) = mpsc::channel();
    //        let (tx_out, rx_out) = unbounded();
    //
    //        let thread_tx_in = tx_in.clone();
    //        let handle_s = thread::spawn(move || {
    //            conn_in.connect();
    //            loop {
    //                let thread_rx_out = rx_out.clone();
    //                let msg_in_raw = conn_in.listen();
    //                thread_tx_in.send(msg_in_raw).unwrap();
    //                println!("-> {:#?}", thread_rx_out.recv().unwrap());
    //            }
    //        });
    //
    //        for received in rx_in {
    //            let rcvd = format!("{}", String::from_utf8(received.clone()).unwrap());
    //            self.take_action(received);
    //            tx_out.send(rcvd).unwrap();
    //        }
    //
    //        handle_s.join().unwrap();
    //    }
}
