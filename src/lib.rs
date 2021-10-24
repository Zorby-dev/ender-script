pub mod lexer;
pub mod parser;

pub mod util;
mod colors;

#[cfg(test)]
mod tests {
    use std::fs;
    use super::lexer;
    use super::parser;

    fn make_tokens(test_name: &str) -> Vec<lexer::Token> {
        let file_name = format!("c:/Users/Jachym/Documents/Code/Rust/EnderScript/ender-script/tests/{}.es", test_name);
        let content = fs::read_to_string(&file_name).unwrap();
        let result = lexer::make_tokens(&file_name, &content);
        match result.0 {
            lexer::Result::Err(_) => panic!(),
            lexer::Result::Ok(tokens) => tokens
        }
    }

    fn parse(tokens: Vec<lexer::Token>) -> parser::Node {
        let result = parser::parse(tokens);
        match result {
            parser::Result::Failure(_) => panic!(),
            parser::Result::Success(node) => node
        }
    }

    #[test]
    fn test0() {
        parse(make_tokens("test0"));
    }
}