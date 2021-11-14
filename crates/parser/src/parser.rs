use logos::Lexer;
use utilities::{cursor::Cursor, message::{Message, MessageType::*, details}};

use crate::{ast::Expression, token::Token};

struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    current: Token,
    peek: Token,
    cursor: Cursor
}
impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a, Token>, file_name: &str, text: &str) -> Self {
        Self {
            lexer,
            current: Token::EoF,
            peek: Token::EoF,
            cursor: Cursor::new(file_name, text)
        }
    }

    fn init(&mut self) {
        self.advance();
        self.advance();
    }

    fn advance(&mut self) {
        self.current = self.peek.clone();
        self.peek = if let Some(token) = self.lexer.next() { token.clone() } else { Token::EoF };
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
            
        ));
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