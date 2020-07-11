use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let f = File::open("/dev/ttyUSB0").expect("oh no");
    let mut f = BufReader::new(f);
    let mut read_buffer: Vec<u8> = Vec::new();
    f.read_until(b'V', &mut read_buffer).expect("reading from cursor won't fail");
    print!("{:?}", f);

}
