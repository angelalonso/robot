use std::path::Path;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::env;
use std::io;
use std::io::{stdin,stdout,Write, BufReader};

pub fn proxy_action(mode: String) {
    let mode_split: Vec<&str> = mode.split("_").collect();
    match mode_split[0] {
        "init" => {
            if mode.replace("init", "") != "" {
                println!("ERROR");
            } else {init()};
        },
        "get" => {
            let mode_clean = mode.replace("get_", "");
            if mode_clean == "" {
                println!("ERROR");
            } else {get_stuff(mode_clean)};
        },
        "do" => {
            let mode_clean = mode.replace("do_", "");
            if mode_clean == "" {
                println!("ERROR");
            } else {do_stuff(mode_clean)};
        },
        &_ => println!("ERROR"),
    };
}

fn init() {
    println!("INIT mode");
    build_dot_env();
}

fn build_dot_env() -> Result<String, std::io::Error> {
    let file = ".env";
    let file_template = ".env.template";
    let envvars = ["ROBOT_IP", "SSH_COMMAND", "SSH_CONFIG", "CODE_BRANCH", "ARDUINO_FILES_PATH", "CARGO"];
    let mut dotenv_content: String = "".to_string();

    // check if there is a .env file, no? -> create it
    if Path::new(file).exists() {
        println!(".env exists (Press <Enter> to keep current values)");
    } else {
      println!(".env DOES NOT exist, using values from the template");
      //match File::create(file) {
      match fs::copy(format!("{}.template", file), file) {
          Ok(_) => (),
          Err(_) => panic!(),
      };
    }

    // TODO
    // go over envvars, 
    //   get current value from .env, 
    //   ask user showing current value as default
    //   once all vars have a value, write them to .env
    for envvar in envvars.iter() {
        let mut fenv = File::open(file)?;
        let mut reader = BufReader::new(fenv);
        let mut envvar_found = "".to_string();
        for line_opt in reader.lines() {
            match line_opt {
                Ok(l) => { 
                    let l_keyval: Vec<&str> = l.split(":").collect();
                    if l_keyval[0].contains(envvar) {envvar_found = l }
                },
                Err(_) => (),
            }
        };
        dotenv_content.push_str(&get_new_envvar(envvar.to_string(), envvar_found.clone()));
    };
    //println!("{}", dotenv_content);
    let mut f = File::create(file)?;
    f.write_all(dotenv_content.as_bytes()).expect("Unable to write data");
    Ok("".to_string())
}

fn get_new_envvar(entry: String, previous_entry: String) -> String {
    let mut s = String::new();
    // get previous value
    let prev_keyval: Vec<&str> = previous_entry.split(":").collect();
    print!("Please enter a value for {}: [{}] ", entry, prev_keyval[1]);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    if s == "" {s.push_str(prev_keyval[1])};
    if s.chars().nth(0).unwrap() == ' ' {s.remove(0);};
    println!("Added: '{}: {}'", entry, s);

    return format!("{}: {}\n", entry, s)
}

fn get_stuff(what: String) {
    println!("GET mode with {} parameters", what);
}

fn do_stuff(what: String) {
    println!("DO mode with {} parameters", what);
}
