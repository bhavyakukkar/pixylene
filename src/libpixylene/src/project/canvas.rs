use crate::{
    types::{ Coord, Pixel, BlendMode },
    project::{ Scene, Layer, Palette },
};

#[derive(Savefile)]
pub struct Canvas {
    pub dimensions: Coord,
    pub layers: Vec<Layer>,
    pub palette: Palette,
}

impl Canvas {
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
                    &self.layers[k],
                    &net_layer,
                    BlendMode::Normal
                ).unwrap(),
                opacity: 255,
                mute: false,
            };
        }
        net_layer.scene
    }
}
