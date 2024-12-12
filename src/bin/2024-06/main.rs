use advent_of_code::{grid::Direction, parsing::parse_grid};

mod logic;

advent_of_code::solution!(2024, 6);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_grid::<i32, char>(input);
    let position = logic::find_guard(&grid);
    let direction = Direction::North;

    logic::walk(&grid, position, direction).map(|path| path.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_grid::<i32, char>(input);
    let position = logic::find_guard(&grid);
    let direction = Direction::North;

    let path = logic::walk(&grid, position, direction).unwrap();

    let mut loops = 0;
    for path_step in path {
        let mut grid = grid.clone();
        grid.insert(path_step, '#');

        if logic::walk(&grid, position, direction).is_none() {
            loops += 1;
        }
    }

    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(5404));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(1984));
    }
}
