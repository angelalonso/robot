pub mod comm;
pub mod config;
pub mod action;
pub mod tests;
pub mod mockduino;

pub mod pollwatch;
// pub mod tailwatch;

pub use pollwatch::PollWatcher;
// pub use tailwatch::TailWatcher;

pub trait Watcher<'a> {
    fn new(filename: &str, period_milliseconds: u64) -> Self;
    fn register(&mut self, callback: Box<dyn FnMut(String) + 'a>);
    fn watch(&mut self);
    fn watch_once(&mut self);
}

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    trigger: String,
    actions: Vec<String>,
}

