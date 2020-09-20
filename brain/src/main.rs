use rust_gpiozero::*;
use std::{thread, time};

fn main() {
    // Create a new LED attached to Pin 17
    let mut motor_a = Motor::new(17, 27);
    let mut motor_a_ena = PWMOutputDevice::new(22);
    let mut motor_b = Motor::new(23, 24);
    let mut motor_b_ena = PWMOutputDevice::new(25);


    let wait = time::Duration::from_millis(100);
    motor_a.forward();
    motor_a_ena.on();
    motor_a_ena.set_value(0.0);
    motor_b.forward();
    motor_b_ena.on();
    motor_b_ena.set_value(0.0);
    println!("FORWARD");
    for i in 0..100 {
        let i = i as f64 * 0.01;
        println!("{:?}", i);
        motor_a_ena.set_value(i);
        motor_b_ena.set_value(i);
        thread::sleep(wait);
    }
    motor_a.backward();
    motor_a_ena.on();
    motor_a_ena.set_value(0.0);
    motor_b.backward();
    motor_b_ena.on();
    motor_b_ena.set_value(0.0);
    println!("BACKWARD");
    for i in 0..100 {
        let i = i as f64 * 0.01;
        println!("{:?}", i);
        motor_a_ena.set_value(i);
        motor_b_ena.set_value(i);
        thread::sleep(wait);
    }
    motor_a.stop();
    motor_a_ena.off();
    motor_b.stop();
    motor_b_ena.off();

}
