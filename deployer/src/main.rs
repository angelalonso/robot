use dotenv;
use std::env;
use std::error::Error;
use std::process::{Command, Stdio};
#[macro_use]
extern crate simple_error;

type BoxResult<T> = Result<T,Box<Error>>;

fn run_over_ssh(destination: &str, command: &str) -> BoxResult<i32> {
    let mut child = Command::new("ssh")
        .arg(destination)
        .arg(command)
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

fn get_env_var(key: &str) -> std::string::String {
    // Accessing an env var
    match env::var(key) {
        Ok(val) => return val,
        Err(_e) => return key.to_string(),
    }
}

fn main() {
    // We don't (yet) need to catch arguments
    //let args: Vec<String> = env::args().skip(1).collect();

    dotenv::dotenv().ok();
    let raw_files = get_env_var("ARDUINO_FILES");
    let files: Vec<&str> = raw_files.split_whitespace().collect();
    //let files = raw_files.split(" ");

    let login_and_destination: &str = &get_env_var("CONNECTION").to_string();

    //for file in files {
    //    println!("Copying {}", file);
    //    match scp(&file, login_and_destination) {
    //        Ok(x) => println!("Success!"),
    //        Err(_) => std::process::exit(2),
    //    }
    //}

    println!("{:?}", files.iter());
    // run the program
    //run_over_ssh("pi@127.0.0.1", "'/home/pi/test.sh'");
}

#[cfg(test)]
mod tests {

    /// Here we test what happens if an env var is not present
    #[test]
    fn test_get_env_var() {
        std::env::set_var("VAR_OK", "This is fine");
        let test_var: &str = &super::get_env_var("VAR_OK").to_string();
        assert_eq!("This is fine", test_var);
        std::env::remove_var("VAR_ERR");
        let test_var_err: &str = &super::get_env_var("VAR_ERR").to_string();
        assert_ne!("This is fine", test_var_err);
    }

    #[test]
    fn scp_file_list() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn run_several_progs_remotely() {
        assert_eq!(2 + 2, 4);
    }
}
