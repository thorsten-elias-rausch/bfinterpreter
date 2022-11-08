use std::fs::File;
use std::io::{Read, stdin, Write};

pub enum Input {
    FromStdin(InputFromStdin),
    FromFile(InputFromFile),
}

impl Input {
    pub fn stdin() -> Input {
        Input::FromStdin(InputFromStdin::new())
    }

    pub fn file(path: &str) -> Input {
        Input::FromFile(InputFromFile::new(path))
    }

    pub fn get(&mut self) -> char {
        return match self {
            Input::FromStdin(input) => input.get(),
            Input::FromFile(input) => input.get()
        };
    }
}

pub enum Output {
    ToStdout(OutputToStdout),
    ToFile(OutputToFile),
}

impl Output {
    pub fn stdout() -> Output {
        Output::ToStdout(OutputToStdout::new())
    }

    pub fn file(path: &str) -> Output {
        Output::ToFile(OutputToFile::new(path))
    }

    pub fn put(&mut self, c: char) {
        match self {
            Output::ToStdout(output) => output.put(c),
            Output::ToFile(output) => output.put(c)
        }
    }
}

pub fn file_to_chars(path: &str) -> Vec<char> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");
    buffer.chars().collect()
}

pub struct InputFromFile {
    content: Vec<char>,
    index: usize,
}

impl InputFromFile {
    fn new(path: &str) -> InputFromFile {
        let content = file_to_chars(path);
        return InputFromFile { content, index: 0 };
    }

    fn get(&mut self) -> char {
        return match self.content.get(self.index) {
            Some(value) => {
                self.index += 1;
                *value
            }
            None => '\0',
        };
    }
}

pub struct InputFromStdin {
    latest_line: Vec<char>,
    index: usize,
}

impl InputFromStdin {
    fn new() -> InputFromStdin {
        InputFromStdin {
            latest_line: Vec::new(),
            index: 0,
        }
    }

    fn get(&mut self) -> char {
        return match self.latest_line.get(self.index) {
            Some(value) => {
                self.index += 1;
                *value
            }
            None => {
                let mut buffer = String::new();
                stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
                self.latest_line = buffer.chars().collect();
                self.index = 0;
                return self.get();
            }
        };
    }
}

pub struct OutputToStdout {}

impl OutputToStdout {
    fn new() -> OutputToStdout {
        return OutputToStdout {};
    }

    fn put(&mut self, c: char) -> () {
        print!("{}", c);
    }
}

pub struct OutputToFile {
    file: File,
}

impl OutputToFile {
    fn new(path: &str) -> OutputToFile {
        OutputToFile { file: File::create(path).expect("Failed to create file") }
    }

    fn put(&mut self, c: char) -> () {
        let buffer = &mut [];
        c.encode_utf8(buffer);
        self.file.write(buffer).expect("Failed to write to file");
    }
}
