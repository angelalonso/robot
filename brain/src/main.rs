extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let motor_pin1 = Pin::new(2);
    let motor_pin2 = Pin::new(3);
    motor_pin1.with_exported(|| {
        motor_pin1.set_direction(Direction::Out).unwrap();
        motor_pin2.set_direction(Direction::Out).unwrap();
        loop {
            println!("OFF");
            motor_pin1.set_value(0).unwrap();
            motor_pin2.set_value(0).unwrap();
            sleep(Duration::from_millis(1000));
            println!("ON");
            motor_pin1.set_value(1).unwrap();
            motor_pin2.set_value(1).unwrap();
            sleep(Duration::from_millis(1000));
        }
    }).unwrap();
}
