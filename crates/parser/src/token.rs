use logos::{Lexer, Logos};

fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_string())
}

pub fn to_i64(lex: &Lexer<Token>) -> Option<i64> {
    lex.slice().to_string().replace("_", "").parse::<i64>().ok()
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("|")]
    Pipe,
    #[token("&")]
    Ampersand,
    #[token("!")]
    Bang,
    #[token("/")]
    Slash,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("=")]
    Assign,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("<=")]
    LessThanOrEqual,

    #[token("function")]
    Function,
    #[token("while")]
    While,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("and")]
    #[token("&&")]
    And,
    #[token("or")]
    #[token("||")]
    Or,
    #[token("not")]
    Not,

    #[regex(r##"[a-zA-Z][a-zA-Z0-9]*"##)]
    Identifier,
    #[regex(r##""(?:\\"|[^"])*""##)]
    String,
    #[regex(r##"-?\d+(?:_\d+)*"##)]
    Integer,

    #[error]
    Error,
    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,
    EoF
}