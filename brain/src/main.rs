use std::error::Error;
use brain::brain::Brain;
use brain::brain::Brainy;
use std::thread;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate our Brain object
    let mut main_brain = Brain::new("Main Brain", "main_cfg.yaml", "from_mockduino.q", "sto_mockduino.q", None).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Main Brain: {}", err);
        process::exit(1);
    });
    // Generate a Mockduino object
    let mut arduino = Brain::new("Mockduino", "mockduino_cfg.yaml", "to_mockduino.q", "from_mockduino.q", None).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Mockduino: {}", err);
        process::exit(1);
    });

    // Simulate some delay on booting up the Mockduino
    let _boot = arduino.bootload();

    // Arduino starts listening on Bus
    let _arduino_read = thread::spawn(move || {
        let _arduino_thread = arduino.read_msgs();
    });
    // Send the first file that triggers our ping-pong loop of messages to start.
    main_brain.send("ping");
    //main_brain.sendfileserial("../../tests/000_blick_internal_led_seconds/000_blick_internal_led_seconds.ino.hex");
    // Listening on Bus
    let _brain_thread = main_brain.read_msgs();

    Ok(())
}
