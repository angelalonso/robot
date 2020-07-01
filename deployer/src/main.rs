use std::env;
use std::process::{Command, Stdio};
//use std::io::{self, Write};
use std::io::{self, Error, ErrorKind};
use compare::{Compare, natural};
use std::cmp::Ordering::{Less, Equal, Greater};
use dotenv;

// TODO
//
// Define setup on .env instead of as args

fn scp(file_to_scp: &str, destination: &str) -> io::Result<()> {
    let mut child = Command::new("scp")
        .arg(file_to_scp)
        .arg(destination)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let _child_stdin = child.stdin.as_mut().unwrap();
    }

    let output = child.wait_with_output()?;

    let cmp = natural();
    if cmp.compares_gt(&output.status.code(), &Some(0)){
    //if output.status.code() > Some(0){
        let message = output.status.code().unwrap_or(1).to_string();
        Err(Error::new(ErrorKind::Other, message))
    } else {
        Ok(())
    }

    // TODO: catch output = Output { status: ExitStatus(ExitStatus(256)), stdout: "", stderr: "" }
}

fn load_dot_env() {
    dotenv::dotenv().ok();
}

fn get_env_var(key: &str) -> std::string::String {
    // Accessing an env var
    match env::var(key) {
        Ok(val) => return val,
        Err(_e) => return key.to_string(),
    }
}

fn print_env_var(key: &str) {
    // Accessing an env var
    match env::var(key) {
        Ok(val) => println!("{}: {}", key, val),
        Err(e) => println!("Couldn't print env var {}: {}", key, e),
    }
}

fn main() {
    // We don't (yet) need to catch arguments
    //let mut args: Vec<String> = env::args().collect();
    //args.drain(0..1);

    load_dot_env();
    let raw_files = get_env_var("ARDUINO_FILES");
    let files = raw_files.split(" ");

    let login_and_destination: &str = &get_env_var("CONNECTION").to_string();

    for file in files {
        println!("Copying {}", file);
        if let retest = scp(&file, login_and_destination) {
            std::process::exit(2);
        }
    }

}
