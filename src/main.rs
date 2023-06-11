mod demo;

use bitreader_rs::bitreader::Bitreader;
use std::{env, fs, process};

use crate::demo::Demo;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Error: No provided arguments.");
        process::exit(1)
    }

    let file_name = &args[1];

    let file = fs::read(file_name).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1)
    });

    let bitreader = Bitreader::new(file.as_slice());

    let mut demo = Demo::new(bitreader);

    demo.parse_header().unwrap_or_else(|err| {
        eprintln!("Error parsing demo header: {}", err);
        process::exit(1)
    });

    println!("{:?}", demo.header);

    println!("LENGTH: {}\n POSITION: {}", demo.bitreader.length, demo.bitreader.position);

    demo.parse_frames().unwrap_or_else(|err| {
        eprintln!("Error parsing demo frames: {}", err);
        process::exit(1)
    });
}