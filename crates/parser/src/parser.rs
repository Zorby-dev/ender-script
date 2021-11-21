use std::clone;

use logos::{Lexer, internal::LexerInternal};
use utilities::{cursor::Cursor, message::{Message, MessageType::*, details}};

use crate::{ast::{Expression, Parameter, Type}, token::Token};

struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    current: Token,
    peek: Token,
    cursor: Cursor,
    peek_cursor: Cursor
}
impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a, Token>, file_name: &str, text: &str) -> Self {
        Self {
            lexer,
            current: Token::EoF,
            peek: Token::EoF,
            cursor: Cursor::new(file_name, text),
            peek_cursor: Cursor::new(file_name, text)
        }
    }

    fn init(&mut self) {
        self.advance();
        self.advance();
    }

    fn advance(&mut self) {
        self.current = self.peek.clone();
        self.cursor = self.peek_cursor.clone();
        self.peek = if let Some(token) = self.lexer.next() { token.clone() } else { Token::EoF };
        self.peek_cursor.update(self.lexer.span());
    }

    fn next(&self) -> Result<Option<Expression>, Message> {
        if self.current == Token::EoF {
            Ok(None)
        } else {
            Ok(Some(self.parse_statement()?))
        }
    }

    fn parse_statement(&self) -> Result<Expression, Message> {
        match self.current {
            Token::Function => self.parse_function(),
            Token::Identifier => self.parse_identifier()
        }
    }

    fn parse_function(&self) -> Result<Expression, Message> {
        self.advance();
        let name = self.expect_identifier_and_advance(Message::error(
            MissingMemberName,
            details::MissingMemberName!("function"),
            self.cursor.clone()
        ))?;
        self.expect_and_advance(Token::LeftParen, Message::error(
            MissingCaseOpening,
            details::MissingCaseOpening!("parameter declaration"),
            self.cursor.clone()
        ))?;
        let mut parameters: Vec<Parameter> = Vec::new();
        while let Some(identifier) = self.suspect_identifier_and_advance() {
            self.expect_and_advance(Token::Colon, Message::error(
                MissingMemberType,
                details::MissingMemberType!("parameter"),
                self.cursor.clone()
            ))?;
            let typ = self.expect_identifier_and_advance(Message::error(
                MissingMemberType,
                details::MissingMemberType!("parameter"),
                self.cursor.clone()
            ))?;
            parameters.push(Parameter {
                name: identifier,
                type_: Type { name: typ }
            });
            match self.current {
                Token::LeftParen => break,
                Token::Comma => continue,
                _ => return Err(Message::error(
                    MissingSeparatorOrCaseClosure,
                    details::MissingSeparatorOrCaseClosure!(),
                    self.cursor.clone()
                ))
            }
        }
        self.expect_and_advance(Token::Colon, Message::error(
            MissingMemberType,
            details::MissingMemberType!("return"),
            self.cursor.clone()
        ))?;
        let typ = self.expect_identifier_and_advance(Message::error(
            MissingMemberType,
            details::MissingMemberType!("return"),
            self.cursor.clone()
        ))?;

    }

    fn suspect(&self, token: Token) -> Option<&Token> {
        if self.current == token {
            Some(&self.current)
        } else {
            None
        }
    }

    fn suspect_identifier(&self) -> Option<String> {
        self.suspect(Token::Identifier)?;
        Some(self.lexer.slice().to_string())
    }

    fn suspect_and_advance(&mut self, token: Token) -> Option<&Token> {
        let out = self.suspect(token);
        self.advance();
        out
    }

    fn suspect_identifier_and_advance(&mut self) -> Option<String> {
        let out = self.suspect_identifier();
        self.advance();
        out
    }

    fn expect(&self, token: Token, error: Message) -> Result<&Token, Message> {
        if self.current == token {
            Ok(&self.current)
        } else {
            Err(error)
        }
    }

    fn expect_identifier(&self, error: Message) -> Result<String, Message> {
        self.expect(Token::Identifier, error)?;
        Ok(self.lexer.slice().to_string())
    }

    fn expect_and_advance(&mut self, token: Token, error: Message) -> Result<&Token, Message> {
        let out = self.expect(token, error);
        self.advance();
        out
    }

    fn expect_identifier_and_advance(&mut self, error: Message) -> Result<String, Message> {
        let out = self.expect_identifier(error);
        self.advance();
        out
    }
}