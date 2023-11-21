use std::fmt;

#[derive(Copy, Clone, Default)]
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
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


#[derive(Copy, Clone)]
pub struct Pixel {
    pub r: u8,  // r: red (0-255)
    pub g: u8,  // g: green (0-255)
    pub b: u8,  // b: blue (0-255)
    pub a: u8,  // a: alpha (0-255)
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self{ r, g, b, a } = self;
        write!(f, "#{:0>2}{:0>2}{:0>2}{:0>2}",
            format!("{:x}", r),
            format!("{:x}", g),
            format!("{:x}", b),
            format!("{:x}", a)
        )
    }
}
