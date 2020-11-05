pub mod config;
pub mod config_test;
pub mod brain;
pub mod brain_test;
pub mod arduino;
pub mod arduino_test;
pub mod motors;
pub mod leds;

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
