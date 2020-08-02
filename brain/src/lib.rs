pub mod config;
pub mod tests;
pub mod mockduino;
pub mod brain;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    trigger: String,
    actions: Vec<String>,
}

