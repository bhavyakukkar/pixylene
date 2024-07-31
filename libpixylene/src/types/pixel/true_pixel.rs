use super::Pixel;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An RGBA quadrant to represent a color, composed of 8-bit red, green, blue & alpha values
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone, Savefile)]
pub struct TruePixel {
    /// red level (0-255)
    pub r: u8,
    /// green level (0-255)
    pub g: u8,
    /// blue level (0-255)
    pub b: u8,
    /// alpha level (0-255)
    pub a: u8,
}

impl Pixel for TruePixel {
    /// Returns an empty #00000000 i.e. (0,0,0,0) pixel
    fn empty() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

impl TruePixel {
    /// The Black color with full opacity i.e. #000000
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    /// The Black color with no opacity i.e. #00000000
    pub const EMPTY: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    /// My favourite color with full opacity i.e. #f5abb9
    pub const FAVOURITE: Self = Self {
        r: 245,
        g: 171,
        b: 185,
        a: 255,
    };

    /// Tries to create a Pixel from a CSS-like hex-triplet string (6-digit or 8-digit),
    /// eg: "#239920"
    ///
    /// This method may fail with the [`HexError`][he] or [`BytesLength`][bl] error variants only.
    ///
    /// [he]: TruePixelError::HexError
    /// [bl]: TruePixelError::BytesLength
    pub fn from_hex(color_hex: &str) -> Result<Self, TruePixelError> {
        use TruePixelError::{BytesLength, HexError};
        let color_hex = String::from(color_hex);

        match hex::decode(color_hex.get(1..).ok_or(BytesLength(0))?) {
            Ok(bytes) => match bytes.len() {
                4 => Ok(Self {
                    r: bytes[0],
                    g: bytes[1],
                    b: bytes[2],
                    a: bytes[3],
                }),
                3 => Ok(Self {
                    r: bytes[0],
                    g: bytes[1],
                    b: bytes[2],
                    a: 255,
                }),
                l => Err(BytesLength(l)),
            },
            Err(from_hex_error) => Err(HexError(color_hex, from_hex_error)),
        }
    }

    /// Returns a solid black #000000ff i.e. (0,0,0,255) pixel
    pub fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    /// Darken operation as descibed by [`Porter & Duff`][pd]
    ///
    /// [pd]: https://dl.acm.org/doi/abs/10.1145/800031.808606
    pub fn darken(self, factor: u8) -> Self {
        Self {
            r: (self.r as u16 * factor as u16)
                .checked_div(255)
                .unwrap() //Clearly dividing by 255 not 0
                .try_into()
                .unwrap(), //guaranteed to be in range (0,255)
            g: (self.g as u16 * factor as u16)
                .checked_div(255)
                .unwrap() //Clearly dividing by 255 not 0
                .try_into()
                .unwrap(), //guaranteed to be in range (0,255)
            b: (self.b as u16 * factor as u16)
                .checked_div(255)
                .unwrap() //Clearly dividing by 255 not 0
                .try_into()
                .unwrap(), //guaranteed to be in range (0,255)
            a: 255,
        }
    }

    /// Dissolve operation as descibed by [`Porter & Duff`][pd]
    ///
    /// [pd]: https://dl.acm.org/doi/abs/10.1145/800031.808606
    pub fn dissolve(self, factor: u8) -> Self {
        Self {
            r: (self.r as u16 * factor as u16)
                .checked_div(255)
                .unwrap() //Clearly dividing by 255 not 0
                .try_into()
                .unwrap(), //guaranteed to be in range (0,255)
            g: (self.g as u16 * factor as u16)
                .checked_div(255)
                .unwrap() //Clearly dividing by 255 not 0
                .try_into()
                .unwrap(), //guaranteed to be in range (0,255)
            b: (self.b as u16 * factor as u16)
                .checked_div(255)
                .unwrap() //Clearly dividing by 255 not 0
                .try_into()
                .unwrap(), //guaranteed to be in range (0,255)
            a: (self.a as u16 * factor as u16)
                .checked_div(255)
                .unwrap() //Clearly dividing by 255 not 0
                .try_into()
                .unwrap(), //guaranteed to be in range (0,255)
        }
    }
}

impl fmt::Display for TruePixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self { r, g, b, a } = self;
        write!(
            f,
            "#{:0>2}{:0>2}{:0>2}{:0>2}",
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
pub enum TruePixelError {
    /// Error propagated by the [`hex`] crate when trying to parse the hex-string passed to
    /// [`from_hex`](TruePixel::from_hex)
    HexError(String, hex::FromHexError),

    /// Error that occurs when the parsed hex-string passed to [`from_hex`](TruePixel::from_hex) is
    /// not of appropriate length to construct an RGB or an RGBA pixel
    BytesLength(usize),
}
impl fmt::Display for TruePixelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TruePixelError::*;
        match self {
            HexError(color, error) => {
                write!(f, "failed to parse hex color '{}': {}", color, error,)
            }
            BytesLength(length) => write!(
                f,
                "invalid length of bytes for hex color, expecting 3 (RGB) or 4 (RGBA), found: {}",
                length,
            ),
        }
    }
}
