use crate::colors::*;
use crate::util::Position;
use ansi_term::ANSIString;
use ansi_term::ANSIStrings;
use ansi_term::Color::{self, Fixed, White};

pub mod details {
    #[macro_export]
    macro_rules! IllegalCharacter {
        ($chr: tt) => {
            format!("Character '{}' is not allowed", $chr).as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingCharacter {
        ($chr: tt, $op: tt) => {
            format!("Expected character '{}' to complete {}", $chr, $op).as_str()
        };
    }

    #[macro_export]
    macro_rules! UnexpectedEOF {
        () => {
            "Expected anything except End Of File"
        };
    }

    #[macro_export]
    macro_rules! MissingExpression {
        () => {
            "Expected any expression"
        };
    }

    #[macro_export]
    macro_rules! MissingCodeblockClosure {
        () => {
            "Expected '}' to close a codeblock"
        };
    }

    #[macro_export]
    macro_rules! MissingConditionOpening {
        () => {
            "Expected '(' to open a condition"
        };
    }

    #[macro_export]
    macro_rules! MissingConditionClosure {
        () => {
            "Expected ')' to close a condition"
        };
    }

    #[macro_export]
    macro_rules! MissingMemberDeclaration {
        ($name: tt, $member_type: tt) => {
            format!("Expected ':' to declare {} '{}'", $name, $member_type).as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingMemberTypeOrValueAssignment {
        ($name: tt, $member_type: tt) => {
            format!(
                "Expected {} type or '=' to assign '{}' a value",
                $member_type, $name
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingExpressionOrCodeblockOpening {
        () => {
            "Expected any expression or '{' to open a codeblock"
        };
    }

    #[macro_export]
    macro_rules! MissingMemberType {
        ($member_type: tt) => {
            format!("Expected {} type", $member_type).as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingMemberName {
        ($member_type: tt) => {
            format!("Expected {} name", $member_type).as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingTupleOpening {
        () => {
            "Expected '(' to open a tuple"
        };
    }

    #[macro_export]
    macro_rules! MissingTupleClosure {
        () => {
            "Expected ')' to close a tuple"
        };
    }

    #[macro_export]
    macro_rules! DivisionByZero {
        ($value: tt) => {
            format!("Attempt to divide {} by zero", $value).as_str()
        };
    }

    pub(crate) use {
        IllegalCharacter, MissingCharacter, MissingCodeblockClosure, MissingConditionClosure,
        MissingConditionOpening, MissingExpression, MissingExpressionOrCodeblockOpening,
        MissingMemberDeclaration, MissingMemberName, MissingMemberType,
        MissingMemberTypeOrValueAssignment, MissingTupleClosure, MissingTupleOpening,
        UnexpectedEOF,
    };
}

#[derive(PartialEq)]
pub enum MessageType {
    Warning,
    Error,
}
use self::MessageType::*;

#[derive(Clone)]
pub enum MessageVariant {
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
    MissingExpressionOrCodeblockOpening,
    DivisionByZero
}
impl MessageVariant {
    pub fn parameters(&self) -> (MessageType, &'static str, &'static str) {
        match self {
            /*
            ES 0 00 E
               | |  |
               | |  #- message type (E or W)
               | |
               | #---- message code (00 to 99)
               |
               #------ message process:
                        - Lexer       = 0
                        - Parser      = 1
                        - Compiler    = 2
                        - Interpreter = 3
            */
            MessageVariant::IllegalCharacter => (Error, "ES000E", "Illegal character"),
            MessageVariant::MissingCharacter => (Error, "ES001E", "Missing character"),
            MessageVariant::InvalidEscapeSequence => (Error, "ES002E", "Invalid escape sequence"),
            MessageVariant::UnexpectedEOF => (Error, "ES100E", "Unexpected end of file"),
            MessageVariant::MissingExpression => (Error, "ES101E", "Missing expression"),
            MessageVariant::MissingCodeblockClosure => {
                (Error, "ES102E", "Missing codeblock closure")
            }
            MessageVariant::MissingConditionOpening => {
                (Error, "ES103E", "Missing condition opening")
            }
            MessageVariant::MissingConditionClosure => {
                (Error, "ES104E", "Missing condition closure")
            }
            MessageVariant::MissingTupleOpening => (Error, "ES105E", "Missing tuple opening"),
            MessageVariant::MissingTupleClosure => (Error, "ES106E", "Missing tuple closure"),
            MessageVariant::MissingMemberDeclaration => {
                (Error, "ES107E", "Missing member declaration")
            }
            MessageVariant::MissingMemberName => (Error, "ES108E", "Missing member name"),
            MessageVariant::MissingMemberType => (Error, "ES109E", "Missing member type"),
            MessageVariant::MissingMemberTypeOrValueAssignment => {
                (Error, "ES110E", "Missing member type or value assignment")
            }
            MessageVariant::MissingExpressionOrCodeblockOpening => {
                (Error, "ES111E", "Missing expression or codeblock opening")
            }
            MessageVariant::DivisionByZero => {
                (Error, "ES300E", "Division by zero")
            },
        }
    }

    pub fn is_error(&self) -> bool {
        self.parameters().0 == Error
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
    pub message_type: MessageVariant,
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
            return self
                .color()
                .bold()
                .paint(format!("Error {}: ", self.message_type.code()))
                .to_string();
        }
        panic!();
    }

    fn file_name(&self) -> String {
        let strings: &[ANSIString<'static>] = &[
            Grey!().paint("["),
            White.paint(format!(
                "{}:{}:{}",
                self.start_pos.file_name,
                self.start_pos.ln + 1,
                self.start_pos.col + 1
            )),
            Grey!().paint("]"),
        ];
        return ANSIStrings(strings).to_string();
    }

    fn line(&self) -> String {
        let line = self
            .start_pos
            .text
            .split('\n')
            .nth(self.start_pos.ln)
            .unwrap()
            .to_string();
        let chars: Vec<char> = line.chars().collect();
        let first: String = chars[..self.start_pos.col].iter().collect();
        let mut end = self.end_pos.col;
        if self.end_pos.col > chars.len() {
            end = chars.len();
        }
        println!("{}", self.file_name());
        let err: String = chars[self.start_pos.col..end].iter().collect();
        let last: String = chars[end..].iter().collect();
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
        for _ in middle + 1..error_len {
            underline += "─";
        }
        result.push(self.light_color().paint(underline).to_string());
        let mut error_line = String::new();
        for _ in 0..self.start_pos.col + middle {
            error_line += " ";
        }
        error_line += "╰─";
        for _ in 0..error_len - middle {
            error_line += "─";
        }
        error_line += " ";
        result.push(self.light_color().paint(error_line).to_string() + &self.details);
        result
    }

    fn body(&self) -> String {
        let ln = (self.start_pos.ln + 1).to_string();
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
        self.header() + self.message_type.name() + "\n" + &self.body()
    }

    pub fn error(
        message_type: MessageVariant,
        details: &str,
        start_pos: Position,
        end_pos: Position,
    ) -> Self {
        return Self {
            details: details.to_string(),
            end_pos,
            message_type,
            start_pos,
        };
    }
}
