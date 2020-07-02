use dotenv;
use std::env;
//use std::io::{Error};
use std::process::{Command, Stdio};
#[macro_use]
extern crate simple_error;

use std::error::Error;

type BoxResult<T> = Result<T,Box<Error>>;

fn scp(file_to_scp: &str, destination: &str) -> BoxResult<i32> {
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

    if &output.status.code() > &Some(0) {
        bail!("ERROR copying !")
    } else {
        Ok(0)
    }
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

fn main() {
    // We don't (yet) need to catch arguments
    //let mut args: Vec<String> = env::args().collect();
    //args.drain(0..1);

    load_dot_env();
    let raw_files = get_env_var("ARDUINO_FILES");
    let files = raw_files.split(" ");

    let login_and_destination: &str = &get_env_var("CONNECTION").to_string();

    //for file in files {
    //    println!("Copying {}", file);
    //    match scp(&file, login_and_destination) {
    //        Ok(x) => println!("Success!"),
    //        Err(_) => std::process::exit(2),
    //    }
    //}

    // run the program
}
