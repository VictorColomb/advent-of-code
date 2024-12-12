use std::collections::{HashMap, HashSet};

use advent_of_code::grid::{Coordinate, Direction};

pub(super) fn find_guard(grid: &HashMap<Coordinate<i32>, char>) -> Coordinate<i32> {
    for (position, tile) in grid {
        if *tile == '^' {
            return *position;
        }
    }

    unreachable!("The input does not include a guard!");
}

pub(super) fn walk(
    grid: &HashMap<Coordinate<i32>, char>,
    mut position: Coordinate<i32>,
    mut direction: Direction,
) -> Option<HashSet<Coordinate<i32>>> {
    let mut visited = HashSet::new();

    loop {
        if !visited.insert((position, direction)) {
            return None;
        }

        let next_position = position + direction;
        if let Some(tile) = grid.get(&next_position) {
            if *tile == '#' {
                direction = direction.rotate_cw();
            } else {
                position = next_position;
            }
        } else {
            break;
        }
    }

    let visited: HashSet<Coordinate<i32>> = HashSet::from_iter(visited.iter().map(|v| v.0));
    Some(visited)
}
