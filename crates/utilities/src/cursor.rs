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

    pub fn update(&mut self, index: usize, text: &str) {
        // TODO Overhaul this system
        self.index = index;
        let text_to_index: String = text.chars().take(index + 1).collect();
        let lines_to_index: Vec<&str> = text_to_index.split("\n").collect();
        self.line_num = lines_to_index.len() - 1;
        let len = lines_to_index.last().unwrap().len();
        if len != 0 {
            self.col = len - 1;
        } else if lines_to_index.len() > 1 {
            self.col = lines_to_index.get(lines_to_index.len() - 2).unwrap().len();
        }
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

    pub fn update(&mut self, span: std::ops::Range<usize>) {
        self.start.update(span.start, &self.text);
        self.end.update(span.end, &self.text);
    }
}
