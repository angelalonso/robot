use std::error::Error;
use brain::brain::Brain;
use std::process;
use std::env;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

#[macro_use]
extern crate log;

fn show_help() {
    println!("\nSYNTAX: ");
    println!(" brain [mode] <auto_moves_config_file>");
    println!();
    println!("   , where:");
    println!(" - mode        - optional, default is classic");
    println!("     is the trigger with which the Brain starts. ");
    println!("     accepted values: live, reset, test and record. \n");
    println!(" - auto_moves_config_file - mandatory");
    println!("     is the path to the config yaml for rules to have the robot automatically move. ");
}

/// parse arguments given to the program itself
fn argparser(modes: Vec<&str>) -> (String, String) {
    let mut args: Vec<String> = env::args().collect();
    let mut b_cfg = String::from("");
    let mode: String;
    match args.len() {
        1 => {
            error!("ERROR: not enough parameters received");
            show_help();
            process::exit(1);
        },
        // two or more argument(s) passed? join them with spaces to allow phrases
        2 => {
            // remove the prog name itself
            args.remove(0);
            // drain variables
            mode = args.drain(0..1).collect();
            // we have this in case we ever have a mode that does not need the config files
            if mode == modes[0] || mode == modes[1]{
                // fail because there arent enough parameters
                error!("ERROR: not enough parameters received for mode {}", mode);
                show_help();
                process::exit(1);
            } else if mode == modes[2] {
               // nothing 
            } else {
                // fail if mode is not recognized
                error!("ERROR: mode not recognised");
                show_help();
                process::exit(1);
            }
       },
        _ => {
            // remove the prog name itself
            args.remove(0);
            // drain variables
            mode = args.drain(0..1).collect();
            b_cfg = args.drain(0..1).collect();
        },
    }
    (b_cfg, mode)
}

/// check the parameters and start the related mode
fn main() -> Result<(), Box<dyn Error>> {
    //env_logger::init();
    env_logger::builder()
    .format_timestamp_millis()
    .init();
    let modes = vec!["live", "reset", "record", "test"];
    let (setup_file, start_mode) = argparser(modes);
    let _args: Vec<String> = env::args().collect();
    let mut run_time = None;
    let precision_th_of_a_sec = 100;
    match start_mode.as_str() {
        // Generate our Brain object
        "live" => {
            match run_time {
                Some(t) => println!("...starting Brain in Mode {} for {} secs", start_mode, t),
                None => println!("...starting Brain in Mode {}", start_mode),
            }
            // Generate our Brain object
            let mut main_brain = Brain::new("Main Brain".to_string(), "live".to_string(), setup_file).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Main Brain: {}", err);
                process::exit(1);
            });
            let (s, _r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let handle = thread::spawn(move || {
                let _actions = main_brain.run(run_time, precision_th_of_a_sec, s);
            });
            handle.join().unwrap();
        },
        "reset" => {
            run_time = Some(0.2);
            match run_time {
                Some(t) => println!("...starting Brain in Mode {} for {} secs", start_mode, t),
                None => println!("...starting Brain in Mode {}", start_mode),
            }
            // Generate our Brain object
            let mut main_brain = Brain::new("Main Brain".to_string(), "live".to_string(), setup_file).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Main Brain: {}", err);
                process::exit(1);
            });
            let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let handle = thread::spawn(move || {
                let _actions = main_brain.run(run_time, precision_th_of_a_sec, s);
            });
            handle.join().unwrap();
            let mut got = [].to_vec();
            while let Ok(m) = r.try_recv() {
                got.push(m);

            }
            //loop {
            //    match r.try_recv() {
            //        Ok(m) => got.push(m),
            //        Err(_) => break,
            //    };
            //}
        },
        "record" => {
            match run_time {
                Some(t) => println!("...starting Brain in Mode {} for {} secs", start_mode, t),
                None => println!("...starting Brain in Mode {}", start_mode),
            }
            // Generate our Brain object
            let mut main_brain = Brain::new("Main Brain".to_string(), "live_record".to_string(), setup_file).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Main Brain: {}", err);
                process::exit(1);
            });
            let (s, _r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let handle = thread::spawn(move || {
                let _actions = main_brain.run(run_time, precision_th_of_a_sec, s);
            });
            handle.join().unwrap();
        },
        "test" => {
            //run_time = Some(20.0);
            match run_time {
                Some(t) => println!("...starting Brain in Mode {} for {} secs", start_mode, t),
                None => println!("...starting Brain in Mode {}", start_mode),
            }
            // Generate our Brain object
            let mut main_brain = Brain::new("Main Brain".to_string(), "dryrun".to_string(), setup_file).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Main Brain: {}", err);
                process::exit(1);
            });
            let (s, _r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let handle = thread::spawn(move || {
                let _actions = main_brain.run(run_time, precision_th_of_a_sec, s);
            });
            handle.join().unwrap();
        }
        &_ => {
            error!("ERROR: Mode {} not recognized", start_mode);
            show_help();
            process::exit(1);
        }
    }
    Ok(())
}
