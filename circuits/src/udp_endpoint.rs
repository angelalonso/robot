use std::net;

// based on https://gist.github.com/lanedraex/bc01eb399614359470cfacc9d95993fb

#[derive(Debug, Default)]
struct HostConfig {
    local_ip: String,
    local_port: String,
    local_host: String,
    remote_ip: String,
    remote_port: String,
    remote_host: String,
}

pub struct UDPConn<'a> {
    ip: &'a str,
    port_in: String,
    port_out: String,
    socket: Option<net::UdpSocket>,
    host_cfg_remote: Option<String>,
}

impl<'a> UDPConn<'a> {
    pub fn new(port_in: String, port_out: String) -> Self {
        UDPConn {
            ip: "127.0.0.1",
            port_in,
            port_out,
            socket: None,
            host_cfg_remote: None,
        }
    }

    pub fn connect(&mut self) {
        let capacity = 128;
        let mut host_config = HostConfig {
            local_ip: self.ip.to_owned(),
            local_port: self.port_in.to_owned(),
            local_host: String::with_capacity(capacity),
            // TODO: allow for comms outside of localhost?
            remote_ip: self.ip.to_owned(),
            remote_port: self.port_out.to_owned(),
            remote_host: String::with_capacity(capacity),
        };
        let default_msg = "1, 2, one, two...";

        host_config.local_host =
            self.set_host_parameters(&host_config.local_ip, &host_config.local_port);
        host_config.remote_host =
            self.set_host_parameters(&host_config.remote_ip, &host_config.remote_port);
        self.host_cfg_remote = Some(host_config.remote_host);

        //let mut message = String::with_capacity(capacity);
        //message = "hey".to_owned();

        let socket: net::UdpSocket = self.init_host(&host_config.local_host);
        self.socket = Some(socket);
        //println!("host config: {:?}", host_config);
        //println!("socket: {:?}", socket);
        let message = default_msg.to_owned();
        let _msg_bytes = message.into_bytes();

        // TODO(alex): Remove this sleep timer.
        let sleep_time = std::time::Duration::from_secs(1);
        std::thread::sleep(sleep_time);
    }

    pub fn start_listening(&mut self) {
        loop {
            let _received_msg = self.listen();
            //println!("{:#?}", String::from_utf8_lossy(&received_msg));
            //self.send(&socket, &host_config.remote_host, &msg_bytes);
        }
    }

    fn set_host_parameters(&self, ip: &str, port: &str) -> String {
        // TODO(alex): Create a constant for default string capacity values.
        let mut host = String::with_capacity(128);
        host.push_str(ip);
        host.push_str(":");
        host.push_str(port);

        host
    }

    fn init_host(&self, host: &str) -> net::UdpSocket {
        println!("initializing host: {:?}", host);
        let socket = net::UdpSocket::bind(host).expect("failed to bind host socket");
        // TODO(alex): Create a constant for this duration timeout value.
        let duration = std::time::Duration::new(1, 0);
        let dur = std::option::Option::Some(duration);
        let _res = socket.set_read_timeout(dur).expect("failed to set timeout");

        socket
    }

    pub fn listen(&self) -> Vec<u8> {
        // TODO(alex): Create constants for these buffer size values.
        let mut buf: [u8; 20] = [0; 20];
        let _number_of_bytes: usize = 0;
        let mut result: Vec<u8> = Vec::new();
        let socket = self.socket.as_ref().unwrap();
        match socket.recv_from(&mut buf) {
            Ok((number_of_bytes, _src_addr)) => {
                //println!("received message: {:?}", buf);
                result = Vec::from(&buf[0..number_of_bytes]);
            }
            Err(fail) => {
                // dont show code 11 WouldBlock
                if fail.kind() != std::io::ErrorKind::WouldBlock {
                    println!("failed listening {:?}", fail);
                }
            }
        }

        let display_result = result.clone();
        let result_str = String::from_utf8(display_result).unwrap();
        if result_str != "" {
            println!("{} - received message: {:?}", self.port_in, result_str);
        };
        result
    }

    pub fn send(&self, msg: &Vec<u8>) -> usize {
        // TODO: log message sent in plain text too
        //println!("{} - sending message: {:?}", self.port_in, msg);
        let result: usize = 0;
        //let socket = self.socket.as_ref().unwrap();
        let _socket = match self.socket.as_ref() {
            Some(s) => {
                let receiver = self.host_cfg_remote.as_ref().unwrap();
                match s.send_to(&msg, receiver) {
                    Ok(_number_of_bytes) => (), //println!("{:?}", number_of_bytes),
                    Err(fail) => println!("failed sending {:?}", fail),
                }
                //   result
            }
            _ => (),
        };
        result
    }
}
