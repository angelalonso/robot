use crate::udp_endpoint::*;

use crossbeam_channel::unbounded;
use std::sync::mpsc;
use std::thread;

pub struct ActionServerNode<'a> {
    port_in: &'a str,
    port_out: &'a str,
}

impl<'a> ActionServerNode<'a> {
    pub fn new(port_in: &'a str, port_out: &'a str) -> Self {
        ActionServerNode { port_in, port_out }
    }

    pub fn get_port(&self) -> &str {
        return &self.port_in;
    }

    fn talk(&self, rx: Vec<u8>) {
        println!("hey {}", String::from_utf8_lossy(&rx));
    }

    pub fn run(&self) {
        // Connection In
        let mut conn_in = UDPConn::new(self.port_in.to_owned(), self.port_out.to_owned());

        let (tx_in, rx_in) = mpsc::channel();
        let (tx_out, rx_out) = unbounded();

        let thread_tx_in = tx_in.clone();
        let handle_s = thread::spawn(move || {
            conn_in.connect();
            loop {
                let thread_rx_out = rx_out.clone();
                let msg_in_raw = conn_in.listen();
                thread_tx_in.send(msg_in_raw).unwrap();
                println!("-> {:#?}", thread_rx_out.recv().unwrap());
            }
        });

        for received in rx_in {
            let rcvd = format!("{}", String::from_utf8(received.clone()).unwrap());
            self.talk(received);
            tx_out.send(rcvd).unwrap();
        }

        handle_s.join().unwrap();
    }
}
