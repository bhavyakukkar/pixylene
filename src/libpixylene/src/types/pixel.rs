use std::fmt;


#[derive(Copy, Clone, Savefile)]
pub struct Pixel {
    pub r: u8,  // r: red (0-255)
    pub g: u8,  // g: green (0-255)
    pub b: u8,  // b: blue (0-255)
    pub a: u8,  // a: alpha (0-255)
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


// Error Types

#[derive(Debug)]
pub enum PixelError {
    HexError(String, hex::FromHexError),
    BytesLength(usize),
}
impl fmt::Display for PixelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
