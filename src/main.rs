use std::env;

mod arg_parser;
mod interpreter;
mod io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = arg_parser::parse(args);

    let script = io::file_to_chars(options.script_file.as_str());

    let mut input = match options.in_file {
        None => io::Input::stdin(),
        Some(val) => io::Input::file(val.as_str()),
    };

    let mut output = match options.out_file {
        None => io::Output::stdout(),
        Some(val) => io::Output::file(val.as_str()),
    };

    interpreter::interpret(&*script, &mut input, &mut output);
}
