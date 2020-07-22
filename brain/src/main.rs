use std::error::Error;
use brain::comm::Messages;

fn main() -> Result<(), Box<dyn Error>> {
    let mut test = Messages::new();
    let read_result = test.read_the_buffer_on_test();
    println!("{:?}", read_result);
    Ok(())
}
