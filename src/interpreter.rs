use crate::io::{Input, Output};

pub fn interpret(script: &[char], input: &mut Input, output: &mut Output) {
    let mut data: [u8; DATA_LENGTH] = [0; DATA_LENGTH];
    let mut data_ptr: usize = 0;
    let mut script_ptr: usize = 0;

    loop {
        if data_ptr + 1 == 0 { data_ptr += DATA_LENGTH }
        if data_ptr > DATA_LENGTH { data_ptr -= DATA_LENGTH }

        let token = match script.get(script_ptr) {
            Some(val) => Token::from(*val),
            None => break
        };
        match token {
            Token::None => {}
            Token::Incr => data[data_ptr] += 1,
            Token::Decr => data[data_ptr] -= 1,
            Token::Next => data_ptr += 1,
            Token::Prev => data_ptr -= 1,
            Token::PutC => output.put(data[data_ptr] as char),
            Token::GetC => data[data_ptr] = input.get() as u8,
            Token::JmpF => jump_forward(script, &mut data, &mut data_ptr, &mut script_ptr),
            Token::JmpB => jump_back(script, &mut data, &mut data_ptr, &mut script_ptr),
        }
        script_ptr += 1;
    }
}

fn jump_forward(script: &[char], data: &mut [u8; DATA_LENGTH], data_ptr: &mut usize, script_ptr: &mut usize) {
    if data[*data_ptr] != 0 { return; }
    let mut depth = 0;
    loop {
        let token = Token::from(*script.get(*script_ptr).expect("Unmatched forward jump."));
        match token {
            Token::JmpF => depth += 1,
            Token::JmpB => {
                depth -= 1;
                if depth == 0 { return; }
            }
            _ => {}
        }
        *script_ptr += 1;
    }
}

fn jump_back(script: &[char], data: &mut [u8; DATA_LENGTH], data_ptr: &mut usize, script_ptr: &mut usize) {
    if data[*data_ptr] == 0 { return; }
    let mut depth = 0;
    loop {
        let token = Token::from(*script.get(*script_ptr).expect("Unmatched back jump."));
        match token {
            Token::JmpB => depth += 1,
            Token::JmpF => {
                depth -= 1;
                if depth == 0 { return; }
            }
            _ => {}
        }
        *script_ptr -= 1;
    }
}

#[repr(u8)]
enum Token {
    Incr = b'+',
    Decr = b'-',
    Next = b'>',
    Prev = b'<',
    PutC = b'.',
    GetC = b',',
    JmpF = b'[',
    JmpB = b']',
    None = b' ',
}

impl Into<char> for Token {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl Token {
    fn from(c: char) -> Token {
        match c {
            '+' => Token::Incr,
            '-' => Token::Decr,
            '>' => Token::Next,
            '<' => Token::Prev,
            '.' => Token::PutC,
            ',' => Token::GetC,
            '[' => Token::JmpF,
            ']' => Token::JmpB,
            _ => Token::None
        }
    }
}

const DATA_LENGTH: usize = 30000;

