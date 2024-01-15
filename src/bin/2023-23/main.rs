mod graph;
mod parse;

use std::collections::{HashMap, VecDeque};

use advent_of_code::grid::Coordinate;
use graph::graph;
use parse::parse;

advent_of_code::solution!(2023, 23);

fn longest_path(
    graph: HashMap<Coordinate<usize>, HashMap<Coordinate<usize>, usize>>,
    max_y: usize,
) -> usize {
    let mut longest_path = 0;
    let mut queue = VecDeque::new();
    queue.push_back(vec![Coordinate { x: 1, y: 0 }]);

    while let Some(path) = queue.pop_front() {
        let position = path.iter().last().unwrap();

        if position.y == max_y {
            let mut path_length = 0;
            for i in 0..path.len() - 1 {
                path_length += graph.get(&path[i]).unwrap().get(&path[i + 1]).unwrap();
            }

            if path_length > longest_path {
                longest_path = path_length;
            }
        } else {
            for neighbor in graph.get(position).unwrap().keys() {
                if !path.contains(neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(*neighbor);
                    queue.push_back(new_path);
                }
            }
        }
    }

    longest_path
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, max_y) = parse(input);
    let graph = graph(&grid, true);

    Some(longest_path(graph, max_y))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, max_y) = parse(input);
    let graph = graph(&grid, false);

    Some(longest_path(graph, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(2086));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(154));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(6526));
    }
}
