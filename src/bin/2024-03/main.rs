use regex::Regex;

advent_of_code::solution!(2024, 3);

pub fn part_one(input: &str) -> Option<u32> {
    let re: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum = 0;

    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        let left = left.parse::<u32>().unwrap();
        let right = right.parse::<u32>().unwrap();

        sum += left * right;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let mut sum = 0;
    let re: Regex = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    for capture in re.captures_iter(input) {
        match capture.get(0).unwrap().as_str().split_once("(").unwrap().0 {
            "do" => {
                enabled = true;
            }
            "don't" => {
                enabled = false;
            }
            "mul" => {
                if !enabled {
                    continue;
                }

                let string = capture.get(0).unwrap().as_str();
                let (left, right) = string[0..string.len() - 1]
                    .split_once("(")
                    .unwrap()
                    .1
                    .split_once(",")
                    .unwrap();

                let left = left.parse::<u32>().unwrap();
                let right = right.parse::<u32>().unwrap();
                sum += left * right;
            }
            i => unreachable!("Unknown instruction: {}", i),
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(173517243))
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 2));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(100450138))
    }
}
