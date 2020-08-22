use std::error::Error;
use brain::brain::Brain;
//use std::thread;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate our Brain object
    let mut main_brain = Brain::new_serial("Main Brain", "cfg.yaml", None).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Main Brain: {}", err);
        process::exit(1);
    });
    // Send the first file that triggers our ping-pong loop of messages to start.
    let _send_first = main_brain.install_to_arduino("../arduino/001_test_pong/001_test_pong.ino.hex").unwrap_or_else(|err| {
        eprintln!("Problem sending the first program to the Arduino: {}", err);
        process::exit(1);
    });
    // Listening on Bus
    let mut _brain_thread = main_brain.read_loop();
    Ok(())
}
