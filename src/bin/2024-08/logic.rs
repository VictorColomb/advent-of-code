use std::collections::{HashMap, HashSet};

use advent_of_code::grid::{Coordinate, SparseGrid};
use num::integer::gcd;

pub(super) fn find_frequencies(
    grid: &SparseGrid<i32, char>,
) -> HashMap<char, Vec<Coordinate<i32>>> {
    let mut frequencies = HashMap::new();

    for (position, tile) in &grid.objects {
        frequencies
            .entry(*tile)
            .or_insert(Vec::new())
            .push(*position)
    }

    frequencies
}

#[inline]
fn is_within_bounds(width: i32, height: i32, position: &Coordinate<i32>) -> bool {
    position.x >= 0 && position.y >= 0 && position.x < width && position.y < height
}

pub(super) fn find_antinodes_part1(
    width: i32,
    height: i32,
    positions: &[Coordinate<i32>],
) -> HashSet<Coordinate<i32>> {
    let mut antinodes = HashSet::new();

    for (index, left) in positions.iter().enumerate() {
        for right in &positions[index + 1..] {
            let diff = *left - *right;

            let antinode = *left + diff;
            if is_within_bounds(width, height, &antinode) {
                antinodes.insert(antinode);
            }

            let antinode = *right - diff;
            if is_within_bounds(width, height, &antinode) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes
}

fn until_edge(
    antinodes: &mut HashSet<Coordinate<i32>>,
    start: &Coordinate<i32>,
    diff: Coordinate<i32>,
    width: i32,
    height: i32,
    reverse: bool,
) {
    let mut position = *start;

    loop {
        if !is_within_bounds(width, height, &position) {
            break;
        }
        antinodes.insert(position);

        position = if reverse {
            position - diff
        } else {
            position + diff
        };
    }
}

pub(super) fn find_antinodes_part2(
    width: i32,
    height: i32,
    positions: &[Coordinate<i32>],
) -> HashSet<Coordinate<i32>> {
    let mut antinodes = HashSet::new();

    for (index, left) in positions.iter().enumerate() {
        for right in &positions[index + 1..] {
            let diff = *left - *right;
            let gcd = gcd(diff.x, diff.y);
            let diff = Coordinate {
                x: diff.x / gcd,
                y: diff.y / gcd,
            };

            until_edge(&mut antinodes, left, diff, width, height, false);
            until_edge(&mut antinodes, right, diff, width, height, true);
        }
    }

    antinodes
}
