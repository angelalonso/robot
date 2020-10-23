use test_action_buffer::actionbuffer::{Action, ActionBuffer};
use std::process;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let st = SystemTime::now();
    let start_time = match st.duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_millis(),
        Err(_e) => 0,
    };
    let ab = Arc::new(Mutex::new(ActionBuffer::new().unwrap_or_else(|err| {
                eprintln!("Problem Initializing Action Buffer: {}", err);
                process::exit(1);
            })));
    let a1 = Action{
        action_type: "move".to_string(),
        value: "0_0".to_string(),
        time: 1.001,
    };
    let a2 = Action{
        action_type: "move".to_string(),
        value: "60_60".to_string(),
        time: 4.361,
    };
    let a3 = Action{
        action_type: "move".to_string(),
        value: "-60_-60".to_string(),
        time: 3.02,
    };
    ab.lock().unwrap().add(a1);
    ab.lock().unwrap().add(a2);
    ab.lock().unwrap().add(a3);

    let a_b = Arc::clone(&ab);
    let handle = thread::spawn(move || {
        a_b.lock().unwrap().do_all(start_time as f64);
    });

    thread::sleep(Duration::from_millis(1000));
    let a4 = Action{
        action_type: "move".to_string(),
        value: "80_80".to_string(),
        time: 2.01,
    };
    ab.lock().unwrap().add(a4);
    thread::sleep(Duration::from_millis(1000));
    let a5 = Action{
        action_type: "move".to_string(),
        value: "40_40".to_string(),
        time: 4.01,
    };
    ab.lock().unwrap().add(a5);
    thread::sleep(Duration::from_millis(1000));
    let a6 = Action{
        action_type: "move".to_string(),
        value: "20_20".to_string(),
        time: 5.01,
    };
    ab.lock().unwrap().add(a6);

    handle.join().unwrap();
}
