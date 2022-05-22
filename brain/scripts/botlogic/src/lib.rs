use std::collections::HashMap;
use serde::{Deserialize};
use thiserror::Error;

// TODO:
//   - Handle conditions better, maybe divide into conditions and interpret
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
struct Config {
    name: String,
    condition: String,
    action: String,
}


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
    fn get_action(&mut self) -> Result<String, BrainDeadError> {
        let mut result: String = String::new();
        for v in self.config.clone().iter() {
            //println!("{:?}", self.test_conditions(v.condition.clone()).unwrap());
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
            println!("{:?} != {:?}", self.get_state(vec_cmp_ne[0]).unwrap(), vec_cmp_ne[1]);
            if self.get_state(vec_cmp_ne[0]).unwrap() != vec_cmp_ne[1] {
                return Ok(true)
            };
        } else {
            let vec_cmp_eq: Vec<&str> = cmp.split("==").collect();
            if vec_cmp_eq.len() == 2 {
                println!("{:?} == {:?}", self.get_state(vec_cmp_eq[0]).unwrap(), vec_cmp_eq[1]);
                if self.get_state(vec_cmp_eq[0]).unwrap() == vec_cmp_eq[1] {
                    return Ok(true)
                };
            };
        };
        // >= and > must be exclusive to each other to avoid misunderstandings
        let vec_cmp_ge: Vec<&str> = cmp.split(">=").collect();
        if vec_cmp_ge.len() == 2 {
            println!("{:?} >= {:?}", self.get_state(vec_cmp_ge[0]).unwrap(), vec_cmp_ge[1]);
            if self.get_state(vec_cmp_ge[0]).unwrap().parse::<f32>().unwrap() >= vec_cmp_ge[1].parse::<f32>().unwrap() {
                return Ok(true)
            };
        } else {
            let vec_cmp_gt: Vec<&str> = cmp.split(">").collect();
            if vec_cmp_gt.len() == 2 {
                println!("{:?} > {:?}", self.get_state(vec_cmp_gt[0]).unwrap(), vec_cmp_gt[1]);
                if self.get_state(vec_cmp_gt[0]).unwrap().parse::<f32>().unwrap() > vec_cmp_gt[1].parse::<f32>().unwrap() {
                    return Ok(true)
                };
            };
        };
        // <= and < must be exclusive to each other to avoid misunderstandings
        let vec_cmp_le: Vec<&str> = cmp.split("<=").collect();
        if vec_cmp_le.len() == 2 {
            println!("{:?} <= {:?}", self.get_state(vec_cmp_le[0]).unwrap(), vec_cmp_le[1]);
            if self.get_state(vec_cmp_le[0]).unwrap().parse::<f32>().unwrap() <= vec_cmp_le[1].parse::<f32>().unwrap() {
                return Ok(true)
            };
        } else {
            let vec_cmp_lt: Vec<&str> = cmp.split("<").collect();
            if vec_cmp_lt.len() == 2 {
                println!("{:?} < {:?}", self.get_state(vec_cmp_lt[0]).unwrap(), vec_cmp_lt[1]);
                if self.get_state(vec_cmp_lt[0]).unwrap().parse::<f32>().unwrap() < vec_cmp_lt[1].parse::<f32>().unwrap() {
                    return Ok(true)
                };
            };
        };
        return Ok(false)
    }
}

#[cfg(test)]
mod test;
