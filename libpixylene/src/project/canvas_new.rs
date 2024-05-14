use crate::types::{ Pixel, TruePixel, IndexedPixel };
use super::Layers

pub enum LayersType {
    True(Layers<TruePixel>),
    Indexed(Layers<IndexedPixel>),
}

pub struct Canvas {
    pub layers: LayersType,
    pub palette: Palette,
}

impl Canvas {
    pub fn dim(&self) -> PCoord {
        match self.layers {
            LayersType::True(layers) => layers.dim(),
            LayersType::Indexed(layers) => layers.dim(),
        }
    }
}
