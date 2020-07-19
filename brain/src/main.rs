use std::error::Error;
use brain::comm::Messages;
// use brain::comm::Messages;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut test = Messages::new();
    // test.add(true, "Test Message".to_string());
    let mut test = Messages::new();
    let read_expected: String = String::from("Test");
    let read_result = test.read_the_buffer_on_test();
    Ok(())
}
