use std::error::Error;
use brain::brain::Brain;
use std::process;
use std::env;

fn show_help() {
    println!("SYNTAX: ");
    println!(" brain <config_file> [mode]");
    println!("");
    println!("   , where:");
    println!(" - config_file - mandatory");
    println!("     is the path to the config yaml for triggers. ");
    println!(" - mode        - optional, default is start");
    println!("     is the trigger with which the Brain starts. ");
}

fn argparser() -> (String, String) {
    let mut args: Vec<String> = env::args().collect();
    match args.len() {
        // No arguments passed? show error
        1 => {
            println!("ERROR: not enough parameters received");
            show_help();
            process::exit(1);
        },
        // one argument passed? then it's the config file, and mode is default one, "start"
        2 => {
            let ref configfile = &args[1];
            (configfile.to_string(), "start".to_string())
        },
        // two or more argument(s) passed? join them with spaces to allow phrases
        _ => {
            // remove the prog name itself
            args.remove(0);
            // drain the config file path
            let configfile : String = args.drain(0..1).collect();
            (configfile.to_string(), args.join(" "))
        },
    }
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
    let (config_file, start_mode) = argparser();
    let args: Vec<String> = env::args().collect();
    check_self_running(&args[0]).unwrap();
    println!("Starting Brain with Mode {}", start_mode);
    // Generate our Brain object
    let mut main_brain = Brain::new("Main Brain", config_file, None).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Main Brain: {}", err);
        process::exit(1);
    });
    // Send the first trigger to start.
    let _send_start = main_brain.get_actions(&start_mode).unwrap_or_else(|err| {
        eprintln!("Problem sending the first trigger to the Arduino: '{}' - {}", &start_mode, err);
        process::exit(1);
    });
    // Listening on Comm
    main_brain.read();
    Ok(())
}
