use std::env;

mod arg_parser;
mod interpreter;
mod io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = arg_parser::parse(args);

    let script = io::file_to_chars(options.script_file);
    println!("{:?}", script);
}
