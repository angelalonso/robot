use crate::arduino::Arduino;
use crate::config::Config;
use crate::log;
use crate::mover::Mover;
use std::fs::File;
use std::process;
use std::str;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainDeadError {
    /// It used to represent an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    /// Now it's just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("Config contains no related entries")]
    NoConfigFound,

    #[error("Something went wrong while working with timestamps")]
    SystemTimeError,
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
    pub starttime: u128,
    pub config: Config,
    pub arduino: Arduino<'a>,
    pub serialport: &'a str,
    pub timeout: u64,
    pub mover: Mover<'a>,
}

impl Brain<'static> {
    pub fn new(brain_name: &'static str, config_file: String, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        let st = SystemTime::now();
        let start_time = match st.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
        };
        let cfg = Config::new(config_file);
        let sp = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0",
        };
        // TODO: add vector of latest metrics
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
            starttime: start_time,
            config: cfg,
            arduino: a,
            serialport: sp,
            timeout: 4,
            mover: m,
        })
    }

    pub fn get_input(mut self) {
        let mut latest_metrics: Vec<MetricEntry> = [].to_vec();
        loop {
            let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let msgs = s.clone();
            let mut arduino = self.arduino.clone();
            let mut avatar = self.clone();
            let this_name = self.name.clone();
            let _handle = thread::spawn(move || {
                let _received = match arduino.read_channel(msgs){
                    Ok(rcv) => {
                        let _taken_actions = match avatar.get_brain_actions(&rcv){
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
                let mut msg_sensors = String::new();
                // TODO: is this needed with a ruleset?
                let actionmsg = msg.clone();
                let sensormsg = msg.clone();
                if actionmsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "ACTION".to_string() {
                    println!("got ACTION");
                    msg_actions.push(msg.unwrap().replace("ACTION: ", ""));
                } else if sensormsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "SENSOR".to_string() {
                    println!("got SENSOR");
                    msg_sensors = msg.unwrap().replace("SENSOR: ", "");
                }
                self.do_brain_actions(msg_actions).unwrap();
                // TODO: use the following ones to build the current metric
                let current_metric = self.build_crbllum_input(msg_sensors).unwrap();
                println!("CURRENT METRIC{:?}", current_metric);
                self.do_crbllum_actions(&current_metric, &mut latest_metrics).unwrap();
            }
        }
    }
    ///------------------------------------------------------///
    ///  Brain
    ///------------------------------------------------------///
    /// Get the action that relates to the trigger received and call to apply it
    /// Hm...maybe this one and do_brain_actions should go together?
    pub fn get_brain_actions(&mut self, trigger: &str) -> Result<Vec<String>, BrainDeadError> {
        log(Some(&self.name), "D", &format!("Received {}", trigger));
        let actions = self.config.get_actions(&trigger);
        match actions {
            Ok(acts) => {
                match acts {
                    Some(a) => {
                        self.do_brain_actions(a.clone()).unwrap();
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
    pub fn do_brain_actions(&mut self, actions: Vec<String>) -> Result<(), BrainDeadError> {
        for action in &actions {
            let action_vec: Vec<&str> = action.split('_').collect();
            match action_vec[0] {
                "install" => self.arduino.install(&action_vec[1..].to_vec().join("_")).unwrap(),
                "move" => self.mover.set_move(action_vec[1..].to_vec().join("_")),
                "data" => self.decide_on(action_vec[1..].to_vec().join("_")),
                _ => self.do_brain_nothing().unwrap(),
            };
        }
        Ok(())
    }

    /// Do nothing, but take note that we have nothing to do
    /// TODO: can this be removed?
    pub fn do_brain_nothing(&mut self) -> Result<(), BrainDeadError> {
        log(Some(&self.name), "D", "Relaxing here...");
        Ok(())
    }


    ///------------------------------------------------------///
    ///  Cerebellum
    ///------------------------------------------------------///
    pub fn get_crbllum_config(&mut self) -> Result<Vec<RuleEntry>, BrainDeadError> {
        let filepointer = File::open("move_cfg.yaml").unwrap();
        let rules: Vec<RuleEntry> = serde_yaml::from_reader(filepointer).unwrap();
        Ok(rules)
    }

    pub fn build_crbllum_input(&mut self, sensors: String) -> Result<MetricEntry, BrainDeadError> {
        log(Some(&self.name), "I", &format!("Moving : {}", self.mover.movement));
        let m_l: i16;
        let m_r: i16;
        if self.mover.movement == "forwards" {
            m_l = 100;
            m_r = 100;
        } else if self.mover.movement == "forwards_slow" {
            m_l = 55;
            m_r = 55;
        } else if self.mover.movement == "backwards" {
            m_l = -100;
            m_r = -100;
        } else if self.mover.movement == "rotate_left" {
            m_l = -70;
            m_r = 70;
        } else if self.mover.movement == "rotate_right" {
            m_l = 70;
            m_r = -70;
        } else {
            let motor_values: Vec<i16> = match self.mover.movement.split("_")
                .map(|s| s.parse())
                .collect() {
                    Ok(v) => v,
                    Err(_e) => [0,0].to_vec(),
                };
            m_l = motor_values[0];
            m_r = motor_values[1];
        };
        //TODO: adapt time and sensor
        let ct = SystemTime::now();
        let current_time = match ct.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => return Err(BrainDeadError::SystemTimeError),
        };
        let diff_time: f64 = (current_time as f64 - self.starttime as f64) as f64 / 100 as f64;
        println!("SENSORS {:?}", sensors);
        let sns = sensors.replace("data_tracker_", "");
        println!("SNS   {:?}", sns);
        let m = MetricEntry {
            time: diff_time,
            motor_l: m_l,
            motor_r: m_r,
            sensor: true,
        };
        Ok(m)
    }

    /// TODO: modify this code to maybe do more of what the title mentions (probably take over the
    /// previous function)
    pub fn get_crbllum_input<'a>(&mut self, metric: &'a MetricEntry, latest_metrics: &'a mut Vec<MetricEntry>) -> Result<&'a mut Vec<MetricEntry>, BrainDeadError> {
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
        Ok(latest_metrics)
    }

    pub fn choose_crbllum_actions(&mut self, rules: Vec<RuleEntry>, metric: &MetricEntry) -> Result<Vec<RuleEntry>, BrainDeadError>{
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

    pub fn do_crbllum_actions<'a>(&mut self, metric: &'a MetricEntry, latest_metrics: &'a mut Vec<MetricEntry>) -> Result<Vec<RuleEntry>, BrainDeadError> {
        self.get_crbllum_input(metric, latest_metrics).unwrap();
        let ruleset = match self.get_crbllum_config(){
            Ok(r) => self.choose_crbllum_actions(r, metric),
            Err(e) => Err(e),
        };
        ruleset
    }

    /// TODO: remove? move somewhere else?
    pub fn decide_on(&mut self, info: String) {
        println!("{}", info);
    }
}
