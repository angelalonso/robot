use std::error::Error;
use brain::comm::Messages;
use brain::mockduino::Mockduino;

fn main() -> Result<(), Box<dyn Error>> {
    let mut arduino = Mockduino::new("testfile");
    let get_bootload = arduino.bootload();
    println!("{:?}", get_bootload);

    let mut test = Messages::new();
    let read_result = test.read_the_buffer_mock();
    println!("{:?}", read_result);
    Ok(())
}
