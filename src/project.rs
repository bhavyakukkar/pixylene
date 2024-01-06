use crate::elements::{
    common::{ Coord, Pixel, BlendMode },
    palette::Palette,
    layer::{ Scene, Camera, CameraPixel, Layer }
};

#[derive(Savefile)]
pub struct Project {
    pub dimensions: Coord,
    pub layers: Vec<Layer>,
    pub selected_layer: usize,
    pub camera: Camera,
    pub palette: Palette,
}

impl Project {
    pub fn get_num_layers(&self) -> usize {
        self.layers.len()
    }
    pub fn merged_scene(&self) -> Scene {
        let mut net_layer = Layer::new_with_solid_color(self.dimensions, Some(Pixel::background()));
        for k in 0..self.layers.len() {
            if self.layers[k].mute { continue; }
            net_layer = Layer {
                scene: Layer::merge(
                    self.dimensions,
                    &self.layers[0],
                    &net_layer,
                    BlendMode::Normal
                ).unwrap(),
                opacity: 255,
                mute: false,
            };
        }
        net_layer.scene
    }
    pub fn render(&self) -> Vec<CameraPixel> {
        self.camera.render_scene(&self.merged_scene())
    }
}
