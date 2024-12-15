use std::collections::{BinaryHeap, HashMap};

use advent_of_code::{
    grid::{Coordinate, Direction},
    misc::ScoredItem,
    parsing::parse_grid,
};

advent_of_code::solution!(2023, 17);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Crucible {
    position: Coordinate<i32>,
    direction: Direction,
    straight_moves: i32,
}

fn path(grid: &HashMap<Coordinate<i32>, i32>, min_move: i32, max_move: i32) -> i32 {
    // Setup
    let mut frontier = BinaryHeap::new();
    let mut cost_so_far = HashMap::new();

    let goal = Coordinate {
        x: grid.keys().map(|c| c.x).max().unwrap(),
        y: grid.keys().map(|c| c.y).max().unwrap(),
    };

    frontier.push(ScoredItem {
        item: Crucible {
            position: Coordinate { x: 0, y: 0 },
            direction: Direction::South,
            straight_moves: 0,
        },
        score: 0,
    });
    frontier.push(ScoredItem {
        item: Crucible {
            position: Coordinate { x: 0, y: 0 },
            direction: Direction::East,
            straight_moves: 0,
        },
        score: 0,
    });

    // Dijkstra
    while let Some(current) = frontier.pop() {
        // Useless path
        let best_cost_so_far = cost_so_far.get(&current.item).unwrap_or(&i32::MAX);
        if current.score <= *best_cost_so_far {
            continue;
        }
        cost_so_far.insert(current.item, current.score);

        // If goal, break
        if current.item.position == goal {
            if current.item.straight_moves >= min_move {
                return current.score;
            } else {
                continue;
            }
        }

        // Move forward possible
        if current.item.straight_moves < max_move {
            let next_coord = current.item.position.neighbor(current.item.direction);
            if grid.contains_key(&next_coord) {
                frontier.push(ScoredItem {
                    item: Crucible {
                        position: next_coord,
                        direction: current.item.direction,
                        straight_moves: current.item.straight_moves + 1,
                    },
                    score: current.score + grid.get(&next_coord).unwrap(),
                })
            }
        }

        // Turn possible
        if current.item.straight_moves >= min_move {
            for direction in current.item.direction.turns() {
                let next_coord = current.item.position.neighbor(direction);
                if grid.contains_key(&next_coord) {
                    frontier.push(ScoredItem {
                        item: Crucible {
                            position: next_coord,
                            direction,
                            straight_moves: 1,
                        },
                        score: current.score + grid.get(&next_coord).unwrap(),
                    })
                }
            }
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<i32> {
    let grid = parse_grid::<i32, i32>(input);
    Some(path(&grid, 0, 3))
}

pub fn part_two(input: &str) -> Option<i32> {
    let grid = parse_grid::<i32, i32>(input);
    Some(path(&grid, 4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::read_example;

    #[test]
    fn example_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn example_part_two2() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(94));
    }
}
