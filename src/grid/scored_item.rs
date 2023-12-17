use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScoredItem<N, T> {
    pub item: T,
    pub score: N,
}

impl<N: Ord, T: Ord> Ord for ScoredItem<N, T> {
    fn cmp(&self, other: &ScoredItem<N, T>) -> Ordering {
        (&other.score, &other.item).cmp(&(&self.score, &self.item))
    }
}

impl<N: Ord, T: Ord> PartialOrd for ScoredItem<N, T> {
    fn partial_cmp(&self, other: &ScoredItem<N, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::*;

    #[test]
    fn test_scored_item() {
        let mut heap = BinaryHeap::new();
        heap.push(ScoredItem {
            item: 1,
            score: 1,
        });
        heap.push(ScoredItem {
            item: 2,
            score: 1,
        });
        heap.push(ScoredItem {
            item: 3,
            score: -2,
        });
        heap.push(ScoredItem {
            item: 4,
            score: 4,
        });
        heap.push(ScoredItem {
            item: 5,
            score: 5,
        });
        heap.push(ScoredItem {
            item: 6,
            score: 6,
        });

        assert_eq!(heap.pop().unwrap().item, 3);
        assert_eq!(heap.pop().unwrap().item, 1);
        assert_eq!(heap.pop().unwrap().item, 2);
        assert_eq!(heap.pop().unwrap().item, 4);
        assert_eq!(heap.pop().unwrap().item, 5);
        assert_eq!(heap.pop().unwrap().item, 6);
    }
}
