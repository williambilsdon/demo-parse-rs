mod demo;

use std::{fs, process, env};
use bitreader_rs::bitreader::Bitreader;

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

    let mut bitreader = Bitreader::new(file.as_slice());

    let demo_header = Demo::parse(&mut bitreader).unwrap_or_else(|err| {
        eprintln!("Error parsing demo header: {}", err);
        process::exit(1)
    });

    println!("{:?}", demo_header);
}