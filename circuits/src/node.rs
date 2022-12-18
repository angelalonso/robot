use crate::udp_endpoint::*;

pub struct Node<'a> {
    name: &'a str,
    port_in: &'a str,
    port_out: &'a str,
}

impl<'a> Node<'a> {
    pub fn new(name: &'a str, port_in: &'a str, port_out: &'a str) -> Self {
        Node {
            name,
            port_in,
            port_out,
        }
    }

    pub fn get_port(&self) -> &str {
        return &self.port_in;
    }

    pub fn run(&self) {
        let mut conn_out = UDPConn::new(self.port_in, self.port_out);
        conn_out.start();
    }
}
