mod demo;

use std::{fs, process};

use crate::demo::Demo;

fn main() {
    println!("Hello, World");
    let file_name = "test_demo.dem";
    let file = fs::read(file_name).unwrap();

    let demo_header = Demo::parse(file).unwrap_or_else(|err| {
        eprintln!("Error parsing demo header: {}", err);
        process::exit(1)
    });

    println!("{:?}", demo_header);
}