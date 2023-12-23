use std::fmt::Debug;

use advent_of_code::grid::Coordinate3D;
use num::{Integer, Num};

use crate::brick::Brick;

/// Parse the input into a vector of bricks, sorted by `z` coordinate.
pub(super) fn parse<T: Integer + Copy>(input: &str) -> Vec<Brick<T>>
where
    <T as Num>::FromStrRadixErr: Debug,
{
    let mut bricks = Vec::new();

    for line in input.trim_end_matches('n').lines() {
        let (start, end) = line.split_once('~').unwrap();

        let start = start
            .split(',')
            .map(|s| T::from_str_radix(s, 10).unwrap())
            .collect::<Vec<T>>();
        let end = end
            .split(',')
            .map(|s| T::from_str_radix(s, 10).unwrap())
            .collect::<Vec<T>>();

        bricks.push(Brick::new(
            Coordinate3D::new(start[0], start[1], start[2]),
            Coordinate3D::new(end[0], end[1], end[2]),
        ))
    }

    bricks.sort_unstable_by_key(|b| b.start().z);
    bricks
}
