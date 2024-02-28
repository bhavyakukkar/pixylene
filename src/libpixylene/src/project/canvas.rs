use crate::{
    types::{ PCoord, Pixel, BlendMode },
    project::{ Scene, Layer, Palette },
};

use std::fmt;


const MAX_LAYERS: u16 = u16::MAX;

#[derive(Savefile)]
pub struct Canvas {
    dimensions: PCoord,
    layers: Vec<Layer>,
    pub palette: Palette,
}

impl Canvas {
    pub fn new(dimensions: PCoord, palette: Palette) -> Canvas {
        Canvas{ dimensions, layers: Vec::new(), palette }
    }
    pub fn dim(&self) -> PCoord {
        self.dimensions
    }
    pub fn merged_scene(&self, background: Option<Pixel>) -> Scene {
        let mut net_layer = Layer::new_with_solid_color(self.dimensions, background);
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
    pub fn num_layers(&self) -> u16 {
        u16::try_from(self.layers.len()).unwrap()
    }
    /*
    pub fn new_layer(&mut self) -> Result<(), CanvasError> {
        use CanvasError::{ MaxLayers };
        if self.layers.len() <= usize::from(MAX_LAYERS) {
            self.layers.push(Layer::new_with_solid_color(self.dimensions, None));
            Ok(())
        } else {
            Err(MaxLayers)
        }
    }
    */
    pub fn add_layer(&mut self, layer: Layer) -> Result<(), CanvasError> {
        use CanvasError::{ MaxLayers };
        if self.layers.len() <= usize::from(MAX_LAYERS) {
            self.layers.push(layer);
            Ok(())
        } else {
            Err(MaxLayers)
        }
    }
    pub fn get_layer(&self, index: u16) -> Result<Layer, CanvasError> {
        use CanvasError::{ IndexOutOfBounds };
        if usize::from(index) < self.layers.len() {
            Ok(self.layers[usize::from(index)].clone())
        } else {
            Err(IndexOutOfBounds(index.into(), self.layers.len()))
        }
    }
    pub fn del_layer(&mut self, index: u16) -> Result<(), CanvasError> {
        use CanvasError::{ IndexOutOfBounds };
        if usize::from(index) < self.layers.len() {
            self.layers.remove(usize::from(index));
            Ok(())
        } else {
            Err(IndexOutOfBounds(index.into(), self.layers.len()))
        }
    }
    pub fn duplicate_layer(&mut self, index: u16) -> Result<(), CanvasError> {
        use CanvasError::{ IndexOutOfBounds, MaxLayers };
        if self.layers.len() <= usize::from(MAX_LAYERS) {
            let index = usize::from(index);
            if index < self.layers.len() {
                let duplicate = self.layers[index].clone();
                self.layers.insert(index + 1, duplicate);
                Ok(())
            } else {
                Err(IndexOutOfBounds(index.into(), self.layers.len()))
            }
        } else {
            Err(MaxLayers)
        }
    }
}

#[derive(Debug)]
pub enum CanvasError {
    MaxLayers,
    IndexOutOfBounds(usize, usize),
}
impl fmt::Display for CanvasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CanvasError::*;
        match self {
            MaxLayers => write!(
                f,
                "cannot add a layer as canvas has the maximum allowed number of layers: {}",
                MAX_LAYERS,
            ),
            IndexOutOfBounds(index, length) => write!(
                f,
                "cannot access layer at given index {} as canvas only contains {} layers",
                index,
                length,
            ),
        }
    }
}
