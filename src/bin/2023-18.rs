use advent_of_code::grid::{Coordinate, Direction};

advent_of_code::solution!(2023, 18);

struct DigInstruction {
    direction: Direction,
    length: u64,
}

/// Parse the input into a vector of `Dig`s for the first part of the puzzle.
fn parse_input1(input: &str) -> Vec<DigInstruction> {
    let mut dig_instructions = Vec::new();

    for line in input.trim_end_matches('\n').split('\n') {
        let mut parts = line.split(' ');

        let direction = match parts.next().unwrap() {
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
            dir => panic!("Invalid direction {}", dir),
        };
        let length = parts.next().unwrap();

        dig_instructions.push(DigInstruction {
            direction,
            length: length.parse().unwrap(),
        });
    }

    dig_instructions
}

/// Parse the input into a vector of `Dig`s for the second part of the puzzle.
fn parse_input2(input: &str) -> Vec<DigInstruction> {
    let mut dig_instructions = Vec::new();

    for line in input.trim_end_matches('\n').split('\n') {
        let mut parts = line.split(' ');
        parts.next();
        parts.next();

        let color = &parts.next().unwrap()[2..8];

        dig_instructions.push(DigInstruction {
            direction: match color.chars().nth(5).unwrap() {
                '0' => Direction::East,
                '1' => Direction::South,
                '2' => Direction::West,
                '3' => Direction::North,
                d => unreachable!("Invalid direction {}", d),
            },
            length: u64::from_str_radix(&color[..5], 16).unwrap(),
        });
    }

    dig_instructions
}

fn calculate_dig_area(instructions: Vec<DigInstruction>) -> u64 {
    let mut trench_vertices = Vec::new();
    let mut trench_length = 0;

    // Record vertices of the trench
    let mut current_position = Coordinate { x: 0, y: 0 };
    trench_vertices.push(current_position);
    for ins in instructions {
        // Move
        current_position += match ins.direction {
            Direction::North => Coordinate {
                x: 0,
                y: -(ins.length as i64),
            },
            Direction::South => Coordinate {
                x: 0,
                y: ins.length as i64,
            },
            Direction::East => Coordinate {
                x: ins.length as i64,
                y: 0,
            },
            Direction::West => Coordinate {
                x: -(ins.length as i64),
                y: 0,
            },
        };

        // Record vertex
        trench_vertices.push(current_position);
        trench_length += ins.length;
    }

    // Calculate area (Triangle formula)
    // The starting position is both the first and last element of `trench vertices`.
    let mut area = 0;
    for i in 0..trench_vertices.len() - 1 {
        let a = trench_vertices[i];
        let b = trench_vertices[i + 1];
        area += a.x * b.y - b.x * a.y;
    }
    if area < 0 {
        area = -area;
    }

    // At this point we have the inner area of the dig.
    // We are half of the length of the trench + 1 (.25 * 4 edges)
    area = (area + (trench_length as i64)) / 2 + 1;

    area as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = parse_input1(input);
    Some(calculate_dig_area(instructions))
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = parse_input2(input);
    Some(calculate_dig_area(instructions))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(44436));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(952408144115));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(106941819907437));
    }
}
