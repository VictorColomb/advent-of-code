use std::{collections::HashMap, hash::Hash};

use num::Integer;

use crate::grid::{Coordinate, SparseGrid};

pub fn parse_sparse_grid<T>(data: &str, ignore: char) -> SparseGrid<T, char>
where
    T: Integer + Copy + TryFrom<usize> + Hash,
{
    let mut grid: HashMap<Coordinate<T>, char> = HashMap::new();
    let data = data.trim_end_matches('\n');

    let lines = data.lines().collect::<Vec<&str>>();
    let height = T::try_from(lines.len()).ok().unwrap();
    let width = T::try_from(lines[0].len()).ok().unwrap();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == ignore {
                continue;
            }
            grid.insert(
                Coordinate {
                    x: T::try_from(x).ok().unwrap(),
                    y: T::try_from(y).ok().unwrap(),
                },
                c,
            );
        }
    }

    SparseGrid {
        height,
        width,
        objects: grid,
    }
}
