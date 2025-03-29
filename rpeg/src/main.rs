#![allow(warnings)]
use csc411_image::{Read, Rgb, RgbImage, Write};
use std::env;
// use std::env;
// use rpeg::codec::{compress, decompress};
use rpeg::codec::{compress, decompress, process_input};
fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 2 || argnum == 3);
    let filename = args.iter().nth(2).unwrap();
    match args[1].as_str() {
        "-c" => compress(Some(filename)),
        "-d" => decompress(Some(filename)),
        _ => {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }
}
