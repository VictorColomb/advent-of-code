use std::path::Path;

use aoc_client::{AocClient, AocResult};

use super::Day;

fn get_client(year: u16, day: Day) -> AocResult<AocClient> {
    const AOC_COOKIE_FILE: &str = ".adventofcode.session";

    let mut client = AocClient::builder();

    if Path::new(&AOC_COOKIE_FILE).exists() {
        client.session_cookie_from_file(AOC_COOKIE_FILE)?;
    } else {
        client.session_cookie_from_default_locations()?;
    };

    client.year(year.into())?.day(day.into())?.build()
}

pub fn get_input(year: u16, day: Day) -> AocResult<String> {
    let client = get_client(year, day)?;
    client.get_input()
}
