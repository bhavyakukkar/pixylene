use crate::elements::common::Pixel;

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
}
