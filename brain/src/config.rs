use std::fs::File;
use thiserror::Error;

extern crate serde_yaml;

#[derive(Error, Debug)]
pub enum BrainConfigError {
    /// This is just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    /// Represents a failure to get any related actions, even if it's because there's none
    #[error("No related Config error")]
    NoConfigError,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigEntry {
    trigger: String,
    actions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    entries: Vec<ConfigEntry>,
}

impl Config {
    pub fn new(configfile: String) -> Self {
        let filepointer = File::open(configfile).unwrap();

        let reactions: Vec<ConfigEntry> = serde_yaml::from_reader(filepointer).unwrap();
        Self {
            entries: reactions,
        }
    }

    /// Print out the configs.
    /// Note: I dont test this because why?
    pub fn print(&mut self) {
        println!("{:?}", self.entries);
    }

    /// Gets the actions related to a trigger
    /// , which we normalize here.
    /// Also: we return a BrainConfigError if there's no related actions for now...
    pub fn get_actions(&mut self, raw_trigger: &str) -> Result<Option<Vec<String>>, BrainConfigError> {
        let configentry = raw_trigger.to_lowercase()
            .replace("\\r", "")
            .replace("\r", "")
            .replace("\\n", "")
            .replace("\n", "");
        let entry = self.entries.iter()
            .find(|&x| x.trigger.to_lowercase() == configentry);
        match entry {
            Some(x) => Ok(Some(x.actions.to_vec())),
            None => Err(BrainConfigError::NoConfigError),
        }
    }
}
