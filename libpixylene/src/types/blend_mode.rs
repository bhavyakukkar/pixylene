use super::Pixel;

use std::fmt;
use serde::{ Serialize, Deserialize };


/// Enum of the different types of [blend-modes][b]
///
/// [b]: https://en.wikipedia.org/wiki/Blend_modes
#[non_exhaustive]
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone, Savefile)]
pub enum BlendMode {
    /// Composite with specified fractions of contribution by pixel `a` and pixel `b`
    /// respectively, as described by [`Porter & Duff`][pd]
    ///
    /// [pd]: https://dl.acm.org/doi/abs/10.1145/800031.808606
    Composite(u8, u8),

    /// [`Standard blend mode`][n] that will treat 'a' as the top pixel and 'b' as the bottom pixel
    ///
    /// [n]: https://en.wikipedia.org/wiki/Blend_modes#Normal_blend_mode
    Normal,

    /// Blend mode that overwrites the top pixel onto the bottom pixel
    Overwrite,
}

impl BlendMode {

    /// Blends two RGBA [`Pixels`](Pixel) using self's blend-mode variant & return the resultant
    /// [`Pixel`]
    ///
    /// `Note`: This method may fail with the [`FractionsDoNotSumToWhole`][fd] error variant only.
    ///
    /// [fd]: BlendError::FractionsDoNotSumToWhole
    pub fn blend(&self, a: Pixel, b: Pixel) -> Result<Pixel, BlendError> {
        use BlendError::{ FractionsDoNotSumToWhole };

        match self {
            Self::Composite(frac_a, frac_b) => {
                if *frac_a as u16 + *frac_b as u16 != 255 {
                    return Err(FractionsDoNotSumToWhole((*frac_a, *frac_b)));
                }

                let red: u8   = (a.r as u16 * *frac_a as u16 + b.r as u16 * *frac_b as u16)
                                .checked_div(255).unwrap() //Clearly dividing by 255 not 0
                                .try_into().unwrap(); //guaranteed to be in range (0,255)

                let green: u8 = (a.g as u16 * *frac_a as u16 + b.g as u16 * *frac_b as u16)
                                .checked_div(255).unwrap() //Clearly dividing by 255 not 0
                                .try_into().unwrap(); //guaranteed to be in range (0,255)

                let blue: u8  = (a.b as u16 * *frac_a as u16 + b.b as u16 * *frac_b as u16)
                                .checked_div(255).unwrap() //Clearly dividing by 255 not 0
                                .try_into().unwrap(); //guaranteed to be in range (0,255)

                let alpha: u8 = (a.a as u16 * *frac_a as u16 + b.a as u16 * *frac_b as u16)
                                .checked_div(255).unwrap() //Clearly dividing by 255 not 0
                                .try_into().unwrap(); //guaranteed to be in range (0,255)

                Ok(Pixel { r: red, g: green, b: blue, a: alpha })
            },
            Self::Normal => Ok(
                BlendMode::Composite(a.a, 255 - a.a).blend(a, b)
                .unwrap() //Guaranteed to sum to 255
            ),
            Self::Overwrite => Ok(a),
        }
    }
}


// Error Types

/// Error enum to describe various errors returns by BlendMode methods
#[derive(Debug)]
pub enum BlendError {
    /// Error when trying to merge pixels using a [`Composite`](BlendMode::Composite) variant
    /// whose fractions do not sum up to the whole, i.e., 255
    FractionsDoNotSumToWhole((u8, u8)),
}

impl fmt::Display for BlendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BlendError::*;
        match self {
            FractionsDoNotSumToWhole(received) => write!(
                f,
                "received Composite blend-mode fractions that do not sum up to whole (255): \
                ({frac_a},{frac_b})",
                frac_a = received.0,
                frac_b = received.1,
            ),
        }
    }
}
