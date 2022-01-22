use std::cmp::max;

#[derive(Debug, Clone)]
enum PositionRole {
    Start,
    End,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub index: usize,
    pub col: usize,
    pub line_num: usize,
    role: PositionRole,
}
impl Position {
    fn new(role: PositionRole) -> Self {
        Self {
            index: 0,
            col: 0,
            line_num: 0,
            role,
        }
    }

    pub fn update(&mut self, index: usize, text: &str) {
        match self.role {
            | PositionRole::Start => {
                self.index = index;

                let overflow: usize = max(
                    0,
                    (index as isize) - (text.len() as isize) + 1,
                )
                .try_into()
                .unwrap();

                let text_to_index: String = text.chars().take(index + 1).collect();
                let lines_to_index: Vec<&str> = text_to_index.split("\n").collect();

                self.line_num = max(
                    0,
                    (lines_to_index.len() as isize) - 1,
                )
                .try_into()
                .unwrap();

                let last_line: &str = lines_to_index.last().unwrap().to_owned();

                let col: usize = max(
                    0,
                    (last_line.len() as isize) - 1,
                )
                .try_into()
                .unwrap();
                self.col = col + overflow;
            }
            | PositionRole::End => {
                self.index = index;

                let overflow: usize = max(
                    0,
                    (index as isize) - (text.len() as isize) + 1,
                )
                .try_into()
                .unwrap();

                let text_to_index: String = text.chars().take(index).collect();
                let lines_to_index: Vec<&str> = text_to_index.split("\n").collect();

                self.line_num = lines_to_index.len() - 1;

                let last_line: &str = lines_to_index.last().unwrap().to_owned();

                self.col = last_line.len() + overflow;
            }
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
            start: Position::new(PositionRole::Start),
            end: Position::new(PositionRole::End),
            file_name: file_name.to_string(),
            text: text.to_string(),
        }
    }

    pub fn update(&mut self, span: std::ops::Range<usize>) {
        self.start.update(
            span.start, &self.text,
        );
        self.end.update(
            span.end, &self.text,
        );
    }

    pub fn clone_with_start(&self, start: &Position) -> Self {
        Self {
            start: start.clone(),
            end: self.end.clone(),
            file_name: self.file_name.clone(),
            text: self.text.clone(),
        }
    }
}
