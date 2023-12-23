use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use num::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Coordinate3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

/// Renders Coordinate as `(x,y)`
impl<T: Display> Display for Coordinate3D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<T: Add<Output = T>> Add for Coordinate3D<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate3D<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Coordinate3D<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: SubAssign> SubAssign for Coordinate3D<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Integer + Copy> Coordinate3D<T> {
    pub fn manhattan_distance(&self, other: &Self) -> T {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
            + self.z.max(other.z)
            - self.z.min(other.z)
    }
}
