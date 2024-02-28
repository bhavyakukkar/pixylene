use std::fmt;

use crate::types::{ PCoord };


/// An integer coordinate type composed of two 32-bit integers.
#[derive(Copy, Clone, PartialEq, Default, Debug, Savefile)]
pub struct Coord { pub x: i32, pub y: i32 }

impl Coord {
    pub fn area(&self) -> i64 {
        i64::from(self.x) * i64::from(self.y)
    }
    pub fn zero() -> Self {
        Self{ x: 0, y: 0 }
    }
    pub fn add(self, coord: Coord) -> Self {
        Self{ x: self.x + coord.x, y: self.y + coord.y }
    }
    pub fn multiply(self, coord: Coord) -> Self {
        Self{ x: self.x * coord.x, y: self.y * coord.y }
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
