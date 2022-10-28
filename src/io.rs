use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::str::Chars;

use crate::interpreter;

pub fn file_to_chars(path: &str) -> Vec<char> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");
    buffer.chars().collect()
}

pub fn input_from_stdin() {
    todo!()
}

pub fn input_from_file(path: &str) -> impl interpreter::Input {
    return InputFromFile {};
}

pub fn output_to_stdout() {
    todo!()
}

pub fn output_to_file(path: &str) {
    todo!()
}

struct InputFromFile {}

impl interpreter::Input for InputFromFile {
    fn get(&self) -> char {
        todo!()
    }
}
