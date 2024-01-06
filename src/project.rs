use crate::elements::{ palette::Palette, layer::{ Camera, Layer }};

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
}
