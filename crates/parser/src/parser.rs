use logos::{Lexer, Logos};
use utilities::{
    cursor::Cursor,
    message::{details, Message, MessageType::*},
    MissingExpression,
};

use crate::{
    ast::{Expression, Parameter, Type, Argument},
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
    // MISC

    fn new(lexer: Lexer<'a, Token>, file_name: &str, text: &str) -> Self {
        Self {
            lexer,
            current: Token::EoF,
            slice: String::new(),
            peek: Token::EoF,
            cursor: Cursor::new(
                file_name, text,
            ),
            peek_cursor: Cursor::new(
                file_name, text,
            ),
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

    fn suspect(&self, token: Token) -> Result<Option<Token>, Message> {
        if self.current == token {
            Ok(Some(
                self.current.clone(),
            ))
        } else if self.current == Token::Error {
            Err(
                Message::error(
                    IllegalCharacter,
                    details::IllegalCharacter!(&self.slice),
                    self.cursor.clone(),
                ),
            )
        } else {
            Ok(None)
        }
    }

    fn suspect_identifier(&self) -> Result<Option<String>, Message> {
        let ident = self.suspect(Token::Identifier)?;
        if let None = ident {
            return Ok(None);
        }
        Ok(Some(
            self.slice.to_string(),
        ))
    }

    fn suspect_and_advance(&mut self, token: Token) -> Result<Option<Token>, Message> {
        let out = self.suspect(token)?;
        match out {
            | Some(out) => {
                self.advance();
                Ok(Some(out))
            }
            | None => Ok(None),
        }
    }

    fn suspect_identifier_and_advance(&mut self) -> Result<Option<String>, Message> {
        let out = self.suspect_identifier()?;
        match out {
            | Some(out) => {
                self.advance();
                Ok(Some(out))
            }
            | None => Ok(None),
        }
    }

    fn expect(&self, token: Token, error: Message) -> Result<Token, Message> {
        if self.current == token {
            Ok(self.current.clone())
        } else if self.current == Token::Error {
            Err(
                Message::error(
                    IllegalCharacter,
                    details::IllegalCharacter!(&self.slice),
                    self.cursor.clone(),
                ),
            )
        } else {
            Err(error)
        }
    }

    fn expect_identifier(&self, error: Message) -> Result<String, Message> {
        self.expect(
            Token::Identifier,
            error,
        )?;
        Ok(self.slice.to_string())
    }

    fn expect_and_advance(&mut self, token: Token, error: Message) -> Result<Token, Message> {
        let out = self.expect(
            token, error,
        );
        self.advance();
        out
    }

    fn expect_identifier_and_advance(&mut self, error: Message) -> Result<String, Message> {
        let out = self.expect_identifier(error);
        self.advance();
        out
    }

    fn atom(&mut self) -> Result<Expression, Message> {
        match self.current {
            | Token::Integer => Ok(
                Expression::Integer(
                    token::to_i64(&self.slice),
                    self.cursor.clone(),
                ),
            ),
            | Token::String => Ok(
                Expression::String(
                    token::to_string(&self.slice),
                    self.cursor.clone(),
                ),
            ),
            | Token::Identifier => {
                if self.peek == Token::Assign {
                    let name = self.slice.clone();
                    let start = self.cursor.start.clone();

                    self.advance();
                    self.advance();
                    let value = Box::new(self.statement()?);

                    Ok(
                        Expression::VariableAssign {
                            name,
                            value,
                            cursor: self.cursor.clone_with_start(&start)
                        }
                    )
                }
                else if self.peek == Token::LeftParen {
                    let mut arguments: Vec<Argument> = Vec::new();

                    let name = self.slice.clone();

                    loop {
                        match self.current {
                            | Token::RightParen => {
                                self.advance();
                                break;
                            }
                            | Token::Comma => {
                                self.advance();
                                continue;
                            }
                            _ => {
                                arguments.push(
                                    Argument {
                                        expression: self.statement()?
                                    }
                                );
                            }
                        }
                    }

                    Ok(
                        Expression::FunctionCall {
                            name,
                            arguments,
                            cursor: self.cursor.clone()
                        }
                    )
                }
                else {
                    Ok(
                        Expression::VariableAccess(
                            self.slice.clone(),
                            self.cursor.clone(),
                        ),
                    )
                }
            },
            | Token::LeftParen => {
                self.advance();
                let statement = self.statement()?;
                self.advance();
                self.expect(
                    Token::RightParen,
                    Message::error(
                        MissingCaseClosure,
                        details::MissingCaseClosure!(),
                        self.cursor.clone(),
                    ),
                )?;
                Ok(statement)
            }
            | Token::Error => Err(
                Message::error(
                    IllegalCharacter,
                    details::IllegalCharacter!(&self.slice),
                    self.cursor.clone(),
                ),
            ),
            | _ => Err(
                Message::error(
                    MissingExpression,
                    details::MissingExpression!(),
                    self.cursor.clone(),
                ),
            ),
        }
    }

    fn math_expr_1(&mut self) -> Result<Expression, Message> {
        let mut left = self.atom()?;

        while self.peek == Token::Asterisk || self.peek == Token::Slash {
            self.advance();
            match self.current {
                | Token::Asterisk => {
                    self.advance();

                    left = Expression::Multiplication {
                        left: Box::new(left),
                        right: Box::new(self.atom()?),
                        cursor: self.cursor.clone(),
                    }
                }
                | Token::Slash => {
                    self.advance();

                    left = Expression::Division {
                        left: Box::new(left),
                        right: Box::new(self.atom()?),
                        cursor: self.cursor.clone(),
                    }
                }
                | _ => unreachable!(),
            }
        }

        Ok(left)
    }

    fn math_expr_2(&mut self) -> Result<Expression, Message> {
        let mut left = self.math_expr_1()?;

        while self.peek == Token::Plus || self.peek == Token::Minus {
            self.advance();
            match self.current {
                | Token::Plus => {
                    self.advance();

                    left = Expression::Addition {
                        left: Box::new(left),
                        right: Box::new(self.math_expr_1()?),
                        cursor: self.cursor.clone(),
                    }
                }
                | Token::Minus => {
                    self.advance();

                    left = Expression::Subtraction {
                        left: Box::new(left),
                        right: Box::new(self.math_expr_1()?),
                        cursor: self.cursor.clone(),
                    }
                }
                | _ => unreachable!(),
            }
        }

        Ok(left)
    }

    fn parse_let(&mut self) -> Result<Expression, Message> {
        let start_pos = self.cursor.start.clone();

        self.advance();

        let name = self.expect_identifier_and_advance(
            Message::error(
                MissingMemberName,
                details::MissingMemberName!("variable"),
                self.cursor.clone(),
            ),
        )?;

        let variable_type: Option<Type>;

        match self.current {
            | Token::Colon => {
                self.advance();
                variable_type = Some(Type {
                    name: self.expect_identifier_and_advance(
                        Message::error(
                            MissingMemberType,
                            details::MissingMemberType!("variable"),
                            self.cursor.clone(),
                        ),
                    )?,
                });
            }
            | Token::Assign => variable_type = None,
            | _ => {
                return Err(
                    Message::error(
                        MissingMemberTypeOrValueAssignment,
                        details::MissingMemberTypeOrValueAssignment!("variable"),
                        self.cursor.clone(),
                    ),
                )
            }
        }

        let value: Option<Expression>;

        if let Some(_) = self.suspect(Token::Assign)? {
            self.advance();
            value = Some(self.statement()?);
        } else {
            value = None;
        }

        Ok(
            Expression::VariableDeclaration {
                name,
                variable_type,
                value: if let Some(expr) = value {
                    Some(Box::new(
                        expr,
                    ))
                } else {
                    None
                },
                cursor: Cursor {
                    start: start_pos,
                    end: self.cursor.end.clone(),
                    file_name: self.cursor.file_name.clone(),
                    text: self.cursor.text.clone(),
                },
            },
        )
    }

    fn parse_function(&mut self) -> Result<Expression, Message> {
        let start_pos = self.cursor.start.clone();

        self.advance();

        // function name

        let name = self.expect_identifier_and_advance(
            Message::error(
                MissingMemberName,
                details::MissingMemberName!("function"),
                self.cursor.clone(),
            ),
        )?;

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

        /*while let Some(identifier) = self.suspect_identifier_and_advance()? {
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
        }*/
        loop {
            match self.current {
                | Token::RightParen => {
                    self.advance();
                    break;
                }
                | Token::Comma => {
                    self.advance();
                    continue;
                }
                | Token::Identifier => {
                    let identifier = self.slice.clone();

                    self.advance();

                    self.expect_and_advance(
                        Token::Colon,
                        Message::error(
                            MissingMemberType,
                            details::MissingMemberTypeColon!("parameter"),
                            self.cursor.clone(),
                        ),
                    )?;

                    let typ = self.expect_identifier_and_advance(
                        Message::error(
                            MissingMemberType,
                            details::MissingMemberType!("parameter"),
                            self.cursor.clone(),
                        ),
                    )?;

                    parameters.push(
                        Parameter {
                            name: identifier,
                            type_: Type { name: typ },
                        },
                    );
                }
                | _ => {
                    return Err(
                        Message::error(
                            MissingMemberName,
                            details::MissingMemberName!("parameter"),
                            self.cursor.clone(),
                        ),
                    )
                }
            }
        }

        // return type

        let colon = self.suspect(Token::Colon)?;

        let return_type = match colon {
            | Some(_) => Some(Type {
                name: self.expect_identifier_and_advance(
                    Message::error(
                        MissingMemberType,
                        details::MissingMemberType!("return"),
                        self.cursor.clone(),
                    ),
                )?,
            }),
            | None => None,
        };

        // block

        self.expect_and_advance(
            Token::LeftBrace,
            Message::error(
                MissingBlock,
                details::MissingBlock!(),
                self.cursor.clone(),
            ),
        )?;

        if self.current == Token::RightBrace {
            return Ok(
                Expression::FunctionDeclaration {
                    name,
                    parameters,
                    return_type,
                    body: Vec::new(),
                    cursor: self.cursor.clone_with_start(&start_pos)
                },
            );
        }

        let mut body: Vec<Expression> = Vec::new();

        loop {
            body.push(self.statement()?);
            self.advance();
            match self.current {
                | Token::RightBrace => break,
                | Token::NewLine => {
                    self.advance();
                    continue;
                }
                | _ => {
                    return Err(
                        Message::error(
                            MissingBlockSeparatorOrClosure,
                            details::MissingBlockSeparatorOrClosure!(),
                            self.cursor.clone(),
                        ),
                    )
                }
            }
        }

        // construction

        Ok(
            Expression::FunctionDeclaration {
                name,
                parameters,
                return_type,
                body,
                cursor: self.cursor.clone_with_start(&start_pos)
            },
        )
    }

    fn parse_raw(&mut self) -> Result<Expression, Message> {
        let start = self.cursor.start.clone();

        self.advance();

        let string = self.statement()?;
        if let Expression::String(string, _) = string {
            Ok(
                Expression::RawCode {
                    string,
                    cursor: Cursor {
                        start,
                        end: self.cursor.end.clone(),
                        file_name: self.cursor.file_name.clone(),
                        text: self.cursor.text.clone(),
                    },
                },
            )
        } else {
            Err(
                Message::error(
                    MissingExpression,
                    MissingExpression!(),
                    string.get_cursor().clone(),
                ),
            )
        }
    }

    fn statement(&mut self) -> Result<Expression, Message> {
        match self.current {
            | Token::Let => self.parse_let(),
            | Token::Function => self.parse_function(),
            | Token::Raw => self.parse_raw(),
            | _ => self.math_expr_2(),
        }
    }

    fn parse(&mut self) -> Result<Vec<Expression>, Message> {
        let mut out: Vec<Expression> = Vec::new();
        while self.current != Token::EoF {
            out.push(self.statement()?);
            self.advance();
            match self.current {
                | Token::EoF => break,
                | Token::NewLine => self.advance(),
                | _ => {
                    return Err(
                        Message::error(
                            MissingBlockSeparatorOrClosure,
                            details::MissingBlockSeparatorOrClosure!(),
                            self.cursor.clone(),
                        ),
                    )
                }
            }
        }
        Ok(out)
    }
}

pub fn parse(text: impl ToString) -> Result<Vec<Expression>, Message> {
    let text_string = text.to_string();
    let text_str = text_string.as_str();
    let mut parser = Parser::new(
        token::Token::lexer(text_str),
        "<stdin>",
        text_str,
    );
    parser.init();
    parser.parse()
}
