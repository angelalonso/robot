use std::fs;
// TODO
// We need to put latest of a named (argument) arduino program into the raspberry
// , then we run the controller application
//
// TASKS
// - find out latest .hex on /tmp/arduino_build_* 
//
fn main() {
    let paths = fs::read_dir("/tmp/").unwrap();
    let path_pattern = "arduino_build";

    for path in paths {
        let t = path.unwrap().path().display().to_string();
        if t.contains(path_pattern) {
            println!("Name: {}", t)
        }
    }
}
