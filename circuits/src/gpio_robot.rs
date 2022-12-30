use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//use rust_pigpio::pwm::*;

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
    is_real: bool,
}

impl GPIOLed {
    pub fn new(pin: u8) -> Self {
        let is_real = export(pin);
        if is_real {
            std::thread::sleep(std::time::Duration::from_millis(50));
            set_direction(pin, Directions::Output);
        }
        GPIOLed { pin, is_real }
    }

    pub fn on(&self) {
        if self.is_real {
            write(self.pin, true);
        } else {
            println!(" Mocked ON, pin {}", self.pin);
        }
    }

    pub fn off(&self) {
        if self.is_real {
            write(self.pin, false);
        } else {
            println!(" Mocked OFF, pin {}", self.pin);
        }
    }
}

pub struct GPIOMotor {
    pin1: u8,
    pin2: u8,
    pin_enabler: u8,
    is_real: bool,
}

impl GPIOMotor {
    pub fn new(pin1: u8, pin2: u8, pin_enabler: u8) -> Self {
        if (export(pin1) && export(pin2) && export(pin_enabler)) {
            let is_real = true;
            std::thread::sleep(std::time::Duration::from_millis(50));
            set_direction(pin1, Directions::Output);
            set_direction(pin2, Directions::Output);
            set_direction(pin_enabler, Directions::Output);
            // TODO:
            // I1=24
            // I2=23
            // IE=25
            // GPIO.setmode(GPIO.BCM)
            // GPIO.setup(I1, GPIO.OUT)
            // GPIO.setup(I2, GPIO.OUT)
            // GPIO.setup(IE, GPIO.OUT)
            // GPIO.output(I1, GPIO.LOW)
            // GPIO.output(I2, GPIO.LOW)
            // p = GPIO.PWM(IE, 1000)
            // p.start(100)
            // GPIO.output(I1, GPIO.LOW)
            // GPIO.output(I2, GPIO.HIGH)
        } else {
            let is_real = false;
        }
        GPIOMotor {
            pin1,
            pin2,
            pin_enabler,
            is_real: false,
        }
    }
}
// AUX Functions

fn open_file(filepath: &String) -> Result<File, std::io::Error> {
    let file: File;
    let path = Path::new(&filepath);
    match fs::OpenOptions::new().write(true).open(path) {
        Err(why) => return Err(why),
        Ok(f) => file = f,
    };

    Ok(file)
}

pub fn export(gpio_num: u8) -> bool {
    let mut is_real: bool = false;
    let mut file: File;
    let filepath = String::from("/sys/class/gpio/export");

    match open_file(&filepath) {
        Err(why) => println!("couldn't open {}: {}", filepath, why),
        Ok(f) => {
            file = f;
            is_real = true;
            if let Err(why) = file.write_all(gpio_num.to_string().as_bytes()) {
                println!("couldn't write to {}: {}", filepath, why);
                is_real = false;
            } else {
                is_real = true;
            }
        }
    }
    is_real
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