use std::{collections::HashMap, fmt::Debug, hash::Hash, str::FromStr};

use num::Integer;

use crate::grid::Coordinate;

/// Parses a grid of characters in the form of a string to a `HashMap<Coordinate, T>`
///
/// The X compoment of the coordinates represents the horizontal axis. Similarly, the Y component
/// represents the vertical axis.
pub fn parse_grid<T, V>(data: &str) -> HashMap<Coordinate<T>, V>
where
    T: Integer + Copy + TryFrom<usize> + Hash,
    V: FromStr,
    <V as FromStr>::Err: Debug,
{
    let mut grid: HashMap<Coordinate<T>, V> = HashMap::new();
    let data = data.trim_end_matches('\n');

    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                Coordinate {
                    x: T::try_from(x).ok().unwrap(),
                    y: T::try_from(y).ok().unwrap(),
                },
                String::from(c).parse::<V>().unwrap(),
            );
        }
    }

    grid
}
