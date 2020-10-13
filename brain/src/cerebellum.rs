use std::fs::File;
use thiserror::Error;

extern crate serde_yaml;

#[derive(Error, Debug)]
pub enum CerebellumError {
    /// Now it's just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    /// Represents a failure to get any related actions, even if it's because there's none
    #[error("No related Config error")]
    NoConfigError,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CrbllumAction {
    pub motor_l: i16,
    pub motor_r: i16,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Rule {
    pub time: String,
    pub motor_l: String,
    pub motor_r: String,
    pub tracker: String,
    pub distance: String,
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CrbllumEntry {
    pub input: Vec<Rule>,
    pub action: CrbllumAction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cerebellum {
    pub entries: Vec<CrbllumEntry>,
}

impl Cerebellum {
    pub fn new(crbllumconfigfile: String) -> Self {
        let filepointer = File::open(crbllumconfigfile).unwrap();

        let e: Vec<CrbllumEntry> = serde_yaml::from_reader(filepointer).unwrap();
        Self {
            entries: e,
        }
    }
}
