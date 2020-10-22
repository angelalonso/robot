use test_action_buffer::actionbuffer::{Action, ActionBuffer};
use std::process;

fn main() {
    println!("Hello, world!");
    let mut ab = ActionBuffer::new().unwrap_or_else(|err| {
                eprintln!("Problem Initializing Action Buffer: {}", err);
                process::exit(1);
            });
    let a1 = Action{
        action_type: "move".to_string(),
        value: "0_0".to_string(),
        time: 2.0,
    };
    let a2 = Action{
        action_type: "move".to_string(),
        value: "60_60".to_string(),
        time: 2.0,
    };
    let a3 = Action{
        action_type: "move".to_string(),
        value: "-60_-60".to_string(),
        time: 2.0,
    };
    ab.add(a1);
    ab.add(a2);
    ab.add(a3);
    ab.print_all();
    ab.do_all();
    // TODO: set up a timer to trigger this:
    //ab.do_next();
}
