use crate::elements::layer::{ Camera, Layer };
use crate::elements::Palette;

#[derive(Savefile)]
pub struct Project {
    pub layers: Vec<Layer>,
    pub selected_layer: usize,
    pub camera: Camera,
    pub palette: Palette,
}

impl Project {
    pub fn get_num_layers(&self) -> usize {
        self.layers.len()
    }
    /*
    fn save_project(&self);
    fn open_project();
    fn export_png();
    fn import_png();
    fn save_color_palette();
    fn load_color_palette();
    fn get_pixel();
    */
}
