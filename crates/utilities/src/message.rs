use ansi_term::ANSIString;
use ansi_term::ANSIStrings;
use ansi_term::Color::{self, Fixed, White};
use std::cmp::{max, min};

use crate::color::*;
use crate::cursor::Cursor;

pub mod details {

    #[macro_export]
    macro_rules! IllegalCharacter {
        ($chr: expr) => {
            format!(
                "Character '{}' is not allowed",
                $chr
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingCharacter {
        ($chr: tt) => {
            format!(
                "Character '{}' is missing",
                $chr
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingExpression {
        () => {
            "Expected any expression"
        };
    }

    #[macro_export]
    macro_rules! MissingSpecificExpression {
        ($spec: tt) => {
            format!(
                "Expected {} expression",
                $spec
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingMemberDeclaration {
        ($name: tt, $member_type: tt) => {
            format!(
                "Expected ':' to declare {} '{}'",
                $name, $member_type
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingMemberTypeOrValueAssignment {
        ($member_type: tt) => {
            format!(
                "Expected ':' to declare {} type or '=' to assign a value",
                $member_type
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingMemberType {
        ($member_type: tt) => {
            format!(
                "Expected {} type",
                $member_type
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingMemberTypeColon {
        ($member_type: tt) => {
            format!(
                "Expected ':' to declare {} type",
                $member_type
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingMemberName {
        ($member_type: tt) => {
            format!(
                "Expected {} name",
                $member_type
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingCase {
        ($case_type: tt) => {
            format!(
                "Expected '(' to open {}",
                $case_type
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MissingCaseClosure {
        () => {
            "Expected ')'"
        };
    }

    #[macro_export]
    macro_rules! MissingCaseSeparatorOrClosure {
        () => {
            "Expected ',' or ')'"
        };
    }

    #[macro_export]
    macro_rules! MissingBlock {
        () => {
            "Expected '{' to open a block"
        };
    }

    #[macro_export]
    macro_rules! MissingBlockSeparatorOrClosure {
        () => {
            "Expected a new line or '}'"
        };
    }

    #[macro_export]
    macro_rules! UnknownType {
        ($type_name: expr) => {
            format!(
                "Type \"{}\" does not exist in this scope",
                $type_name
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! IntegerBoundsExceeded {
        ($byte_limit: tt) => {
            format!(
                "Provided integer exceeds the {} byte limit",
                $byte_limit
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! TypeMismatch {
        ($expected: tt, $got: expr) => {
            format!(
                "Expected value of type {}, got {}",
                Cyan!().paint($expected),
                Cyan!().paint($got)
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! UnknownMember {
        ($member_type: tt, $member_name: expr) => {
            format!(
                "{} '{}' is not declared in this scope",
                $member_type, $member_name
            )
            .as_str()
        };
    }

    #[macro_export]
    macro_rules! MemberRedeclaration {
        ($member_type: tt, $member_name: expr) => {
            format!(
                "{} '{}' had already been declared",
                $member_type, $member_name
            )
            .as_str()
        };
    }

    pub use {
        IllegalCharacter, IntegerBoundsExceeded, MemberRedeclaration, MissingBlock,
        MissingBlockSeparatorOrClosure, MissingCase, MissingCaseClosure,
        MissingCaseSeparatorOrClosure, MissingCharacter, MissingExpression,
        MissingMemberDeclaration, MissingMemberName, MissingMemberType, MissingMemberTypeColon,
        MissingMemberTypeOrValueAssignment, TypeMismatch, UnknownMember, UnknownType, MissingSpecificExpression
    };
}

#[derive(Clone)]
pub enum MessageType {
    IllegalCharacter,
    MissingExpression,
    MissingMemberDeclaration,
    MissingMemberName,
    MissingMemberType,
    MissingMemberTypeOrValueAssignment,
    MissingCase,
    MissingCaseClosure,
    MissingCaseSeparatorOrClosure,
    MissingBlock,
    MissingBlockClosure,
    MissingBlockSeparatorOrClosure,
    UnknownType,
    IntegerBoundsExceeded,
    TypeMismatch,
    UnknownMember,
    MemberRedeclaration,
}
impl MessageType {
    pub fn parameters(
        &self,
    ) -> (
        bool,
        &'static str,
        &'static str,
    ) {
        match self {
            /*
            ES 0 00 E
               | |  |
               | |  #- message type (E or W)
               | |
               | #---- message code (00 to 99)
               |
               #------ message process:
                        - Parser   = 0
                        - Compiler = 1
                        - Builder  = 2
            */
            | MessageType::IllegalCharacter => (
                true,
                "ES000E",
                "Illegal character",
            ),
            | MessageType::MissingExpression => (
                true,
                "ES001E",
                "Missing expression",
            ),
            | MessageType::MissingMemberDeclaration => (
                true,
                "ES002E",
                "Missing member declaration",
            ),
            | MessageType::MissingMemberName => (
                true,
                "ES003E",
                "Missing member name",
            ),
            | MessageType::MissingMemberType => (
                true,
                "ES004E",
                "Missing member type",
            ),
            | MessageType::MissingMemberTypeOrValueAssignment => (
                true,
                "ES005E",
                "Missing member type or value assignment",
            ),
            | MessageType::MissingCase => (
                true,
                "ES006E",
                "Missing case",
            ),
            | MessageType::MissingCaseClosure => (
                true,
                "ES007E",
                "Missing case closure",
            ),
            | MessageType::MissingCaseSeparatorOrClosure => (
                true,
                "ES008E",
                "Missing case separator or closure",
            ),
            | MessageType::MissingBlock => (
                true,
                "ES009E",
                "Missing block",
            ),
            | MessageType::MissingBlockClosure => (
                true,
                "ES010E",
                "Missing block closure",
            ),
            | MessageType::MissingBlockSeparatorOrClosure => (
                true,
                "ES011E",
                "Missing block separator or closure",
            ),
            | MessageType::UnknownType => (
                true,
                "ES100E",
                "Unknown type",
            ),
            | MessageType::IntegerBoundsExceeded => (
                true,
                "ES101E",
                "Integer bounds exceeded",
            ),
            | MessageType::TypeMismatch => (
                true,
                "ES102E",
                "Type mismatch",
            ),
            | MessageType::UnknownMember => (
                true,
                "ES103E",
                "Unknown member",
            ),
            | MessageType::MemberRedeclaration => (
                true,
                "ES104E",
                "Member redeclaration",
            ),
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
    pub cursor: Cursor,
}

impl Message {
    fn color(&self) -> Color {
        match self.message_type.is_error() {
            | true => Red!(),
            | false => todo!(),
        }
    }

    fn light_color(&self) -> Color {
        match self.message_type.is_error() {
            | true => LightRed!(),
            | false => todo!(),
        }
    }

    fn header(&self) -> String {
        if self.message_type.is_error() {
            return self
                .color()
                .bold()
                .paint(format!(
                    "Error {}: ",
                    self.message_type.code()
                ))
                .to_string();
        }
        panic!();
    }

    fn file_name(&self) -> String {
        let strings: &[ANSIString<'static>] = &[
            Grey!().paint("["),
            White.paint(format!(
                "{}:{}:{}",
                self.cursor.file_name, self.cursor.start.line_num, self.cursor.start.col
            )),
            Grey!().paint("]"),
        ];
        return ANSIStrings(strings).to_string();
    }

    fn line(&self) -> String {
        let line = self
            .cursor
            .text
            .split('\n')
            .nth(self.cursor.start.line_num)
            .unwrap()
            .to_string();
        let first: String = line.chars().take(self.cursor.start.col).collect();
        let err: String = line
            .chars()
            .skip(self.cursor.start.col)
            .take(self.cursor.end.col - self.cursor.start.col)
            .collect();
        let last: String = line.chars().skip(self.cursor.end.col).collect();
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
        for _ in middle + 1..error_len {
            underline += "─";
        }
        result.push(self.light_color().paint(underline).to_string());
        let mut error_line = String::new();
        for _ in 0..self.cursor.start.col + middle {
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
        output += &Grey!()
            .paint(format!(
                " {} │ ",
                line_num
            ))
            .to_string();
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

    pub fn error(message_type: MessageType, details: &str, cursor: Cursor) -> Self {
        return Self {
            details: details.to_string(),
            message_type,
            cursor,
        };
    }
}
impl ToString for Message {
    fn to_string(&self) -> String {
        self.header() + self.message_type.name() + "\n" + &self.body()
    }
}
