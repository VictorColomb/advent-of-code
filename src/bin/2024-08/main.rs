use std::collections::HashSet;

use advent_of_code::parsing::parse_sparse_grid;

mod logic;

advent_of_code::solution!(2024, 8);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_sparse_grid::<i32>(input, '.');
    let frequencies = logic::find_frequencies(&grid);

    let mut antinodes = HashSet::new();
    for (_, positions) in frequencies {
        antinodes.extend(logic::find_antinodes_part1(
            grid.width,
            grid.height,
            &positions,
        ));
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_sparse_grid::<i32>(input, '.');
    let frequencies = logic::find_frequencies(&grid);

    let mut antinodes = HashSet::new();
    for (_, positions) in frequencies {
        antinodes.extend(logic::find_antinodes_part2(
            grid.width,
            grid.height,
            &positions,
        ));
    }

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(400));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(1280));
    }
}
