use std::collections::HashMap;
use std::net;
use std::sync::mpsc::Sender;

// This one is used by all nodes using comms, so for now it stays here
pub fn get_port<'a>(
    name: &'a str,
    conns: HashMap<&'a str, &'a str>,
) -> Result<&'a str, &'static str> {
    let res = match conns.get(name) {
        Some(r) => Ok(*r),
        None => Err("Nothing found."),
    };
    res
}

//#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
pub struct UDPComms<'a> {
    ip: &'a str,
    pub rx: &'a str,
    port_in: String,
}

impl<'a> UDPComms<'_> {
    pub fn new(port_in: String) -> Self {
        let ip = "127.0.0.1"; // TODO: make this default but not the only choice
        UDPComms {
            ip,
            rx: "",
            port_in,
        }
    }

    pub fn listen(&mut self) -> String {
        let local = format!("{}:{}", self.ip, self.port_in);
        let socket = net::UdpSocket::bind(&local).expect("failed to bind host socket");
        socket
            .set_read_timeout(None)
            .expect("set_read_timeout call failed");
        let mut buf = [0; 100];
        let recvd = match socket.recv(&mut buf) {
            Ok(received) => format!("{}", String::from_utf8_lossy(&buf[..received]).to_owned()),
            Err(_) => "".to_owned(),
        };
        //println!("received at {} --> {}", self.port_in, recvd);
        recvd.clone()
    }

    pub fn get_data(&mut self, callback: Sender<String>) {
        let local = format!("{}:{}", self.ip, self.port_in);
        let socket = net::UdpSocket::bind(&local).expect("failed to bind host socket");
        socket
            .set_read_timeout(None)
            .expect("set_read_timeout call failed");
        let mut buf = [0; 100];
        let _result = match socket.recv(&mut buf) {
            Ok(received) => callback.send(format!("{}", String::from_utf8_lossy(&buf[..received]))),
            Err(_) => Ok(()),
        };
    }

    //pub fn send_to(&self, msg: &Vec<u8>, dest_port: &str) -> usize {
    pub fn send_to(&self, msg: &Vec<u8>, dest_port: &str) {
        let local = format!("{}:{}", self.ip, self.port_in);
        // TODO: control errors here:
        let socket = net::UdpSocket::bind(&local).expect("failed to bind host socket");
        socket
            .set_read_timeout(None)
            .expect("set_read_timeout call failed");
        let destination = format!("{}:{}", self.ip, dest_port);
        //println!(
        //    "Transmitting data from {} to {} --> {:#?}",
        //    self.port_in,
        //    destination,
        //    String::from_utf8_lossy(msg)
        //);
        let _result = socket
            .send_to(msg, &destination)
            .expect("failed to send message");

        //result
    }
}
