use brain::r#move::Move;
use std::process;
use std::error::Error;
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>> {
    let wait = time::Duration::from_millis(1000);
    let mut movin = Move::new().unwrap_or_else(|_err| {
        eprintln!("Problem starting da movin");
        process::exit(1);
    });
    loop {
        println!("FWD");
        movin.edit_move("forwards".to_string());
        movin.edit_move("forwards".to_string());
        movin.edit_move("forwards".to_string());
        movin.edit_move("forwards".to_string());
        movin.edit_move("forwards".to_string());
        thread::sleep(wait);
        println!("BWD");
        movin.edit_move("backwards".to_string());
        thread::sleep(wait);
        println!("STP");
        movin.edit_move("stop".to_string());
        thread::sleep(wait);

    }

}
