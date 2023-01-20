use crate::comms::*;

use serialport::*;
use std::collections::HashMap;

pub struct ArduinoNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
    mocked: bool,
    portpath: &'a str,
    baudrate: u32,
    connected: bool,
}

impl<'a> ArduinoNode<'a> {
    pub fn new(
        name: &'a str,
        conns: HashMap<&'a str, &'a str>,
        mocked: bool,
        portpath: &'a str,
    ) -> Self {
        let node = match get_port(name, conns.clone()) {
            Ok(c) => ArduinoNode {
                port_in: c,
                conns,
                mocked,
                portpath,
                baudrate: 9600,
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

    pub fn connect(&mut self) -> Result<&'a str> {
        if self.mocked {
            Ok("mocked")
        } else {
            let port = serialport::new(self.portpath, self.baudrate)
                .timeout(std::time::Duration::from_millis(100))
                .open();
            match port {
                Ok(_) => {
                    self.connected = true;
                    Ok(self.portpath)
                }
                Err(e) => Err(e),
            }
        }

        //let mut port = serialport::new(self.portpath, self.baudrate)
        //    .timeout(std::time::Duration::from_millis(100))
        //    .open()
        //    .expect("Failed to open port");
        //// Write data
        //let output = "This is a test.".as_bytes();
        //let written_bytes = port.write(output).expect("Write failed!");
        //println!("Written bytes len = {}", written_bytes);
        //println!("Written bytes = {:?}", output);
        //self.connected = true;
    }

    pub fn listen(&mut self) -> &'a str {
        ""
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
            "/dev/ttyACM0",
        );
        let _test_node2 = ArduinoNode::new(
            "arduino",
            get_conns(["arduino"].to_vec()),
            false,
            "/dev/ttyACM0",
        );
    }
    #[test]
    #[should_panic]
    fn test_object_created_not_ok() {
        use crate::get_conns;
        let _test_node1 = ArduinoNode::new(
            "arduino",
            get_conns(["motor_l", "motor_r", "status"].to_vec()),
            false,
            "/dev/ttyACM0",
        );
        let _test_node2 = ArduinoNode::new(
            "arduino",
            get_conns(["status"].to_vec()),
            false,
            "/dev/ttyACM0",
        );
    }

    #[test]
    fn test_connect() {
        use crate::get_conns;
        use std::io::ErrorKind::NotFound;
        let mut test_node_ok = ArduinoNode::new(
            "arduino",
            get_conns(["arduino", "status"].to_vec()),
            true,
            "this path is not needed",
        );
        assert_eq!(test_node_ok.connect().unwrap(), "mocked");
        let mut test_node_err = ArduinoNode::new(
            "arduino",
            get_conns(["arduino", "status"].to_vec()),
            false,
            "/dev/ttyACM0",
        );
        assert_eq!(
            test_node_err.connect().map_err(|e| e.kind()),
            Err(serialport::ErrorKind::Io(NotFound))
        );
    }

    #[test]
    fn test_listen() {
        use crate::get_conns;
        let mut test_node_ok = ArduinoNode::new(
            "arduino",
            get_conns(["arduino", "status"].to_vec()),
            true,
            "this path is not needed",
        );
        assert_eq!(test_node_ok.connect().unwrap(), "mocked");
        assert_eq!(test_node_ok.listen(), "");
    }
    //TODO: test AND build: buffer of input from the arduino (mocked) to be cut and passed over
}
