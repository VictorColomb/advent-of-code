mod logic;
mod parse;

advent_of_code::solution!(2024, 7);

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse::parse(input);

    let mut sum = 0;
    for equation in equations {
        if logic::solve_equation(&equation, false) {
            sum += equation.result;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse::parse(input);

    let mut sum = 0;
    for equation in equations {
        if logic::solve_equation(&equation, true) {
            sum += equation.result;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(3351424677624));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(204976636995111));
    }
}
