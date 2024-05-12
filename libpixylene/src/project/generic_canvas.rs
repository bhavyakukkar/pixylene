use crate::{
    types::{ PCoord, Pixel, TruePixel, IndexedPixel, BlendMode },
    project::{ Scene, Layer, Layers, Palette, Canvas },
};

use serde::{ Serialize, Deserialize };


/// A controlled set of [`Layers`](Layer) with uniform dimensions and a [`Palette`](Palette).
///
/// The Canvas is the minimum amount of data needed to describe any piece of pixel art (dimensions
/// and layer are the pixel art itself, and palettes are required in indexed PNGs). 
///
/// `Note`: All Canvas methods to access Layers use 0-based indexes
#[derive(Serialize, Deserialize, PartialEq, Clone, Savefile)]
pub struct GenericCanvas<T=TruePixel>
where T: Pixel
{
    layers: Layers<T>,
    palette: Palette,
}

impl<T: Pixel> GenericCanvas<T> {
    pub fn new(dimensions: PCoord, palette: Palette) -> Self {
        Self {
            layers: Layers::<T>::new(dimensions),
            palette
        }
    }

    pub fn layers(&self) -> &Layers<T> {
        &self.layers
    }

    pub fn layers_mut(&mut self) -> &mut Layers<T> {
        &mut self.layers
    }
}

pub type TrueCanvas = GenericCanvas<TruePixel>;

impl Canvas for TrueCanvas {
    fn layers_true(&self) -> Layers<TruePixel> {
        self.layers.clone()
        //LayersIter{ layers: self.layers.layers().collect::<VecDeque<Layer<TruePixel>>>() }
    }

    /// This method always panics
    fn layers_indexed(&self) -> Layers<IndexedPixel> {
        panic!()
    }

    
    fn palette<'a>(&'a mut self) -> &'a mut Palette {
        &mut self.palette
    }

    fn dim(&self) -> PCoord {
        self.layers.dim()
    }

    fn num_layers(&self) -> u16 {
        self.layers.len()
        //u16::try_from(self.layers_true().len()).unwrap()
    }

    /// Merges all the layers of the truecolor Canvas into a single [`Scene`] with an optional
    /// background-color
    fn merged_scene(&self, background: Option<TruePixel>) -> Scene {
        let mut net_layer = Layer::<TruePixel>::new_with_solid_color(self.layers.dim(), background);
        for k in 0..self.layers.len() {
            if self.layers[k].mute { continue; }
            net_layer = Layer {
                scene: Layer::merge(
                    self.layers.dim(),
                    &self.layers[k],
                    &net_layer,
                    BlendMode::Normal
                ).unwrap(),
                opacity: 255,
                mute: false,
                blend_mode: self.layers[k].blend_mode,
            };
        }
        net_layer.scene
    }
}


pub type IndexedCanvas = GenericCanvas<IndexedPixel>;

impl Canvas for IndexedCanvas {
    fn layers_true(&self) -> Layers<TruePixel> {
        Layers::<TruePixel>::try_from(self.layers.layers()
            .map(|layer_indexed| Layer::<TruePixel> {
                scene: Scene::<TruePixel>::new(
                    layer_indexed.scene.dim(),
                    layer_indexed.scene.grid()
                        .map(|index_maybe| match index_maybe {
                            Some(index) => self.palette.get_color(index.0)
                                .map(|true_pixel| Some(true_pixel.clone()))
                                .unwrap_or(None),
                            None => None,
                        })
                        .collect::<Vec<Option<TruePixel>>>()
                ).unwrap(), //cant fail because x.dim() is used to construct scene from x.grid()
                            //which are consistent
                opacity: layer_indexed.opacity,
                mute: layer_indexed.mute,
                blend_mode: layer_indexed.blend_mode.clone(),
            })
            .collect::<Vec<Layer<TruePixel>>>()
            .iter())
            .unwrap()
    }

    fn layers_indexed(&self) -> Layers<IndexedPixel> {
        self.layers.clone()
    }
    
    fn palette<'a>(&'a mut self) -> &'a mut Palette {
        &mut self.palette
    }

    fn dim(&self) -> PCoord {
        self.layers.dim()
    }

    fn num_layers(&self) -> u16 {
        self.layers.len()
    }

    /// Merges all the layers of the indexed-color Canvas into a single [`Scene`], using the
    /// Canvas's palette, with an optional background-color
    fn merged_scene(&self, background: Option<TruePixel>) -> Scene {
        let mut net_layer = Layer::<TruePixel>::new_with_solid_color(self.layers.dim(), background);
        let layers: Vec<Layer<TruePixel>> = self.layers_true().layers()
            .map(|l| l.clone())
            .collect::<Vec<Layer<TruePixel>>>();
        for k in 0..self.layers.len() {
            if layers[k as usize].mute { continue; }
            net_layer = Layer {
                scene: Layer::merge(
                    self.layers.dim(),
                    &layers[k as usize],
                    &net_layer,
                    BlendMode::Normal
                ).unwrap(),
                opacity: 255,
                mute: false,
                blend_mode: layers[k as usize].blend_mode,
            };
        }
        net_layer.scene
    }
}
