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
                    log(Some(&self.name), "D", &format!("Moving : {}", self.movement));
                    self.motor1.lock().unwrap().forward();
                    self.motor2.lock().unwrap().forward();
                    self.motor1_ena.lock().unwrap().on();
                    self.motor2_ena.lock().unwrap().on();
                    self.motor1_ena.lock().unwrap().set_value(1.0);
                    self.motor2_ena.lock().unwrap().set_value(1.0);
                }
            },
            "backwards" => {
                if self.movement != "backwards"{
                    self.movement = "backwards";
                    log(Some(&self.name), "D", &format!("Moving : {}", self.movement));
                    self.motor1.lock().unwrap().backward();
                    self.motor2.lock().unwrap().backward();
                    self.motor1_ena.lock().unwrap().on();
                    self.motor2_ena.lock().unwrap().on();
                    self.motor1_ena.lock().unwrap().set_value(1.0);
                    self.motor2_ena.lock().unwrap().set_value(1.0);
                }
            },
            "stop" => {
                if self.movement != "stop"{
                    self.movement = "stop";
                    log(Some(&self.name), "D", &format!("Moving : {}", self.movement));
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
