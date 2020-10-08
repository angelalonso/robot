use crate::config::Config;
use crate::arduino::Arduino;
use crate::mover::Mover;
use crate::log;
use std::process;
use std::str;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use thiserror::Error;
use std::fs::File;

#[derive(Error, Debug)]
pub enum BrainDeadError {
    /// It used to represent an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    /// Now it's just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("Config contains no related entries")]
    NoConfigFound,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MetricEntry {
    time: f64,
    motor_l: i16,
    motor_r: i16,
    sensor: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MoveAction {
    motor_l: i16,
    motor_r: i16,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RuleInput {
    time: String,
    motor_l: String,
    motor_r: String,
    sensor: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RuleEntry {
    input: Vec<RuleInput>,
    action: MoveAction,
}

#[derive(Clone)]
pub struct Brain<'a> {
    pub name: &'a str,
    pub config: Config,
    pub arduino: Arduino<'a>,
    pub serialport: &'a str,
    pub timeout: u64,
    pub mover: Mover<'a>,
}

impl Brain<'static> {
    pub fn new(brain_name: &'static str, config_file: String, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        let configdata = Config::new(config_file);
        let sp = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0",
        };
        let a = Arduino::new("arduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        let m = Mover::new().unwrap_or_else(|err| {
            eprintln!("Problem Initializing Mover: {}", err);
            process::exit(1);
        });
        Ok(Self {
            name: brain_name,
            config: configdata,
            arduino: a,
            serialport: sp,
            timeout: 4,
            mover: m,
        })
    }

    pub fn read(mut self) {
        loop {
            self.show_move();
            let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let msgs = s.clone();
            let mut arduino = self.arduino.clone();
            let mut avatar = self.clone();
            let this_name = self.name.clone();
            let _handle = thread::spawn(move || {
                let _received = match arduino.read_channel(msgs){
                    Ok(rcv) => {
                        let _taken_actions = match avatar.get_actions(&rcv){
                            Ok(acts) => println!("Taking action {:?}", acts.join(", ")),
                            Err(_) => log(Some(&this_name), "D", "No actions were found for trigger"),
                        };
                    },
                    Err(_) => log(Some(&this_name), "D", "Nothing read from Channel"),
                };
            });
            loop {
                let msg = r.recv();
                let mut msg_actions = Vec::new();
                msg_actions.push(msg.unwrap().replace("ACTION: ", "").replace("SENSOR: ", ""));
                self.apply_actions(msg_actions).unwrap();
                self.show_move();
            }
        }
    }

    /// Get the action that relates to the trigger received and call to apply it
    /// Hm...maybe this one and apply_actions should go together?
    pub fn get_actions(&mut self, trigger: &str) -> Result<Vec<String>, BrainDeadError> {
        log(Some(&self.name), "D", &format!("Received {}", trigger));
        let actions = self.config.get_actions(&trigger);
        match actions {
            Ok(acts) => {
                match acts {
                    Some(a) => {
                        self.apply_actions(a.clone()).unwrap();
                        Ok(a)
                    },
                    None => {
                        log(Some(&self.name), "D", "Nothing to do");
                        Err(BrainDeadError::NoConfigFound)
                    },
                }
            },
            Err(_e) => Err(BrainDeadError::NoConfigFound),
        }
    }

    /// Call the action needed
    /// , which, right now, is just installing a new hex into the arduino
    pub fn apply_actions(&mut self, actions: Vec<String>) -> Result<(), BrainDeadError> {
        for action in &actions {
            let action_vec: Vec<&str> = action.split('_').collect();
            match action_vec[0] {
                "install" => self.arduino.install(&action_vec[1..].to_vec().join("_")).unwrap(),
                "move" => self.mover.set_move(action_vec[1..].to_vec().join("_")),
                "data" => self.decide_on(action_vec[1..].to_vec().join("_")),
                _ => self.do_nothing().unwrap(),
            };
        }
        Ok(())
    }

    /// Do nothing, but take note that we have nothing to do
    pub fn do_nothing(&mut self) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "D", "Relaxing here...");
        Ok(())
    }


    /// Show current movement values at both engines
    pub fn show_move(&mut self) {
        log(Some(&self.name), "I", &format!("Moving : {}", self.mover.movement));
    }

    pub fn read_rules(&mut self) -> Result<Vec<RuleEntry>, Box<std::error::Error>> {
        let filepointer = File::open("predefined_rules.yaml").unwrap();
        let rules: Vec<RuleEntry> = serde_yaml::from_reader(filepointer).unwrap();
        Ok(rules)
    }

    pub fn update_metrics<'a>(&mut self, metric: &'a MetricEntry, latest_metrics: &'a mut Vec<MetricEntry>) -> &'a mut Vec<MetricEntry> {
        if latest_metrics.len() == 0 {
            latest_metrics.push(metric.clone());
            
        } else {
            if metric.motor_l == latest_metrics[0].motor_l &&
               metric.motor_r == latest_metrics[0].motor_r &&
               metric.sensor == latest_metrics[0].sensor
            {
                latest_metrics[0].time += 0.1;
            } else {
                latest_metrics.push(metric.clone());
                latest_metrics.rotate_right(1);
                latest_metrics[0].time = 0.1;
            }
            if latest_metrics.len() > 5{
                latest_metrics.pop();
            }
        }
        latest_metrics
    }

    /// -- Decission making
    pub fn decide_on(&mut self, info: String) {
        println!("{}", info);
    }

    pub fn act_from_metrics<'a>(&mut self, metric: &'a MetricEntry, mut latest_metrics: &'a mut Vec<MetricEntry>) -> Vec<RuleEntry>{
        latest_metrics = self.update_metrics(metric, latest_metrics);
        let ruleset = match self.read_rules(){
            Ok(r) => self.choose_rule(r, metric),
            Err(e) => Err(e),
        };
        ruleset.unwrap()
    }

    pub fn choose_rule(&mut self, rules: Vec<RuleEntry>, metric: &MetricEntry) -> Result<Vec<RuleEntry>, Box<std::error::Error>>{
        // add partially matching rules, then add to matching_rules only those matching all
        let mut partial_rules: Vec<RuleEntry> = [].to_vec();
        // Check time
        for rule in rules {
            if rule.input[0].time != "*" {
                if metric.time >= rule.input[0].time.parse::<f64>().unwrap() {
                    partial_rules.push(rule);
                }
            } else {
                partial_rules.push(rule);
            }
        }
        // Check motors
        for rule in partial_rules.clone() {
            if rule.input[0].motor_l != "*" {
                if metric.motor_l != rule.input[0].motor_l.parse::<i16>().unwrap() {
                    partial_rules.retain(|x| *x != rule);
                }
            }
        }
        for rule in partial_rules.clone() {
            if rule.input[0].motor_r != "*" {
                if metric.motor_r != rule.input[0].motor_r.parse::<i16>().unwrap() {
                    partial_rules.retain(|x| *x != rule);
                }
            }
        }
        for rule in partial_rules.clone() {
            if rule.input[0].sensor != "*" {
                if metric.sensor != rule.input[0].sensor.parse::<bool>().unwrap() {
                    partial_rules.retain(|x| *x != rule);
                }
            }
        }
        Ok(partial_rules)
    }
}
