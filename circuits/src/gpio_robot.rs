use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Thanks to: https://michm.de/blog/rust/ansteuern-von-raspberry-pi-gpio-in-rust/

pub type Directions = self::directions::Directions;

pub mod directions {
    pub enum Directions {
        Input,
        Output,
    }

    impl Directions {
        pub fn as_str(&self) -> &'static str {
            match *self {
                Directions::Input => "in",
                Directions::Output => "out",
            }
        }

        pub fn as_bytes(&self) -> &[u8] {
            self.as_str().as_bytes()
        }
    }
}

pub struct GPIOLed {
    pin: u8,
}

impl GPIOLed {
    pub fn new(pin: u8) -> Self {
        export(pin);
        set_direction(pin, Directions::Output);
        GPIOLed { pin }
    }

    pub fn on(&self) {
        write(self.pin, true);
    }

    pub fn off(&self) {}
}

fn open_file(filepath: &String) -> Result<File, std::io::Error> {
    let file: File;
    let path = Path::new(&filepath);
    match fs::OpenOptions::new().write(true).open(path) {
        Err(why) => return Err(why),
        Ok(f) => file = f,
    };

    Ok(file)
}

pub fn export(gpio_num: u8) {
    let mut file: File;
    let filepath = String::from("/sys/class/gpio/export");

    match open_file(&filepath) {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(f) => file = f,
    }

    if let Err(why) = file.write_all(gpio_num.to_string().as_bytes()) {
        panic!("couldn't write to {}: {}", filepath, why);
    }
}

pub fn unexport(gpio_num: u8) {
    let filepath = String::from("/sys/class/gpio/unexport");
    let mut file: File;

    match open_file(&filepath) {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(f) => file = f,
    }

    if let Err(why) = file.write_all(gpio_num.to_string().as_bytes()) {
        panic!("couldn't write to {}: {}", filepath, why);
    }
}

pub fn write(gpio_num: u8, signal: bool) {
    let mut file: File;
    let mut filepath = String::from("/sys/class/gpio/gpio%/value");
    let gpio = gpio_num.to_string();
    filepath = filepath.replace("%", gpio.as_str());

    match open_file(&filepath) {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(f) => file = f,
    }

    if let Err(why) = file.write_all(u8::from(signal).to_string().as_bytes()) {
        panic!("couldn't write to {}: {}", filepath, why);
    }
}

pub fn read(gpio_num: u8) -> bool {
    let mut filepath = String::from("/sys/class/gpio/gpio%/value");
    let gpio = gpio_num.to_string();
    filepath = filepath.replace("%", gpio.as_str());

    match fs::read_to_string(filepath) {
        Err(why) => panic!("couldn't read: {}", why),
        Ok(contents) => contents.parse::<u8>().unwrap() > 0,
    }
}

pub fn set_direction(gpio_num: u8, direction: Directions) {
    let mut file: File;
    let mut filepath = String::from("/sys/class/gpio/gpio%/direction");
    let gpio = gpio_num.to_string();
    filepath = filepath.replace("%", gpio.as_str());

    match open_file(&filepath) {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(f) => file = f,
    }

    if let Err(why) = file.write_all(direction.as_bytes()) {
        panic!("couldn't write to {}: {}", filepath, why);
    }
}
