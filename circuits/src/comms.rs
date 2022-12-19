use std::env;
use std::net;

pub struct UDPComms<'a> {
    ip: &'a str,
    port_in: String,
    port_out: String,
    buf: Vec<u8>,
    socket: net::UdpSocket,
    remote: String,
    msg_bytes: Vec<u8>,
}

impl<'a> UDPComms<'a> {
    pub fn new(port_in: String, port_out: String) -> Self {
        let local = format!("127.0.0.1:{}", port_in);
        let remote = format!("127.0.0.1:{}", port_out);
        let mut buf: Vec<u8> = Vec::with_capacity(100);
        let socket = net::UdpSocket::bind(&local).expect("failed to bind host socket");
        let message = String::from("hello");
        let msg_bytes = message.into_bytes();
        UDPComms {
            ip: "127.0.0.1",
            port_in,
            port_out,
            buf,
            socket,
            remote,
            msg_bytes,
        }
    }
    pub fn run(&mut self) {
        loop {
            while self.listen() != 0 {
                println!("boo");
            }
            self.send(&self.socket, &self.remote, &self.msg_bytes);
        }
    }

    fn listen(&mut self) -> usize {
        let (number_of_bytes, src_addr) = self
            .socket
            .recv_from(&mut self.buf)
            .expect("no data received");

        println!("{:?}", number_of_bytes);
        println!("{:?}", src_addr);

        number_of_bytes
    }

    fn send(&self, socket: &net::UdpSocket, receiver: &str, msg: &Vec<u8>) -> usize {
        println!("sending data {:#?}", msg);
        let result = socket
            .send_to(msg, receiver)
            .expect("failed to send message");

        result
    }

    pub fn transmit(&self, msg: &Vec<u8>) -> usize {
        println!("sending data {:#?}", msg);
        let result = self
            .socket
            .send_to(msg, &self.remote)
            .expect("failed to send message");

        result
    }
}
