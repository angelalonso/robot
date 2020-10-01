#[cfg(test)]
mod test {
    use std::process;
    use std::time::SystemTime;
    use crate::{read_metrics_list, get_metrics_for_timestamp};

    #[test]
    fn check_read_metrics() {
        let metrics = read_metrics_list().unwrap_or_else(|err| {
            eprintln!("Problem Reading Metrics List: {}", err);
            process::exit(1);
        });
        let start = SystemTime::now();
        let mut time = 0;
        loop {
            let new_time = match SystemTime::now().duration_since(start) {
                Ok(n) => n.as_secs(),
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            };
            if new_time > time {
                let m = crate::get_metrics_for_timestamp(&metrics, new_time);
                println!("{:?}", m);
                time = new_time;
            }
        }
    }
}
