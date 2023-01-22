use crate::comms::*;

use load_dotenv::load_dotenv;
use log::{debug, error, info};
use serialport;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::thread;
//use std::sync::mpsc;
//use std::thread;
use std::time::Duration;

pub struct ArduinoNode<'a> {
    //port_in: &'a str,
    //conns: HashMap<&'a str, &'a str>,
    //mocked: bool,
    portpath: &'a str,
    baudrate: u32,
    msg: &'a str,
    //connected: bool,
}

// TODO: connect on creation
// TODO: check ports on connection
// https://github.com/serialport/serialport-rs/blob/main/examples/list_ports.rs
// TODO: HW check??
// https://github.com/serialport/serialport-rs/blob/main/examples/hardware_check.rs
// TODO Transmit function??
// https://github.com/serialport/serialport-rs/blob/main/examples/transmit.rs
impl<'a> ArduinoNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>, _mocked: bool) -> Self {
        load_dotenv!(); //TODO: is it better to pass parameters when needed?
        let portpath = env!("ARDUINO_USB_DEV");
        let baudrate = env!("ARDUINO_BAUDRATE").parse::<u32>().unwrap();
        let msg = "";
        let node = match get_port(name, conns.clone()) {
            Ok(_c) => ArduinoNode {
                //port_in: c,
                //conns,
                //mocked,
                portpath,
                baudrate,
                msg,
                //connected: false,
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
        let arduino_read_delay = match env!("ARDUINO_READ_DELAY").parse::<u64>() {
            Ok(a) => a,
            Err(e) => {
                info!("------------------------ ERROR: {}", e);
                0
            }
        };
        info!("------------- {}", arduino_read_delay);
        let serial_port = match serialport::new(self.portpath, self.baudrate)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(sp) => Some(sp),
            Err(_) => None,
        };
        match serial_port {
            None => debug!("Mocking and not Receiving data"),
            Some(mut sp) => {
                loop {
                    let mut serial_buf: Vec<u8> = vec![0; 1000];
                    match sp.write(" ".as_bytes()) {
                        Ok(_) => {
                            std::io::stdout().flush().unwrap();
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => error!("{:?}", e),
                    }
                    match sp.read(serial_buf.as_mut_slice()) {
                        // TODO: cleanup self.msg, use get_msg instead
                        Ok(t) => {
                            thread::sleep(Duration::from_millis(arduino_read_delay));
                            let newmsg_raw = serial_buf[..t].to_vec();
                            let newmsg = std::str::from_utf8(&newmsg_raw).unwrap();
                            let values_rcvd = get_msg(newmsg);
                            for (k, v) in values_rcvd.unwrap() {
                                info!("{}-{}", k, v);
                            }
                            //self.msg.to_owned().push_str(newmsg);
                            //info!("->{}<-...{}", newmsg, self.msg);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => error!("{:?}", e),
                    }
                }
            }
        }
    }
}

pub fn get_msg(raw_msg: &str) -> Option<HashMap<String, String>> {
    let mut output = HashMap::new();
    let ok_start: Vec<char> = "SENSOR: ".to_owned().chars().collect();
    let msg: Vec<char> = raw_msg.to_owned().chars().collect();
    let mut clean_msg_vec = [].to_vec();
    // length is greater than the minimum starting message, ok_start
    if msg.len() < ok_start.len() {
        None
    } else {
        // it must contain ok_start at the beginning
        for ix in 0..ok_start.len() {
            if msg[ix] != ok_start[ix] {
                return None;
            }
        }
        // we only compute full messages
        if msg[msg.len() - 1] != '|' {
            return None;
        } else {
            for (ix, item) in msg.iter().enumerate().take(msg.len() - 1) {
                if ix >= ok_start.len() {
                    clean_msg_vec.push(item);
                }
            }
            let clean_msg: String = clean_msg_vec.into_iter().collect();
            let entries: Vec<&str> = clean_msg.split('|').collect();
            for e in entries {
                let keyval: Vec<&str> = e.split('=').collect();
                output.insert(keyval[0].to_owned(), keyval[1].to_owned());
            }
        };
        Some(output)
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

    #[test]
    fn test_get_msg() {
        let failing_examples: [&str; 18] = [
            ": laser=70|distance=101|",
            "=103|",
            "OR: laser=68|distance=103|",
            "SENS",
            "SENSOR",
            "SENSOR:",
            "SENSOR: ",
            "SENSOR: las",
            "SENSOR: laser=59|di",
            "SENSOR: laser=59|distance=103",
            "SOR: laser=69|distance=102|",
            "ance=103|",
            "ce=103|",
            "distance=103|",
            "er=62|distance=103|",
            "laser=67|distance=103|",
            "ser=78|distance=102|",
            "stance=103|",
        ];
        for e in failing_examples {
            assert_eq!(None, get_msg(e));
        }
        // TODO: sort these out

        let mut output1 = HashMap::new();
        output1.insert("laser".to_owned(), "59".to_owned());
        assert_eq!(Some(output1), get_msg("SENSOR: laser=59|"));
        let mut output2 = HashMap::new();
        output2.insert("laser".to_owned(), "59".to_owned());
        output2.insert("distance".to_owned(), "103".to_owned());
        assert_eq!(Some(output2), get_msg("SENSOR: laser=59|distance=103|"));
    }
}
