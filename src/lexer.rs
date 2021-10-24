use crate::util::*;
use std::fmt::Debug;

pub enum Result<T> {
    Ok(T),
    Err(Error),
}

type ResultWrapper<T> = (Result<T>, Vec<Warning>);

use self::Result::*;

#[allow(nonstandard_style)]
fn IllegalCharacterError(details: String, start_pos: Position, end_pos: Position) -> Error {
    return Error {
        name: String::from("Illegal Character"),
        details,
        start_pos,
        end_pos,
    };
}

#[allow(nonstandard_style)]
fn ExpectedCharacterError(details: String, start_pos: Position, end_pos: Position) -> Error {
    return Error {
        name: String::from("Expected Character"),
        details,
        start_pos,
        end_pos,
    };
}

#[allow(nonstandard_style)]
fn InvalidEscapeSequenceError(details: String, start_pos: Position, end_pos: Position) -> Error {
    return Error {
        name: String::from("Invalid Escape Sequence"),
        details,
        start_pos,
        end_pos,
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    And,
    Or,
    Not,
    If,
    Else,
    While,
    Function,
}

impl Keyword {
    fn parse(string: &str) -> Option<Keyword> {
        match string {
            "and" => Some(Keyword::And),
            "or" => Some(Keyword::Or),
            "not" => Some(Keyword::Not),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "function" => Some(Keyword::Function),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum TokenType {
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
    Colon,
    String(String),
    NL,
    Placeholder,
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenType::Int(value) => write!(f, "Int({:?})", value),
            TokenType::Float(value) => write!(f, "Float({:?})", value),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Mul => write!(f, "Mul"),
            TokenType::Div => write!(f, "Div"),
            TokenType::LParen => write!(f, "LParen"),
            TokenType::RParen => write!(f, "RParen"),
            TokenType::LCParen => write!(f, "LCParen"),
            TokenType::RCParen => write!(f, "RCParen"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Keyword(value) => write!(f, "Keyword({:?})", value),
            TokenType::ID(value) => write!(f, "ID({:?})", value),
            TokenType::EQ => write!(f, "EQ"),
            TokenType::EE => write!(f, "EE"),
            TokenType::NE => write!(f, "NE"),
            TokenType::LT => write!(f, "LT"),
            TokenType::GT => write!(f, "GT"),
            TokenType::LTE => write!(f, "LTE"),
            TokenType::GTE => write!(f, "GTE"),
            TokenType::Not => write!(f, "Not"),
            TokenType::And => write!(f, "And"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::String(value) => write!(f, "String({:?})", value),
            TokenType::NL => write!(f, "NL"),
            TokenType::Placeholder => write!(f, "Placeholder"),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start_pos: Position,
    pub end_pos: Position,
}

impl Token {
    pub fn new(token_type: TokenType, start_pos: &Position, end_pos: &Position) -> Token {
        return Token {
            token_type,
            start_pos: start_pos.clone(),
            end_pos: end_pos.clone(),
        };
    }

    pub fn single_char(token_type: TokenType, pos: &Position) -> Token {
        return Token::new(token_type, pos, pos);
    }

    pub fn is(&self, token_type: TokenType) -> bool {
        if token_type == self.token_type {
            return true;
        }
        return false;
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Token:{:?}", self.token_type);
    }
}

fn get_current_char(pos: &Position) -> Option<char> {
    if pos.index < pos.text.len() {
        return Some(pos.text.chars().nth(pos.index).unwrap());
    } else {
        return None;
    }
}

fn advance(pos: &mut Position) -> Option<&Position> {
    if let Some(current_char) = get_current_char(pos) {
        return Some(pos.advance(current_char));
    }
    return None;
}

fn make_number(pos: &mut Position) -> Token {
    let mut token_string = String::new();
    let mut dot_count: u8 = 0;
    let start_pos = pos.clone();

    while let Some(current_char) = get_current_char(&pos) {
        if !((String::from(".") + DIGITS).contains(current_char)) {
            break;
        }
        if current_char == '.' {
            if dot_count == 1 {
                break;
            }
            dot_count += 1;
        }
        token_string.push(current_char);
        advance(pos);
    }

    if dot_count == 0 {
        return Token::new(
            TokenType::Int(token_string.parse().unwrap()),
            &start_pos,
            pos,
        );
    }
    return Token::new(
        TokenType::Float(token_string.parse().unwrap()),
        &start_pos,
        pos,
    );
}

fn make_identifier(pos: &mut Position) -> Token {
    let mut id_string = String::new();
    let start_pos = pos.clone();

    while let Some(current_char) = get_current_char(&pos) {
        if !(VALID_CHARS.contains(current_char)) {
            break;
        }
        id_string.push(current_char);
        advance(pos);
    }

    match Keyword::parse(&id_string) {
        Some(keyword) => Token::new(TokenType::Keyword(keyword), &start_pos, pos),
        None => Token::new(TokenType::ID(id_string.clone()), &start_pos, pos),
    }
}

fn make_combo_token(
    pos: &mut Position,
    default: TokenType,
    second: char,
    second_type: TokenType,
) -> Token {
    let mut token_type = default;
    let start_pos = pos.clone();
    advance(pos);
    if let Some(current_char) = get_current_char(pos) {
        if current_char == second {
            advance(pos);
            token_type = second_type;
        }
    }
    return Token::new(token_type, &start_pos, pos);
}

fn make_equals(pos: &mut Position) -> Token {
    return make_combo_token(pos, TokenType::EQ, '=', TokenType::EE);
}

fn make_not(pos: &mut Position) -> Token {
    return make_combo_token(pos, TokenType::Not, '=', TokenType::NE);
}

fn make_less_than(pos: &mut Position) -> Token {
    return make_combo_token(pos, TokenType::LT, '=', TokenType::LTE);
}

fn make_greater_than(pos: &mut Position) -> Token {
    return make_combo_token(pos, TokenType::GT, '=', TokenType::GTE);
}

fn make_and(pos: &mut Position) -> Result<Token> {
    let mut start_pos = pos.clone();
    let token = make_combo_token(pos, TokenType::Placeholder, '&', TokenType::And);
    match token.token_type {
        TokenType::And => Ok(token),
        _ => {
            advance(&mut start_pos);
            let mut end_pos = pos.clone();
            advance(&mut end_pos);
            return Err(ExpectedCharacterError(
                String::from("'&'"),
                start_pos,
                end_pos,
            ));
        }
    }
}

fn make_or(pos: &mut Position) -> Result<Token> {
    let mut start_pos = pos.clone();
    let token = make_combo_token(pos, TokenType::Placeholder, '|', TokenType::Or);
    match token.token_type {
        TokenType::Or => Ok(token),
        _ => {
            advance(&mut start_pos);
            let mut end_pos = pos.clone();
            advance(&mut end_pos);
            return Err(ExpectedCharacterError(
                String::from("'|'"),
                start_pos,
                end_pos,
            ));
        }
    }
}

pub fn make_tokens(file_name: &str, text: &str) -> ResultWrapper<Vec<Token>> {
    let mut warnings: Vec<Warning> = Vec::new();
    let mut pos = Position::new(file_name, text);
    warnings.push(Warning {
        details: String::from("Details"),
        name: String::from("Test"),
        start_pos: pos.clone(),
        end_pos: pos.clone().advance('.').to_owned(),
    });
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(current_char) = get_current_char(&pos) {
        if " \t".contains(current_char) {
            advance(&mut pos);
        } else if DIGITS.contains(current_char) {
            tokens.push(make_number(&mut pos));
        } else if ['\n', ';'].contains(&current_char) {
            tokens.push(Token::single_char(TokenType::NL, &pos));
            advance(&mut pos);
        } else if VALID_CHARS.contains(current_char) {
            tokens.push(make_identifier(&mut pos));
        } else if current_char == '+' {
            tokens.push(Token::single_char(TokenType::Plus, &pos));
            advance(&mut pos);
        } else if current_char == '-' {
            tokens.push(Token::single_char(TokenType::Minus, &pos));
            advance(&mut pos);
        } else if current_char == '*' {
            tokens.push(Token::single_char(TokenType::Mul, &pos));
            advance(&mut pos);
        } else if current_char == '/' {
            tokens.push(Token::single_char(TokenType::Div, &pos));
            advance(&mut pos);
        } else if current_char == '(' {
            tokens.push(Token::single_char(TokenType::LParen, &pos));
            advance(&mut pos);
        } else if current_char == ')' {
            tokens.push(Token::single_char(TokenType::RParen, &pos));
            advance(&mut pos);
        } else if current_char == '{' {
            tokens.push(Token::single_char(TokenType::LCParen, &pos));
            advance(&mut pos);
        } else if current_char == '}' {
            tokens.push(Token::single_char(TokenType::RCParen, &pos));
            advance(&mut pos);
        } else if current_char == ',' {
            tokens.push(Token::single_char(TokenType::Comma, &pos));
            advance(&mut pos);
        } else if current_char == ':' {
            tokens.push(Token::single_char(TokenType::Colon, &pos));
            advance(&mut pos);
        } else if current_char == '=' {
            tokens.push(make_equals(&mut pos));
        } else if current_char == '!' {
            tokens.push(make_not(&mut pos));
        } else if current_char == '<' {
            tokens.push(make_less_than(&mut pos));
        } else if current_char == '>' {
            tokens.push(make_greater_than(&mut pos));
        } else if current_char == '&' {
            match make_and(&mut pos) {
                Ok(token) => {
                    tokens.push(token);
                }
                Err(error) => {
                    return (Err(error), warnings);
                }
            }
        } else if current_char == '|' {
            match make_or(&mut pos) {
                Ok(token) => {
                    tokens.push(token);
                }
                Err(error) => {
                    return (Err(error), warnings);
                }
            }
        } else if current_char == '"' {
            //tokens.push(make_string(&mut pos));
            todo!();
        } else {
            let start_pos = pos.clone();
            advance(&mut pos);
            return (
                Err(IllegalCharacterError(
                    format!("'{}'", current_char),
                    start_pos,
                    pos.clone(),
                )),
                warnings,
            );
        }
    }
    tokens.push(Token::single_char(TokenType::EOF, &pos));
    return (Ok(tokens), warnings);
}
