use std::fmt;

/// An `U`nsigned `Coord`inate type composed of two 16-bit unsigned integers.
///
/// `This type can be constructed directly`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Savefile)]
pub struct UCoord {
    /// The 'x' coordinate of the UCoord
    pub x: u16,
    /// The 'y' coordinate of the UCoord
    pub y: u16
}

impl UCoord {

    /// Returns a UCoord with coordinates (0,0)
    pub fn zero() -> UCoord {
        UCoord{ x: 0, y: 0 }
    }

    /// Returns the product of the UCoord's coordinates
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
