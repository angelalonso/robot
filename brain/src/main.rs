use std::error::Error;
use brain::brain::Brain;
//use std::thread;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate our Brain object
    let mut main_brain = Brain::new_serial("Main Brain", "main_cfg.yaml", None).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Main Brain: {}", err);
        process::exit(1);
    });
    // Send the first file that triggers our ping-pong loop of messages to start.
    let _send_first = main_brain.install_to_arduino("../tests/000_blick_internal_led_seconds/000_blick_internal_led_seconds.ino.hex").unwrap_or_else(|err| {
    //let _send_first = main_brain.install_to_arduino("../tests/000_blick_internal_led_seconds_inverted/000_blick_internal_led_seconds_inverted.ino.hex").unwrap_or_else(|err| {
    //let _send_first = main_brain.install_to_arduino("../tests/000_blick_internal_led_seconds_inverted/000_blick_internal_led_seconds_inverted.inos.hex").unwrap_or_else(|err| {
        eprintln!("Problem sending the first program to the Arduino: {}", err);
        process::exit(1);
    });
    // Listening on Bus
    let mut _brain_thread = main_brain.read_loop_from_arduino();
    Ok(())
}
