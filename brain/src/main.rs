use std::error::Error;
use brain::brain::Brain;
use std::process;
use std::env;


fn argparser() -> String {
    let mut args: Vec<String> = env::args().collect();
    match args.len() {
        // No arguments passed? "start" is the default
        1 => "start".to_string(),
        // one or more argument(s) passed? join them with spaces to allow phrases
        _ => {
            args.remove(0);
            args.join(" ")
        },
    }
}

/// Load a new brain, send the first trigger, and enter the reading loop
fn main() -> Result<(), Box<dyn Error>> {
    let start_mode = argparser();
    // Generate our Brain object
    let mut main_brain = Brain::new("Main Brain", "cfg.yaml", None).unwrap_or_else(|err| {
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
