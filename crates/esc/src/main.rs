use compiler::compiler;
use parser::parser;
use std::{fs::{self, File, read_dir, create_dir_all}, env::current_dir, error::Error, io::Read, ffi::OsString, path::Path};
use clap::{Command, crate_version, crate_name, arg};

mod init;
mod config;

use config::Config;
use init::{init, gen_files};

fn build_entry(filename: String) -> Result<(), String> {
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
            return Err(error.to_string());
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
            return Err(error.to_string());
        }
    };

    Ok(())
}

fn build_namespace(namespace: OsString) {

}

fn build(config_path: Option<&str>, source_path: Option<&str>, output_path: Option<&str>) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config_path.unwrap_or("./esconfig.json"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config = json::parse(contents.as_str())?;

    let source_path = Path::new(source_path.unwrap_or_else(|| config["source"].as_str().unwrap()));
    let output_path = Path::new(output_path.unwrap_or_else(|| config["output"].as_str().unwrap()));
    let name = config["name"].as_str().unwrap();

    let pack = output_path.join(name).as_path();

    create_dir_all(pack)?;

    let mcmeta = File::create(pack.join("pack.mcmeta"));

    for path in read_dir(source_path)? {
        if let Ok(path) = path {
            let meta = path.metadata()?;
        
            if meta.is_dir() {
                build_namespace(path.file_name());
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new(crate_name!())
        .about("EnderScript compiler")
        .version(crate_version!())
        .subcommand(
            Command::new("init")
                .alias("i")
        )
        .subcommand(
            Command::new("build")
                .alias("b")
        )
        .arg(
            arg!(-c --config [CONFIG] "Optionally sets a config file to use")
        )
        .arg(
            arg!(-s --source [SOURCE] "Overrides the source folder specified in esconfig.json")
        )
        .arg(
            arg!(-o --output [OUTPUT] "Overrides the output folder specified in esconfig.json")
        )
        .get_matches();
    
    match matches.subcommand() {
        Some(("init", _)) => {
            match init() {
                Ok(Some(config)) => gen_files(current_dir().unwrap().as_path(), &config)?,
                Ok(None) => println!("Setup wizard aborted."),
                Err(error) => println!("err: {}", error)
            }
        },
        Some((_, _)) => unreachable!(),

        None => {
            build(matches.value_of("config"), matches.value_of("source"), matches.value_of("output"))?;
        },
    }

    Ok(())
}
