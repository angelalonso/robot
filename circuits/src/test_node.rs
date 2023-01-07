use crate::comms::*;

use log::debug;
use std::collections::HashMap;
//use std::sync::mpsc;
//use std::thread;

pub struct TestNode<'a> {
    _port_in: &'a str,
    _conns: HashMap<&'a str, &'a str>,
}

impl<'a> TestNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        let node = match get_port(name, conns.clone()) {
            Ok(c) => TestNode {
                _port_in: c,
                _conns: conns,
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
        debug!("Test - Nothing to do");
    }
}
