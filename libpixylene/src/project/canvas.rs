use crate::types::{PCoord, BlendMode, TruePixel, IndexedPixel};
use super::{Scene, Layer, Layers, Palette};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, PartialEq,Savefile, Serialize, Deserialize)]
pub enum LayersType {
    True(Layers<TruePixel>),
    Indexed(Layers<IndexedPixel>),
}

impl LayersType {
    pub fn dim(&self) -> PCoord {
        match self {
            LayersType::True(layers) => layers.dim(),
            LayersType::Indexed(layers) => layers.dim(),
        }
    }

    pub fn len(&self) -> u16 {
        match self {
            LayersType::True(layers) => layers.len(),
            LayersType::Indexed(layers) => layers.len(),
        }
    }

    pub fn to_true(&self) -> Result<&Layers<TruePixel>, ()> {
        if let Self::True(layers) = self {
            Ok(layers)
        } else {
            Err(())
        }
    }
    pub fn to_true_mut(&mut self) -> Result<&mut Layers<TruePixel>, ()> {
        if let Self::True(layers) = self {
            Ok(layers)
        } else {
            Err(())
        }
    }

    pub fn to_indexed(&self) -> Result<&Layers<IndexedPixel>, ()> {
        if let Self::Indexed(layers) = self {
            Ok(layers)
        } else {
            Err(())
        }
    }
    pub fn to_indexed_mut(&mut self) -> Result<&mut Layers<IndexedPixel>, ()> {
        if let Self::Indexed(layers) = self {
            Ok(layers)
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Savefile, Serialize, Deserialize)]
pub struct Canvas {
    pub layers: LayersType,
    pub palette: Palette,
}

impl Canvas {
    pub fn merged_scene(&self, background: Option<TruePixel>) -> Scene<TruePixel> {
        let mut net_layer = Layer::<TruePixel>::new_with_solid_color(self.layers.dim(), background);
        let layer_conv;
        let layers_true: &Layers<TruePixel> = match &self.layers {
            LayersType::True(layers_true) => layers_true,
            LayersType::Indexed(layers_indexed) => {
                layer_conv = layers_indexed.to_true_layers(&self.palette);
                &layer_conv
            },
        };
        for k in 0..layers_true.len() {
            if layers_true[k].mute { continue; }
            net_layer = Layer {
                scene: Layer::merge(
                    layers_true.dim(),
                    &layers_true[k],
                    &net_layer,
                    BlendMode::Normal
                ).unwrap(),
                opacity: 255,
                mute: false,
                blend_mode: layers_true[k].blend_mode,
            };
        }
        net_layer.scene
    }
}
