use crate::elements::common::{ Pixel, PixelError };

#[derive(Debug)]
pub enum PaletteError {
    InvalidIndex(usize, usize),
    PixelError(PixelError),
}

impl std::fmt::Display for PaletteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PaletteError::*;
        match self {
            InvalidIndex(index, length) => write!(
                f,
                "cannot get color {} from palette of {} colors (hint: palette indexing starts from \
                1)",
                index,
                length,
            ),
            PixelError(pixel_error) => write!(f, "{}", pixel_error),
        }
    }
}

#[derive(Savefile)]
pub struct Palette {
    pub colors: Vec<Option<Pixel>>
}
impl Palette {
    pub fn get_color(&self, index: usize) -> Result<Option<Pixel>, PaletteError> {
        use PaletteError::InvalidIndex;
        if index >= 1 && index <= self.colors.len() {
            Ok(self.colors[index - 1usize])
        } else {
            Err(InvalidIndex(index, self.colors.len()))
        }
    }
    pub fn change_to_index(&mut self, index: usize, to_index: usize) -> Result<(), PaletteError> {
        use PaletteError::InvalidIndex;
        if index >= 1 && index <= self.colors.len() {
            if to_index >= 1 && to_index <= self.colors.len() {
                self.colors[index - 1usize] = self.colors[to_index - 1usize];
                Ok(())
            } else {
                Err(InvalidIndex(index, self.colors.len()))
            }
        } else {
            Err(InvalidIndex(index, self.colors.len()))
        }
    }
    pub fn change_to(&mut self, index: usize, color_hex: String) -> Result<(), PaletteError> {
        use PaletteError::{ InvalidIndex, PixelError };
        if index >= 1 && index <= self.colors.len() {
            self.colors[index - 1usize] = Some(
                match Pixel::from_hex(color_hex) {
                    Ok(pixel) => pixel,
                    Err(pixel_error) => { return Err(PixelError(pixel_error)) },
                }
            );
            Ok(())
        } else {
            Err(InvalidIndex(index, self.colors.len()))
        }
    }
}
