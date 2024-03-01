use std::fmt;

use crate::types::{ PCoord };


/// An integer coordinate type composed of two 32-bit integers.
/// This type can be constructed directly
#[derive(Copy, Clone, PartialEq, Default, Debug, Savefile)]
pub struct Coord { pub x: i32, pub y: i32 }

impl Coord {

    /// Returns the product of the Coord's coordinates
    pub fn area(&self) -> i64 {
        i64::from(self.x) * i64::from(self.y)
    }

    /// Returns a Coord with coordinates 0, 0
    pub fn zero() -> Self {
        Self{ x: 0, y: 0 }
    }

    /// Return a Coord composed of the overflowing sums of two Coord's coordinates
    /// This method consumes both its parent and the argument; to mutably add a coordinate to an
    /// existing coordinate, use [`add_mut`](#method.add_mut)
    pub fn add(self, coord: Coord) -> Self {
        Self{ x: self.x.overflowing_add(coord.x).0, y: self.y.overflowing_add(coord.y).0 }
    }

    /// Add another Coord's coordinates into this Coord's coordinates overflowingly
    pub fn add_mut(&mut self, coord: &Coord) {
        self.x = self.x.overflowing_add(coord.x).0;
        self.y = self.y.overflowing_add(coord.y).0;
    }

    /// Return a Coord composed of the overflowing products of two Coord's coordinates
    /// This method consumes both its parent and the argument; to mutably multiply a coordinate to
    /// an existing coordinate, use [`mul_mut`](#method.mul_mut)
    pub fn mul(self, coord: Coord) -> Self {
        Self{ x: self.x.overflowing_mul(coord.x).0, y: self.y.overflowing_mul(coord.y).0 }
    }

    /// Multiply another Coord's coordinates into this Coord's coordinates overflowingly
    pub fn mul_mut(&mut self, coord: &Coord) {
        self.x = self.x.overflowing_mul(coord.x).0;
        self.y = self.y.overflowing_mul(coord.y).0;
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(isize, isize)> for Coord {
    fn from(item: (isize, isize)) -> Coord {
        Coord{ x: i32::try_from(item.0).unwrap(), y: i32::try_from(item.1).unwrap() }
    }
}
impl From<&PCoord> for Coord {
    fn from(item: &PCoord) -> Coord {
        Coord{ x: i32::from(item.x()), y: i32::from(item.y()) }
    }
}


// Error Types

/// Error enum to describe various errors returned by Coord methods
#[derive(Debug)]
pub enum CoordError {
    AddOverflow(Coord, Coord),
    MulOverflow(Coord, Coord),
}
impl fmt::Display for CoordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CoordError::*;
        match self {
            AddOverflow(first, second) => write!(
                f,
                "overflow occurred while adding the two coordinates: {} and {}",
                first,
                second,
            ),
            MulOverflow(first, second) => write!(
                f,
                "overflow occurred while multiplying the two coordinates: {} and {}",
                first,
                second,
            ),
        }
    }
}
