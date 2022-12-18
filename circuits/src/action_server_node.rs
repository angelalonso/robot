use crate::udp_endpoint::*;

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
        let mut conn_out = UDPConn::new(self.port_in.to_owned(), self.port_out.to_owned());
        //conn_out.connect();
        //conn_out.start_listening();
        let (tx, rx) = mpsc::channel();
        let handle_s = thread::spawn(move || {
            conn_out.connect();
            loop {
                let msg_in_raw = conn_out.listen();
                tx.send(msg_in_raw).unwrap();
            }
        });
        for received in rx {
            self.talk(received);
        }

        handle_s.join().unwrap();
    }
}
