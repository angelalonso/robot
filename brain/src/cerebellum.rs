use crate::brain::{BrainDeadError, MetricEntry};
use crate::log;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

extern crate serde_yaml;

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
    pub name: String,
    pub entries: Vec<CrbllumEntry>,
    pub current_metric: MetricEntry,
    pub metrics: Vec<MetricEntry>,
}

impl Cerebellum {
    pub fn new(crbllumconfigfile: String) -> Self {
        let filepointer = File::open(crbllumconfigfile).unwrap();

        let e: Vec<CrbllumEntry> = serde_yaml::from_reader(filepointer).unwrap();
        let cm = MetricEntry {
            time: 0.0,
            motor_l: 0,
            motor_r: 0,
            tracker: false,
            distance: 0,
        };
        let mtr: Vec<MetricEntry> = [].to_vec();
        Self {
            name: "Cerebellum".to_string(),
            entries: e,
            current_metric: cm,
            metrics: mtr,
        }
    }
    pub fn do_actions<'a>(&mut self) -> Result<Vec<CrbllumEntry>, BrainDeadError> {
        let ruleset = self.choose_actions();
        ruleset
    }
    
    pub fn choose_actions(&mut self) -> Result<Vec<CrbllumEntry>, BrainDeadError>{
        // add partially matching rules, then add to matching_rules only those matching all
        let mut partial_rules: Vec<CrbllumEntry> = [].to_vec();
        // Check time
        for rule in &self.entries {
            if rule.input[0].time != "*" {
                if self.current_metric.time >= rule.input[0].time.parse::<f64>().unwrap() {
                    partial_rules.push(rule.clone());
                }
            } else {
                partial_rules.push(rule.clone());
            }
        }
        // Check motors
        for rule in partial_rules.clone() {
            if rule.input[0].motor_l != "*" {
                if self.current_metric.motor_l != rule.input[0].motor_l.parse::<i16>().unwrap() {
                    partial_rules.retain(|x| *x != rule);
                }
            }
        }
        for rule in partial_rules.clone() {
            if rule.input[0].motor_r != "*" {
                if self.current_metric.motor_r != rule.input[0].motor_r.parse::<i16>().unwrap() {
                    partial_rules.retain(|x| *x != rule);
                }
            }
        }
        for rule in partial_rules.clone() {
            if rule.input[0].tracker != "*" {
                if self.current_metric.tracker != rule.input[0].tracker.parse::<bool>().unwrap() {
                    partial_rules.retain(|x| *x != rule);
                }
            }
        }
        for rule in partial_rules.clone() {
            if rule.input[0].distance != "*" {
                let rule_dissected = rule.input[0].distance.split("_").collect::<Vec<_>>();
                if rule_dissected[0] == "=" {
                    if self.current_metric.distance != rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                } else if rule_dissected[0] == ">=" {
                    if self.current_metric.distance < rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                } else if rule_dissected[0] == "<=" {
                    if self.current_metric.distance > rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                } else if rule_dissected[0] == ">" {
                    if self.current_metric.distance <= rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                } else if rule_dissected[0] == "<" {
                    if self.current_metric.distance >= rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                }
            }
        }
        Ok(partial_rules)
    }

    pub fn get_input<'a>(&mut self) {
        if self.metrics.len() == 0 {
            self.metrics.push(self.current_metric.clone());
            
        } else {
            if self.current_metric.motor_l == self.metrics[0].motor_l &&
               self.current_metric.motor_r == self.metrics[0].motor_r &&
               self.current_metric.tracker == self.metrics[0].tracker &&
               self.current_metric.distance == self.metrics[0].distance
            {
                self.metrics[0].time += 0.1;
            } else {
                self.metrics.push(self.current_metric.clone());
                self.metrics.rotate_right(1);
                self.metrics[0].time = 0.1;
            }
            if self.metrics.len() > 5{
                self.metrics.pop();
            }
        }
    }

    pub fn get_values_from_sensor_msg(&mut self, sensor_msg: String) -> (bool, u16) {
        let prev_metric = self.current_metric.clone();
        let split_msg = sensor_msg.split("_").collect::<Vec<_>>();
        let mut trck: bool = prev_metric.tracker;
        let mut dist: u16 = prev_metric.distance;
        if split_msg[1] == "tracker" {
            let trck_int: u8 = split_msg[2].parse().unwrap();
            trck = trck_int != 0;
        } else if split_msg[1] == "distance" {
            dist = split_msg[2].parse().unwrap();
            println!("MESSAGE IS ->{}<-", sensor_msg);
        }
        (trck, dist)
    }

    pub fn build_crbllum_input(&mut self, starttime: u128, sensors: String, movement: String) -> Result<MetricEntry, BrainDeadError> {
        log(Some(&self.name), "I", &format!("Moving : {}", movement));
        let m_l: i16;
        let m_r: i16;
        if movement == "forwards" {
            m_l = 100;
            m_r = 100;
        } else if movement == "forwards_slow" {
            m_l = 55;
            m_r = 55;
        } else if movement == "backwards" {
            m_l = -100;
            m_r = -100;
        } else if movement == "rotate_left" {
            m_l = -70;
            m_r = 70;
        } else if movement == "rotate_right" {
            m_l = 70;
            m_r = -70;
        } else {
            let motor_values: Vec<i16> = match movement.split("_")
                .map(|s| s.parse())
                .collect() {
                    Ok(v) => v,
                    Err(_e) => [0,0].to_vec(),
                };
            m_l = motor_values[0];
            m_r = motor_values[1];
        };
        let ct = SystemTime::now();
        let current_time = match ct.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => return Err(BrainDeadError::SystemTimeError),
        };
        let diff_time: f64 = (current_time as f64 - starttime as f64) as f64 / 100 as f64;
        let (trckr_msg, dist_msg) = self.get_values_from_sensor_msg(sensors);
        let m = MetricEntry {
            time: diff_time,
            motor_l: m_l,
            motor_r: m_r,
            tracker: trckr_msg,
            distance: dist_msg,
        };
        Ok(m)
    }
}
