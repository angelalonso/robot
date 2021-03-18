use std::path::Path;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::env;
use std::io;
use std::io::BufReader;

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
    let envvars = ["ROBOT_IP", "SSH_CONFIG", "SSH_COMMAND", "CODE_BRANCH", "ARDUINO_FILES_PATH", "CARGO"];
    let mut dotenv_content: String = "".to_string();

    // check if there is a .env file, no? -> create it
    if Path::new(file).exists() {
        println!(".env exists");
    } else {
      println!(".env DOES NOT exist");
      match File::create(file) {
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
        println!("checking {}...", envvar);
        let mut fenv = File::open(file)?;
        let mut reader = BufReader::new(fenv);
        let mut envvar_found = false;
        for line_opt in reader.lines() {
            match line_opt {
                Ok(l) => {
                    if l.contains(envvar) {
                        println!("fooooooound!");
                        envvar_found = true;
                    };
                },
                Err(_) => ()
            }
        };
        if !envvar_found {dotenv_content.push_str(&format!("{}: null\n", envvar));}
    };
    println!("{}", dotenv_content);
    let mut f = File::open(file)?;
    f.write_all(dotenv_content.as_bytes()).expect("Unable to write data");
    Ok("".to_string())
}

fn get_stuff(what: String) {
    println!("GET mode with {} parameters", what);
}

fn do_stuff(what: String) {
    println!("DO mode with {} parameters", what);
}
