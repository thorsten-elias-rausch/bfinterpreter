mod arg_parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options =  arg_parser::parse(args);
    println!("{}", options.script_file.unwrap());
    println!("{}", options.in_file.unwrap());
    println!("{}", options.out_file.unwrap());
}
