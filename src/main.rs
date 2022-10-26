use std::env;
use std::fs::File;
use std::io::{Read};

mod arg_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = arg_parser::parse(args);

    let script = file_to_chars(options.script_file);

    println!("{:?}", script);
}

fn file_to_chars(path: String) -> Vec<char> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");
    buffer.chars().collect()
}