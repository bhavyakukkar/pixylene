use std::fmt;

/// A positive coordinate type composed of two positive (1 or greater) 16-bit unsigned integers.
#[derive(Copy, Clone, PartialEq, Default, Debug, Savefile)]
pub struct PCoord { x: u16, y: u16 }

impl PCoord {
    pub fn new(x: u16, y: u16) -> Result<Self, ()> {
        if x > 0 && y > 0 { Ok(PCoord{x, y}) }
        else { Err(()) }
    }

    pub fn x(&self) -> u16 { self.x }
    pub fn set_x(&mut self, x: u16) -> Result<(), ()> {
        if x > 0 {
            self.x = x;
            return Ok(());
        } else {
            Err(())
        }
    }

    pub fn y(&self) -> u16 { self.y }
    pub fn set_y(&mut self, y: u16) -> Result<(), ()> {
        if y > 0 {
            self.y = y;
            return Ok(());
        } else {
            Err(())
        }
    }

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
