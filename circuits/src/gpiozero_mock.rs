use rust_gpiozero;
use std::panic;

pub struct LED {
    pin: u8,
    real: Option<rust_gpiozero::LED>,
}

impl LED {
    pub fn new(pin: u8) -> Self {
        let led_object = match panic::catch_unwind(|| {
            let _ = rust_gpiozero::LED::new(21);
        }) {
            Ok(_) => {
                let real = Some(rust_gpiozero::LED::new(21));
                LED { pin, real }
            }
            Err(_) => {
                let real = None;
                LED { pin, real }
            }
        };
        led_object
    }

    pub fn on(&self) {
        match &self.real {
            Some(l) => l.on(),
            None => println!(" Mocked ON, pin {}", self.pin),
        }
    }

    pub fn off(&self) {
        match &self.real {
            Some(l) => l.off(),
            None => println!(" Mocked OFF, pin {}", self.pin),
        }
    }
}
