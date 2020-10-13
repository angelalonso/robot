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
}

impl Cerebellum {
    pub fn new(crbllumconfigfile: String) -> Self {
        let filepointer = File::open(crbllumconfigfile).unwrap();

        let e: Vec<CrbllumEntry> = serde_yaml::from_reader(filepointer).unwrap();
        Self {
            entries: e,
        }
    }
    pub fn do_actions<'a>(&mut self, metric: &'a MetricEntry) -> Result<Vec<CrbllumEntry>, BrainDeadError> {
        let ruleset = self.choose_actions(self.entries.clone(), metric);
        ruleset
    }
    
    pub fn choose_actions(&mut self, rules: Vec<CrbllumEntry>, metric: &MetricEntry) -> Result<Vec<CrbllumEntry>, BrainDeadError>{
        // add partially matching rules, then add to matching_rules only those matching all
        let mut partial_rules: Vec<CrbllumEntry> = [].to_vec();
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
            if rule.input[0].tracker != "*" {
                if metric.tracker != rule.input[0].tracker.parse::<bool>().unwrap() {
                    partial_rules.retain(|x| *x != rule);
                }
            }
        }
        for rule in partial_rules.clone() {
            if rule.input[0].distance != "*" {
                let rule_dissected = rule.input[0].distance.split("_").collect::<Vec<_>>();
                if rule_dissected[0] == "=" {
                    if metric.distance != rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                } else if rule_dissected[0] == ">=" {
                    if metric.distance < rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                } else if rule_dissected[0] == "<=" {
                    if metric.distance > rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                } else if rule_dissected[0] == ">" {
                    if metric.distance <= rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                } else if rule_dissected[0] == "<" {
                    if metric.distance >= rule_dissected[1].parse::<u16>().unwrap() {
                        partial_rules.retain(|x| *x != rule);
                    }
                }
            }
        }
        Ok(partial_rules)
    }

}
