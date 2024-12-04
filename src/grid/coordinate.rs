use num::{Integer, Num};
use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use super::{Direction, ExtendedDirection};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

/// Renders Coordinate as `(x,y)`
impl<T: Display> Display for Coordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Coordinate<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Coordinate<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: SubAssign> SubAssign for Coordinate<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Num> Add<Direction> for Coordinate<T> {
    type Output = Coordinate<T>;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Coordinate {
                x: self.x,
                y: self.y - T::one(),
            },
            Direction::South => Coordinate {
                x: self.x,
                y: self.y + T::one(),
            },
            Direction::East => Coordinate {
                x: self.x + T::one(),
                y: self.y,
            },
            Direction::West => Coordinate {
                x: self.x - T::one(),
                y: self.y,
            },
        }
    }
}

impl<T: Num> Add<ExtendedDirection> for Coordinate<T> {
    type Output = Coordinate<T>;

    fn add(self, rhs: ExtendedDirection) -> Self::Output {
        match rhs {
            ExtendedDirection::North => Coordinate {
                x: self.x,
                y: self.y - T::one(),
            },
            ExtendedDirection::South => Coordinate {
                x: self.x,
                y: self.y + T::one(),
            },
            ExtendedDirection::East => Coordinate {
                x: self.x + T::one(),
                y: self.y,
            },
            ExtendedDirection::West => Coordinate {
                x: self.x - T::one(),
                y: self.y,
            },
            ExtendedDirection::NorthEast => Coordinate {
                x: self.x + T::one(),
                y: self.y - T::one(),
            },
            ExtendedDirection::NorthWest => Coordinate {
                x: self.x - T::one(),
                y: self.y - T::one(),
            },
            ExtendedDirection::SouthEast => Coordinate {
                x: self.x + T::one(),
                y: self.y + T::one(),
            },
            ExtendedDirection::SouthWest => Coordinate {
                x: self.x - T::one(),
                y: self.y + T::one(),
            },
        }
    }
}

// Y values are treated as more significant than X values
impl<T: Eq + PartialEq + Ord + Copy> Ord for Coordinate<T> {
    // Reading order: Y then X
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}
impl<T: Ord + PartialOrd + Copy> PartialOrd for Coordinate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Copy + 'static> Coordinate<T> {
    pub fn from<U: num::cast::AsPrimitive<T>>(other: Coordinate<U>) -> Coordinate<T> {
        Coordinate {
            x: other.x.as_(),
            y: other.y.as_(),
        }
    }
}

impl<T: Integer + Copy> Coordinate<T> {
    pub fn manhattan_distance(&self, other: &Self) -> T {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }

    /// Returns the 4 cardinal neighbors of this coordinate
    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x + T::one(),
                y: self.y,
            },
            Self {
                x: self.x - T::one(),
                y: self.y,
            },
            Self {
                x: self.x,
                y: self.y + T::one(),
            },
            Self {
                x: self.x,
                y: self.y - T::one(),
            },
        ]
    }

    pub fn neighbor(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - T::one(),
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + T::one(),
            },
            Direction::East => Self {
                x: self.x + T::one(),
                y: self.y,
            },
            Direction::West => Self {
                x: self.x - T::one(),
                y: self.y,
            },
        }
    }
}
