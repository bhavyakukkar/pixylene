use super::Pixel;

use std::fmt;


const DIVZERO: &str  = "Clearly dividing by 255 not 0";
const SUM255: &str   = "Will guaranteed sum to 0";
const CMPSTMSG: &str = "Since (frac_a + frac_b) is in range (0,255), range of computed composite \
                        is guaranteed to be in range (0,255)";

/// Enum of the different types of [blend-modes][b]
///
/// [b]: https://en.wikipedia.org/wiki/Blend_modes
#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum BlendMode {
    /// Composite with specified fractions of contribution by pixel [`a`] and pixel [`b`]
    /// respectively, as described by [`Porter & Duff`][pd]
    ///
    /// [pd]: https://dl.acm.org/doi/abs/10.1145/800031.808606
    Composite(u8, u8),

    /// [Standard blend mode][n]
    ///
    /// [n]: https://en.wikipedia.org/wiki/Blend_modes#Normal_blend_mode
    Normal,

    /// Blend mode that overwrites the top pixel onto the bottom pixel
    Overwrite,
}

impl BlendMode {

    /// Blend two RGBA [`Pixel`][P] using self's blend-mode variant & return the resultant Pixel
    ///
    /// [P]: Pixel
    pub fn blend(&self, a: Pixel, b: Pixel) -> Result<Pixel, BlendError> {
        use BlendError::{ FractionsDoNotSumToOne };

        match self {
            Self::Composite(frac_a, frac_b) => {
                if *frac_a as u16 + *frac_b as u16 != 255 {
                    return Err(FractionsDoNotSumToOne((*frac_a, *frac_b)));
                }

                let red: u8   = (a.r as u16 * *frac_a as u16 + b.r as u16 * *frac_b as u16)
                                .checked_div(255).expect(DIVZERO).try_into().expect(CMPSTMSG);

                let green: u8 = (a.r as u16 * *frac_a as u16 + b.r as u16 * *frac_b as u16)
                                .checked_div(255).expect(DIVZERO).try_into().expect(CMPSTMSG);

                let blue: u8  = (a.r as u16 * *frac_a as u16 + b.r as u16 * *frac_b as u16)
                                .checked_div(255).expect(DIVZERO).try_into().expect(CMPSTMSG);

                let alpha: u8 = (a.r as u16 * *frac_a as u16 + b.r as u16 * *frac_b as u16)
                                .checked_div(255).expect(DIVZERO).try_into().expect(CMPSTMSG);

                Ok(Pixel { r: red, g: green, b: blue, a: alpha })
            }
            Self::Normal => { Ok(BlendMode::Composite(a.a, 255 - a.a).blend(a, b).expect(SUM255)) }
            Self::Overwrite => { Ok(a) }
        }
    }
}


// Error Types

/// Error enum to describe various errors returns by BlendMode methods
#[derive(Debug)]
pub enum BlendError {
    FractionsDoNotSumToOne((u8, u8)),
}
impl fmt::Display for BlendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BlendError::*;
        match self {
            FractionsDoNotSumToOne(received) => write!(
                f,
                "received Composite blend-mode fractions that do not sum up to one: \
                ({frac_a},{frac_b})",
                frac_a = received.0,
                frac_b = received.1,
            ),
        }
    }
}
