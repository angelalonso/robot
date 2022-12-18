use crate::udp_endpoint::*;

use std::thread;

pub struct ActionClientNode<'a> {
    port_in: &'a str,
    port_out: &'a str,
    status: bool,
}

impl<'a> ActionClientNode<'a> {
    pub fn new(port_in: &'a str, port_out: &'a str) -> Self {
        ActionClientNode {
            port_in,
            port_out,
            status: false,
        }
    }

    pub fn get_port(&self) -> &str {
        return &self.port_in;
    }

    pub fn run(&mut self) {
        let mut conn_out = UDPConn::new(self.port_in.to_owned(), self.port_out.to_owned());
        let mut status_tmp = self.status.clone();
        let handle_c = thread::spawn(move || {
            conn_out.connect();
            loop {
                if status_tmp == false {
                    conn_out.send(&"on".as_bytes().to_vec());
                    status_tmp = true;
                } else {
                    conn_out.send(&"off".as_bytes().to_vec());
                    status_tmp = false;
                }
                let sleep_time = std::time::Duration::from_secs(1);
                std::thread::sleep(sleep_time);
            }
        });

        handle_c.join().unwrap();
    }
}
