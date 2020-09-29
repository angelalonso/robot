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

// TODO: change motor1, motor2 to motor_L and motor_R, when the robot is mounted properly

#[derive(Clone)]
pub struct Mover<'a > {
    pub name: &'a str,
    pub movement: &'a str,
    pub motor1: Arc<Mutex<Motor>>,
    pub motor1_ena: Arc<Mutex<PWMOutputDevice>>,
    pub motor2: Arc<Mutex<Motor>>,
    pub motor2_ena: Arc<Mutex<PWMOutputDevice>>,
}

impl Mover<'_> {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Self {
            name: "movement",
            movement: "stop",
                    // Temporarily inverted motor1: Arc::new(Mutex::new(Motor::new(17, 27))),
            motor1: Arc::new(Mutex::new(Motor::new(27, 17))),
            motor1_ena: Arc::new(Mutex::new(PWMOutputDevice::new(22))),
                    // Temporarily inverted too 
            motor2: Arc::new(Mutex::new(Motor::new(24, 23))),
            motor2_ena: Arc::new(Mutex::new(PWMOutputDevice::new(25))),
        })
    }

    pub fn set_move(&mut self, movement: String) {
        match movement.as_str() {
            "forwards" => {
                if self.movement != "forwards"{
                    self.movement = "forwards";
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor1.lock().unwrap().forward();
                    self.motor2.lock().unwrap().forward();
                    self.motor1_ena.lock().unwrap().on();
                    self.motor2_ena.lock().unwrap().on();
                    self.motor1_ena.lock().unwrap().set_value(1.0);
                    self.motor2_ena.lock().unwrap().set_value(1.0);
                }
            },
            "forwards_slow" => {
                if self.movement != "forwards_slow"{
                    self.movement = "forwards_slow";
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor1.lock().unwrap().forward();
                    self.motor2.lock().unwrap().forward();
                    self.motor1_ena.lock().unwrap().on();
                    self.motor2_ena.lock().unwrap().on();
                    self.motor1_ena.lock().unwrap().set_value(0.55);
                    self.motor2_ena.lock().unwrap().set_value(0.55);
                }
            },
            "backwards" => {
                if self.movement != "backwards"{
                    self.movement = "backwards";
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor1.lock().unwrap().backward();
                    self.motor2.lock().unwrap().backward();
                    self.motor1_ena.lock().unwrap().on();
                    self.motor2_ena.lock().unwrap().on();
                    self.motor1_ena.lock().unwrap().set_value(1.0);
                    self.motor2_ena.lock().unwrap().set_value(1.0);
                }
            },
            "rotate_random" => {
                if self.movement != "rotate_random"{
                    self.movement = "rotate_random";
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    if rand::random() {
                        self.motor1.lock().unwrap().forward();
                        self.motor2.lock().unwrap().backward();
                    } else {
                        self.motor1.lock().unwrap().backward();
                        self.motor2.lock().unwrap().forward();
                    }
                    self.motor1_ena.lock().unwrap().on();
                    self.motor2_ena.lock().unwrap().on();
                    self.motor1_ena.lock().unwrap().set_value(0.7);
                    self.motor2_ena.lock().unwrap().set_value(0.7);
                }
            },
            "stop" => {
                if self.movement != "stop"{
                    self.movement = "stop";
                    log(Some(&self.name), "E", &format!("Moving : {}", self.movement));
                    self.motor1.lock().unwrap().stop();
                    self.motor2.lock().unwrap().stop();
                    self.motor1_ena.lock().unwrap().off();
                    self.motor2_ena.lock().unwrap().off();
                    self.motor1_ena.lock().unwrap().set_value(0.0);
                    self.motor2_ena.lock().unwrap().set_value(0.0);
                }
            },
            &_ => (),
        }
    }
}
