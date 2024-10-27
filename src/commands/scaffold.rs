use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::Write,
    path::Path,
    process,
};

use crate::commands::Day;

use super::download;

const MODULE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/template.txt"));

fn create_file_safe(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file_and_dirs(path: &str) -> Result<File, std::io::Error> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

pub fn handle(year: u16, day: Day, download: bool) {
    let module_dir = format!("src/bin/{}-{}", year, day);
    let module_file = format!("{}/main.rs", module_dir);
    let input_file = format!("data/inputs/{}/{}.txt", year, day);
    let example_file = format!("data/examples/{}/{}-1.txt", year, day);

    // Create the module directory if it doesn't exist.
    create_dir_all(module_dir).unwrap_or_else(|e| {
        eprintln!("🫎 Failed to create module directory: {}", e);
        process::exit(1);
    });

    // Create the module file if it doesn't exist.
    if !Path::new(&module_file).exists() {
        let mut file = match create_file_safe(&module_file) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("🫎 Failed to create module file: {}", e);
                process::exit(1);
            }
        };

        match file.write_all(
            MODULE_TEMPLATE
                .replace("%YEAR%", &year.to_string())
                .replace("%DAY%", &day.to_string())
                .as_bytes(),
        ) {
            Ok(_) => println!("🎄 Created module file: {}", module_file),
            Err(e) => {
                eprintln!("🫎 Failed to write to module file: {}", e);
                process::exit(1);
            }
        }
    }

    // Create the example directory and file.
    match create_file_and_dirs(&example_file) {
        Ok(_) => println!("🎄 Created example file: {}", example_file),
        Err(e) => {
            eprintln!("🫎 Failed to create example file: {}", e);
            process::exit(1);
        }
    }

    // Download if requested
    if download {
        download::handle(year, day);
    } else {
        // Create the input directory and file.
        match create_file_and_dirs(&input_file) {
            Ok(_) => println!("🎄 Created input file: {}", input_file),
            Err(e) => {
                eprintln!("🫎 Failed to create input file: {}", e);
                process::exit(1);
            }
        }
    }
}
