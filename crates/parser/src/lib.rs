mod token;
pub mod ast;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::token::Token;
    use logos::Logos;

    #[test]
    fn strings() {
        let mut lexer = Token::lexer("\"Hello, World!\"");

        assert_eq!(lexer.next(), Some(Token::String));
        assert_eq!(lexer.slice(), "\"Hello, World!\"");
    }

    /*#[test]
    fn integers() {
        let mut lexer = Token::lexer("45 500_000_000 -1");

        assert_eq!(lexer.next(), Some(Token::Integer));
        assert_eq!(to_i64(&lexer), 45);
        assert_eq!(lexer.next(), Some(Token::Integer));
        assert_eq!(to_i64(&lexer), 500_000_000);
        assert_eq!(lexer.next(), Some(Token::Integer));
        assert_eq!(to_i64(&lexer), -1);
    }*/

    #[test]
    fn identifiers() {
        let mut lexer = Token::lexer("x y z abc Class");

        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "x");
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "y");
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "z");
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "abc");
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "Class");
    }

    #[test]
    fn keywords() {
        let mut lexer = Token::lexer("function while if else or not and");

        assert_eq!(lexer.next(), Some(Token::Function));
        assert_eq!(lexer.next(), Some(Token::While));
        assert_eq!(lexer.next(), Some(Token::If));
        assert_eq!(lexer.next(), Some(Token::Else));
        assert_eq!(lexer.next(), Some(Token::Or));
        assert_eq!(lexer.next(), Some(Token::Not));
        assert_eq!(lexer.next(), Some(Token::And));
    }

    #[test]
    fn symbols() {
        let mut lexer = Token::lexer("== != <= >= < > && || + - * / ! ( ) { } : ,");

        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.next(), Some(Token::NotEqual));
        assert_eq!(lexer.next(), Some(Token::LessThanOrEqual));
        assert_eq!(lexer.next(), Some(Token::GreaterThanOrEqual));
        assert_eq!(lexer.next(), Some(Token::LessThan));
        assert_eq!(lexer.next(), Some(Token::GreaterThan));
        assert_eq!(lexer.next(), Some(Token::And));
        assert_eq!(lexer.next(), Some(Token::Or));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Asterisk));
        assert_eq!(lexer.next(), Some(Token::Slash));
        assert_eq!(lexer.next(), Some(Token::Bang));
        assert_eq!(lexer.next(), Some(Token::LeftParen));
        assert_eq!(lexer.next(), Some(Token::RightParen));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Comma));
    }
}
