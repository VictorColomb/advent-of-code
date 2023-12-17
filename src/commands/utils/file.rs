use std::{env, fs::read_to_string};

use super::Day;

#[must_use]
#[allow(dead_code)]
pub fn read_input(year: u16, day: Day) -> String {
    read_file("inputs", year, day, None)
}

#[must_use]
#[allow(dead_code)]
pub fn read_example(year: u16, day: Day, part: u8) -> String {
    read_file("examples", year, day, Some(part))
}

fn read_file(folder: &str, year: u16, day: Day, part: Option<u8>) -> String {
    let filename = match part {
        Some(part) => format!("{}-{}.txt", day, part),
        None => format!("{}.txt", day),
    };

    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(year.to_string())
        .join(filename);
    let f = read_to_string(filepath);

    match f {
        Ok(f) => f,
        Err(e) => {
            eprintln!("🫎 Could not open input file: {}", e);
            std::process::exit(1);
        },
    }
}
