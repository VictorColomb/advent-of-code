use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::grid::{Coordinate, Direction};

use crate::parse::Tile;

pub(super) fn graph(
    grid: &HashMap<Coordinate<usize>, Tile>,
    slopes: bool,
) -> HashMap<Coordinate<usize>, HashMap<Coordinate<usize>, usize>> {
    let mut graph: HashMap<Coordinate<usize>, HashMap<Coordinate<usize>, usize>> = HashMap::new();

    // Find all junctions
    for (position, tile) in grid.iter() {
        match tile {
            Tile::Start | Tile::End => {
                graph.insert(*position, HashMap::new());
            }
            Tile::Path => {
                if position
                    .neighbors()
                    .iter()
                    .filter(|n| grid.contains_key(n))
                    .count()
                    > 2
                {
                    graph.insert(*position, HashMap::new());
                }
            }
            Tile::Slope(_) => {}
        }
    }

    // Link junctions between themselves
    for junction in graph.keys().cloned().collect::<Vec<_>>().iter() {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((*junction, 0));

        while let Some((position, distance)) = queue.pop_front() {
            if graph.contains_key(&position) && position != *junction {
                // Found another junction
                graph.get_mut(junction).unwrap().insert(position, distance);
            } else {
                match grid.get(&position) {
                    Some(Tile::Path) => {
                        if !visited.contains(&position) {
                            visited.insert(position);
                            position
                                .neighbors()
                                .iter()
                                .for_each(|n| queue.push_back((*n, distance + 1)));
                        }
                    }
                    Some(Tile::Slope(direction)) => {
                        if slopes {
                            queue.push_back((position.neighbor(*direction), distance + 1));
                        } else if !visited.contains(&position) {
                            visited.insert(position);
                            position
                                .neighbors()
                                .iter()
                                .for_each(|n| queue.push_back((*n, distance + 1)));
                        }
                    }
                    Some(Tile::Start) => {
                        visited.insert(position);
                        queue.push_back((position.neighbor(Direction::South), distance + 1))
                    }
                    Some(Tile::End) | None => {}
                }
            }
        }
    }

    graph
}
