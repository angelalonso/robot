use rocket::State;
use rocket::http::RawStr;
use rocket::config::{Config, Environment};
//use log::{debug, info};
use std::sync::mpsc::SyncSender;
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


//#[get("/")]
//fn all(channel: State<SyncSender<String>>) -> String {
//    channel.send("DO: TEST=TEST|TEST2=TEST2".to_string()).unwrap();
//    "OK".to_string()
//}

// curl -X POST 127.0.0.1:8000/do/test=1,test=2
#[post("/<do_stuff>")]
fn post_do(do_stuff: &RawStr, channel: State<SyncSender<String>>) -> String {
    let do_stuff_corrected = do_stuff.replace(",", "|");
    channel.send(format!("DO: {}", do_stuff_corrected.as_str())).unwrap();
    "OK".to_string()
}

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
        //let config = match Config::build(Environment::Staging)
        //    .address("192.168.43.110")
        //    .port(80)
        //    .finalize() {
        //        Ok(cfg) => {
        //            let app = rocket::custom(cfg);
        //        },
        //        Err(_) => ()
        //    };

        rocket::ignite()
            .manage(channel)
            //.mount("/", routes![all])
            .mount("/do/", routes![post_do],)
            .launch();
    }
}
