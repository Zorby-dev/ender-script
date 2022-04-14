use compiler::compiler;
use dialoguer::{Input, theme::ColorfulTheme, Confirm};
use parser::parser;
use std::{fs, env::current_dir, error::Error};
use clap::{Command, crate_version, crate_name};
use std::env;

struct Config {
    namespace: String,
    source_folder: String,
    output_folder: String,
    gen_file_struct: bool
}

fn init() -> Result<Option<Config>, Box<dyn Error>> {
    let theme = ColorfulTheme::default();

    println!("This utility will wlk you through creating a esconfig.json file.\nIt only covers the most common items, and tries to guess sensible defaults.\n\nPress ^C at any time to quit.");

    let cur_dir = current_dir().unwrap().file_name().unwrap().to_str().unwrap().to_string();

    let namespace = Input::with_theme(&theme)
        .with_prompt("Namespace")
        .default(cur_dir)
        .interact_text()?;
    
    let source_folder = Input::with_theme(&theme)
        .with_prompt("Source folder")
        .default("./src".to_string())
        .interact_text()?;

    let output_folder = Input::with_theme(&theme)
        .with_prompt("Output folder")
        .default("./out".to_string())
        .interact_text()?;

    let gen_file_struct = Confirm::with_theme(&theme)
        .with_prompt("Generate coresponding file structure?")
        .default(true)
        .interact()?;

    println!("");

    let proceed = Confirm::with_theme(&theme)
        .with_prompt("Proceed with the setup?")
        .default(true)
        .interact()?;

    if proceed {
        Ok(Some(Config {
            namespace,
            gen_file_struct,
            output_folder,
            source_folder
        }))
    } else {
        Ok(None)
    }
}

fn main() {
    /*let filename = "./examples/variables/variables.es";

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
    };*/
    let matches = Command::new(crate_name!())
        .about("EnderScript compiler")
        .version(crate_version!())
        .subcommand(
            Command::new("init")
        ).get_matches();
    
    match matches.subcommand() {
        Some(("init", init_matches)) => {
            match init() {
                Ok(Some(config)) => println!("OK"),
                Ok(None) => println!("Setup wizard aborted."),
                Err(error) => println!("err: {}", error)
            }
        },
        Some((_, _)) => unreachable!(),

        None => {
            println!("comiping...")
        },
    }
}
