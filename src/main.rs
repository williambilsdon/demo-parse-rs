mod demo;

use std::{fs, process, env};
use crate::demo::Demo;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        eprintln!("Error: No provided arguments.");
        process::exit(1)
    }
    let file_name = &args[0];
    let file = fs::read(file_name).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1)
    });

    let demo_header = Demo::parse(file).unwrap_or_else(|err| {
        eprintln!("Error parsing demo header: {}", err);
        process::exit(1)
    });

    println!("{:?}", demo_header);
}