mod args;

use advent_of_code::commands::{download, scaffold, solve, utils::Day};
use args::{Arguments, Command};
use chrono::Datelike;
use clap::Parser;

fn main() {
    let args = Arguments::parse();

    let year = match args.year {
        Some(year) => year,
        None => {
            let now = chrono::Utc::now();
            if now.month() == 12 {
                now.year()
            } else {
                now.year() - 1
            }
        }
        .try_into()
        .unwrap(),
    };

    match &args.command {
        Command::Scaffold { day, download } => {
            scaffold::handle(year, day.unwrap_or_else(Day::today_safe), *download)
        }
        Command::Solve { day, release } => {
            solve::handle(year, day.unwrap_or_else(Day::today_safe), *release)
        }
        Command::Download { day } => download::handle(year, day.unwrap_or_else(Day::today_safe)),
    }
}
