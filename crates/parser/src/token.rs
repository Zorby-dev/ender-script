use logos::Logos;

pub fn to_i64(slice: &impl ToString) -> i64 {
    slice.to_string().replace("_", "").parse::<i64>().unwrap()
}

pub fn to_string(slice: &impl ToString) -> String {
    let str_slice = slice.to_string();
    str_slice.as_str().chars().skip(1).take(str_slice.len()-2).collect()
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    /*#[token("|")]
    Pipe,
    #[token("&")]
    Ampersand,
    #[token("!")]
    Bang,*/
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
    /*#[token(".")]
    Dot,*/
    #[token("=")]
    Assign,
    /*#[token("==")]
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
    LessThanOrEqual,*/

    #[token("function")]
    Function,
    #[token("let")]
    Let,
    #[token("raw")]
    Raw,
    /*#[token("while")]
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
    Not,*/

    #[regex(r##"[a-zA-Z][a-zA-Z0-9]*"##)]
    Identifier,
    #[regex(r##""(?:\\"|[^"])*""##)]
    String,
    #[regex(r##"-?\d+(?:_\d+)*"##)]
    Integer,

    #[token("\n")]
    NewLine,
    #[error]
    Error,
    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,
    EoF,
}
