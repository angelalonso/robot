pub mod test;

use std::fs::File;
use serde::{Deserialize, Serialize}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricEntry {
    time: u64,
    motor1: i16,
    motor2: i16,
    sensor: bool,
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

