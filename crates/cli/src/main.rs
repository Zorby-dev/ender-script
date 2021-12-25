use compiler::compiler;
use parser::parser;
use std::fs;

fn main() {
    let filename = "C:\\Users\\Jachym\\Documents\\Code\\Rust\\ender-script\\examples\\variables\\main.es";

    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
        
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
