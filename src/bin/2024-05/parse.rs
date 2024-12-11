use std::collections::HashMap;

pub(super) fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (input_ordering, input_updates) = input.trim_end_matches('\n').split_once("\n\n").unwrap();

    let mut ordering = HashMap::new();
    for line in input_ordering.lines() {
        let (left, right) = line.split_once('|').unwrap();
        let left = left.parse::<u32>().unwrap();
        let right = right.parse::<u32>().unwrap();

        ordering.entry(left).or_insert_with(Vec::new).push(right);
    }

    let mut updates = Vec::new();
    for line in input_updates.lines() {
        updates.push(
            line.split(',')
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    (ordering, updates)
}
