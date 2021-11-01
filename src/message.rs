use crate::util::Position;
use crate::colors::*;
use ansi_term::ANSIString;
use ansi_term::Color::{ self, Fixed, White };
use ansi_term::ANSIStrings;

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
    macro_rules! UnexpectedEOF {
        () => ("Expected anything except End Of File");
    }

    #[macro_export]
    macro_rules! MissingExpression {
        () => ("Expected any expression");
    }

    #[macro_export]
    macro_rules! MissingCodeblockClosure {
        () => ("Expected '}' to close a codeblock");
    }

    #[macro_export]
    macro_rules! MissingConditionOpening {
        () => ("Expected '(' to open a condition");
    }

    #[macro_export]
    macro_rules! MissingConditionClosure {
        () => ("Expected ')' to close a condition");
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
    macro_rules! MissingTupleOpening {
        () => ("Expected '(' to open a tuple");
    }

    #[macro_export]
    macro_rules! MissingTupleClosure {
        () => ("Expected ')' to close a tuple");
    }

    pub(crate) use { IllegalCharacter, MissingCharacter, MissingCodeblockClosure, MissingConditionOpening, MissingConditionClosure, MissingMemberDeclaration, MissingMemberTypeOrValueAssignment, MissingMemberType, MissingMemberName, MissingTupleOpening, MissingTupleClosure, UnexpectedEOF, MissingExpression };
}

#[derive(Clone)]
pub enum MessageType {
    IllegalCharacter,
    MissingCharacter,
    InvalidEscapeSequence,
    UnexpectedEOF,
    MissingExpression,
    MissingCodeblockClosure,
    MissingConditionOpening,
    MissingConditionClosure,
    MissingTupleOpening,
    MissingTupleClosure,
    MissingMemberDeclaration,
    MissingMemberName,
    MissingMemberType,
    MissingMemberTypeOrValueAssignment,
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
            MessageType::UnexpectedEOF                       => (true, "ES100E", "Unexpected end of file"),
            MessageType::MissingExpression                   => (true, "ES101E", "Missing expression"),
            MessageType::MissingCodeblockClosure             => (true, "ES102E", "Missing codeblock closure"),
            MessageType::MissingConditionOpening             => (true, "ES103E", "Missing condition opening"),
            MessageType::MissingConditionClosure             => (true, "ES104E", "Missing condition closure"),
            MessageType::MissingTupleOpening                 => (true, "ES105E", "Missing tuple opening"),
            MessageType::MissingTupleClosure                 => (true, "ES106E", "Missing tuple closure"),
            MessageType::MissingMemberDeclaration            => (true, "ES107E", "Missing member declaration"),
            MessageType::MissingMemberName                   => (true, "ES108E", "Missing member name"),
            MessageType::MissingMemberType                   => (true, "ES109E", "Missing member type"),
            MessageType::MissingMemberTypeOrValueAssignment  => (true, "ES110E", "Missing member type or value assignment"),
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
    pub start_pos: Position,
    pub end_pos: Position,
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
            White.paint(format!("{}:{}:{}", self.start_pos.file_name, self.start_pos.ln, self.start_pos.col)),
            Grey!().paint("]"),
        ];
        return ANSIStrings(strings).to_string();
    }

    fn line(&self) -> String {
        let line = self.start_pos.text.split('\n').nth(self.start_pos.ln).unwrap().to_string();
        let first = line[..self.start_pos.col].to_string();
        let err = line[self.start_pos.col..self.end_pos.col].to_string();
        let last = line[self.end_pos.col..].to_string();
        let strings: &[ANSIString<'static>] = &[
            Grey!().paint(first),
            self.light_color().bold().paint(err),
            Grey!().paint(last),
        ];
        return ANSIStrings(strings).to_string();
    }

    fn content(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let error_len = self.end_pos.col - self.start_pos.col;
        let middle: usize = ((error_len / 2) as f64 + 0.25) as usize;
        let mut underline = String::new();
        for _ in 0..self.start_pos.col {
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
        for _ in 0..self.start_pos.col+middle {
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
        let ln = self.start_pos.ln.to_string();
        let padding_width = ln.len() + 2;
        let mut output = String::new();
        
        let mut padding = String::new();
        for _ in 0..padding_width {
            padding += " ";
        }
        output += &Grey!().paint(padding.clone() + "╭─").to_string();
        output += &(self.file_name() + "\n");
        output += &Grey!().paint(padding.clone() + "│\n").to_string();
        output += &Grey!().paint(format!(" {} │ ", ln)).to_string();
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

    pub fn error(message_type: MessageType, details: &str, start_pos: Position, end_pos: Position) -> Self {
        return Self {
            details: details.to_string(),
            end_pos,
            message_type,
            start_pos
        }
    }
}