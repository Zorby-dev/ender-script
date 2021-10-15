use crate::util::*;

fn IllegalCharacterError(details: String, start_pos: Position, end_pos: Position) -> Error {
    return Error { name: String::from("Illegal Character"), details, start_pos, end_pos }
}

fn ExpectedCharacterError(details: String, start_pos: Position, end_pos: Position) -> Error {
    return Error { name: String::from("Expected Character"), details, start_pos, end_pos }
}

fn InvalidEscapeSequenceError(details: String, start_pos: Position, end_pos: Position) -> Error {
    return Error { name: String::from("Invalid Escape Sequence"), details, start_pos, end_pos }
}

pub enum Keyword {
	Int,
	And,
	Or,
	Not,
	If,
	Else,
	While,
	Function,
}

pub enum Token {
	Int(isize),
	Float(f64),
	Plus,
	Minus,
	Mul,
	Div,
	LParen,
	RParen,
	LCParen,
	RCParen,
	EOF,
	Keyword(Keyword),
	ID(String),
	EQ,
	EE,
	NE,
	LT,
	GT,
	LTE,
	GTE,
	Not,
	And,
	Or,
	Comma,
	String(String),
	NL,
}

pub struct Lexer {

}

impl Lexer {
    pub fn make_tokens(&self) {
        
    }
}