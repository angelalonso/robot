use crate::comms::*;

use load_dotenv::load_dotenv;
//use log::{debug, info};
use serialport::*;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::thread;
//use std::sync::mpsc;
//use std::thread;
use std::time::Duration;

pub struct ArduinoNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
    mocked: bool,
    portpath: &'a str,
    baudrate: u32,
    msg: &'a str,
    connected: bool,
}

// TODO: connect on creation
// TODO: loop to read
// https://github.com/serialport/serialport-rs/blob/main/examples/receive_data.rs
// TODO: check ports on connection
// https://github.com/serialport/serialport-rs/blob/main/examples/list_ports.rs
// TODO: HW check??
// https://github.com/serialport/serialport-rs/blob/main/examples/hardware_check.rs
// TODO Transmit function??
// https://github.com/serialport/serialport-rs/blob/main/examples/transmit.rs
impl<'a> ArduinoNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>, mocked: bool) -> Self {
        load_dotenv!(); //TODO: is it better to pass parameters when needed?
        let portpath = env!("ARDUINO_USB_DEV");
        let baudrate = env!("ARDUINO_BAUDRATE").parse::<u32>().unwrap();
        let msg = "";
        let node = match get_port(name, conns.clone()) {
            Ok(c) => ArduinoNode {
                port_in: c,
                conns,
                mocked,
                portpath,
                baudrate,
                msg,
                connected: false,
            },
            Err(_) => {
                panic!(
                    "couldn't find a port to itself: {} (HINT: check name at main.rs)",
                    name
                );
            }
        };
        node
    }

    pub fn talk(&mut self) {
        //let status_node = get_port("status", self.conns.clone()).unwrap();
        //let comms = UDPComms::new(self.port_in.to_owned());
        let _status: HashMap<String, String> = HashMap::new();
        let serialport = match serialport::new(self.portpath, self.baudrate)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(sp) => Some(sp),
            Err(_) => None,
        };
        match serialport {
            None => println!("Mocking and not Receiving data"),
            Some(mut sp) => {
                loop {
                    let mut serial_buf: Vec<u8> = vec![0; 1000];
                    match sp.write(" ".as_bytes()) {
                        Ok(_) => {
                            std::io::stdout().flush().unwrap();
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => eprintln!("{:?}", e),
                    }
                    match sp.read(serial_buf.as_mut_slice()) {
                        // TODO: fill up self.msg
                        // TODO: write on each loop?
                        // TODO: wait on each loop?
                        Ok(t) => {
                            thread::sleep(Duration::from_millis(175));
                            let newmsg_raw = serial_buf[..t].to_vec();
                            let newmsg = std::str::from_utf8(&newmsg_raw).unwrap();
                            self.msg.to_owned().push_str(newmsg);
                            println!("->{}<-...{}", newmsg, self.msg);
                            //io::stdout().write_all(&serial_buf[..t]).unwrap();
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            }
        }
    }
}

mod arduino_node_tests {
    #[cfg(test)]
    use super::*;
    #[test]
    fn test_object_created_ok() {
        use crate::get_conns;
        let _test_node1 = ArduinoNode::new(
            "arduino",
            get_conns(["motor_l", "motor_r", "arduino", "status"].to_vec()),
            false,
        );
        let _test_node2 = ArduinoNode::new("arduino", get_conns(["arduino"].to_vec()), false);
    }
    #[test]
    #[should_panic]
    fn test_object_created_not_ok() {
        use crate::get_conns;
        let _test_node1 = ArduinoNode::new(
            "arduino",
            get_conns(["motor_l", "motor_r", "status"].to_vec()),
            false,
        );
        let _test_node2 = ArduinoNode::new("arduino", get_conns(["status"].to_vec()), false);
    }

    #[test]
    #[ignore]
    fn test_talk() {
        use crate::get_conns;
        let mut test_node_ok =
            ArduinoNode::new("arduino", get_conns(["arduino", "status"].to_vec()), true);
        let handle_ar = thread::spawn(move || {
            test_node_ok.talk();
        });
        handle_ar.join().unwrap();
    }
    //TODO: test AND build: buffer of input from the arduino (mocked) to be cut and passed over
}
