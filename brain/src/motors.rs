use rust_gpiozero::{Motor, PWMOutputDevice};
use log::info;

pub struct Motors {
    name: String,
    movement: String,
    motor_l: Option<Motor>,
    motor_l_ena: Option<PWMOutputDevice>,
    motor_l_speed: i8,
    motor_r: Option<Motor>,
    motor_r_ena: Option<PWMOutputDevice>,
    motor_r_speed: i8,
}

impl Motors {
    pub fn new(mode: String) -> Result<Self, &'static str> {
        let m_l: std::option::Option<Motor>;
        let m_l_e: std::option::Option<PWMOutputDevice>;
        let m_r: std::option::Option<Motor>;
        let m_r_e: std::option::Option<PWMOutputDevice>;
        if mode == "classic" {
            m_l = Some(Motor::new(27, 17));
            m_l_e = Some(PWMOutputDevice::new(22));
            m_r = Some(Motor::new(24, 23));
            m_r_e = Some(PWMOutputDevice::new(25));
        } else {
            m_l = None;
            m_l_e = None;
            m_r = None;
            m_r_e = None;
        }
        Ok(Self {
            name: "Motors".to_string(),
            movement: "0_0".to_string(),
            motor_l: m_l,
            motor_l_ena: m_l_e,
            motor_l_speed: 0,
            motor_r: m_r,
            motor_r_ena: m_r_e,
            motor_r_speed: 0,
        })
    }

    pub fn set(&mut self, movement: String) {
        match movement.as_str() {
            &_ => {
                let move_vector = movement.split("_").collect::<Vec<_>>();
                let prev_move_vector = self.movement.split("_").collect::<Vec<_>>();
                if move_vector != prev_move_vector {
                    let move_l = move_vector[0];
                    let move_r = move_vector[1];
                    if move_l != prev_move_vector[0] {
                        if move_l.parse::<i16>().unwrap() == 0 {
                            self.motor_l.stop();
                            self.motor_l_ena.off();
                        } else {
                            self.motor_l_ena.on();
                            if move_l.parse::<i16>().unwrap() > 0 {
                                self.motor_l.forward();
                            } else if move_l.parse::<i16>().unwrap() < 0 {
                                self.motor_l.backward();
                            }
                            let l_value = (move_l.parse::<i16>().unwrap().abs() as f64 / 100.0) as f64;
                            self.motor_l_ena.set_value(l_value);
                        }
                    }
                    if move_r != prev_move_vector[1] {
                        if move_r.parse::<i16>().unwrap() == 0 {
                            self.motor_r.stop();
                            self.motor_r_ena.off();
                        } else {
                            self.motor_r_ena.on();
                            if move_r.parse::<i16>().unwrap() > 0 {
                                self.motor_r.forward();
                            } else if move_r.parse::<i16>().unwrap() < 0 {
                                self.motor_r.backward();
                            }
                            let r_value = (move_r.parse::<i16>().unwrap().abs() as f64 / 100.0) as f64;
                            self.motor_r_ena.set_value(r_value);
                        }
                    }
                    self.movement = movement.clone();
                    info!("Changing Move to {}", self.movement);
                }
            },
        }
    }
}
