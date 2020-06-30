use std::env;
use std::process::{Command, Stdio};
//use std::io::{self, Write};
use std::io::{self};
use dotenv;

// TODO
//
// Define setup on .env instead of as args
//https://docs.rs/dotenv/0.15.0/dotenv/fn.dotenv.html

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

    println!("output = {:?}", output);

    Ok(())
    // TODO: catch output = Output { status: ExitStatus(ExitStatus(256)), stdout: "", stderr: "" }
}

fn print_env_var(key: &str) {
    // Accessing an env var
    match env::var(key) {
        Ok(val) => println!("{}: {}", key, val),
        Err(e) => println!("Couldn't print env var {}: {}", key, e),
    }
}

fn main() {
    dotenv::dotenv().ok();
    let mut args: Vec<String> = env::args().collect();
    args.drain(0..1);

    let login_and_destination: &str = "pi@192.168.0.11:/home/pi/";

    for arg in args.iter() {
        println!("Copying {}", arg);
        let retest = scp(&arg, login_and_destination);
    }

    println!("Listing all env vars:");
    for (key, val) in env::vars() {
        println!("{}: {}", key, val);
    }
    let key = "TESTVAR";
    print_env_var(key);
     println!("Setting env var {}", key);
     // Setting an env var for the current process
     env::set_var(key, "8080");
 
     print_env_var(key);
 
     // Removing an env var for the current process
     println!("Removing env var {}", key);
     env::remove_var(key);
 
     print_env_var(key);
}
