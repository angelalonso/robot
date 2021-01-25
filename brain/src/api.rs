use rocket::State;
use log::{debug, info};
use std::sync::mpsc::{SyncSender, Receiver};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum BrainArduinoError {
    /// It used to represent an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    /// Now it's just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("{0} is NOT installed (or something went wrong while checking that it is)")]
    ProgNotInstalledError(String),

    #[error("AvrDude could not install the program to your Arduino!")]
    AvrdudeError,

    #[error("Source contains no data")]
    IOError,
}


#[get("/")]
fn all(channel: State<SyncSender<String>>) -> String {
    channel.send("DO: TEST=TEST|TEST2=TEST2".to_string()).unwrap();
    "OK".to_string()
}

//fn main() {
//    let channel = "TEST".to_string();
//    rocket::ignite()
//        .manage(channel)
//        .mount("/",
//               routes![all],
//               ).launch();
//}

#[derive(Clone)]
pub struct Api {
    pub channel: String,
}

impl Api {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            channel: "s".to_string(),
        })
    }

    pub fn run(&mut self, channel: SyncSender<String>) {
        rocket::ignite()
            .manage(channel)
            .mount("/",
                   routes![all],
                   ).launch();
    }

}
