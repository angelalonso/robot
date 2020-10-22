use test_action_buffer::actionbuffer::{Action, ActionBuffer};
use std::process;

fn main() {
    println!("Hello, world!");
    let mut ab = ActionBuffer::new().unwrap_or_else(|err| {
                eprintln!("Problem Initializing Action Buffer: {}", err);
                process::exit(1);
            });
    let act = Action{
        action_type: "move".to_string(),
        value: "60_60".to_string(),
        time: 2.0,
    };
    ab.add(act);
    ab.print_all();
}
