use super::UCoord;

use std::fmt;
use serde::{ Serialize, Deserialize };

/// A `P`ositive `Coord`inate type composed of two positive (>= 1) 16-bit unsigned integers.
///
/// `This type can not be constructed directly, use `[`PCoord::new`][new]` to construct.`
///
/// [new]: #method.new
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Default, Debug, Savefile)]
pub struct PCoord { x: u16, y: u16 }

impl PCoord {
    /// The largest value allowed as a coordinate of a PCoord
    pub const MAX: isize = u16::MAX as isize;

    /// The smallest value allowed as a coordinate of a PCoord
    pub const MIN: isize = 1;


    /// Tries to construct & return a new PCoord with the given 'x' and 'y' coordinates
    pub fn new(x: u16, y: u16) -> Result<Self, ()> {
        if x > 0 && y > 0 { Ok(PCoord{x, y}) }
        else { Err(()) }
    }

    /// Gets the 'x' coordinate of the PCoord
    pub fn x(&self) -> u16 { self.x }

    /// Tries to set the 'x' coordinate of the PCoord
    pub fn set_x(&mut self, x: u16) -> Result<(), ()> {
        if x > 0 {
            self.x = x;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Gets the 'y' coordinate of the PCoord
    pub fn y(&self) -> u16 { self.y }

    /// Tries to set the 'y' coordinate of the PCoord
    pub fn set_y(&mut self, y: u16) -> Result<(), ()> {
        if y > 0 {
            self.y = y;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Returns the product of the PCoord's coordinates
    pub fn area(&self) -> u32 {
        u32::from(self.x) * u32::from(self.y)
    }
}

impl fmt::Display for PCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(usize, usize)> for PCoord {
    fn from(item: (usize, usize)) -> PCoord {
        PCoord{ x: u16::try_from(item.0).unwrap(), y: u16::try_from(item.1).unwrap() }
    }
}

impl TryFrom<UCoord> for PCoord {
    type Error = String;
    fn try_from(item: UCoord) -> Result<PCoord, String> {
        if item.x == 0 {
            Err(String::from("Cannot convert to PCoord since x of UCoord found to be 0"))
        }
        else if item.y == 0 {
            Err(String::from("Cannot convert to PCoord since y of UCoord found to be 0"))
        }
        else {
            Ok(PCoord::new(item.x, item.y).unwrap()) //wont fail
        }
    }
}
