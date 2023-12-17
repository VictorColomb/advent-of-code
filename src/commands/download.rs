use std::fs::{OpenOptions, create_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::process;

use super::utils::aoc::get_input;
use super::Day;

fn create_file_and_dirs(path: &str) -> Result<File, std::io::Error> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    OpenOptions::new().write(true).create(true).open(path)
}

pub fn handle(year: u16, day: Day) {
    let input = match get_input(year.into(), day.into()) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("🫎 Failed to download input: {}", e);
            process::exit(1);
        }
    };

    let input_file = format!("data/inputs/{}/{}.txt", year, day);
    match create_file_and_dirs(&input_file) {
        Ok(mut file) => file.write_all(input.as_bytes()).unwrap(),
        Err(e) => {
            eprintln!("🫎 Failed to write to input file: {}", e);
            process::exit(1);
        },
    }

    println!("🎄 Downloaded input into file: {}", input_file);
}
