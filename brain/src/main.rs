use rust_gpiozero::*;
use std::io;
use std::io::prelude::*;

fn main() {
    // Create a new LED attached to Pin 17
    let mut motor_a = Motor::new(2, 3);
    let mut motor_a_ena = PWMLED::new(25);

    motor_a.forward();

}
