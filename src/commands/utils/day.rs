use std::str::FromStr;
use std::{error::Error, fmt::Display};

use chrono::{Datelike, Local};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day(u8);

impl Day {
    /// Creates a new [`Day`] from a [`u8`], but returns [`None`] if the value is not in the range 1..=25.
    pub fn new(day: u8) -> Option<Self> {
        if !(1..=25).contains(&day) {
            None
        } else {
            Some(Self(day))
        }
    }

    #[doc(hidden)]
    pub const fn __new_unchecked(day: u8) -> Self {
        Self(day)
    }

    /// Returns the current day, or [`None`] if the current day is not a valid day for AoC.
    pub fn today() -> Option<Self> {
        let today = Local::now();
        if today.month() == 12 && today.day() <= 25 {
            Self::new(today.day() as u8)
        } else {
            None
        }
    }

    /// Returns the current day, or panics if the current day is not a valid day for AoC.
    pub fn today_safe() -> Self {
        Self::today().unwrap_or_else(|| {
            eprintln!("🫎 Today is not a valid day for AoC");
            std::process::exit(1);
        })
    }
}

impl From<Day> for u8 {
    fn from(day: Day) -> u8 {
        day.0
    }
}

impl From<Day> for u32 {
    fn from(day: Day) -> u32 {
        day.0.into()
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl PartialEq<u8> for Day {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<u8> for Day {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl FromStr for Day {
    type Err = DayFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = s.parse().map_err(|_| DayFromStrError)?;
        Self::new(day).ok_or(DayFromStrError)
    }
}

#[derive(Debug)]
pub struct DayFromStrError;

impl Error for DayFromStrError {}

impl Display for DayFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("🫎 Expected a number between 1 and 25")
    }
}

/// Creates a [`Day`] value in a const context.
#[macro_export]
macro_rules! day {
    ($day:expr) => {{
        const _ASSERT: () = assert!(
            $day != 0 && $day <= 25,
            concat!(
                "🫎 Invalid day number `",
                $day,
                "`, expecting a value between 1 and 25"
            ),
        );
        $crate::commands::Day::__new_unchecked($day)
    }};
}
