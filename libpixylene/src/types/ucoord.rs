use std::fmt;

/// An unsigned integer coordinate type composed of two 16-bit unsigned integers.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Savefile)]
pub struct UCoord { pub x: u16, pub y: u16 }

impl UCoord {
    pub fn new(x: u16, y: u16) -> UCoord {
        UCoord{ x, y }
    }
    pub fn zero() -> UCoord {
        UCoord{ x: 0, y: 0 }
    }
    pub fn area(&self) -> u32 {
        u32::from(self.x) * u32::from(self.y)
    }
}

impl fmt::Display for UCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(usize, usize)> for UCoord {
    fn from(item: (usize, usize)) -> UCoord {
        UCoord{ x: u16::try_from(item.0).unwrap(), y: u16::try_from(item.1).unwrap() }
    }
}
