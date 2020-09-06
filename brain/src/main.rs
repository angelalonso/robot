use std::error::Error;
use brain::brain::Brain;
use std::process;

/// We need to load a new brain, install the first trigger to arduino, and enter the reading loop
fn main() -> Result<(), Box<dyn Error>> {
    // Generate our Brain object
    let mut main_brain = Brain::new("Main Brain", "cfg.yaml", None).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Main Brain: {}", err);
        process::exit(1);
    });
    // Send the first file that triggers our ping-pong loop of messages to start.
    let _send_first = main_brain.arduino.install("../arduino/001_test_wheel/001_test_wheel.ino.hex").unwrap_or_else(|err| {
        eprintln!("Problem sending the first program to the Arduino: {}", err);
        process::exit(1);
    });
    // Listening on Comm
    main_brain.read();
    Ok(())
}
