mod args;

use advent_of_code::commands::{download, scaffold, solve, utils::Day};
use args::{Arguments, Command};
use chrono::Datelike;
use clap::Parser;

fn main() {
    let args = Arguments::parse();

    let year = match args.year {
        Some(year) => year,
        None => match std::env::var("AOC_YEAR") {
            Ok(year) => year.parse().unwrap_or_else(|_| {
                panic!(
                    "Invalid year specified in AOC_YEAR environment variable: {}",
                    year
                )
            }),
            Err(_) => {
                let now = chrono::Utc::now();
                if now.month() == 12 {
                    now.year()
                } else {
                    now.year() - 1
                }
            }
        }
        .try_into()
        .unwrap(),
    };

    match &args.command {
        Command::Scaffold { day, download } => {
            scaffold::handle(year, day.unwrap_or(Day::today_safe()), *download)
        }
        Command::Solve { day, release } => {
            solve::handle(year, day.unwrap_or(Day::today_safe()), *release)
        }
        Command::Download { day } => download::handle(year, day.unwrap_or(Day::today_safe())),
    }
}
