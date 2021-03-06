extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new(option_env!("CARGO_PKG_NAME").unwrap())
                          .version(option_env!("CARGO_PKG_VERSION").unwrap())
                          .author(option_env!("CARGO_PKG_AUTHORS").unwrap())
                          .about("Does awesome things")
                          .arg(Arg::with_name("MODE")
                               .help("Sets the running mode to use")
                               .required(true)
                               .takes_value(true)
							   .possible_value("check")
							   .possible_value("live")
							   .possible_value("reset")
							   .possible_value("record")
							   .possible_value("test")
                               .index(1))
                          .arg(Arg::with_name("SETUP_FILE")
                               .help("Sets the setup file to use")
                               .required(false)
                               .index(2))
                          .get_matches();

    let mode = matches.value_of("MODE").unwrap_or("live");
    let setup_file = matches.value_of("SETUP_FILE").unwrap_or("setup.yaml");
    println!("Using mode: {}", mode);
    println!("Using setup file: {}", setup_file);
}
