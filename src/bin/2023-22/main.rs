use brick::Brick;

mod brick;
mod parse;

advent_of_code::solution!(2023, 22);

fn drop_brick(bricks: &[Brick<i32>], brick: Brick<i32>) -> Brick<i32> {
    let mut brick = brick;
    let mut dropped_brick;

    while brick.start().z > 1 {
        dropped_brick = brick.drop();
        if bricks
            .iter()
            .filter(|b| b.end().z == dropped_brick.start().z)
            .any(|b| b.overlaps(&dropped_brick))
        {
            break;
        }

        brick = dropped_brick;
    }

    brick
}

pub fn part_one(input: &str) -> Option<u32> {
    let bricks = parse::parse::<i32>(input);
    let mut fallen_bricks: Vec<Brick<i32>> = Vec::new();

    // Drop all bricks as far as possible
    for brick in bricks {
        let dropped = drop_brick(&fallen_bricks, brick);
        fallen_bricks.push(dropped);
    }

    let mut count = 0;
    for idx in 0..fallen_bricks.len() {
        let brick = fallen_bricks.remove(idx);

        if !fallen_bricks
            .iter()
            .filter(|b| b.start().z == brick.end().z + 1)
            .any(|b| *b != drop_brick(&fallen_bricks, *b))
        {
            count += 1;
        }

        fallen_bricks.insert(idx, brick);
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let bricks = parse::parse::<i32>(input);
    let mut fallen_bricks: Vec<Brick<i32>> = Vec::new();

    // Drop all bricks as far as possible
    for brick in bricks {
        let dropped = drop_brick(&fallen_bricks, brick);
        fallen_bricks.push(dropped);
    }

    let mut count = 0;
    for desintegrated_idx in 0..fallen_bricks.len() {
        let disintegrated = fallen_bricks.remove(desintegrated_idx);
        let mut bricks_copy = fallen_bricks.clone();

        for idx in fallen_bricks
            .iter()
            .enumerate()
            .filter(|(_, b)| b.start().z > disintegrated.end().z)
            .map(|(i, _)| i)
        {
            let brick = bricks_copy.remove(idx);
            let fallen = drop_brick(&bricks_copy, brick);

            if brick != fallen {
                count += 1;
            }

            bricks_copy.insert(idx, fallen);
        }

        fallen_bricks.insert(desintegrated_idx, disintegrated);
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(375));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(72352));
    }
}
