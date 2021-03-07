use std::error::Error;
use brain::brain::Brain;
use std::process;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate log;

/// check the parameters and start the related mode
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
    .format_timestamp_millis()
    .init();

    let matches = App::new(option_env!("CARGO_PKG_NAME").unwrap())
                          .version(option_env!("CARGO_PKG_VERSION").unwrap())
                          .author(option_env!("CARGO_PKG_AUTHORS").unwrap())
                          .about("Does awesome things")
                          .arg(Arg::with_name("MODE")
                               .help("Sets the running mode to use")
                               .required(true)
                               .takes_value(true)
							   .possible_value("check")
							   .possible_value("live")
							   .possible_value("reset")
							   .possible_value("record")
							   .possible_value("test")
                               .index(1))
                          .arg(Arg::with_name("SETUP_FILE")
                               .help("Sets the setup file to use")
                               .required(false)
                               .index(2))
                          .get_matches();
    let start_mode = matches.value_of("MODE").unwrap_or("live");
    let setup_file = matches.value_of("SETUP_FILE").unwrap_or("setup.yaml").to_string();
    let mut run_time = None;
    let mut actual_mode = "live".to_string();
    let precision_th_of_a_sec = 100;
    // Setup configs for each mode
    match start_mode {
        "check" => {
            run_time = Some(20.0);
        },
        "reset" => {
            run_time = Some(0.2);
        },
        "record" => {
            actual_mode = "live_record".to_string();
        },
        "test" => {
            run_time = Some(20.0);
            actual_mode = "dryrun".to_string();
        },
        "live" | &_ => {
            ()
        }
    }
    // Common tasks for all modes
    match run_time {
        Some(t) => println!("...starting Brain in Mode {} for {} secs", start_mode, t),
        None => println!("...starting Brain in Mode {}", start_mode),
    }
    let mut main_brain = Brain::new("Main Brain".to_string(), actual_mode, setup_file).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Main Brain: {}", err);
        process::exit(1);
    });
    let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        let _actions = main_brain.run(run_time, precision_th_of_a_sec, s);
    });
    handle.join().unwrap();
    // Extra steps for some modes (so far only reset)
    if start_mode == "reset" {
        let mut got = [].to_vec();
        while let Ok(m) = r.try_recv() {
            got.push(m);
        }
        //
        //loop {
        //    match r.try_recv() {
        //        Ok(m) => got.push(m),
        //        Err(_) => break,
        //    };
        //}
    }

    Ok(())
}
