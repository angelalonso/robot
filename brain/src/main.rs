use rust_gpiozero::*;
use std::io;
use std::io::prelude::*;
use std::{thread, time};

fn main() {
    // Create a new LED attached to Pin 17
    let mut motor_a = Motor::new(2, 3);
    let mut motor_a_ena = OutputDevice::new(25);

    motor_a.forward();
    motor_a_ena.on();
    //motor_a_ena.set_active_high(true);
    let sec = time::Duration::from_millis(1000);
    thread::sleep(sec);
    motor_a.backward();
    thread::sleep(sec);
    motor_a.stop();
    motor_a_ena.off();

}
