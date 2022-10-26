use std::process::exit;

const TAG_IN_FILE: &str = "-i";
const TAG_OUT_FILE: &str = "-o";
const TAG_SCRIPT_FILE: &str = "-f";

pub struct ParsedArgs {
    pub execution_path: String,
    pub script_file: String,
    pub in_file: Option<String>,
    pub out_file: Option<String>,
}

pub fn parse(args: Vec<String>) -> ParsedArgs {
    let execution_path = args.get(0).expect(
        "Argument parser received a vector without any arguments. There should always be at \
        least one argument, that being the path to this executable."
    ).clone();

    let mut script_file: Option<String> = None;
    let mut in_file: Option<String> = None;
    let mut out_file: Option<String> = None;

    let mut i = 1;
    loop {
        let tag = match args.get(i) {
            Some(key) => key.as_str(),
            None => break,
        };

        match tag {
            TAG_IN_FILE => {
                if in_file != None { exit_duplicate_arg(tag) };
                match args.get(i + 1) {
                    Some(value) => in_file = Some(value.clone()),
                    None => exit_empty_arg(tag)
                }
                i += 2
            }
            TAG_OUT_FILE => {
                if out_file != None { exit_duplicate_arg(tag) };
                match args.get(i + 1) {
                    Some(value) => out_file = Some(value.clone()),
                    None => exit_empty_arg(tag)
                }
                i += 2
            }
            TAG_SCRIPT_FILE => {
                if script_file != None { exit_duplicate_arg(tag) };
                match args.get(i + 1) {
                    Some(value) => script_file = Some(value.clone()),
                    None => exit_empty_arg(tag)
                }
                i += 2
            }
            _ => exit_nonexistent_arg(tag)
        }
    }

    return ParsedArgs {
        execution_path,
        in_file,
        out_file,
        script_file: script_file.expect("Script file not set."),
    };
}

fn exit_nonexistent_arg(arg: &str) {
    eprintln!("Option '{}' does not exist.", arg);
    display_usage();
    exit(-1);
}

fn exit_empty_arg(arg: &str) {
    eprintln!("Option '{}' requires an argument.", arg);
    display_usage();
    exit(-1);
}

fn exit_duplicate_arg(arg: &str) {
    eprintln!("Duplicate argument '{}'.", arg);
    display_usage();
    exit(-1);
}

fn display_usage() {
    eprintln!("Options: ");
    eprintln!(" {}\tPath to file containing the script to run. (Required)", TAG_SCRIPT_FILE);
    eprintln!(" {}\tPath to file to read inputs from. (Uses console by default.)", TAG_IN_FILE);
    eprintln!(" {}\tPath to file to write outputs to. (Uses console by default.)", TAG_OUT_FILE);
}