use crate::utils::messages::{ DIVZERO, CMPSTMSG };

use std::fmt;


/// An RGBA quadrant to represent a color, composed of 8-bit red, green, blue & alpha values.
#[derive(Debug, Copy, Clone, Savefile)]
pub struct Pixel {
    /// red level (0-255)
    pub r: u8,
    /// green level (0-255)
    pub g: u8,
    /// blue level (0-255)
    pub b: u8,
    /// alpha level (0-255)
    pub a: u8,
}

impl Pixel {

    /// Tries to create a Pixel from a CSS-like hex-triplet string (6-digit or 8-digit),
    /// eg: "#239920"
    ///
    /// This method may fail with the [`HexError`][he] or [`BytesLength`][bl] error variants only.
    ///
    /// [he]: PixelError::HexError
    /// [bl]: PixelError::BytesLength
    pub fn from_hex(color_hex: &str) -> Result<Pixel, PixelError> {
        use PixelError::{ HexError, BytesLength };
        let color_hex = String::from(color_hex);

        match hex::decode(color_hex.get(1..).ok_or(BytesLength(0))?) {
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

    /// Returns an empty #00000000 i.e. (0,0,0,0) pixel
    pub fn empty() -> Pixel {
        Pixel{ r: 0, g: 0, b: 0, a: 0 }
    }

    /// Returns a solid black #000000ff i.e. (0,0,0,255) pixel
    pub fn black() -> Pixel {
        Pixel{ r: 0, g: 0, b: 0, a: 255 }
    }

    /// Darken operation as descibed by [`Porter & Duff`][pd]
    ///
    /// [pd]: https://dl.acm.org/doi/abs/10.1145/800031.808606
    pub fn darken(&mut self, factor: u8) {
        self.r = (self.r as u16 * factor as u16).checked_div(255)
            .expect(DIVZERO).try_into().expect(CMPSTMSG);
        self.g = (self.g as u16 * factor as u16).checked_div(255)
            .expect(DIVZERO).try_into().expect(CMPSTMSG);
        self.b = (self.b as u16 * factor as u16).checked_div(255)
            .expect(DIVZERO).try_into().expect(CMPSTMSG);
    }

    /// Dissolve operation as descibed by [`Porter & Duff`][pd]
    ///
    /// [pd]: https://dl.acm.org/doi/abs/10.1145/800031.808606
    pub fn dissolve(&mut self, factor: u8) {
        self.r = (self.r as u16 * factor as u16).checked_div(255)
            .expect(DIVZERO).try_into().expect(CMPSTMSG);
        self.g = (self.g as u16 * factor as u16).checked_div(255)
            .expect(DIVZERO).try_into().expect(CMPSTMSG);
        self.b = (self.b as u16 * factor as u16).checked_div(255)
            .expect(DIVZERO).try_into().expect(CMPSTMSG);
        self.a = (self.a as u16 * factor as u16).checked_div(255)
            .expect(DIVZERO).try_into().expect(CMPSTMSG);
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

/// Error enum to describe various errors returns by Pixel methods
#[derive(Debug)]
pub enum PixelError {

    /// Error propagated by the [`hex`] crate when trying to parse the hex-string passed to
    /// [`from_hex`](Pixel::from_hex)
    HexError(String, hex::FromHexError),

    /// Error that occurs when the parsed hex-string passed to [`from_hex`](Pixel::from_hex) is not
    /// of appropriate length to construct an RGB or an RGBA pixel
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
