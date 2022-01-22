use compiler::compiler;
use parser::parser;
use std::fs;

fn main() {
    let filename = "./examples/variables/variables.es";

    let text = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let ast = match parser::parse(text) {
        | Ok(expr) => {
            println!(
                "{:#?}",
                expr
            );
            expr
        }
        | Err(error) => {
            println!(
                "{}",
                error.to_string()
            );
            return;
        }
    };
    let out = match compiler::compile(ast) {
        | Ok(out) => {
            for fun in out {
                println!(
                    "{}\n",
                    fun
                )
            }
        }
        | Err(error) => {
            println!(
                "{}",
                error.to_string()
            );
            return;
        }
    };
}
