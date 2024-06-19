use super::UCoord;

use std::fmt;
use serde::{ Serialize, Deserialize };

/// A `P`ositive `Coord`inate type composed of two positive (>= 1) unsigned integers.
///
/// `This type can not be constructed directly, use `[`PCoord::new`][new]` to construct.`
///
/// [new]: #method.new
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Default, Debug, Savefile)]
pub struct PCoord<T=u16>
where T: Into<u128> + Copy
{
    x: T,
    y: T,
}


impl PCoord<u16> {
    /// The largest value allowed as a coordinate of a PCoord
    pub const MAX: isize = u16::MAX as isize;

    /// The smallest value allowed as a coordinate of a PCoord
    pub const MIN: isize = 1;

    /// Returns the product of the PCoord's coordinates
    pub fn area(&self) -> u32 {
        u32::from(self.x) * u32::from(self.y)
    }
}

impl PCoord<u32> {
    /// The largest value allowed as a coordinate of a PCoord
    pub const MAX: isize = u32::MAX as isize;

    /// The smallest value allowed as a coordinate of a PCoord
    pub const MIN: isize = 1;

    /// Returns the product of the PCoord's coordinates
    pub fn area(&self) -> u64 {
        u64::from(self.x) * u64::from(self.y)
    }
}


impl<T: Into<u128> + TryFrom<u128> + Copy> PCoord<T> {

    /// Tries to construct & return a new PCoord with the given 'x' and 'y' coordinates, failing if
    /// any of the coordinates are 0
    pub fn new(x: T, y: T) -> Result<PCoord<T>, ()> {
        if x.into() != 0 && y.into() != 0u128 { Ok(PCoord{x, y}) }
        else { Err(()) }
    }

    /// Gets the 'x' coordinate of the PCoord
    pub fn x(&self) -> T { self.x }

    /// Tries to set the 'x' coordinate of the PCoord, failing if 0 is provided
    pub fn set_x(&mut self, x: T) -> Result<(), ()> {
        if x.into() != 0u128 {
            self.x = x;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Gets the 'y' coordinate of the PCoord
    pub fn y(&self) -> T { self.y }

    /// Tries to set the 'y' coordinate of the PCoord, failing if 0 is provided
    pub fn set_y(&mut self, y: T) -> Result<(), ()> {
        if y.into() != 0u128 {
            self.y = y;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn mul(self, other: Self) -> Result<Self, ()> {
        Ok(Self {
            x: T::try_from(self.x.into() * other.x.into())
                .map_err(|_| ())?,
            y: T::try_from(self.x.into() * other.x.into())
                .map_err(|_| ())?,
        })
    }
}

/// Contains a PCoord, because compiler won't let me impl [From<PCoord<S>>] for [PCoord<T>] without
/// this
pub struct PCoordContainer<T: Into<u128> + Copy>(pub PCoord<T>);

impl<S, T> From<PCoord<S>> for PCoordContainer<T>
where
    S: Into<u128> + Copy,
    T: Into<u128> + Copy + From<S>,
{
    fn from(item: PCoord<S>) -> PCoordContainer<T> {
        PCoordContainer(PCoord{ x: T::from(item.x), y: T::from(item.y) })
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
