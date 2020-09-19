use rust_gpiozero::*;
use std::io;
use std::io::prelude::*;
use std::{thread, time};

fn main() {
    // Create a new LED attached to Pin 17
    let mut motor_a = Motor::new(2, 3);
    //let mut motor_a_ena = OutputDevice::new(25);
    let mut motor_a_ena = PWMOutputDevice::new(25);

    println!("FORWARD");
    motor_a.forward();
    motor_a_ena.on();
    motor_a_ena.set_value(0.01);
    let sec = time::Duration::from_millis(1000);
    thread::sleep(sec);
    println!("BACKWARD");
    motor_a_ena.set_value(0.5);
    motor_a.backward();
    thread::sleep(sec);
    motor_a.stop();
    motor_a_ena.off();

    let wait = time::Duration::from_millis(100);
    motor_a.forward();
    motor_a_ena.on();
    motor_a_ena.set_value(0);
    println!("FORWARD");
    for i in 0..100 {
        let i = i as f64 * 0.01;
        motor_a_ena.set_value(i);
        thread::sleep(wait);
    }
    motor_a.backward();
    motor_a_ena.on();
    motor_a_ena.set_value(0);
    println!("BACKWARD");
    for i in 0..100 {
        let i = i as f64 * 0.01;
        motor_a_ena.set_value(i);
        thread::sleep(wait);
    }
    motor_a.stop();
    motor_a_ena.off();

}
