use crate::elements::common::Pixel;

#[derive(Savefile)]
pub struct Palette {
    pub colors: Vec<Option<Pixel>>
}
impl Palette {
    pub fn get_color(&self, index: usize) -> Result<Option<Pixel>, String> {
        if index >= 1 && index <= self.colors.len() {
            Ok(self.colors[index - 1usize])
        } else {
            Err(format!("cannot get color {} from palette of {} colors (hint: palette indexing starts from 1)", index, self.colors.len()))
        }
    }
    pub fn change_color_to_existing(&mut self, index: usize, to_index: usize) -> Result<(), String> {
        if index >= 1 && index <= self.colors.len() {
            if to_index >= 1 && to_index <= self.colors.len() {
                self.colors[index - 1usize] = self.colors[to_index - 1usize];
                Ok(())
            } else {
                Err(format!("cannot change to color {} from palette of {} colors (hint: palette indexing starts from 1)", to_index, self.colors.len()))
            }
        } else {
            Err(format!("cannot change color {} from palette of {} colors (hint: palette indexing starts from 1)", index, self.colors.len()))
        }
    }
    pub fn change_color_to(&mut self, index: usize, color_hex: String) -> Result<(), String> {
        if index >= 1 && index <= self.colors.len() {
            self.colors[index - 1usize] = Some(Pixel::from_hex(color_hex)?);
            Ok(())
        } else {
            Err(format!("cannot change color {} from palette of {} colors (hint: palette indexing starts from 1)", index, self.colors.len()))
        }
    }
}
