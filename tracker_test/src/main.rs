extern crate serde_yaml;

use std::time::{SystemTime, UNIX_EPOCH};
use tracker::{MetricEntry, 
    read_metrics_list, 
    get_metrics_for_timestamp,
    act_from_metrics};

fn main() -> Result<(), Box<std::error::Error>>{
    let metrics = read_metrics_list("test_metrics.yaml".to_string())?;
    let mut latest_metrics: Vec<MetricEntry> = [].to_vec();
    let st = SystemTime::now();
    let start_time = st
        .duration_since(UNIX_EPOCH)?.as_millis();
    // Time is down to decs of a second
    let mut time: u64 = 0;
    loop {
        let ct = SystemTime::now();
        let current_time = ct
            .duration_since(UNIX_EPOCH)?.as_millis();
        let diff_time: u64 = (current_time as f64 - start_time as f64) as u64 / 100 as u64;
        if diff_time > time {
            let m = match get_metrics_for_timestamp(&metrics, diff_time){
                Some(x) => x,
                None => break Ok(()),
            };
            let action = act_from_metrics(m, & mut latest_metrics);
            for i in &latest_metrics {
                println!("    {:?}", i);
            }
            println!("   ACTION -> {:?}\n", action);
            time = diff_time;
        }
    }
}
