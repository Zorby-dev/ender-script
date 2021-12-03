use compiler::compiler;
use parser::parser;

fn main() {
    let text = "let variable: int = 5 \n let anotherVariable = variable ";
    let ast = match parser::parse(text) {
        Ok(expr) => {
            println!("{:#?}", expr);
            expr
        }
        Err(error) => {
            println!("{}", error.to_string());
            return;
        }
    };
    let out = match compiler::compile(ast) {
        Ok(out) => {
            println!("{}", out);
            out
        }
        Err(error) => {
            println!("{}", error.to_string());
            return;
        }
    };
}
