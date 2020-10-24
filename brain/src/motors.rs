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
    movement: String,
    motor_l: MotorObj,
    motor_r: MotorObj,
}

impl Motors {
    pub fn new(mode: String) -> Result<Self, &'static str> {
        let mut m_l_o = None;
        let mut m_l_e = None;
        let mut m_r_o = None;
        let mut m_r_e = None;
        if mode == "classic" {
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
        Ok(Self {
            name: "Motors".to_string(),
            movement: "0_0".to_string(),
            motor_l: m_l,
            motor_r: m_r,
        })
    }

    pub fn set_stop(&mut self, motor: MotorObj) {
        match motor.object {
            Some(o) => {
                match motor.enabler {
                    Some(e) => e.lock().unwrap().off(),
                    None => debug!("Here the enabler of {:?} would be set to OFF", motor.name),
                }
                o.lock().unwrap().stop();
            },
            None => debug!("Here {:?} would be set to STOP", motor.name),
        }
    }

    pub fn set_move(&mut self, motor: MotorObj, movement: i16, value: f64) {
        match motor.object {
            Some(o) => {
                match motor.enabler {
                    Some(e) => {
                        e.lock().unwrap().on();
                        e.lock().unwrap().set_value(value);
                    },
                    None => debug!("Here the enabler of {:?} would be set to ON", motor.name),
                }
                if movement > 0 {
                    o.lock().unwrap().forward();
                } else if movement < 0 {
                    o.lock().unwrap().backward();
                }
            },
            None => debug!("Here {:?} would be set to MOVE {:?}", motor.name, value),
        }
    }

    // TODO: remove set_stop and set_move and do everything here
    // TODO: do it for both motors
    pub fn set(&mut self, movement: String) {
        match movement.as_str() {
            &_ => {
                let move_vector = movement.split("_").collect::<Vec<_>>();
                let prev_move_vector = self.movement.split("_").collect::<Vec<_>>();
                if move_vector != prev_move_vector {
                    let move_l = move_vector[0];
                    let _move_r = move_vector[1];
                    if move_l != prev_move_vector[0] {
                        if move_l.parse::<i16>().unwrap() == 0 {
                            //self.set_stop(self.motor_l);
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
                        }
                    }
                    self.movement = movement.clone();
                    info!("Changing Move to {}", self.movement);
                }
            },
        }
    }
}
