use crate::colors::Color;

pub static DIGITS: &str = "0123456789";
pub static VALID_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";

#[derive(Debug)]
pub struct Position {
    pub index: usize,
    pub ln: usize,
    pub col: usize,
    pub file_name: String,
    pub text: String
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {index: self.index, ln: self.ln, col: self.col, file_name: self.file_name.clone(), text: self.text.clone()}
    }
}

impl Position {
    pub fn new(file_name: &str, text: &str) -> Position {
        return Position {
            index: 0,
            ln: 0,
            col: 0,
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

mod message {
    use chrono::Local;
    use crate::colors::{Color, colored};
    use super::*;

    fn get_timestamp() -> String {
        return Local::now().format(&colored(Color::Grey, "%H:%M:%S")).to_string();
    }

    fn get_details(details: &String) -> String {
        if details != "" {
            return String::from(": ") + details;
        } else {
            return String::from("");
        }
    }

    fn get_file(pos: &Position) -> String {
        colored(Color::LightYellow, &format!("\"{}:{}:{}\"", pos.file_name, pos.ln, pos.col))
    }

    fn get_prev_line(pos: &Position) -> String {
        let lines: Vec<&str> = pos.text.split("\n").collect();
        let prev_line: Option<&&str>;
        if pos.ln > 0 {
            prev_line = lines.get(pos.ln - 1);
        }
        else {
            prev_line = None
        }
        let prev_line_str: String;
        match prev_line {
            Some(x) => { prev_line_str = colored(Color::Grey, &format!("\n {} | \x1b[0m{}", pos.ln, x)); }
            None => { prev_line_str = String::from(""); }
        }
        return prev_line_str;
    }

    fn get_line(start_pos: &Position, end_pos: &Position, color: Color) -> String {
        let lines: Vec<&str> = start_pos.text.split("\n").collect();
        let line = lines[start_pos.ln].to_string();
        return colored(Color::Grey, &colored(Color::Grey, &format!(" {} | ", start_pos.ln + 1)))
            + &line[..start_pos.col] + &colored(color, &line[start_pos.col..end_pos.col]) + &line[end_pos.col..];
    }

    fn get_underline(start_pos: &Position, end_pos: &Position, color: Color) -> String {
        let mut underline = color.to_string();
        for  _ in 0..4+(start_pos.ln+1).to_string().len()+start_pos.col {
            underline += " ";
        }
        for _ in start_pos.col..end_pos.col {
            underline += "^";
        }
        underline += Color::reset();
        return underline;
    }

    fn generate_message(message_type: String, color: Color, timestamp: &String, name: &String, details: &String) -> String {
        format!("[{}] {} {}{}", colored(color, &message_type), timestamp, colored(color, &name), details)
    }

    pub fn get_message(message_type: String, color: Color, name: &String, details: &String) -> String {
        let timestamp = get_timestamp();
        let name = name.clone();
        let details = get_details(details);
        return generate_message(message_type, color, &timestamp, &name, &details);
    }

    fn generate_message_with_traceback(message_type: String, color: Color, name: &String, details: &String, file: &String, prev_line: &String, line: &String, underline: &String) -> String {
        format!("{}\nFile {}{}\n{}\n{}", get_message(message_type, color, name, details), file, prev_line, line, underline)
    }

    pub fn get_message_with_traceback(message_type: String, color: Color, name: &String, details: &String, start_pos: &Position, end_pos: &Position) -> String {
        let name = name.clone();
        let file = get_file(start_pos);
        let prev_line = get_prev_line(start_pos);
        let line = get_line(start_pos, end_pos, color);
        let underline = get_underline(start_pos, end_pos, color);
        return generate_message_with_traceback(message_type, color, &name, &details, &file, &prev_line, &line, &underline);
    }
}

#[derive(Clone)]
pub struct Error {
    pub name: String,
    pub details: String,
    pub start_pos: Position,
    pub end_pos: Position
}

impl Error {

    pub fn to_string(&mut self) -> String {
		message::get_message_with_traceback(String::from("ERROR"), Color::Red, &self.name, &self.details, &self.start_pos, &self.end_pos)
	}

}

pub struct Warning {
    pub name: String,
    pub details: String,
    pub start_pos: Position,
    pub end_pos: Position
}

impl Warning {

    pub fn to_string(&mut self) -> String {
		message::get_message_with_traceback(String::from("WARNING"), Color::Yellow, &self.name, &self.details, &self.start_pos, &self.end_pos)
	}

}

pub struct Info {
    pub name: String,
    pub details: String
}

impl Info {

    pub fn to_string(&mut self) -> String {
		message::get_message(String::from("INFO"), Color::LightBlue, &self.name, &self.details)
	}

}