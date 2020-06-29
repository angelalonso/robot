use std::env;
use std::process::{Command, Stdio};
use std::io::{self, Write};

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
        let child_stdin = child.stdin.as_mut().unwrap();
    }

    let output = child.wait_with_output()?;

    println!("output = {:?}", output);

    Ok(())
    // TODO: catch output = Output { status: ExitStatus(ExitStatus(256)), stdout: "", stderr: "" }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.drain(0..1);

    let login_and_destination: &str = "pi@192.168.0.11:/home/pi/";

    for arg in args.iter() {
        println!("Copying {}", arg);
        scp(&arg, login_and_destination);
    }
}
