use std::collections::HashMap;

use num::Num;

use super::Coordinate;

pub struct SparseGrid<T, V> {
    pub width: T,
    pub height: T,
    pub objects: HashMap<Coordinate<T>, V>,
}

impl<T: PartialOrd + Num, V> SparseGrid<T, V> {
    pub fn within_bounds(&self, other: Coordinate<T>) -> bool {
        other.x >= T::zero()
            && other.y >= T::zero()
            && other.x <= self.width
            && other.y <= self.height
    }
}
