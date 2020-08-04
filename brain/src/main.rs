use std::error::Error;
use brain::brain::Brain;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate our Brain object
    let mut main_brain = Brain::new("Main Brain", "main.cfg.yaml", "from_mockduino.q", "to_mockduino.q");
    // Generate a Mockduino object
    let mut arduino = Brain::new("Mockduino", "mockduino.cfg.yaml", "to_mockduino.q", "from_mockduino.q");

    // Simulate some delay on booting up the Mockduino
    let _boot = arduino.bootload();

    // Arduino starts listening on Bus
    let _arduino_read = thread::spawn(move || {
        let _arduino_thread = arduino.read_msgs();
    });
    // Send the first message that triggers our ping-pong loop
    main_brain.send("Do->Ping");
    // Listening on Bus
    let _brain_thread = main_brain.read_msgs();

    Ok(())
}
