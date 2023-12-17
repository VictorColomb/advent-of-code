pub mod aoc;
mod day;
mod file;
mod runner;

pub use day::Day;
pub use file::{read_example, read_input};
pub use runner::run_part;

/// Creates the constants `YEAR` and `DAY` and sets up the input and runner for each part.
///
/// The optional, second parameter (1 or 2) allows you to only run a single part of the solution.
#[macro_export]
macro_rules! solution {
    ($year:expr, $day:expr) => {
        $crate::solution!(@impl $year, $day, [part_one, 1] [part_two, 2]);
    };
    ($year:expr, $day:expr, 1) => {
        $crate::solution!(@impl $year, $day, [part_one, 1]);
    };
    ($year:expr, $day:expr, 2) => {
        $crate::solution!(@impl $year, $day, [part_two, 2]);
    };

    (@impl $year:expr, $day:expr, $( [$func:expr, $part:expr] )*) => {
        /// The current day.
        const DAY: $crate::commands::Day = $crate::day!($day);

        /// The current year.
        const YEAR: u16 = $year;

        fn main() {
            use $crate::commands::run_part;
            let input = $crate::commands::read_input(YEAR, DAY);
            $( run_part($func, &input, DAY, $part); )*
        }
    };
}
