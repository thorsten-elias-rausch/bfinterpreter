pub trait Input {
    fn get(&self) -> char;
}

pub trait Output {
    fn put(&self, c: char) -> ();
}

pub fn interpret<>(script: &[char], input: &impl Input, output: &impl Output) {
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
            Token::JmpF => { todo!() }
            Token::JmpB => { todo!() }
        }
        script_ptr += 1;
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

