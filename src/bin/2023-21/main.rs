use std::collections::{HashSet, VecDeque};

use advent_of_code::grid::Coordinate;

advent_of_code::solution!(2023, 21, 2);

fn parse(input: &str) -> (Coordinate<i32>, HashSet<Coordinate<i32>>, (i32, i32)) {
    let mut gardens: HashSet<Coordinate<i32>> = HashSet::new();
    let mut start: Option<Coordinate<i32>> = None;

    for (y, line) in input.trim_end_matches('\n').lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => {
                    gardens.insert(Coordinate {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                '#' => {}
                'S' => {
                    start = Some(Coordinate {
                        x: x as i32,
                        y: y as i32,
                    });
                    gardens.insert(Coordinate {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                _ => panic!("Invalid char: {}", char),
            }
        }
    }

    let max_x = gardens.iter().map(|c| c.x).max().unwrap();
    let max_y = gardens.iter().map(|c| c.y).max().unwrap();

    (start.unwrap(), gardens, (max_x, max_y))
}

fn sub_part_one(input: &str, nb_steps: usize) -> Option<usize> {
    let (start, gardens, (max_x, max_y)) = parse(input);

    let mut stack = VecDeque::new();
    stack.push_back((start, 0));
    let mut seen = HashSet::new();
    let mut end_positions = 0;

    while let Some((pos, steps)) = stack.pop_front() {
        if !seen.insert(pos) {
            continue;
        }

        if steps % 2 == nb_steps % 2 {
            // The current tile could be a finish tile
            end_positions += 1;
        }

        if steps == nb_steps {
            continue;
        }

        for neighbor in pos.neighbors() {
            if gardens.contains(&Coordinate {
                x: neighbor.x.rem_euclid(max_x + 1),
                y: neighbor.y.rem_euclid(max_y + 1),
            }) {
                stack.push_back((neighbor, steps + 1));
            }
        }
    }

    Some(end_positions)
}

pub fn part_one(input: &str) -> Option<usize> {
    sub_part_one(input, 64)
}

pub fn part_two(input: &str) -> Option<i64> {
    const STEPS: i64 = 26501365;

    let height = input.trim_end_matches('\n').lines().count() as i64;
    let max_grid = STEPS / height;
    let mod_grid = STEPS % height;

    assert_eq!(mod_grid, (height - 1) / 2);

    let x0 = sub_part_one(input, mod_grid as usize).unwrap() as i64;
    let x1 = sub_part_one(input, (mod_grid + height) as usize).unwrap() as i64;
    let x2 = sub_part_one(input, (mod_grid + 2 * height) as usize).unwrap() as i64;

    let a0 = x0;
    let a1 = (-3 * x0 + 4 * x1 - x2) / 2;
    let a2 = (x0 - 2 * x1 + x2) / 2;

    // Observation 1: the starting line and columns do not contain any stones.
    // I can therefore reach as far as max_grid * height + mod_grid
    //
    // Observation 2: it takes 130 steps to reach all gardens in the starting grid,
    // which corresponds exactly to the number of steps to reach the corners of the grid.
    //
    // The number of reachable tiles is therefore quadratic in the number of grids.
    // x0 = f(0) ; x1 = f(1) ; x2 = f(2) and f(x) = a0 + a1*x + a2*x^2
    //
    // The calculation of the result is most likely optimizable, but I can't bother.

    Some(a0 + a1 * max_grid + a2 * max_grid * max_grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example_part_one() {
        let result = sub_part_one(&read_example(YEAR, DAY, 1), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(3830));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(637087163925555));
    }
}
