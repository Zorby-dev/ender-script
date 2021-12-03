use logos::{Lexer, Logos};
use utilities::{
    cursor::Cursor,
    message::{details, Message, MessageType::*},
};

use crate::{
    ast::{Expression, Parameter, Type},
    token::{self, Token},
};

struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    current: Token,
    slice: String,
    peek: Token,
    cursor: Cursor,
    peek_cursor: Cursor,
    peek_slice: String,
}
impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a, Token>, file_name: &str, text: &str) -> Self {
        Self {
            lexer,
            current: Token::EoF,
            slice: String::new(),
            peek: Token::EoF,
            cursor: Cursor::new(file_name, text),
            peek_cursor: Cursor::new(file_name, text),
            peek_slice: String::new(),
        }
    }

    fn init(&mut self) {
        self.advance();
        self.advance();
    }

    fn advance(&mut self) {
        self.current = self.peek.clone();
        self.cursor = self.peek_cursor.clone();
        self.slice = self.peek_slice.clone();
        self.peek = if let Some(token) = self.lexer.next() {
            token.clone()
        } else {
            Token::EoF
        };
        self.peek_cursor.update(self.lexer.span());
        self.peek_slice = self.lexer.slice().to_string();
    }

    fn parse_expression(&mut self) -> Result<Expression, Message> {
        match self.current {
            Token::Function => self.parse_function(),
            Token::Let => self.parse_let(),
            Token::Integer => Ok(Expression::Integer(
                token::to_i64(&self.slice),
                self.cursor.clone(),
            )),
            Token::Identifier => Ok(Expression::VariableAccess(
                self.slice.clone(),
                self.cursor.clone(),
            )),
            Token::Error => Err(Message::error(
                IllegalCharacter,
                details::IllegalCharacter!(self.slice),
                self.cursor.clone(),
            )),
            _ => {
                println!("{:?}", self.current);
                todo!();
            }
        }
    }

    fn parse_let(&mut self) -> Result<Expression, Message> {
        let start_pos = self.cursor.start.clone();

        self.advance();

        let name = self.expect_identifier_and_advance(Message::error(
            MissingMemberName,
            details::MissingMemberName!("variable"),
            self.cursor.clone(),
        ))?;

        let variable_type: Option<Type>;

        match self.current {
            Token::Colon => {
                self.advance();
                variable_type = Some(Type {
                    name: self.expect_identifier_and_advance(Message::error(
                        MissingMemberType,
                        details::MissingMemberType!("variable"),
                        self.cursor.clone(),
                    ))?,
                });
            }
            Token::Assign => variable_type = None,
            _ => {
                return Err(Message::error(
                    MissingMemberType,
                    details::MissingMemberTypeOrValueAssignment!("variable"),
                    self.cursor.clone(),
                ))
            }
        }

        let value: Option<Expression>;

        if let Some(_) = self.suspect_and_advance(Token::Assign) {
            value = Some(self.parse_expression()?);
        } else {
            value = None;
        }

        Ok(Expression::VariableDeclaration {
            name,
            variable_type,
            value: if let Some(expr) = value {
                Some(Box::new(expr))
            } else {
                None
            },
            cursor: Cursor {
                start: start_pos,
                end: self.cursor.end.clone(),
                file_name: self.cursor.file_name.clone(),
                text: self.cursor.text.clone(),
            },
        })
    }

    fn parse_function(&mut self) -> Result<Expression, Message> {
        let start_pos = self.cursor.start.clone();

        self.advance();

        // function name

        let name = self.expect_identifier_and_advance(Message::error(
            MissingMemberName,
            details::MissingMemberName!("function"),
            self.cursor.clone(),
        ))?;

        // parameters

        self.expect_and_advance(
            Token::LeftParen,
            Message::error(
                MissingCase,
                details::MissingCase!("parameter declaration"),
                self.cursor.clone(),
            ),
        )?;

        let mut parameters: Vec<Parameter> = Vec::new();

        // SECTION - TODO: convert to loop & match
        // LINK crates/parser/src/parser.rs:142
        // !SECTION - TODO: convert to loop & match
        while let Some(identifier) = self.suspect_identifier_and_advance() {
            self.expect_and_advance(
                Token::Colon,
                Message::error(
                    MissingMemberType,
                    details::MissingMemberType!("parameter"),
                    self.cursor.clone(),
                ),
            )?;

            let typ = self.expect_identifier_and_advance(Message::error(
                MissingMemberType,
                details::MissingMemberType!("parameter"),
                self.cursor.clone(),
            ))?;

            parameters.push(Parameter {
                name: identifier,
                type_: Type { name: typ },
            });

            match self.current {
                Token::RightParen => {
                    self.advance();
                    break;
                }
                Token::Comma => {
                    self.advance();
                    continue;
                }
                _ => {
                    return Err(Message::error(
                        MissingCaseSeparatorOrClosure,
                        details::MissingCaseSeparatorOrClosure!(),
                        self.cursor.clone(),
                    ))
                }
            }
        }

        // return type

        self.expect_and_advance(
            Token::Colon,
            Message::error(
                MissingMemberType,
                details::MissingMemberTypeColon!("return"),
                self.cursor.clone(),
            ),
        )?;

        let return_type = Type {
            name: self.expect_identifier_and_advance(Message::error(
                MissingMemberType,
                details::MissingMemberType!("return"),
                self.cursor.clone(),
            ))?,
        };

        // block

        self.expect_and_advance(
            Token::LeftBrace,
            Message::error(MissingBlock, details::MissingBlock!(), self.cursor.clone()),
        )?;

        if self.current == Token::RightBrace {
            return Ok(Expression::FunctionDeclaration {
                name,
                parameters,
                return_type,
                body: Vec::new(),
                cursor: Cursor {
                    start: start_pos,
                    end: self.cursor.end.clone(),
                    file_name: self.cursor.file_name.clone(),
                    text: self.cursor.text.clone(),
                },
            });
        }

        let mut body: Vec<Expression> = Vec::new();

        loop {
            body.push(self.parse_expression()?);
            self.advance();
            match self.current {
                Token::RightBrace => break,
                Token::NewLine => {
                    self.advance();
                    continue;
                }
                _ => {
                    return Err(Message::error(
                        MissingBlockSeparatorOrClosure,
                        details::MissingBlockSeparatorOrClosure!(),
                        self.cursor.clone(),
                    ))
                }
            }
        }

        // construction

        Ok(Expression::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
            cursor: Cursor {
                start: start_pos,
                end: self.cursor.end.clone(),
                file_name: self.cursor.file_name.clone(),
                text: self.cursor.text.clone(),
            },
        })
    }

    fn suspect(&self, token: Token) -> Option<Token> {
        if self.current == token {
            Some(self.current.clone())
        } else {
            None
        }
    }

    fn suspect_identifier(&self) -> Option<String> {
        self.suspect(Token::Identifier)?;
        Some(self.slice.to_string())
    }

    fn suspect_and_advance(&mut self, token: Token) -> Option<Token> {
        let out = self.suspect(token)?;
        self.advance();
        Some(out)
    }

    fn suspect_identifier_and_advance(&mut self) -> Option<String> {
        let out = self.suspect_identifier();
        self.advance();
        out
    }

    fn expect(&self, token: Token, error: Message) -> Result<Token, Message> {
        if self.current == token {
            Ok(self.current.clone())
        } else {
            Err(error)
        }
    }

    fn expect_identifier(&self, error: Message) -> Result<String, Message> {
        self.expect(Token::Identifier, error)?;
        Ok(self.slice.to_string())
    }

    fn expect_and_advance(&mut self, token: Token, error: Message) -> Result<Token, Message> {
        let out = self.expect(token, error);
        self.advance();
        out
    }

    fn expect_identifier_and_advance(&mut self, error: Message) -> Result<String, Message> {
        let out = self.expect_identifier(error);
        self.advance();
        out
    }

    fn parse(&mut self) -> Result<Vec<Expression>, Message> {
        let mut out: Vec<Expression> = Vec::new();
        while self.current != Token::EoF {
            out.push(self.parse_expression()?);
            self.advance();
            match self.current {
                Token::EoF => break,
                Token::NewLine => self.advance(),
                _ => {
                    return Err(Message::error(
                        MissingBlockSeparatorOrClosure,
                        details::MissingBlockSeparatorOrClosure!(),
                        self.cursor.clone(),
                    ))
                }
            }
        }
        Ok(out)
    }
}

pub fn parse(text: impl ToString) -> Result<Vec<Expression>, Message> {
    let text_string = text.to_string();
    let text_str = text_string.as_str();
    let mut parser = Parser::new(token::Token::lexer(text_str), "<stdin>", text_str);
    parser.init();
    parser.parse()
}
