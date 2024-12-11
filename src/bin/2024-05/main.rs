mod logic;
mod parse;

use parse::parse;

advent_of_code::solution!(2024, 5);

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering, updates) = parse(input);
    let (correct_updates, _) = logic::find_correct_updates(&ordering, updates);

    let mut sum = 0;
    for update in correct_updates {
        let middle_page = update.get(update.len().div_euclid(2)).unwrap();
        sum += middle_page;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering, updates) = parse(input);
    let (_, incorrect_updates) = logic::find_correct_updates(&ordering, updates);

    let mut sum = 0;
    for update in incorrect_updates {
        let update = logic::fix_update(&ordering, update);

        debug_assert!(logic::is_update_correct(&ordering, &update));

        let middle_page = update.get(update.len().div_euclid(2)).unwrap();
        sum += middle_page;
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(5955));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(4030));
    }
}
