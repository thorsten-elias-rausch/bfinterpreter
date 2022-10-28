use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn file_to_chars(path: String) -> Vec<char> {
    let __path = Path::new(path.as_str());

    let mut file = File::open(path).expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");
    buffer.chars().collect()
}
