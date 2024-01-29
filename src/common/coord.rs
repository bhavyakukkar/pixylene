use std::fmt;

#[derive(Copy, Clone, PartialEq, Default, Debug, Savefile)]
pub struct Coord { pub x: isize, pub y: isize }

impl Coord {
    pub fn area(&self) -> isize {
        self.x * self.y
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
