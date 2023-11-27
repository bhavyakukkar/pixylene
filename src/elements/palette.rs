use crate::elements::common::Pixel;

pub struct Palette {
    colors: Vec<Pixel>
}
impl Palette {
    fn get_color(&self, index: usize) -> Result<Pixel, String> {
        if index >= 1 && index <= self.colors.len() {
            Ok(self.colors[index])
        } else {
            Err(format!("cannot get color {} from palette of {} colors (hint: palette indexing starts from 1", index, self.colors.len()))
        }
    }
}
