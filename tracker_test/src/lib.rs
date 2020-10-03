pub mod test;

use std::fs::File;
use serde::{Deserialize, Serialize}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveRule {
    motor_l: String,
    motor_r: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveAction {
    motor_l: i16,
    motor_r: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricEntry {
    time: u64,
    movement: MoveAction,
    sensor: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleEntry {
    time: String,
    movement: MoveRule,
    sensor: String,
    action: MoveAction,
}

pub fn get_metrics_for_timestamp(metrics: &Vec<MetricEntry>, timestamp: u64) -> Option<&MetricEntry>{
    // show metric for the given time
    let the_metric = metrics
        .into_iter()
        .find(|entry| entry.time == timestamp);
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
    let ruleset = read_rules();
    println!("{:?}", metric);
    //TODO: compare metrics and rules, print action

}

