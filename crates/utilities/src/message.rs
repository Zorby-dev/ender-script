use ansi_term::ANSIString;
use ansi_term::Color::{ self, Fixed, White };
use ansi_term::ANSIStrings;

use crate::cursor::Cursor;
use crate::color::*;

pub mod details {
    #[macro_export]
    macro_rules! IllegalCharacter {
        ($chr: tt) => (format!("Character '{}' is not allowed", $chr).as_str());
    }

    #[macro_export]
    macro_rules! MissingCharacter {
        ($chr: tt) => (format!("Character '{}' is missing", $chr).as_str());
    }

    #[macro_export]
    macro_rules! MissingExpression {
        () => ("Expected any expression");
    }

    #[macro_export]
    macro_rules! MissingMemberDeclaration {
        ($name: tt, $member_type: tt) => (format!("Expected ':' to declare {} '{}'", $name, $member_type).as_str());
    }

    #[macro_export]
    macro_rules! MissingMemberTypeOrValueAssignment {
        ($name: tt, $member_type: tt) => (format!("Expected {} type or '=' to assign '{}' a value", $member_type, $name).as_str());
    }

    #[macro_export]
    macro_rules! MissingMemberType {
        ($member_type: tt) => (format!("Expected {} type", $member_type).as_str());
    }

    #[macro_export]
    macro_rules! MissingMemberName {
        ($member_type: tt) => (format!("Expected {} name", $member_type).as_str());
    }

    #[macro_export]
    macro_rules! MissingCaseOpening {
        ($case_type: tt) => (format!("Expected '(' to open {}", $case_type).as_str());
    }

    pub use { IllegalCharacter, MissingCharacter, MissingMemberDeclaration, MissingMemberTypeOrValueAssignment, MissingMemberType, MissingMemberName, MissingExpression, MissingCaseOpening };
}

#[derive(Clone)]
pub enum MessageType {
    IllegalCharacter,
    MissingCharacter,
    InvalidEscapeSequence,
    MissingExpression,
    MissingMemberDeclaration,
    MissingMemberName,
    MissingMemberType,
    MissingMemberTypeOrValueAssignment,
    MissingCaseOpening,
    MissingCaseClosure
}
impl MessageType {
    pub fn parameters(&self) -> (bool, &'static str, &'static str) {
        match self {
            /*
            ES 0 00 E
               | |  |
               | |  #- message type (E or W)
               | |
               | #---- message code (00 to 99)
               |
               #------ message process:
                        - Lexer  = 0
                        - Parser = 1
            */
            MessageType::IllegalCharacter                    => (true, "ES000E", "Illegal character"),
            MessageType::MissingCharacter                    => (true, "ES001E", "Missing character"),
            MessageType::InvalidEscapeSequence               => (true, "ES002E", "Invalid escape sequence"),
            MessageType::MissingExpression                   => (true, "ES100E", "Missing expression"),
            MessageType::MissingMemberDeclaration            => (true, "ES101E", "Missing member declaration"),
            MessageType::MissingMemberName                   => (true, "ES102E", "Missing member name"),
            MessageType::MissingMemberType                   => (true, "ES103E", "Missing member type"),
            MessageType::MissingMemberTypeOrValueAssignment  => (true, "ES104E", "Missing member type or value assignment"),
            MessageType::MissingCaseOpening                  => (true, "ES104E", "Missing case opening"),
            MessageType::MissingCaseClosure                  => (true, "ES104E", "Missing case closure"),
        }
    }

    pub fn is_error(&self) -> bool {
        self.parameters().0
    }

    pub fn code(&self) -> &'static str {
        self.parameters().1
    }

    pub fn name(&self) -> &'static str {
        self.parameters().2
    }
}

#[derive(Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub details: String,
    pub cursor: Cursor
}

impl Message {
    fn color(&self) -> Color {
        match self.message_type.is_error() {
            true => Red!(),
            false => todo!(),
        }
    }

    fn light_color(&self) -> Color {
        match self.message_type.is_error() {
            true => LightRed!(),
            false => todo!(),
        }
    }

    fn header(&self) -> String {
        if self.message_type.is_error() {
            return self.color().bold().paint(format!("Error {}: ", self.message_type.code())).to_string();
        }
        panic!();
    }

    fn file_name(&self) -> String {
        let strings: &[ANSIString<'static>] = &[
            Grey!().paint("["),
            White.paint(format!("{}:{}:{}", self.cursor.file_name, self.cursor.start.line_num, self.cursor.start.col)),
            Grey!().paint("]"),
        ];
        return ANSIStrings(strings).to_string();
    }

    fn line(&self) -> String {
        let line = self.cursor.text.split('\n').nth(self.cursor.start.line_num).unwrap().to_string();
        let first = line[..self.cursor.start.col].to_string();
        let err = line[self.cursor.start.col..self.cursor.end.col].to_string();
        let last = line[self.cursor.end.col..].to_string();
        let strings: &[ANSIString<'static>] = &[
            Grey!().paint(first),
            self.light_color().bold().paint(err),
            Grey!().paint(last),
        ];
        return ANSIStrings(strings).to_string();
    }

    fn content(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let error_len = self.cursor.end.col - self.cursor.start.col;
        let middle: usize = ((error_len / 2) as f64 + 0.25) as usize;
        let mut underline = String::new();
        for _ in 0..self.cursor.start.col {
            underline += " ";
        }
        for _ in 0..middle {
            underline += "─";
        }
        underline += "┬";
        for _ in middle+1..error_len {
            underline += "─";
        }
        result.push(self.light_color().paint(underline).to_string());
        let mut error_line = String::new();
        for _ in 0..self.cursor.start.col+middle {
            error_line += " ";
        }
        error_line += "╰─";
        for _ in 0..error_len-middle {
            error_line += "─";
        }
        error_line += " ";
        result.push(self.light_color().paint(error_line).to_string() + &self.details);
        result
    }

    fn body(&self) -> String {
        let line_num = self.cursor.start.line_num.to_string();
        let padding_width = line_num.len() + 2;
        let mut output = String::new();
        
        let mut padding = String::new();
        for _ in 0..padding_width {
            padding += " ";
        }
        output += &Grey!().paint(padding.clone() + "╭─").to_string();
        output += &(self.file_name() + "\n");
        output += &Grey!().paint(padding.clone() + "│\n").to_string();
        output += &Grey!().paint(format!(" {} │ ", line_num)).to_string();
        output += &(self.line() + "\n");

        let content = self.content();
        for i in 0..content.len() {
            output += &Grey!().paint(padding.clone() + "· ").to_string();
            output += &content[i];
            output += "\n";
        }
        let mut footer = String::new();
        for _ in 0..padding_width {
            footer += "─";
        }
        footer += "╯";
        output += &Grey!().paint(footer).to_string();

        return output;
    }

    pub fn to_string(&self) -> String {
          self.header()
        + self.message_type.name()
        + "\n"
        + &self.body()
    }

    pub fn error(message_type: MessageType, details: &str, cursor: Cursor) -> Self {
        return Self {
            details: details.to_string(),
            message_type,
            cursor
        }
    }
}