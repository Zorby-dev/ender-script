use std::{
    env::current_dir,
    error::Error,
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use json::object;

use crate::Config;

pub fn init() -> Result<Option<Config>, Box<dyn Error>> {
    let theme = ColorfulTheme::default();

    println!("\nThis utility will walk you through creating an EnderScript project.\nIt only covers the most common items, and tries to guess sensible defaults.\n\nPress ^C at any time to quit.");

    let cur_dir = current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let name = Input::with_theme(&theme)
        .with_prompt("Name")
        .default(cur_dir.clone())
        .interact_text()?;

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
            name,
            namespace,
            gen_file_struct,
            output_folder,
            source_folder,
        }))
    } else {
        Ok(None)
    }
}

pub fn gen_files(root: &Path, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(root.join("esconfig.json"))?;
    file.write_all(
        object! {
            name: config.name.as_str(),
            source: config.source_folder.as_str(),
            output: config.output_folder.as_str()
        }
        .pretty(4)
        .as_bytes(),
    )?;

    if config.gen_file_struct {
        create_dir(root.join(&config.source_folder))?;
        create_dir(root.join(&config.output_folder))?;

        let source_dir = root.join(&config.source_folder);

        create_dir(source_dir.join(&config.namespace))?;
    }

    Ok(())
}
