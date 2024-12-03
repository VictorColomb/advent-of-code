advent_of_code::solution!(2024, 2);

fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut reports = Vec::new();

    for line in input.trim_end_matches('\n').lines() {
        let levels = line
            .split(' ')
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        reports.push(levels);
    }

    reports
}

fn is_report_safe(report: &[u32]) -> bool {
    let mut iterator = report.iter();

    let mut previous_value = iterator.next().unwrap();
    let mut previous_direction = None;
    for value in iterator {
        let direction = value >= previous_value;
        if let Some(previous) = previous_direction {
            if previous != direction {
                return false;
            }
        } else {
            previous_direction = Some(direction);
        }

        if !(1..=3).contains(&value.abs_diff(*previous_value)) {
            return false;
        }

        previous_value = value;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse(input);

    let mut sum = 0;
    for report in reports.iter() {
        if is_report_safe(report) {
            sum += 1;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse(input);

    let mut sum = 0;
    'outer: for report in reports.iter() {
        if is_report_safe(report) {
            sum += 1;
            continue;
        }

        for index in 0..report.len() {
            let mut report_copy = report.clone();
            report_copy.remove(index);

            if is_report_safe(&report_copy) {
                sum += 1;
                continue 'outer;
            }
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(421));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(476));
    }
}
