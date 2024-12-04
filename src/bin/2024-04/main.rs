use std::collections::{HashSet, VecDeque};

use advent_of_code::{
    grid::{Coordinate, ExtendedDirection},
    parsing::parse_grid,
};

advent_of_code::solution!(2024, 4);

const CHARS: [char; 4] = ['X', 'M', 'A', 'S'];

struct Position {
    coordinate: Coordinate<i32>,
    direction: ExtendedDirection,
    index: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid::<i32, char>(input);
    let mut deque = VecDeque::<Position>::new();

    for (coordinate, first_letter) in grid.iter() {
        if *first_letter != CHARS[0] {
            continue;
        }
        for direction in ExtendedDirection::iter() {
            deque.push_back(Position {
                coordinate: *coordinate,
                direction,
                index: 1,
            });
        }
    }

    let mut count = 0;
    while let Some(position) = deque.pop_front() {
        if position.index >= CHARS.len() {
            count += 1;
            continue;
        }

        let next_coordinate = position.coordinate + position.direction;
        if let Some(next_char) = grid.get(&next_coordinate) {
            if next_char == CHARS.get(position.index).unwrap() {
                deque.push_back(Position {
                    coordinate: next_coordinate,
                    direction: position.direction,
                    index: position.index + 1,
                })
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid::<i32, char>(input);
    let mut valid_coordinates = HashSet::new();

    let mut count = 0;
    'outer: for (coordinate, value) in grid.iter() {
        if *value != 'A' {
            continue;
        }
        for direction in ExtendedDirection::iter_diagonals() {
            if let Some(next_value) = grid.get(&(*coordinate + direction)) {
                if *next_value != 'M' {
                    continue;
                }

                if let Some(opposite_value) = grid.get(&(*coordinate + direction.opposite())) {
                    if *opposite_value == 'S' && !valid_coordinates.insert(coordinate) {
                        count += 1;
                        continue 'outer;
                    }
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(2557));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(1854));
    }
}
