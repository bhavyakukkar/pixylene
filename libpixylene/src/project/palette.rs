use crate::{
    types::{TruePixel, TruePixelError},
    utils::messages::{EQUIPPEDISINPALETTE, PALETTELEN},
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A `Palette` containing a set of [`true-color pixels`](TruePixel) (maximum 256)
///
/// The palette works by using a hashmap of u8 indexes to pixel definitions, and the most
/// significant color at any time can be chosen by its index and picked.
#[derive(Debug, Serialize, Deserialize, PartialEq, Savefile, Clone)]
pub struct Palette {
    colors: HashMap<u8, TruePixel>,
    equipped: Option<u8>,
}

impl Palette {
    pub const GRUVBOX_COLORS: [(u8, TruePixel); 8] = [
        (
            1,
            TruePixel {
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ),
        (
            2,
            TruePixel {
                r: 235,
                g: 219,
                b: 178,
                a: 255,
            },
        ),
        (
            3,
            TruePixel {
                r: 204,
                g: 36,
                b: 29,
                a: 255,
            },
        ),
        (
            4,
            TruePixel {
                r: 152,
                g: 151,
                b: 26,
                a: 255,
            },
        ),
        (
            5,
            TruePixel {
                r: 215,
                g: 153,
                b: 33,
                a: 255,
            },
        ),
        (
            6,
            TruePixel {
                r: 69,
                g: 133,
                b: 136,
                a: 255,
            },
        ),
        (
            7,
            TruePixel {
                r: 177,
                g: 98,
                b: 134,
                a: 255,
            },
        ),
        (
            8,
            TruePixel {
                r: 104,
                g: 157,
                b: 106,
                a: 255,
            },
        ),
    ];

    pub fn gruvbox() -> Self {
        Self {
            colors: HashMap::from(Self::GRUVBOX_COLORS),
            equipped: Some(1),
        }
    }

    /// Returns an empty Palette
    pub fn new() -> Palette {
        Palette {
            colors: HashMap::new(),
            equipped: None,
        }
    }

    /// Returns a Palette initialized with a collection of (index, color hex-string) pairs, failing
    /// if any of the colors fail to get parsed
    ///
    /// This method may fail with the [`TruePixelError`](PaletteError::TruePixelError) error variant only.
    pub fn from(colors: &[(u8, &str)]) -> Result<Palette, PaletteError> {
        let mut palette = Palette {
            colors: HashMap::new(),
            equipped: None,
        };

        for (index, color_hex) in colors {
            palette.set_color(*index, color_hex)?;
            if let Err(_) = palette.get_equipped_strict() {
                palette.equip(*index).expect(EQUIPPEDISINPALETTE);
            }
        }

        Ok(palette)
    }

    /// Gets the pixel corresponding to a particular index, fails if no pixels correspond
    ///
    /// This method may fail with the [`InvalidIndex`](PaletteError::InvalidIndex) error variant
    /// only.
    pub fn get_color(&self, index: u8) -> Result<&TruePixel, PaletteError> {
        use PaletteError::InvalidIndex;

        self.colors.get(&index).ok_or(InvalidIndex(index))
    }

    /// Gets the equipped pixel, fails if no index has been equipped yet
    ///
    /// This method may fail with the [`NothingEquipped`](PaletteError::NothingEquipped) error
    /// variant only.
    pub fn get_equipped_strict(&self) -> Result<&TruePixel, PaletteError> {
        use PaletteError::NothingEquipped;

        if let Some(index) = self.equipped {
            Ok(self.colors.get(&index).expect(EQUIPPEDISINPALETTE))
        } else {
            Err(NothingEquipped)
        }
    }

    /// Gets the equipped pixel, returning [`my favourite color`][fc] if nothing is equipped yet
    ///
    /// [fc]: TruePixel::FAVOURITE
    pub fn get_equipped(&self) -> &TruePixel {
        if let Some(index) = self.equipped {
            self.colors.get(&index).expect(EQUIPPEDISINPALETTE)
        } else {
            &TruePixel::FAVOURITE
        }
    }

    /// Returns the equipped index, returning 0 if nothing is equipped yet
    pub fn equipped(&self) -> u8 {
        self.equipped.unwrap_or(0)
    }

    /// Equips a particular index (see [`Palette`] documentation), fails if no pixels correspond to
    /// specified index
    ///
    /// This method may fail with the [`InvalidIndex`](PaletteError::InvalidIndex) error variant
    /// only.
    pub fn equip(&mut self, index: u8) -> Result<(), PaletteError> {
        use PaletteError::InvalidIndex;

        if let Some(_) = self.colors.get(&index) {
            self.equipped = Some(index);
            Ok(())
        } else {
            Err(InvalidIndex(index))
        }
    }

    /// Sets a color corresponding to a particular index, overwrites if already present, failing if
    /// the color string failed to be parsed into a [`TruePixel`]
    ///
    /// This method may fail with the [`TruePixelError`](PaletteError::TruePixelError) error variant only.
    pub fn set_color(&mut self, index: u8, color_hex: &str) -> Result<(), PaletteError> {
        use PaletteError::TruePixelError;

        if let None = self.colors.insert(
            index,
            TruePixel::from_hex(color_hex).map_err(|err| TruePixelError(err))?,
        ) {
            //if nothing was equipped, equip this
            self.equipped = Some(self.equipped.unwrap_or(index));
        }

        Ok(())
    }

    /// Unsets a color corresponding to a particular index, and manages the equipped to change to
    /// a suitable index or get disabled
    pub fn unset_color(&mut self, index: u8) {
        if let Some(_) = self.colors.remove(&index) {
            match self.equipped.clone() {
                Some(equipped) => {
                    if equipped == index {
                        if self.colors.len() > 0 {
                            self.equipped = Some(*self.colors.iter().next().expect(PALETTELEN).0);
                        } else {
                            self.equipped = None;
                        }
                    }
                }
                None => (),
            }
        }
    }

    /// Returns the equipped index if an index has been equipped, None otherwise
    pub fn get_equipped_index(&self) -> Option<u8> {
        self.equipped
    }

    /// Returns an iterator to the palette colors with each entry of the iterator being a tuple of
    /// the index, the color, and whether or not it is the equipped color in the palette
    pub fn colors(&self) -> impl Iterator<Item = (&u8, &TruePixel, bool)> {
        self.colors.iter().map(|(index, color)| {
            return (
                index,
                color,
                self.equipped.is_some() && *index == self.equipped.unwrap(),
            );
        })
    }
}

impl From<&Vec<TruePixel>> for Palette {
    fn from(item: &Vec<TruePixel>) -> Palette {
        let colors = item
            .iter()
            .take(u8::MAX.into())
            .map(|p| p.clone())
            .enumerate()
            .map(|(i, p)| (i as u8, p.clone()))
            .collect::<HashMap<u8, TruePixel>>();

        Palette {
            equipped: if item.len() > 0 { Some(0) } else { None },
            colors,
        }
    }
}

// Error Types

/// Error enum to describe various errors returned by Palette methods
#[derive(Debug)]
pub enum PaletteError {
    /// Error that occurs when an index has been received that does not correspond to this palette
    InvalidIndex(u8),

    /// Error that occurs when equipped index is accessed somehow but nothing has been equipped
    NothingEquipped,

    /// Error that is propagated when trying to parse color hex-strings into
    /// [`TruePixel`](TruePixel)
    TruePixelError(TruePixelError),
}

impl std::fmt::Display for PaletteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PaletteError::*;
        match self {
            InvalidIndex(index) => write!(
                f,
                "cannot get color {} from palette as it hasn't been set",
                index,
            ),
            NothingEquipped => write!(f, "cannot get equipped color as nothing has been equipped",),
            TruePixelError(true_pixel_error) => write!(f, "{}", true_pixel_error),
        }
    }
}
