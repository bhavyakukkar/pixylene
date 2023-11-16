use std::fmt;

#[derive(Copy, Clone, Default)]
pub struct Coord { pub x: isize, pub y: isize }

impl Coord {
    pub fn area(&self) -> isize {
        self.x * self.y
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


#[derive(Copy, Clone)]
pub enum Pixel {
    B24{ r: u8, g: u8, b: u8 },
    B8(u8)
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::B24{r, g, b} => write!(f, "#{:0>2}{:0>2}{:0>2}",
                format!("{:x}", r),
                format!("{:x}", g),
                format!("{:x}", b)
            ),
            Self::B8(c) => write!(f, "{}", c)
        }
    }
}
