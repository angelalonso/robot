use std::error::Error;
use brain::brain::Brain;
use std::process;
use std::process::Command;
use std::env;

fn show_help() {
    println!("SYNTAX: ");
    println!(" brain [mode] <actions_config_file> <auto_moves_config_file> [mode]");
    println!("");
    println!("   , where:");
    println!(" - mode        - optional, default is start");
    println!("     is the trigger with which the Brain starts. ");
    println!("     accepted values: start, test and reset. ");
    println!(" - config_file - mandatory for mode start");
    println!("     is the path to the config yaml for triggers and actions ");
    println!(" - auto_moves_config_file - mandatory for mode start");
    println!("     is the path to the config yaml for rules to have the robot automatically move. ");
}

fn argparser(modes: Vec<&str>) -> (String, String, String) {
    let mut args: Vec<String> = env::args().collect();
    let mut b_cfg = String::from("");
    let mut c_cfg = String::from("");
    let mode: String;
    match args.len() {
        1 => {
            println!("ERROR: not enough parameters received");
            show_help();
            process::exit(1);
        },
        // two or more argument(s) passed? join them with spaces to allow phrases
        2 | 3 => {
            // remove the prog name itself
            args.remove(0);
            // drain variables
            mode = args.drain(0..1).collect();
            if mode == modes[0] {
                // fail because there arent enough parameters
                println!("ERROR: not enough parameters received for mode {}", mode);
                show_help();
                process::exit(1);
            } else if mode == modes[1] || mode == modes[2] {
                ();
            } else {
                // fail if mode is not recognized
                println!("ERROR: mode not recognised");
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
            c_cfg = args.drain(0..1).collect();
        },
    }
    (b_cfg.to_string(), c_cfg.to_string(), mode.to_string())
}

/// Check if there is another instance of this running
fn check_self_running(self_comm: &str) -> Result<(), String>{
    let own_ps = process::id();
    let ps_aux = Command::new("ps")
            .arg("aux")
            .output()
            .expect("process failed to execute");
    let result = String::from_utf8_lossy(&ps_aux.stdout);
    let split = result.split("\n");
    let mut blocked = false;
    for s in split {
        if s.contains(self_comm) && !s.contains(&own_ps.to_string()){
            blocked = true;
        };
    }
    if blocked {
        Err("There is another instance of this program running right now".to_string())
    } else {
        Ok(())
    }
}

/// Load a new brain, send the first trigger, and enter the reading loop
fn main() -> Result<(), Box<dyn Error>> {
    let modes = vec!["start", "test", "reset"];
    let (brain_config_file, cerebellum_config_file, start_mode) = argparser(modes);
    let args: Vec<String> = env::args().collect();
    check_self_running(&args[0]).unwrap();
    println!("Starting Brain with Mode {}", start_mode);
    match start_mode.as_str() {
        "start" => {
            // Generate our Brain object
            let mut main_brain = Brain::new("Main Brain", brain_config_file, cerebellum_config_file, None).unwrap_or_else(|err| {
                eprintln!("Problem Initializing Main Brain: {}", err);
                process::exit(1);
            });
            // Send the first trigger to start.
            let _send_start = main_brain.get_brain_actions(&start_mode).unwrap_or_else(|err| {
                eprintln!("Problem sending the first trigger to the Arduino: '{}' - {}", &start_mode, err);
                process::exit(1);
            });
            // Listening on Comm
            main_brain.get_input();
        },
        "reset" => {
            ()
        }
        "test" => {
            ()
        }
        &_ => {
            ()
        }
    }
    Ok(())
}
