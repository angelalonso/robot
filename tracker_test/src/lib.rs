pub mod test;

use std::fs::File;
use serde::{Deserialize, Serialize}; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveRule {
    motor_l: String,
    motor_r: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveAction {
    motor_l: i16,
    motor_r: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricEntry {
    time: f64,
    movement: MoveAction,
    sensor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleEntry {
    time: String,
    movement: MoveRule,
    sensor: String,
    action: MoveAction,
}

pub fn get_metrics_for_timestamp(metrics: &Vec<MetricEntry>, timestamp: u64) -> Option<&MetricEntry>{
    // show metric for the given time
    // we store the metrics as secs with one decimal, get timestamp as integers (so multiply *10 here)
    let the_metric = metrics
        .into_iter()
        .find(|entry| (entry.time * 10.0 as f64 ) as u64 == timestamp);
    the_metric
}

pub fn read_metrics_list() -> Result<Vec<MetricEntry>, Box<std::error::Error>> {
    let filepointer = File::open("test_metrics.yaml").unwrap();
    let metrics: Vec<MetricEntry> = serde_yaml::from_reader(filepointer).unwrap();
    Ok(metrics)
}

pub fn read_rules() -> Result<Vec<RuleEntry>, Box<std::error::Error>> {
    let filepointer = File::open("test_rules.yaml").unwrap();
    let rules: Vec<RuleEntry> = serde_yaml::from_reader(filepointer).unwrap();
    Ok(rules)
}

pub fn act_from_metrics(metric: &MetricEntry) {
    let ruleset = match read_rules(){
        Ok(r) => choose_rule(r, metric),
        Err(_e) => return (),
    };
    for rule in ruleset.unwrap() {
        println!("-> {:?}", rule.action);
    }
}

pub fn choose_rule(rules: Vec<RuleEntry>, metric: &MetricEntry) -> Result<Vec<RuleEntry>, Box<std::error::Error>>{
    let mut matching_rules: Vec<RuleEntry> = [].to_vec();
    println!("{:?}", metric);
    for rule in rules {
        if rule.time != "*" {
            ();
        }
        if rule.movement.motor_l != "*" {
            ();
        }
        if rule.movement.motor_r != "*" {
            ();
        }
        if rule.sensor != "*" {
            let rule_sensor: bool = rule.sensor == "true";
            if rule_sensor == metric.sensor {
                matching_rules.push(rule);
            }
        }
    }
    Ok(matching_rules)
}
