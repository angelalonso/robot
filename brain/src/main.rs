extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let my_led = Pin::new(16); // number depends on chip, etc.
    my_led.with_exported(|| {
        my_led.set_direction(Direction::Out).unwrap();
        loop {
            println!("OFF");
            my_led.set_value(0).unwrap();
            sleep(Duration::from_millis(1000));
            println!("ON");
            my_led.set_value(1).unwrap();
            sleep(Duration::from_millis(1000));
        }
    }).unwrap();
}
