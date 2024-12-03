use std::collections::{BTreeMap, BinaryHeap};

advent_of_code::solution!(2024, 1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for line in input.trim_end_matches('\n').lines() {
        let (left, right) = line.split_once("   ").unwrap();
        let left = left.parse::<u32>().unwrap();
        let right = right.parse::<u32>().unwrap();

        left_heap.push(left);
        right_heap.push(right);
    }
    let mut sum = 0;
    while let Some(left) = left_heap.pop() {
        let right = right_heap.pop().unwrap();
        sum += left.abs_diff(right);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_vec = Vec::new();
    let mut right_map = BTreeMap::new();

    for line in input.trim_end_matches('\n').lines() {
        let (left, right) = line.split_once("   ").unwrap();
        let left = left.parse::<u32>().unwrap();
        let right = right.parse::<u32>().unwrap();

        left_vec.push(left);
        right_map.insert(right, right_map.get(&right).unwrap_or(&0u32) + 1);
    }

    let mut sum = 0;
    for left in left_vec.iter() {
        if let Some(right_count) = right_map.get(left) {
            sum += left * right_count
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(1258579));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(23981443));
    }
}
