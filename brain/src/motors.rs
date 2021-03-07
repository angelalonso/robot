use rust_gpiozero;
use rust_gpiozero::{Motor, PWMOutputDevice};
use log::{debug, info, warn};
use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MotorObj {
    name: String,
    object: Option<Arc<Mutex<Motor>>>,
    enabler: Option<Arc<Mutex<PWMOutputDevice>>>,
    speed: i8,
}

#[derive(Clone)]
pub struct Motors {
    name: String,
    pub movement: String,
    objects: Vec<MotorObj>,
}

impl Motors {
    pub fn new(mode: String, motor_map: HashMap<String, HashMap<String, String>>) -> Result<Self, &'static str> {
        let mut objs = [].to_vec();
        for o in motor_map {
            let mut m_o = None;
            let mut m_e = None;
            if mode != "dryrun" {
            // TODO: use this if no solution found
            //if mode != "dryrun" && mode != "check" {
                let motor_pin1 = o.1["gpio1"].parse::<u8>().unwrap();
                let motor_pin2 = o.1["gpio2"].parse::<u8>().unwrap();
                let motor_enablerpin = o.1["gpio_enabler"].parse::<u8>().unwrap();
                //Am I running on the Raspberry?
                match std::env::var("TARGET") {
                    Ok(_a) => {
                        warn!("If your ar not on a Raspberry and you see this, there's a bug");
                        m_o = Some(Arc::new(Mutex::new(Motor::new(motor_pin1, motor_pin2))));
                        m_e = Some(Arc::new(Mutex::new(PWMOutputDevice::new(motor_enablerpin))));
                    },
                    Err(_e) => {
                        warn!("Not running on a Raspberry, there are probably no GPIOs we can use");
                    },
                };
                //m_o = Some(Arc::new(Mutex::new(Motor::new(motor_pin1, motor_pin2))));
                //m_e = Some(Arc::new(Mutex::new(PWMOutputDevice::new(motor_enablerpin))));
            }
            let m = MotorObj {
                name: o.0.to_string(),
                object: m_o,
                enabler: m_e,
                speed: 0,
            };
            objs.push(m);
        }
        Ok(Self {
            name: "Motors".to_string(),
            movement: "0_0".to_string(),
            objects: objs,
        })
    }

    pub fn change_movement(&mut self, movement: String) {
        let move_vector = movement.split('_').collect::<Vec<_>>();
        let prev_move_vector = self.movement.split('_').collect::<Vec<_>>();
        //println!("{:#x?} vs. {:#x?}", move_vector, prev_move_vector);
        if move_vector != prev_move_vector {
            let move_l = move_vector[0];
            let move_r = move_vector[1];
            //TODO: we can probably do both motors on a loop instead of repeating
            // LEFT MOTOR
            if move_l != prev_move_vector[0] {
                if move_l.parse::<i16>().unwrap() == 0 {
                    match self.objects.iter_mut().find(|x| x.name == "motor_l") {
                        Some(m) => {
                            match m.clone().object {
                                Some(o) => {
                                    match &m.enabler {
                                        Some(e) => e.lock().unwrap().off(),
                                        None => debug!("Here the enabler of {:?} would be set to OFF", m.name),
                                    }
                                    o.lock().unwrap().stop();
                                },
                                None => debug!("Here {:?} would be set to STOP", m.name),
                            }

                        },
                        None => warn!("There's no motor_l"),
                    }
                } else {
                    match self.objects.iter_mut().find(|x| x.name == "motor_l") {
                        Some(m) => {
                            match m.clone().object {
                                Some(o) => {
                                    match move_l.parse::<i16>().unwrap().cmp(&0) {
                                        Ordering::Greater => o.lock().unwrap().forward(),
                                        Ordering::Less => o.lock().unwrap().backward(),
                                        Ordering::Equal => (),
                                    }
                                    match &m.enabler {
                                        Some(e) => {
                                            e.lock().unwrap().on();
                                            let l_value = (move_l.parse::<i16>().unwrap().abs() as f64 / 100.0) as f64;
                                            e.lock().unwrap().set_value(l_value);
                                        }
                                        None => debug!("Here the enabler of {:?} would be set to ON", m.name),
                                    }
                                },
                                None => debug!("Here {:?} would be set to MOVE", m.name),
                                //
                            }
                        },
                        None => warn!("There's no motor_l"),
                    }
                }
            }
            // RIGHT MOTOR
            if move_r != prev_move_vector[1] {
                if move_r.parse::<i16>().unwrap() == 0 {
                    match self.objects.iter_mut().find(|x| x.name == "motor_r") {
                        Some(m) => {
                            match m.clone().object {
                                Some(o) => {
                                    match &m.enabler {
                                        Some(e) => e.lock().unwrap().off(),
                                        None => debug!("Here the enabler of {:?} would be set to OFF", m.name),
                                    }
                                    o.lock().unwrap().stop();
                                },
                                None => debug!("Here {:?} would be set to STOP", m.name),
                            }

                        },
                        None => warn!("There's no motor_r"),
                    }
                } else {
                    match self.objects.iter_mut().find(|x| x.name == "motor_r") {
                        Some(m) => {
                            match m.clone().object {
                                Some(o) => {
                                    match move_r.parse::<i16>().unwrap().cmp(&0) {
                                        Ordering::Greater => o.lock().unwrap().forward(),
                                        Ordering::Less => o.lock().unwrap().backward(),
                                        Ordering::Equal => (),
                                    }
                                    match &m.enabler {
                                        Some(e) => {
                                            e.lock().unwrap().on();
                                            let r_value = (move_r.parse::<i16>().unwrap().abs() as f64 / 100.0) as f64;
                                            e.lock().unwrap().set_value(r_value);
                                        }
                                        None => debug!("Here the enabler of {:?} would be set to ON", m.name),
                                    }
                                },
                                None => debug!("Here {:?} would be set to MOVE", m.name),
                                //
                            }
                        },
                        None => warn!("There's no motor_r"),
                    }
                }
            }
            self.movement = movement.clone();
            info!("Changing Move to {}", self.movement);
        }
    }

    pub fn set(&mut self, motor: String, value: String) {
        let movement = self.movement.clone();
        let movement_vector = movement.split('_').collect::<Vec<_>>();
        match motor.as_str() {
            "motor_l" => {
                self.change_movement(format!("{}_{}", value, movement_vector[1]));
            },
            "motor_r" => {
                self.change_movement(format!("{}_{}", movement_vector[0], value));
            },
            _ => {},
        }
        
    }
}
