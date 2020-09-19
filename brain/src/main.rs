use rust_gpiozero::*;
use std::io;
use std::io::prelude::*;

fn main() {
    // Create a new LED attached to Pin 17
    let mut motor_a_1 = PWMLED::new(3);
    let mut motor_a_ena = PWMLED::new(25);

    // blink the LED 5 times
    motor_a_1.set_blink_count(5);
    println!("BLINKIN");
    motor_a_1.blink(2.0, 2.0, 1.0, 1.0);
    motor_a_ena.set_value(1000.0);

    // wait for key press to exit
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
