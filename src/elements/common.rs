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


#[derive(Copy, Clone, Savefile)]
pub struct Pixel {
    pub r: u8,  // r: red (0-255)
    pub g: u8,  // g: green (0-255)
    pub b: u8,  // b: blue (0-255)
    pub a: u8,  // a: alpha (0-255)
}
#[derive(Debug)]
pub enum PixelError {
    HexError(String, hex::FromHexError),
    BytesLength(usize),
}
impl std::fmt::Display for PixelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PixelError::*;
        match self {
            HexError(color, error) => write!(
                f,
                "failed to parse hex color '{}': {}",
                color,
                error,
            ),
            BytesLength(length) => write!(
                f,
                "invalid length of bytes for hex color, expecting 3 (RGB) or 4 (RGBA), found: {}",
                length,
            ),
        }
    }
}

impl Pixel {
    pub fn from_hex(color_hex: String) -> Result<Pixel, PixelError> {
        use PixelError::{ HexError, BytesLength };
        match hex::decode(color_hex.clone()) {
            Ok(bytes) => {
                match bytes.len() {
                    4 => Ok(Pixel{ r: bytes[0], g: bytes[1], b: bytes[2], a: bytes[3] }),
                    3 => Ok(Pixel{ r: bytes[0], g: bytes[1], b: bytes[2], a: 255 }),
                    l => Err(BytesLength(l)),
                }
            },
            Err(from_hex_error) => Err(HexError(color_hex, from_hex_error))
        }
    }
    pub fn empty() -> Pixel {
        Pixel{ r: 0, g: 0, b: 0, a: 0 }
    }
    pub fn background() -> Pixel {
        Pixel{ r: 0, g: 0, b: 0, a: 255 }
    }
    pub fn get_certain(pixel_maybe: Option<Pixel>) -> Pixel {
        match pixel_maybe {
            Some(pixel) => pixel,
            None => Pixel::empty()
        }
    }
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


pub enum BlendMode {
    Overwrite,
    Normal,
}
impl BlendMode {
    pub fn merge_down(&self, top: Pixel, bottom: Pixel) -> Pixel {
        match self {
            Self::Overwrite => {
                top
            },
            Self::Normal => {
                //todo!();
                if top.a == 255 {
                    top
                }
                else if top.a == 0 {
                    bottom
                }
                else {
                    let mut sum: Pixel = Pixel{ r: 0, g: 0, b: 0, a: 0 };
                    sum.a = top.a + ((bottom.a / 255)*(255 - top.a));
                    sum.r = (((top.a*top.r) + (bottom.a*bottom.r)) as u16/510).try_into().unwrap();
                    sum.g = (((top.a*top.g) + (bottom.a*bottom.g)) as u16/510).try_into().unwrap();
                    sum.b = (((top.a*top.b) + (bottom.a*bottom.b)) as u16/510).try_into().unwrap();
                    sum
                }
                /*
                let r = (((top.a as f32)*(top.r as f32))/((top.a as u16 + bottom.a as u16) as f32) + (((bottom.a as f32)*(bottom.r as f32))/((top.a as u16 + bottom.a as u16) as f32))) as u8;
                let g = (((top.a as f32)*(top.g as f32))/((top.a as u16 + bottom.a as u16) as f32) + (((bottom.a as f32)*(bottom.g as f32))/((top.a as u16 + bottom.a as u16) as f32))) as u8;
                let b = (((top.a as f32)*(top.b as f32))/((top.a as u16 + bottom.a as u16) as f32) + (((bottom.a as f32)*(bottom.b as f32))/((top.a as u16 + bottom.a as u16) as f32))) as u8;
                let a = std::cmp::max(0u16, std::cmp::min(255u16, bottom.a as u16 + (((top.a as f32)/((256 as u16 - bottom.a as u16) as f32)) as u16))) as u8;
                Pixel{ r, g, b, a }
                */
            },
        }
    }
}
