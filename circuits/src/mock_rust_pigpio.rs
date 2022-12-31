pub fn initialize() {
    println!("Mocking initialize of pigpio");
}

pub struct pwm {
    on: bool,
}

impl pwm {
    pub fn pwm(pin_enabler: u32, value: u32) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn set_pwm_frequency(pin_enabler: u32, freq: u32) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn set_pwm_range(pin_enabler: u32, range: u32) -> Result<(), &'static str> {
        Ok(())
    }
}
