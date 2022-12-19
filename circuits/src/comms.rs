use std::net;

pub struct UDPComms {
    port_in: String,
    socket: net::UdpSocket,
    remote: String,
}

impl UDPComms {
    pub fn new(port_in: String, port_out: String) -> Self {
        let local = format!("127.0.0.1:{}", port_in);
        let remote = format!("127.0.0.1:{}", port_out);
        let socket = net::UdpSocket::bind(&local).expect("failed to bind host socket");
        socket
            .set_read_timeout(None)
            .expect("set_read_timeout call failed");
        UDPComms {
            port_in,
            socket,
            remote,
        }
    }

    pub fn listen(&mut self) -> String {
        let mut buf = [0; 10];
        let recvd = match self.socket.recv(&mut buf) {
            Ok(received) => format!("{}", String::from_utf8_lossy(&buf[..received]).to_owned()),
            Err(_) => "".to_owned(),
        };
        println!("listen {} --- {}", self.port_in, recvd);
        recvd
    }

    pub fn transmit(&self, msg: &Vec<u8>) -> usize {
        println!("Transmitting data {:#?}", String::from_utf8_lossy(msg));
        let result = self
            .socket
            .send_to(msg, &self.remote)
            .expect("failed to send message");

        result
    }
}
