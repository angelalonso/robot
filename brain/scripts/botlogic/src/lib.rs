mod conds4comps;

use regex;
use serde::{Deserialize};
use std::collections::HashMap;
use thiserror::Error;
use std::str::FromStr;

// TODO:
//   - Move things outside of here
//

#[derive(Error, Debug)]
pub enum BrainDeadError {
    /// This is just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("Key does not exist")]
    KeyNotFoundError,

    #[error("No action found for the conditions")]
    ActionNotFoundError,

}

#[derive(Clone, Debug)]
pub struct State<'a> {
    data: HashMap<&'a str, &'a str>
}

impl<'a> State<'a> {
    #[allow(dead_code)]
    fn new() -> State<'a> {
        State {data: HashMap::new()}
    }

    #[allow(dead_code)]
    fn set(&mut self, key: &'a str, val: &'a str) {
        self.data.insert(&key, &val);
    }

    #[allow(dead_code)]
    fn get(&mut self, key: &'a str) -> Result<&str, BrainDeadError> {
        if self.data.contains_key(&key) {
            return Ok(self.data[&key])
        } else {
            return Err(BrainDeadError::KeyNotFoundError)
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct Config {
    name: String,
    condition: String,
    action: String,
}


#[derive(Clone, Debug)]
pub struct Logic<'a> {
    state: State<'a>,
    config: Vec<Config>
}

impl<'a> Logic<'a> {
    #[allow(dead_code)]
    fn new(configfile: &str) -> Logic<'a> {
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
        logic_state.set("mode", "mapping");
        Logic {
            state: logic_state,
            config: c,
        }
    }

    #[allow(dead_code)]
    fn set_state(&mut self, key: &'a str, val: &'a str) {
        self.state.set(&key, &val);
    }

    #[allow(dead_code)]
    fn get_state(&mut self, key: &'a str) -> Result<&str, BrainDeadError> {
        return self.state.get(&key)
    }

    #[allow(dead_code)]
    fn get_states(&mut self) -> Result<HashMap<&str, &str>, BrainDeadError> {
        return Ok(self.state.data.clone())
    }

    #[allow(dead_code)]
    fn get_action(&mut self) -> Result<String, BrainDeadError> {
        let mut result: String = String::new();
        for v in self.config.clone().iter() {
            if v.condition.clone() == self.get_state("time").unwrap() {
                result = v.action.clone()
            }
        };
        if result.is_empty() {
            return Err(BrainDeadError::ActionNotFoundError)
        } else {
            return Ok(result)
        }
    }

        // TODO: split by separator ,
        //   then by && and || --> this requires also getting which operator was catched
        //     then by ==, <, >, <=, >= --> this requires also getting which operator was catched
    #[allow(dead_code)]
    fn test_condition(&mut self, c: &'a str) -> Result<bool, BrainDeadError> {
        let mut result: bool = true;
        let vec_c_and: Vec<&str> = c.split("&&").collect();
        if vec_c_and.len() == 2 {
            let mut and_true: bool = true;
            for c_and in vec_c_and {
                if !self.test_comparison(c_and).unwrap() {
                    and_true = false;
                }
            }
            result = and_true;
        }
        let vec_c_or: Vec<&str> = c.split("||").collect();
        if vec_c_or.len() == 2 {
            let mut or_true: bool = false;
            for c_or in vec_c_or {
                if self.test_comparison(c_or).unwrap() {
                    or_true = true;
                }
            }
            result = or_true;
        }
        Ok(result)
    }

    #[allow(dead_code)]
    fn test_comparison(&mut self, cmp: &'a str) -> Result<bool, BrainDeadError> {
        // >= and > must be exclusive to each other to avoid misunderstandings
        let vec_cmp_ne: Vec<&str> = cmp.split("!=").collect();
        if vec_cmp_ne.len() == 2 {
            if self.get_state(vec_cmp_ne[0]).unwrap() != vec_cmp_ne[1] {
                return Ok(true)
            };
        } else {
            let vec_cmp_eq: Vec<&str> = cmp.split("==").collect();
            if vec_cmp_eq.len() == 2 {
                if self.get_state(vec_cmp_eq[0]).unwrap() == vec_cmp_eq[1] {
                    return Ok(true)
                };
            };
        };
        // >= and > must be exclusive to each other to avoid misunderstandings
        let vec_cmp_ge: Vec<&str> = cmp.split(">=").collect();
        if vec_cmp_ge.len() == 2 {
            if self.get_state(vec_cmp_ge[0]).unwrap().parse::<f32>().unwrap() >= vec_cmp_ge[1].parse::<f32>().unwrap() {
                return Ok(true)
            };
        } else {
            let vec_cmp_gt: Vec<&str> = cmp.split(">").collect();
            if vec_cmp_gt.len() == 2 {
                if self.get_state(vec_cmp_gt[0]).unwrap().parse::<f32>().unwrap() > vec_cmp_gt[1].parse::<f32>().unwrap() {
                    return Ok(true)
                };
            };
        };
        // <= and < must be exclusive to each other to avoid misunderstandings
        let vec_cmp_le: Vec<&str> = cmp.split("<=").collect();
        if vec_cmp_le.len() == 2 {
            if self.get_state(vec_cmp_le[0]).unwrap().parse::<f32>().unwrap() <= vec_cmp_le[1].parse::<f32>().unwrap() {
                return Ok(true)
            };
        } else {
            let vec_cmp_lt: Vec<&str> = cmp.split("<").collect();
            if vec_cmp_lt.len() == 2 {
                if self.get_state(vec_cmp_lt[0]).unwrap().parse::<f32>().unwrap() < vec_cmp_lt[1].parse::<f32>().unwrap() {
                    return Ok(true)
                };
            };
        };
        return Ok(false)
    }

    #[allow(dead_code)]
    fn get_grouping(&mut self, g: &'a str) -> Result<u8, BrainDeadError> {
        let mut groups: u8 = 1;
        for c in g.chars() {
            if c == ')' { 
                groups += 1; 
            }
        }
        Ok(groups)
    }

    #[allow(dead_code)]
    fn test_conditions(&mut self, s: &'a str) -> bool {
        // we cant use them directly from get_state at &self in a loop...
        let curr_states = self.get_states().unwrap();
        let mut vars = HashMap::new();
        let re = regex::Regex::new(r"!|=|<|>|&|\|").unwrap();
        for k in re.split(s) {
            if k.clone() != "" {
                if curr_states.contains_key(k) {
                    let value: i32 = FromStr::from_str(curr_states[k]).unwrap();
                    vars.insert(k.to_string(), value);
                };
            };
        }
        let result = conds4comps::get_result(s, vars);

        return result
    }
}

#[cfg(test)]
mod test;
