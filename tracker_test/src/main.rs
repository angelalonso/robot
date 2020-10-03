extern crate serde_yaml;

use std::time::SystemTime;
use tracker::{read_metrics_list, 
    get_metrics_for_timestamp,
    act_from_metrics};

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
            let m = match get_metrics_for_timestamp(&metrics, new_time){
                Some(x) => x,
                None => break Ok(()),
            };
            act_from_metrics(m);
            time = new_time;
        }
    }
}
