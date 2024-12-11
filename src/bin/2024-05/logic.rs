use std::collections::HashMap;

fn is_page_ordered(
    ordering: &HashMap<u32, Vec<u32>>,
    update: &[u32],
    index: usize,
) -> Result<(), usize> {
    let page = update.get(index).unwrap();
    if let Some(superior_pages) = ordering.get(page) {
        for (previous_index, previous_page) in update[..=index].iter().enumerate() {
            if superior_pages.contains(previous_page) {
                return Err(previous_index);
            }
        }
    }

    Ok(())
}

pub(super) fn is_update_correct(ordering: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    for index in 1..update.len() {
        if is_page_ordered(ordering, update, index).is_err() {
            return false;
        }
    }

    true
}

pub(super) fn find_correct_updates(
    ordering: &HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut correct = Vec::new();
    let mut incorrect = Vec::new();

    for update in updates {
        if is_update_correct(ordering, &update) {
            correct.push(update);
        } else {
            incorrect.push(update);
        }
    }

    (correct, incorrect)
}

pub(super) fn fix_update(ordering: &HashMap<u32, Vec<u32>>, mut update: Vec<u32>) -> Vec<u32> {
    let mut index = 0;
    let length = update.len();

    while index < length {
        if let Err(correct_index) = is_page_ordered(ordering, &update, index) {
            let page = update.remove(index);
            update.insert(correct_index, page);

            index = correct_index;
        } else {
            index += 1;
        }
    }

    update
}
