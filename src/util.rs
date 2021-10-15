use chrono::{Local, DateTime};
use crate::colors::{self, colored};

pub static DIGITS: &str = "0123456789";
pub static VALID_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";

pub struct Position {
    pub index: usize,
    pub ln: usize,
    pub col: usize,
    pub file_name: String,
    pub text: String
}

impl Position {
    pub fn new(index: usize, ln: usize, col: usize, file_name: &str, text: &str) -> Position {
        return Position {
            index,
            ln,
            col,
            file_name: file_name.to_string(),
            text: text.to_string()
        }
    }

    pub fn advance(&mut self, current_char: char) -> &Position {
        self.index += 1;
        self.col += 1;

        if current_char == '\n' {
            self.col = 0;
            self.ln += 1;
        }

        return self;
    }
}

pub struct Error {
    pub name: String,
    pub details: String,
    pub start_pos: Position,
    pub end_pos: Position
}

impl Error {

    pub fn to_string(&mut self) -> String {
		let details: String;
		if self.details != "" {
			details = String::from(": ") + &self.details
		} else {
			details = String::from("")
		}

		let today: DateTime<Local> = Local::now();
		let timestamp = today.format("%H:%M:%S").to_string();
		let line_num = self.start_pos.ln + 1;
        let lines: Vec<&str> = self.start_pos.text.split("\n").collect();
        let prev_line: Option<&&str>;
        if self.start_pos.ln > 0 {
            prev_line = lines.get(self.start_pos.ln - 1);
        }
        else {
            prev_line = None
        }
        let prev_line_str: String;
        match prev_line {
            Some(x) => { prev_line_str = String::from(format!("\n {} | {}", line_num - 1, x)); }
            None => { prev_line_str = String::from(""); }
        }
		let mut line = lines[self.start_pos.ln].to_string();
		line = line[..self.start_pos.col].to_string() +
			colors::RED +
			&line[self.start_pos.col..self.end_pos.col].to_string() +
			colors::RESET +
			&line[self.end_pos.col..].to_string();
		let mut underline = colors::RED.to_string();
		for  i in 0..4+line_num.to_string().len()+self.start_pos.col {
			underline += " ";
		}
		for i in self.start_pos.col..self.end_pos.col {
			underline += "^";
		}
		underline += colors::RESET;

        return format!("[\x1b[91mERROR\x1b[0m] \x1b[90m{} \x1b[91m{}\x1b[0m{}\nFile \x1b[93m\"{}:{}:{}\"\x1b[90m{}\n {} | \x1b[0m{}\n{}",
            timestamp, self.name, details, self.end_pos.file_name, line_num, self.start_pos.col, prev_line_str, line_num, line, underline)

		/*return String::from("[") + &colored(colors::RED, "ERROR".to_string()) + "]" + &timestamp + &colored(colors::RED, self.name.to_owned()) + 
            &details + "\nFile" + &colored(colors::YELLOW, self.end_pos.file_name.to_owned() + ":" + &line_num.to_string() + "\"") + 
            &prev_line_str + &colored(colors::GREY, line_num.to_string() + " | ") + &line + "\n     " + &underline;*/
	}

}