use advent_of_code::grid::Coordinate3D;
use num::Num;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Brick<T: Ord + Copy> {
    start: Coordinate3D<T>,
    end: Coordinate3D<T>,
}

impl<T: Ord + Copy> Brick<T> {
    /// Create a new brick while ensuring that the start has a lower
    /// `z` coordinate than the end.
    pub fn new(start: Coordinate3D<T>, end: Coordinate3D<T>) -> Self {
        // Ensure that the brick is 1D
        let x = start.x == end.x;
        let y = start.y == end.y;
        let z = start.z == end.z;
        assert!((z || y) && x || (y && z));

        let mut coords = [start, end];
        coords.sort_by_key(|c| c.x);
        coords.sort_by_key(|c| c.y);
        coords.sort_by_key(|c| c.z);

        Self {
            start: coords[0],
            end: coords[1],
        }
    }

    /// Check if two bricks overlap by even one cube.
    pub fn overlaps(&self, other: &Self) -> bool {
        !(self.start.x > other.end.x
            || self.end.x < other.start.x
            || self.start.y > other.end.y
            || self.end.y < other.start.y
            || self.start.z > other.end.z
            || self.end.z < other.start.z)
    }
}

impl<T: Num + Ord + Copy> Brick<T> {
    pub fn drop(&self) -> Self {
        Brick::new(
            Coordinate3D::new(self.start.x, self.start.y, self.start.z - T::one()),
            Coordinate3D::new(self.end.x, self.end.y, self.end.z - T::one()),
        )
    }
}

impl<T: Ord + Copy> Brick<T> {
    pub fn start(&self) -> Coordinate3D<T> {
        self.start
    }

    pub fn end(&self) -> Coordinate3D<T> {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps() {
        let b0 = Brick::new(Coordinate3D::new(0, 0, 0), Coordinate3D::new(0, 0, 0));
        let b1 = Brick::new(Coordinate3D::new(0, 0, 0), Coordinate3D::new(3, 0, 0));
        let b2 = Brick::new(Coordinate3D::new(0, 1, 0), Coordinate3D::new(0, 0, 0));
        let b3 = Brick::new(Coordinate3D::new(0, 0, 2), Coordinate3D::new(0, 0, 0));
        let b4 = Brick::new(Coordinate3D::new(1, 0, 1), Coordinate3D::new(1, 3, 1));
        let b5 = Brick::new(Coordinate3D::new(1, 0, 0), Coordinate3D::new(1, 3, 0));

        assert_eq!(b0.overlaps(&b0), true);
        assert_eq!(b0.overlaps(&b1), true);
        assert_eq!(b0.overlaps(&b2), true);
        assert_eq!(b0.overlaps(&b3), true);
        assert_eq!(b1.overlaps(&b2), true);
        assert_eq!(b1.overlaps(&b3), true);
        assert_eq!(b2.overlaps(&b3), true);

        assert_eq!(b1.overlaps(&b4), false);
        assert_eq!(b1.overlaps(&b5), true);
    }
}
