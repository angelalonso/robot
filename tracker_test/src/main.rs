extern crate serde_yaml;
use std::fs::File;

use std::time::SystemTime;
use tracker::{read_metrics_list, get_metrics_for_timestamp};

fn main() -> Result<(), Box<std::error::Error>>{
    let metrics = read_metrics_list()?;
    let start = SystemTime::now();
    let mut time = 0;
    loop {
        let new_time = match SystemTime::now().duration_since(start) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        if new_time > time {
            let m = get_metrics_for_timestamp(&metrics, new_time);
            println!("{:?}", m);
            time = new_time;
        }
    }
}
