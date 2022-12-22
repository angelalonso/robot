use crate::comms::*;
//use crate::udp_endpoint::*;

//use crossbeam_channel::unbounded;
//use std::sync::mpsc;
//use std::thread;
use std::collections::HashMap;
use std::error::Error;

// TODO: move these somewhere common to all
#[derive(Debug, Clone, Copy)]
pub struct Conn<'a> {
    pub port_in: &'a str,
    pub port_out: &'a str,
}

pub struct LedActionServerNode<'a> {
    name: &'a str,
    port_in: &'a str,
    comms: UDPComms,
    //TODO: we definitely should build a HashMap of UDPComms
    conns: HashMap<&'a str, Conn<'a>>,
    led_on: bool,
}

fn get_conn<'a>(
    name: &'a str,
    conns: HashMap<&'a str, Conn<'a>>,
) -> Result<Conn<'a>, &'static str> {
    let res = match conns.get(name) {
        Some(r) => Ok(*r),
        None => Err("Nothing found."),
    };
    res
}

impl<'a> LedActionServerNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, Conn<'a>>) -> Self {
        let ports = match get_conn(name, conns.clone()) {
            Ok(c) => {
                let comms = UDPComms::new(c.port_in.to_owned(), c.port_out.to_owned());
                LedActionServerNode {
                    name,
                    port_in: c.port_in,
                    comms,
                    conns,
                    led_on: false,
                }
            }
            Err(_) => {
                // TODO: this should fail instead
                let comms = UDPComms::new("".to_owned(), "".to_owned());
                LedActionServerNode {
                    name,
                    port_in: "",
                    comms,
                    conns,
                    led_on: false,
                }
            }
        };
        ports
    }

    pub fn get_port(&self) -> &str {
        return &self.port_in;
    }

    fn take_action(&mut self, rx: String) {
        if rx == "SET:on" {
            self.led_on = true;
            self.comms.transmit(&"LED:on".as_bytes().to_vec());
        } else if rx == "SET:off" {
            self.led_on = false;
            self.comms.transmit(&"LED:off".as_bytes().to_vec());
        } else if rx == "ping" {
            self.led_on = false;
            self.comms.transmit(&"pong".as_bytes().to_vec());
        } else if rx == "SET:switch" {
            if self.led_on == true {
                self.led_on = false;
            } else {
                self.led_on = true;
            }
        }
    }

    pub fn receive(&mut self) {
        loop {
            let rcvd = self.comms.listen();
            if rcvd != "" {
                self.take_action(rcvd);
            }
        }
    }
}
