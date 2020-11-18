use rust_gpiozero::{Motor, PWMOutputDevice};
use log::{debug, info};
use std::sync::Arc;
use std::sync::Mutex;

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
    motor_l: MotorObj,
    motor_r: MotorObj,
    objects: Vec<MotorObj>,
}

impl Motors {
    pub fn new(mode: String, motor_vector: Vec<String>) -> Result<Self, &'static str> {
        let mut m_l_o = None;
        let mut m_l_e = None;
        let mut m_r_o = None;
        let mut m_r_e = None;
        if mode != "dryrun" {
            m_l_o = Some(Arc::new(Mutex::new(Motor::new(27, 17))));
            m_l_e = Some(Arc::new(Mutex::new(PWMOutputDevice::new(22))));
            m_r_o = Some(Arc::new(Mutex::new(Motor::new(24, 23))));
            m_r_e = Some(Arc::new(Mutex::new(PWMOutputDevice::new(25))));
        };
        let m_l = MotorObj{
            name: "motor_left".to_string(),
            object: m_l_o,
            enabler: m_l_e,
            speed: 0,
        };
        let m_r = MotorObj{
            name: "motor_right".to_string(),
            object: m_r_o,
            enabler: m_r_e,
            speed: 0,
        };
        let mut objs = [].to_vec();
        for o in motor_vector {
            let config = o.split("__").collect::<Vec<_>>();
            let keyval_a = config[1].split("=").collect::<Vec<_>>();
            let keyval_b = config[2].split("=").collect::<Vec<_>>();
            let keyval_c = config[2].split("=").collect::<Vec<_>>();
            let mut m_o = None;
            let mut m_e = None;
            if mode != "dryrun" {
                m_o = Some(Arc::new(Mutex::new(Motor::new(keyval_a[1].parse::<u8>().unwrap(), keyval_b[1].parse::<u8>().unwrap()))));
                m_e = Some(Arc::new(Mutex::new(PWMOutputDevice::new(keyval_c[1].parse::<u8>().unwrap()))));
            }
            let m = MotorObj {
                name: config[0].to_string(),
                object: m_o,
                enabler: m_e,
                speed: 0,
            };
            objs.push(m);
        }
        Ok(Self {
            name: "Motors".to_string(),
            movement: "0_0".to_string(),
            motor_l: m_l,
            motor_r: m_r,
            objects: objs,
        })
    }

    pub fn set(&mut self, movement: String) {
        let move_vector = movement.split("_").collect::<Vec<_>>();
        let prev_move_vector = self.movement.split("_").collect::<Vec<_>>();
        if move_vector != prev_move_vector {
            let move_l = move_vector[0];
            let move_r = move_vector[1];
            if move_l != prev_move_vector[0] {
                if move_l.parse::<i16>().unwrap() == 0 {
                    match self.motor_l.clone().object {
                        Some(o) => {
                            match &self.motor_l.enabler {
                                Some(e) => e.lock().unwrap().off(),
                                None => debug!("Here the enabler of {:?} would be set to OFF", self.motor_l.name),
                            }
                            o.lock().unwrap().stop();
                        },
                        None => debug!("Here {:?} would be set to STOP", self.motor_l.name),
                    }
                } else {
                    match self.motor_l.clone().object {
                        Some(o) => {
                            if move_l.parse::<i16>().unwrap() > 0 {
                                o.lock().unwrap().forward();
                            } else if move_l.parse::<i16>().unwrap() < 0 {
                                o.lock().unwrap().backward();
                            }
                            match &self.motor_l.enabler {
                                Some(e) => {
                                    e.lock().unwrap().on();
                                    let l_value = (move_l.parse::<i16>().unwrap().abs() as f64 / 100.0) as f64;
                                    e.lock().unwrap().set_value(l_value);
                                }
                                None => debug!("Here the enabler of {:?} would be set to ON", self.motor_l.name),
                            }
                        },
                        None => debug!("Here {:?} would be set to MOVE", self.motor_l.name),
                        //
                    }
                }
            }
            if move_r != prev_move_vector[0] {
                if move_r.parse::<i16>().unwrap() == 0 {
                    match self.motor_r.clone().object {
                        Some(o) => {
                            match &self.motor_r.enabler {
                                Some(e) => e.lock().unwrap().off(),
                                None => debug!("Here the enabler of {:?} would be set to OFF", self.motor_r.name),
                            }
                            o.lock().unwrap().stop();
                        },
                        None => debug!("Here {:?} would be set to STOP", self.motor_r.name),
                    }
                } else {
                    match self.motor_r.clone().object {
                        Some(o) => {
                            if move_r.parse::<i16>().unwrap() > 0 {
                                o.lock().unwrap().forward();
                            } else if move_r.parse::<i16>().unwrap() < 0 {
                                o.lock().unwrap().backward();
                            }
                            match &self.motor_r.enabler {
                                Some(e) => {
                                    e.lock().unwrap().on();
                                    let r_value = (move_r.parse::<i16>().unwrap().abs() as f64 / 100.0) as f64;
                                    e.lock().unwrap().set_value(r_value);
                                }
                                None => debug!("Here the enabler of {:?} would be set to ON", self.motor_r.name),
                            }
                        },
                        None => debug!("Here {:?} would be set to MOVE", self.motor_r.name),
                        //
                    }
                }
            }
            // move_r
            self.movement = movement.clone();
            info!("Changing Move to {}", self.movement);
        }
    }
}
