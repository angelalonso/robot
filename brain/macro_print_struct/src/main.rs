use macro_print_struct::check::Check;
use std::process;

fn main() {
    let mut check = Check::new().unwrap_or_else(|err| {
        eprintln!("Problem Initializing Check: {}", err);
        process::exit(1);
    });
    check.show();
}
