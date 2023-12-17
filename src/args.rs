use clap::{Parser, Subcommand};

use advent_of_code::commands::Day;

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,

    /// The target year (defaults to the last AoC event)
    #[arg(global = true, env = "AOC_YEAR")]
    pub year: Option<u16>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scaffold a new day
    Scaffold {
        /// The day to scaffold (during an AoC event, defaults to the current day)
        day: Option<Day>,

        /// Download input from adventofcode.com (requires session cookie)
        #[arg(short, long)]
        download: bool,
    },

    /// Solve the puzzle of a given day
    Solve {
        /// The day to solve (during an AoC event, defaults to the current day)
        day: Option<Day>,

        /// Run the solution in release mode
        #[arg(short, long)]
        release: bool,
    },

    /// Download the puzzle input for a given day
    Download {
        /// The day to download input for (during an AoC event, defaults to the current day)
        day: Option<Day>,
    },
}
