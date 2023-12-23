mod coordinate;
mod coordinate3d;
mod scored_item;

pub use coordinate::Coordinate;
pub use coordinate3d::Coordinate3D;
pub use scored_item::ScoredItem;

/// Cardinal directions
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns an iterator over all possible directions
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::North, Self::South, Self::East, Self::West]
            .iter()
            .copied()
    }

    /// Rotate the direction clockwise
    pub fn rotate_cw(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    /// Rotate the direction counter-clockwise
    pub fn rotate_ccw(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    /// Opposite direction
    pub fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    /// List of directions after a turn
    pub fn turns(self) -> [Self; 2] {
        match self {
            Self::North | Self::South => [Self::East, Self::West],
            Self::East | Self::West => [Self::North, Self::South],
        }
    }
}
