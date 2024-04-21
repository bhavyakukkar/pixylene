use crate::{
    types::{ Pixel, PixelError },
    utils::messages::{ EQUIPPEDISINPALETTE, PALETTELEN },
};

use std::collections::HashMap;
use serde::{ Serialize, Deserialize };


/// A `Palette` containing a set of [`Pixels`](Pixel)
///
/// The palette works by using a hashmap of u8 indexes to Pixel definitions, and the most
/// significant color at any time can be chosen by its index and picked.
#[derive(Serialize, Deserialize, PartialEq, Savefile, Clone)]
pub struct Palette {
    colors: HashMap<u8, Pixel>,
    equipped: Option<u8>,
}

impl Palette {

    /// Returns an empty Palette
    pub fn new() -> Palette { Palette { colors: HashMap::new(), equipped: None } }

    /// Returns a Palette initialized with a collection of (index, color hex-string) pairs, failing
    /// if any of the colors fail to get parsed into [`Pixels`](Pixel)
    ///
    /// This method may fail with the [`PixelError`](PaletteError::PixelError) error variant only.
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

    /// Gets the `Pixel` corresponding to a particular index, fails if no pixels correspond
    ///
    /// This method may fail with the [`InvalidIndex`](PaletteError::InvalidIndex) error variant
    /// only.
    pub fn get_color(&self, index: u8) -> Result<&Pixel, PaletteError> {
        use PaletteError::InvalidIndex;

        self.colors.get(&index).ok_or(InvalidIndex(index))
    }

    /// Gets the equipped pixel (see [`Palette`] documentation), fails if no index has been
    /// equipped yet
    ///
    /// This method may fail with the [`NothingEquipped`](PaletteError::InvalidIndex) error variant
    /// only.
    pub fn get_equipped_strict(&self) -> Result<&Pixel, PaletteError> {
        use PaletteError::NothingEquipped;

        if let Some(index) = self.equipped {
            Ok(self.colors.get(&index).expect(EQUIPPEDISINPALETTE))
        } else {
            Err(NothingEquipped)
        }
    }

    /// Gets the equipped pixel (see [`Palette`] documentation), returns [`my favourite color`](fc)
    /// if nothing is equipped yet
    pub fn get_equipped(&self) -> &Pixel {
        if let Some(index) = self.equipped {
            self.colors.get(&index).expect(EQUIPPEDISINPALETTE)
        } else {
            &Pixel::FAVOURITE
        }
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
    /// the color string failed to be parsed into a [`Pixel`]
    ///
    /// This method may fail with the [`PixelError`](PaletteError::PixelError) error variant only.
    pub fn set_color(&mut self, index: u8, color_hex: &str) -> Result<(), PaletteError> {
        use PaletteError::PixelError;

        if let None = self.colors.insert(
            index,
            Pixel::from_hex(color_hex).map_err(|err| PixelError(err))?
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
                            self.equipped = Some(*self.colors.iter().next()
                                                 .expect(PALETTELEN).0);
                        } else {
                            self.equipped = None;
                        }
                    }
                },
                None => (),
            }
        }
    }

    /// Returns an iterator to the palette colors with each entry of the iterator being a tuple of
    /// the index, the color, and whether or not it is the equipped color in the palette
    pub fn colors(&self) -> impl Iterator<Item = (&u8, &Pixel, bool)> {
        self.colors.iter().map(|(index, color)| {
            return (index, color, self.equipped.is_some() && *index == self.equipped.unwrap());
        })
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

    /// Error that is propagated when trying to parse color hex-strings into [`Pixel`](Pixel}
    PixelError(PixelError),
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
            NothingEquipped => write!(
                f,
                "cannot get equipped color as nothing has been equipped",
            ),
            PixelError(pixel_error) => write!(f, "{}", pixel_error),
        }
    }
}
