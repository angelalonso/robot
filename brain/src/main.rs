#![warn(rust_2018_idioms)]

use std::{env, io, str};
use tokio_util::codec::{Decoder, Encoder};
use futures::stream::StreamExt;

use bytes::BytesMut;

//const DEFAULT_TTY: &str = "/dev/tty7";
const DEFAULT_TTY: &str = "/dev/ttyUSB0";

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, _item: Self::Item, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut args = env::args();
    let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());

    let settings = tokio_serial::SerialPortSettings::default();
    let mut port = tokio_serial::Serial::from_path(tty_path, &settings).unwrap();

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let mut reader = LineCodec.framed(port);

    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        println!("{}", line);
    }
}
// use std::io::{Result, BufRead};
// use std::fs::File;
// 
// fn read_until<R: BufRead>(mut read: R, out: &mut Vec<u8>, pair: (u8, u8)) -> Result<usize> {
//     let mut bytes_read = 0;
//     let mut got_possible_terminator = false;
//     
//     loop {
//         let buf = read.fill_buf()?;
//         if buf.len() == 0 { return Ok(bytes_read); } // EOF
//         
//         let mut seen = 0;
//         
//         for byte in buf.iter().copied() {
//             seen += 1;
//             if got_possible_terminator && byte == pair.1 {
//                 out.pop(); // remove first half of terminator
//                 read.consume(seen);
//                 return Ok(bytes_read + seen - 2);
//             }
//             out.push(byte);
//             got_possible_terminator = byte == pair.0;
//         }
//         let len = buf.len();
//         read.consume(len);
//         bytes_read += len;
//     }
// }
// 
// use std::io::{Cursor, BufReader, Read};
// 
// fn main() {
//     
//     let serialfile = "./test.serial";
// // //    let serialfile = "/dev/ttyUSB0";
//     let f = File::open(serialfile).expect("oh noo");
//     let mut f = BufReader::new(f);
//     
//     let mut vec = Vec::new();
//     println!("{}", read_until(&mut f, &mut vec, (3, 1)).unwrap());
// }
////////////////////////////
// use std::fs::File;
// use std::io::{BufReader, BufRead};
// 
// fn main() {
//     let serialfile = "./test.serial";
// //    let serialfile = "/dev/ttyUSB0";
//     let f = File::open(serialfile).expect("oh noo");
//     let mut f = BufReader::new(f);
//     let mut read_buffer: Vec<u8> = Vec::new();
//     f.read_until(b'V', &mut read_buffer).expect("reading from cursor won't fail");
//     print!("{:?}", f);
// 
// }
