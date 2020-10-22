use test_action_buffer::actionbuffer::{Action, ActionBuffer};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let st = SystemTime::now();
    let mut start_time = match st.duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_millis(),
        Err(_e) => 0,
    };
    let mut ab = ActionBuffer::new().unwrap_or_else(|err| {
                eprintln!("Problem Initializing Action Buffer: {}", err);
                process::exit(1);
            });
    let a1 = Action{
        action_type: "move".to_string(),
        value: "0_0".to_string(),
        time: 4.0,
    };
    let a2 = Action{
        action_type: "move".to_string(),
        value: "60_60".to_string(),
        time: 4.0,
    };
    let a3 = Action{
        action_type: "move".to_string(),
        value: "-60_-60".to_string(),
        time: 4.0,
    };
    ab.add(a1);
    ab.add(a2);
    ab.add(a3);
    //ab.print_all();
    //ab.do_all();
    // TODO: correct the timer, make sure it works like one
    'outer: loop {
        let ct = SystemTime::now();
        let current_time = match ct.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
        };
        let timestamp: f64 = (current_time as f64 - start_time as f64) as f64 / 100 as f64;
        match ab.do_next(timestamp) {
            Ok(a) => match a {
                Some(action) => {
                    println!("{:?}", action);
                    start_time = current_time;
                },
                None => (),
            },
            Err(e) => {
                println!("{}", e);
                break 'outer;
            },
        };
    }
}
