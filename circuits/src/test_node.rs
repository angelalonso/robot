use crate::comms::*;

pub struct TestNode<'a> {
    port_in: &'a str,
    comms: UDPComms,
}

impl<'a> TestNode<'a> {
    pub fn new(port_in: &'a str, port_out: &'a str) -> Self {
        let comms = UDPComms::new(port_in.to_owned(), port_out.to_owned());
        TestNode { port_in, comms }
    }

    pub fn get_port(&self) -> &str {
        return &self.port_in;
    }

    pub fn receive(&mut self) {
        // this first call makes sure status_node is on the other side
        self.comms.transmit(&"ping".as_bytes().to_vec());
        let mut rcvd = self.comms.listen();
        if rcvd != "" {
            loop {
                self.comms.transmit(&"GET:time".as_bytes().to_vec());
                // adding a bit of delay before looking for answer
                std::thread::sleep(std::time::Duration::from_millis(10));
                rcvd = self.comms.listen();
                if rcvd != "" {
                    println!("TEST got {}", rcvd);
                }
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
    }
}
