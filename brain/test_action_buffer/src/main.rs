use test_action_buffer::actionbuffer::{Action, ActionBuffer};
//use std::process;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    let st = SystemTime::now();
    let start_time = match st.duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_millis(),
        Err(_e) => 0,
    };
    let ab = Arc::new(Mutex::new(ActionBuffer::new().unwrap()));
    let a1 = Action{
        action_type: "move".to_string(),
        value: "0_0".to_string(),
        time: 0.020,
    };
    let a2 = Action{
        action_type: "move".to_string(),
        value: "60_60".to_string(),
        time: 0.020,
    };
    let a3 = Action{
        action_type: "move".to_string(),
        value: "-60_-60".to_string(),
        time: 0.020,
    };
    let a4 = Action{
        action_type: "move".to_string(),
        value: "80_80".to_string(),
        time: 0.020,
    };
    let a5 = Action{
        action_type: "move".to_string(),
        value: "40_40".to_string(),
        time: 0.020,
    };
    let a6 = Action{
        action_type: "move".to_string(),
        value: "20_20".to_string(),
        time: 0.020,
    };
    let a7 = Action{
        action_type: "move".to_string(),
        value: "44_44".to_string(),
        time: 10.000,
    };
    ab.lock().unwrap().add(a1);
    ab.lock().unwrap().add(a2);
    ab.lock().unwrap().add(a3);

    // ----------------------------------------------------------------------
    
    let (s, r): (Sender<Action>, Receiver<Action>) = std::sync::mpsc::channel();
    let msgs = s.clone();
    let a_b = ab.clone();
    let hr = thread::spawn(move || {
        loop {
            let msg = r.recv();
            let act = match msg.clone(){
                Ok(a) => {
                    println!("                                                              {:?}", a);
                    let mut ab_r = a_b.lock().unwrap();
                    ab_r.add(a);
                },
                Err(_) => (),
            };
        };
    });

    // TODO: test if this actually works well
    let a_c = ab.clone();
    let hw = thread::spawn(move || {
        loop {
            let mut ab_w = a_c.lock().unwrap();
            let _r = ab_w.do_all(start_time as f64);
        };
    });

    thread::sleep(Duration::from_millis(1000));
    msgs.send(a4).unwrap();
    thread::sleep(Duration::from_millis(1000));
    msgs.send(a5).unwrap();
    thread::sleep(Duration::from_millis(1000));
    msgs.send(a6).unwrap();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(3000));
            msgs.send(a7.clone()).unwrap();
        };
    });
    thread::sleep(Duration::from_millis(100));
    hw.join().unwrap();
}
