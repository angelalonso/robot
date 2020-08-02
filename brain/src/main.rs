use std::error::Error;
use brain::mockduino::Mockduino;
use brain::brain::Brain;
use std::thread;

// TODO: remove this from here if possible
extern crate serde_derive;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate our Brain object
    let mut main_brain = Brain::new("main.cfg.yaml", "from_mockduino.q", "to_mockduino.q");
    // Generate a Mockduino object
    let mockduino_name = "Arduino";
    let mut arduino = Mockduino::new(mockduino_name, "mockduino.cfg.yaml", "to_mockduino.q", "from_mockduino.q");

    // Simulate some delay on booting up the Mockduino
    println!("Booting {}...", mockduino_name);
    let get_bootload = arduino.bootload();
    println!("{:?}", get_bootload);

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
