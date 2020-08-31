pub mod config;
pub mod config_test;
pub mod brain;
pub mod brain_test;
pub mod comm;
pub mod arduino;

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
    // Make sure log_level is one of the expected values
    let log_level = match raw_log_level {
        "D" | "I" | "W" | "E" => raw_log_level.to_string(),
        "d" | "i" | "w" | "e" => raw_log_level.to_uppercase(),
        _ => "D".to_string(),
    };
    // We make the newline characters on the message visible
    let message = raw_message.replace("\n", "\\n");
    // Two modes: Sender important, Sender not important
    match sender {
        Some(has_sender) => println!("{}-{}-{}:{}", get_now(), log_level, has_sender, message),
        None => println!("{}-{}:{}", get_now(), log_level, message),
    }
}
