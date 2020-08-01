use std::error::Error;
use brain::comm::Messages;
use brain::mockduino::Mockduino;
use std::thread;
use std::fs::File;
use brain::Reaction;
use brain::config::Config;

// TODO: remove this from here if possible
#[macro_use]
extern crate serde_derive;

fn main() -> Result<(), Box<dyn Error>> {
    let mut conf = Config::new("main.cfg.yaml");
    conf.print();
    // Simulating Arduino init
    let mut arduino = Mockduino::new("mockduino.cfg.yaml", "to_mockduino.q", "from_mockduino.q");
    println!("Booting arduino...");
    let get_bootload = arduino.bootload();
    println!("{:?}", get_bootload);
    // Listening on Bus
    let mut test = Messages::new();
    let _read = thread::spawn(move || {
        test.read_msgs_mock();
    });
    // Arduino also listening on Bus
    arduino.read_msgs();

    Ok(())
}
