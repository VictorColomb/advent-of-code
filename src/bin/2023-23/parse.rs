use std::collections::HashMap;

use advent_of_code::grid::{Coordinate, Direction};

pub(super) enum Tile {
    Start,
    End,
    Path,
    Slope(Direction),
}

pub(super) fn parse(input: &str) -> (HashMap<Coordinate<usize>, Tile>, usize) {
    let mut grid = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        max_y = y;
        for (x, c) in line.chars().enumerate() {
            max_x = x;

            let here = Coordinate { x, y };
            match c {
                '.' => {
                    grid.insert(here, Tile::Path);
                }
                '^' => {
                    grid.insert(here, Tile::Slope(Direction::North));
                }
                'v' => {
                    grid.insert(here, Tile::Slope(Direction::South));
                }
                '<' => {
                    grid.insert(here, Tile::Slope(Direction::West));
                }
                '>' => {
                    grid.insert(here, Tile::Slope(Direction::East));
                }
                '#' => {}
                _ => unreachable!("Unknown tile: {}", c),
            }
        }
    }

    grid.insert(Coordinate { x: 1, y: 0 }, Tile::Start);
    grid.insert(
        Coordinate {
            x: max_x - 1,
            y: max_y,
        },
        Tile::End,
    );

    (grid, max_y)
}
