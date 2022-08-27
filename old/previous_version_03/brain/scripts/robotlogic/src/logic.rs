use pyo3::prelude::*;
#[pymodule]

mod conds4comps;
use conds4comps::get_result;

use serde::{Deserialize};
use std::collections::HashMap;
use thiserror::Error;

/// We create a set of typical errors BotLogic may find
#[derive(Error, Debug, PartialEq)]
pub enum BrainDeadError {
    /// This is just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    /// Error used when we cannot find a key
    #[error("Key does not exist")]
    KeyNotFoundError,

    /// Error used when we cannot find an action matching the conditions
    #[error("No action found for the conditions")]
    ActionNotFoundError,

    /// Error used when we cannot find an action matching the conditions
    #[error("The Format of a variable is not what it should be")]
    UnexpectedFormatError,
}

/// State is a HashMap of variables and their current status
#[pyclass]
#[derive(Clone, Debug)]
pub struct State {
    data: HashMap<String, String>
}

/// It can only be created empty
impl State {
    #[allow(dead_code)]
    fn new() -> State {
        State {data: HashMap::new()}
    }

/// Creates a new pair of variable (e.g.: temperature) and its state
    #[allow(dead_code)]
    fn set(&mut self, key: String, val: String) {
        self.data.insert(key.to_string(), val.to_string());
    }

    #[allow(dead_code)]
    fn get(&mut self, key: &str) -> Result<&str, BrainDeadError> {
        if self.data.contains_key(key) {
            return Ok(&self.data[key])
        } else {
            return Err(BrainDeadError::KeyNotFoundError)
        }
    }
}

/// Data struct for a single Conditions -> Action pair.  
/// It stores:
/// - A name for the config, useful to keep track (so far unused)  
/// - A condition(s) String  
/// - An action(s) String  
#[pyclass]
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct Config {
    name: String,
    condition: String,
    action: String,
}

/// Data struct with the current states and all [Config]s defined at the moment
/// It stores:
/// - current state
/// - config with actions to take depending on state
#[pyclass]
#[derive(Clone, Debug)]
pub struct Botlogic {
    state: State,
    config: Vec<Config>
}

/// Manages the relation between [Config]s and current [State]s.
impl Botlogic {
    /// The config Vec gets loaded from a Yaml file
    #[allow(dead_code)]
    pub fn new(configfile: &str) -> Botlogic {
        let f = match std::fs::File::open(configfile) {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let mut c: Vec<Config> = [].to_vec();
        c = match serde_yaml::from_reader(f) { 
            Ok(cfg) => cfg,
            Err(_err) => c,
        };
        let mut logic_state = State::new();
        logic_state.set("mode".to_string(), "mapping".to_string());
        Botlogic {
            state: logic_state,
            config: c,
        }
    }

    /// Sets a value for the state of a given key (key being time, distance, amount of light...) 
    #[allow(dead_code)]
    pub fn set_state(&mut self, key: String, val: String) {
        // TODO control what is received here
        self.state.set(key, val);
    }

    /// Gets the value for the state of a given key (key being time, distance, amount of light...) 
    #[allow(dead_code)]
    pub fn get_state(&mut self, key: &str) -> Result<&str, BrainDeadError> {
        return self.state.get(&key)
    }

    /// Returns a copy of the state's data
    #[allow(dead_code)]
    fn get_states(&mut self) -> Result<HashMap<String, String>, BrainDeadError> {
        return Ok(self.state.data.clone())
    }

    /// Returns the action that matches the current set of [State]s
    /// TO BE IMPROVED
    #[allow(dead_code)]
    pub fn get_action(&mut self) -> Result<String, BrainDeadError> {
        let mut result: String = String::new();
        for v in self.config.clone().iter() {
            if get_result(&v.condition, &self.state.data) {
                result = v.action.clone();
            };
        };
        if result.is_empty() {
            return Err(BrainDeadError::ActionNotFoundError)
        } else {
            return Ok(result)
        }
    }
}

mod test;