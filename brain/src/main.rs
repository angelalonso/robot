use std::error::Error;
use brain::brain::Brain;
use std::process;
use std::env;

#[macro_use]
extern crate log;

fn show_help() {
    println!("\nSYNTAX: ");
    println!(" brain [mode] <actions_config_file> <auto_moves_config_file> [mode]");
    println!("");
    println!("   , where:");
    println!(" - mode        - optional, default is classic");
    println!("     is the trigger with which the Brain starts. ");
    println!("     accepted values: classic, test and reset. \n");
    println!(" - config_file - mandatory for mode start");
    println!("     is the path to the config yaml for triggers and actions \n");
    println!(" - auto_moves_config_file - mandatory for mode start");
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
                ();
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
    (b_cfg.to_string(), mode.to_string())
}

/// check the parameters and start the related mode
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let modes = vec!["live", "test"];
    let (move_config_file, start_mode) = argparser(modes);
    let _args: Vec<String> = env::args().collect();
    info!("...starting Brain with Mode {}", start_mode);
    match start_mode.as_str() {
        // Generate our Brain object
        "live" => {
            // Generate our Brain object
            let mut main_brain = Brain::new("Main Brain".to_string(), "live".to_string(), move_config_file).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Main Brain: {}", err);
                process::exit(1);
            });
            main_brain.do_io();
        },
        "test" => {
            // Generate our Brain object
            let mut main_brain = Brain::new("Main Brain".to_string(), "dryrun".to_string(), move_config_file).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Main Brain: {}", err);
                process::exit(1);
            });
            main_brain.do_io();
        }
        &_ => {
            error!("ERROR: Mode {} not recognized", start_mode);
            show_help();
            process::exit(1);
        }
    }
    Ok(())
}
