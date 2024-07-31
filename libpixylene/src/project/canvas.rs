use super::{Layer, Layers, Palette, Scene};
use crate::types::{BlendMode, IndexedPixel, PCoord, TruePixel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Savefile, Serialize, Deserialize)]
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
    /// Merges the Layers of this Canvas into a single true-color [`Scene`] with the provided
    /// background [`true-pixel`](TruePixel)
    pub fn merged_true_scene(&self, background: Option<TruePixel>) -> Scene<TruePixel> {
        let mut net_layer = Layer::<TruePixel>::new_with_solid_color(self.layers.dim(), background);
        let layer_conv;
        let layers_true: &Layers<TruePixel> = match &self.layers {
            LayersType::True(layers_true) => layers_true,
            LayersType::Indexed(layers_indexed) => {
                layer_conv = layers_indexed.to_true_layers(&self.palette);
                &layer_conv
            }
        };
        for k in 0..layers_true.len() {
            if layers_true[k].mute {
                continue;
            }
            net_layer = Layer {
                scene: Layer::merge(
                    layers_true.dim(),
                    &layers_true[k],
                    &net_layer,
                    BlendMode::Normal,
                )
                .unwrap(),
                opacity: 255,
                mute: false,
                blend_mode: layers_true[k].blend_mode,
            };
        }
        net_layer.scene
    }

    /// Merges the Layers of an Indexed Canvas into a single indexed-color [`Scene`] with the
    /// provided background [`indexed-pixel`](IndexedPixel), failing if this Canvas is not Indexed
    pub fn merged_indexed_scene(
        &self,
        background: Option<IndexedPixel>,
    ) -> Result<Scene<IndexedPixel>, ()> {
        match &self.layers {
            LayersType::Indexed(layers) => {
                let mut new_buf = vec![background; self.layers.dim().area() as usize];

                for k in 0..layers.len() {
                    if layers[k].mute {
                        continue;
                    }
                    _ = layers[k]
                        .scene
                        .grid()
                        .enumerate()
                        .map(|(i, p)| {
                            if let Some(p) = p {
                                new_buf[i] = Some(*p);
                            }
                        })
                        .collect::<()>();
                }
                Ok(Scene::new(self.layers.dim(), new_buf).unwrap())
            }
            LayersType::True(_) => Err(()),
        }
    }
}
