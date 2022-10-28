use std::env;

mod arg_parser;
mod interpreter;
mod io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = arg_parser::parse(args);

    let script = io::file_to_chars(options.script_file.as_str());

    let input = match options.in_file {
        None => io::input_from_stdin(),
        Some(val) => io::input_from_file(val.as_str()),
    };

    let output = match options.out_file {
        None => io::output_to_stdout(),
        Some(val) => io::output_to_file(val.as_str()),
    };

    interpreter::interpret(&*script, input, output);
}
