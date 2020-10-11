use thiserror::Error;
use crate::log;
use rust_gpiozero::*;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Error, Debug)]
pub enum BrainMoverError {
    /// It used to represent an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    /// Now it's just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("Config contains no related entries")]
    NoConfigFound,
}

#[derive(Clone)]
pub struct Mover<'a > {
    pub name: &'a str,
    pub movement: String,
    pub motor_l: Arc<Mutex<Motor>>,
    pub motor_l_ena: Arc<Mutex<PWMOutputDevice>>,
    pub motor_r: Arc<Mutex<Motor>>,
    pub motor_r_ena: Arc<Mutex<PWMOutputDevice>>,
}

impl Mover<'_> {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Self {
            name: "movement",
            movement: "0_0".to_string(),
                    // Temporarily inverted motor_l: Arc::new(Mutex::new(Motor::new(17, 27))),
            motor_l: Arc::new(Mutex::new(Motor::new(27, 17))),
            motor_l_ena: Arc::new(Mutex::new(PWMOutputDevice::new(22))),
                    // Temporarily inverted too 
            motor_r: Arc::new(Mutex::new(Motor::new(24, 23))),
            motor_r_ena: Arc::new(Mutex::new(PWMOutputDevice::new(25))),
        })
    }

    // TODO: change this from forwards,backwards... to "100_100", "-100_-100"
    pub fn set_move<'r>(&mut self, movement: String) {
        match movement.as_str() {
            "forwards" => {
                if self.movement != "forwards"{
                    self.movement = "forwards".to_string();
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor_l.lock().unwrap().forward();
                    self.motor_r.lock().unwrap().forward();
                    self.motor_l_ena.lock().unwrap().on();
                    self.motor_r_ena.lock().unwrap().on();
                    self.motor_l_ena.lock().unwrap().set_value(1.0);
                    self.motor_r_ena.lock().unwrap().set_value(1.0);
                }
            },
            "forwards_slow" => {
                if self.movement != "forwards_slow"{
                    self.movement = "forwards_slow".to_string();
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor_l.lock().unwrap().forward();
                    self.motor_r.lock().unwrap().forward();
                    self.motor_l_ena.lock().unwrap().on();
                    self.motor_r_ena.lock().unwrap().on();
                    self.motor_l_ena.lock().unwrap().set_value(0.55);
                    self.motor_r_ena.lock().unwrap().set_value(0.55);
                }
            },
            "backwards" => {
                if self.movement != "backwards"{
                    self.movement = "backwards".to_string();
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor_l.lock().unwrap().backward();
                    self.motor_r.lock().unwrap().backward();
                    self.motor_l_ena.lock().unwrap().on();
                    self.motor_r_ena.lock().unwrap().on();
                    self.motor_l_ena.lock().unwrap().set_value(1.0);
                    self.motor_r_ena.lock().unwrap().set_value(1.0);
                }
            },
            "rotate_left" => {
                if self.movement != "rotate_left"{
                    self.movement = "rotate_left".to_string();
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor_l.lock().unwrap().backward();
                    self.motor_r.lock().unwrap().forward();
                    self.motor_l_ena.lock().unwrap().on();
                    self.motor_r_ena.lock().unwrap().on();
                    self.motor_l_ena.lock().unwrap().set_value(0.7);
                    self.motor_r_ena.lock().unwrap().set_value(0.7);
                }
            },
            "rotate_right" => {
                if self.movement != "rotate_right"{
                    self.movement = "rotate_right".to_string();
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor_l.lock().unwrap().forward();
                    self.motor_r.lock().unwrap().backward();
                    self.motor_l_ena.lock().unwrap().on();
                    self.motor_r_ena.lock().unwrap().on();
                    self.motor_l_ena.lock().unwrap().set_value(0.7);
                    self.motor_r_ena.lock().unwrap().set_value(0.7);
                }
            },
            "stop" => {
                if self.movement != "stop"{
                    self.movement = "stop".to_string();
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor_l.lock().unwrap().stop();
                    self.motor_r.lock().unwrap().stop();
                    self.motor_l_ena.lock().unwrap().off();
                    self.motor_r_ena.lock().unwrap().off();
                    self.motor_l_ena.lock().unwrap().set_value(0.0);
                    self.motor_r_ena.lock().unwrap().set_value(0.0);
                }
            },
            &_ => {
                let move_vector = movement.split("_").collect::<Vec<_>>();
                let prev_move_vector = self.movement.split("_").collect::<Vec<_>>();
                if move_vector != prev_move_vector {
                    println!("############## MOVE ############# {} vs {}", movement, self.movement);
                    let move_l = move_vector[0];
                    let move_r = move_vector[1];
                    if move_l != prev_move_vector[0] {
                        if move_l.parse::<i16>().unwrap() == 0 {
                            self.motor_l.lock().unwrap().stop();
                            self.motor_l_ena.lock().unwrap().off();
                            println!("############## Left is 0");
                        } else {
                            self.motor_l_ena.lock().unwrap().on();
                            if move_l.parse::<i16>().unwrap() > 0 {
                                self.motor_l.lock().unwrap().forward();
                            } else if move_l.parse::<i16>().unwrap() < 0 {
                                self.motor_l.lock().unwrap().backward();
                            }
                            let value = move_l.parse::<i16>().unwrap().abs() as i16 / 100).into();
                            println!("############## Left is {}", value);
                            self.motor_l_ena.lock().unwrap().set_value((move_l.parse::<i16>().unwrap().abs() as i16 / 100).into());
                        }
                    }
                    if move_r != prev_move_vector[1] {
                        if move_r.parse::<i16>().unwrap() == 0 {
                            self.motor_r.lock().unwrap().stop();
                            self.motor_r_ena.lock().unwrap().off();
                        } else {
                            self.motor_r_ena.lock().unwrap().on();
                            if move_r.parse::<i16>().unwrap() > 0 {
                                self.motor_r.lock().unwrap().forward();
                            } else if move_r.parse::<i16>().unwrap() < 0 {
                                self.motor_r.lock().unwrap().backward();
                            }
                        self.motor_r_ena.lock().unwrap().set_value((move_r.parse::<i16>().unwrap().abs() as i16 / 100).into());
                        }
                    }
                    println!("{}", self.movement);
                    self.movement = movement.clone();
                    println!("{}", self.movement);
                } else {
                    println!("------------ NO CHANGE ------------ {} vs {}", movement, self.movement);

                }
            },
        }
    }
}
