use std::io::BufRead;

#[derive(Debug, Clone)]
pub struct Position {
    pub index: usize,
    pub col: usize,
    pub line_num: usize,
}
impl Position {
    fn new() -> Self {
        Self {
            index: 0,
            col: 0,
            line_num: 0,
        }
    }

    pub fn update(&mut self, index: usize, text: String) {
        self.index = index;
        let a = text.chars().take(index).collect::<String>();
        let lines = a.split("\n").collect::<Vec<&str>>();
        self.line_num = lines.len();
        self.col = lines.last().unwrap().len() - 1;
    }
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub start: Position,
    pub end: Position,
    pub file_name: String,
    pub text: String,
}
impl Cursor {
    pub fn new(file_name: &str, text: &str) -> Self {
        Self {
            start: Position::new(),
            end: Position::new(),
            file_name: file_name.to_string(),
            text: text.to_string(),
        }
    }

    pub fn update(&mut self, slice: (usize, usize)) {
        self.start.update(slice.0, self.text.clone());
        self.end.update(slice.1, self.text.clone());
    }
}