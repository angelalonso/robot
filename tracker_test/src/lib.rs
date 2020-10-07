pub mod test;

use std::fs::File;
use serde::{Deserialize, Serialize}; 

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RuleInput {
    time: String,
    motor_l: String,
    motor_r: String,
    sensor: String,
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MoveAction {
    motor_l: i16,
    motor_r: i16,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MetricEntry {
    time: f64,
    motor_l: i16,
    motor_r: i16,
    sensor: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RuleEntry {
    input: Vec<RuleInput>,
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

pub fn read_metrics_list(filename: String) -> Result<Vec<MetricEntry>, Box<std::error::Error>> {
    let filepointer = File::open(filename).unwrap();
    let metrics: Vec<MetricEntry> = serde_yaml::from_reader(filepointer).unwrap();
    Ok(metrics)
}

pub fn read_rules() -> Result<Vec<RuleEntry>, Box<std::error::Error>> {
    let filepointer = File::open("test_rules.yaml").unwrap();
    let rules: Vec<RuleEntry> = serde_yaml::from_reader(filepointer).unwrap();
    Ok(rules)
}

pub fn update_metrics<'a>(metric: &'a MetricEntry, mut latest_metrics: &'a mut Vec<MetricEntry>) -> &'a mut Vec<MetricEntry> {
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

pub fn act_from_metrics<'a>(metric: &'a MetricEntry, mut latest_metrics: &'a mut Vec<MetricEntry>) -> Vec<RuleEntry>{
    latest_metrics = update_metrics(metric, latest_metrics);
    let ruleset = match read_rules(){
        Ok(r) => choose_rule(r, metric),
        Err(e) => Err(e),
    };
    ruleset.unwrap()
}

pub fn choose_rule(rules: Vec<RuleEntry>, metric: &MetricEntry) -> Result<Vec<RuleEntry>, Box<std::error::Error>>{
    let mut matching_rules: Vec<RuleEntry> = [].to_vec();
    //println!("{:?}", metric);
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
