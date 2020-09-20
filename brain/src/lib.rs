pub mod config;
pub mod config_test;
pub mod brain;
pub mod brain_test;
pub mod arduino;
pub mod arduino_test;
pub mod mover;

extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

pub fn get_now() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    datetime.format("%d/%m/%Y-%T").to_string()
}


pub fn log(sender: Option<&str>, raw_log_level: &str, raw_message: &str) {
    // right now, static. This can be changed to ERROR, WARNING. INFO, DEBUG
    let logging_type = "DEBUG";

    // Make sure log_level is one of the expected values
    let log_level = match raw_log_level {
        "D" | "I" | "W" | "E" => raw_log_level.to_string(),
        "d" | "i" | "w" | "e" => raw_log_level.to_uppercase(),
        _ => "D".to_string(),
    };
    // We make the newline characters on the message visible
    let message = raw_message.replace("\n", "\\n");

    //Control Log levels
    // 0-Errors, 1- Warnings, 2-Info, 3-Debug
    let mut logging = 0; // by default, only errors
    match logging_type {
        "DEBUG" => logging = 3,
        "INFO" => logging = 2,
        "WARNING" => logging = 1,
        "ERROR" => logging = 0,
        &_ => (),
    };

    if logging >= 3 && log_level == "D" {
        match sender {
            Some(has_sender) => println!("{}-{}-{}:{}", get_now(), log_level, has_sender, message),
            None => println!("{}-{}:{}", get_now(), log_level, message),
        }
    } 
    if logging >= 2 && log_level == "I" {
        match sender {
            Some(has_sender) => println!("{}-{}-{}:{}", get_now(), log_level, has_sender, message),
            None => println!("{}-{}:{}", get_now(), log_level, message),
        }
    } 
    if logging >= 1 && log_level == "W" {
        match sender {
            Some(has_sender) => println!("{}-{}-{}:{}", get_now(), log_level, has_sender, message),
            None => println!("{}-{}:{}", get_now(), log_level, message),
        }
    } 
    if logging == 0 && log_level == "E" {
        match sender {
            Some(has_sender) => println!("{}-{}-{}:{}", get_now(), log_level, has_sender, message),
            None => println!("{}-{}:{}", get_now(), log_level, message),
        }
    }
    // Two modes: Sender important, Sender not important
}
