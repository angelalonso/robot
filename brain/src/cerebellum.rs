use std::fs::File;
use crate::brain::{BrainDeadError, MetricEntry};

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
}
