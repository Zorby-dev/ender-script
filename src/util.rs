pub static DIGITS: &str = "0123456789";
pub static VALID_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";

#[derive(Debug)]
pub struct Position {
    pub index: usize,
    pub ln: usize,
    pub col: usize,
    pub file_name: String,
    pub text: String,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            index: self.index,
            ln: self.ln,
            col: self.col,
            file_name: self.file_name.clone(),
            text: self.text.clone(),
        }
    }
}

impl Position {
    pub fn new(file_name: &str, text: &str) -> Self {
        return Self {
            index: 0,
            ln: 0,
            col: 0,
            file_name: file_name.to_string(),
            text: text.to_string(),
        };
    }

    pub fn advance(&mut self) -> &Self {
        let current_char = self.text.chars().nth(self.index).unwrap();

        self.forced_advance();

        if current_char == '\n' {
            self.col = 0;
            self.ln += 1;
        }

        return self;
    }

    pub fn forced_advance(&mut self) -> &Self {
        self.index += 1;
        self.col += 1;

        return self;
    }
}