use crate::comms::*;

use std::collections::HashMap;

pub struct ArduinoNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
    connected: bool,
}

impl<'a> ArduinoNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        let node = match get_port(name, conns.clone()) {
            Ok(c) => ArduinoNode {
                port_in: c,
                conns,
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

    pub fn connect(&mut self) {
        self.connected = true;
    }
}

#[cfg(test)]
mod arduino_node_tests {
    use super::*;
    #[test]
    fn test_object_created_ok() {
        use crate::get_conns;
        let _test_node1 = ArduinoNode::new(
            "arduino",
            get_conns(["motor_l", "motor_r", "arduino", "status"].to_vec()),
        );
        let _test_node2 = ArduinoNode::new("arduino", get_conns(["arduino"].to_vec()));
    }
    #[test]
    #[should_panic]
    fn test_object_created_not_ok() {
        use crate::get_conns;
        let _test_node1 = ArduinoNode::new(
            "arduino",
            get_conns(["motor_l", "motor_r", "status"].to_vec()),
        );
        let _test_node2 = ArduinoNode::new("arduino", get_conns(["status"].to_vec()));
    }

    //TODO: test AND build: buffer of input from the arduino (mocked) to be cut and passed over
}
