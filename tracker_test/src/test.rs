#[cfg(test)]
mod test {
    use std::process;
    use std::time::SystemTime;
    use crate::{MetricEntry, read_metrics_list, get_metrics_for_timestamp};

    #[test]
    fn check_read_metrics() {
        let metrics = read_metrics_list("test_metrics.yaml".to_string()).unwrap_or_else(|err| {
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
                let m = match get_metrics_for_timestamp(&metrics, new_time){
                    Some(x) => x,
                    None => break (),
                };
                println!("{:?}", m);
                time = new_time;
            }
        }
    }
// TODO:
//    #[test]
//    fn test_wrongfile() {
//        let metrics = match read_metrics_list("test_metrics_wrongfile.yaml".to_string()) {
//            Ok(n) => (),
//            Err(_) => (),
//        };
//        assert!(metrics.is_err(), "getting an error for a non existing file did not go well");
//    }

    #[test]
    fn test_straightline() -> Result<(), Box<std::error::Error>>{
        use std::time::{SystemTime, UNIX_EPOCH};
        use crate::{MetricEntry, 
            MoveAction,
            read_metrics_list, 
            get_metrics_for_timestamp,
            act_from_metrics};

        let metrics = read_metrics_list("test_metrics_straight.yaml".to_string()).unwrap_or_else(|err| {
            eprintln!("Problem Reading Metrics List: {}", err);
            process::exit(1);
        });
        let mut latest_metrics: Vec<MetricEntry> = [].to_vec();
        let mut found_actions: Vec<MoveAction> = [].to_vec();
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
                    None => break (),
                };
                let actions = act_from_metrics(m, & mut latest_metrics);
                for act in actions {
                    found_actions.push(act.action);
                }
                time = diff_time;
            }
        }
        //TODO: repeated actions probably shouldn't be here, should they?
        assert_eq!(found_actions, 
                   [MoveAction { motor_l: 70, motor_r: 70 }, 
                   MoveAction { motor_l: -70, motor_r: 70 }, 
                   MoveAction { motor_l: 70, motor_r: 70 }, 
                   MoveAction { motor_l: 70, motor_r: 70 }]
                   , "getting an error for a non existing file did not go well");
        Ok(())
    }
    #[test]
    fn test_straightline_angle45() -> Result<(), Box<std::error::Error>>{
        use std::time::{SystemTime, UNIX_EPOCH};
        use crate::{MetricEntry, 
            MoveAction,
            read_metrics_list, 
            get_metrics_for_timestamp,
            act_from_metrics};

        let metrics = read_metrics_list("test_metrics_straight45.yaml".to_string()).unwrap_or_else(|err| {
            eprintln!("Problem Reading Metrics List: {}", err);
            process::exit(1);
        });
        let mut latest_metrics: Vec<MetricEntry> = [].to_vec();
        let mut found_actions: Vec<MoveAction> = [].to_vec();
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
                    None => break (),
                };
                let actions = act_from_metrics(m, & mut latest_metrics);
                for act in actions {
                    found_actions.push(act.action);
                }
                time = diff_time;
            }
        }
        //TODO: repeated actions probably shouldn't be here, should they?
        assert_eq!(found_actions, 
                   [MoveAction { motor_l: 70, motor_r: 70 }, 
                   MoveAction { motor_l: -70, motor_r: 70 }, 
                   MoveAction { motor_l: 70, motor_r: -70 }, 
                   MoveAction { motor_l: 70, motor_r: -70 }, 
                   MoveAction { motor_l: 70, motor_r: -70 }, 
                   MoveAction { motor_l: 70, motor_r: 70 }, 
                   MoveAction { motor_l: 70, motor_r: 70 }, 
                   MoveAction { motor_l: 70, motor_r: 70 }]
                   , "getting an error for a non existing file did not go well");
        Ok(())
    }
    #[test]
    fn test_straightline_angle90() {
    }
    #[test]
    fn test_straightline_angle315() {
    }
}
