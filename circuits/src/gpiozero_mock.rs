use crate::gpio_robot::*;
use rust_gpiozero;
use std::panic;
use sysfs_gpio::{Direction, Pin};

pub struct LED {
    pin: u8,
    real: Option<GPIOLed>,
}

impl LED {
    pub fn new(pin: u8) -> Self {
        let real = Some(GPIOLed::new(pin));
        LED { pin, real }
    }

    pub fn on(&self) -> sysfs_gpio::Result<()> {
        match &self.real {
            Some(l) => {
                l.on();
                Ok(())
            }
            None => {
                println!(" Mocked ON, pin {}", self.pin);
                Ok(())
            }
        }
    }

    pub fn off(&self) -> sysfs_gpio::Result<()> {
        match &self.real {
            Some(l) => {
                l.off();
                Ok(())
            }
            None => {
                println!(" Mocked ON, pin {}", self.pin);
                Ok(())
            }
        }
    }
}

pub struct Motor {
    pin_a: u8,
    real_a: Option<rust_gpiozero::LED>,
    pin_b: u8,
    real_b: Option<rust_gpiozero::LED>,
}

impl Motor {
    pub fn new(pin_a: u8, pin_b: u8) -> Self {
        let motor_object = match panic::catch_unwind(|| {
            let _ = rust_gpiozero::LED::new(pin_a);
        }) {
            Ok(_) => {
                let real_a = Some(rust_gpiozero::LED::new(pin_a));
                let real_b = Some(rust_gpiozero::LED::new(pin_b));
                Motor {
                    pin_a,
                    pin_b,
                    real_a,
                    real_b,
                }
            }
            Err(_) => {
                let real_a = None;
                let real_b = None;
                Motor {
                    pin_a,
                    pin_b,
                    real_a,
                    real_b,
                }
            }
        };
        motor_object
    }

    pub fn fwd(&self) {
        match &self.real_a {
            Some(_) => {
                self.real_a.as_ref().unwrap().off();
                self.real_b.as_ref().unwrap().on();
            }
            None => println!(" Mocked FWD, pins {}-{}", self.pin_a, self.pin_b),
        }
    }

    pub fn bwd(&self) {
        match &self.real_a {
            Some(_) => {
                self.real_a.as_ref().unwrap().on();
                self.real_b.as_ref().unwrap().off();
            }
            None => println!(" Mocked BWD, pins {}-{}", self.pin_a, self.pin_b),
        }
    }

    pub fn stp(&self) {
        match &self.real_a {
            Some(_) => {
                self.real_a.as_ref().unwrap().off();
                self.real_b.as_ref().unwrap().off();
            }
            None => println!(" Mocked STP, pins {}-{}", self.pin_a, self.pin_b),
        }
    }
}
