use crate::{
    types::{ PCoord, Pixel, BlendMode },
    project::{ Scene, Layer, Palette },
};

use std::fmt;


/// The maximum number of Layers that a Canvas is allowed to have
pub const MAX_LAYERS: u16 = u16::MAX;

/// A controlled set of [`Layers`](Layer) with uniform dimensions and a [`Palette`](Palette).
///
/// The Canvas is the minimum amount of data needed to describe any piece of pixel art (dimensions
/// and layer are the pixel art itself, and palettes are required in indexed PNGs). 
///
/// `Note`: All Canvas methods to access Layers use 0-based indexes
#[derive(PartialEq, Clone, Savefile)]
pub struct Canvas {
    dimensions: PCoord,
    layers: Vec<Layer>,
    pub palette: Palette,
}

impl Canvas {

    /// Creates a new empty Canvas with given dimensions and palette and 0 layers
    pub fn new(dimensions: PCoord, palette: Palette) -> Canvas {
        Canvas{ dimensions, layers: Vec::new(), palette }
    }

    /// Creates a new Canvas with given dimensions and palette and the provided layers, fails if
    /// the layer isn't consistent with the dimensions or too many layers are passed
    ///
    /// `Note`: This method may fail with the [`InconsistentDimensions`][id] or [`MaxLayers`][ml]
    /// error variants only.
    ///
    /// [id]: CanvasError::InconsistentDimensions
    /// [ml]: CanvasError::MaxLayers
    pub fn from_layers(
        dimensions: PCoord,
        layers: Vec<Layer>,
        palette: Palette) -> Result<Canvas, CanvasError>
    {
        let mut canvas = Canvas{ dimensions, layers: Vec::new(), palette };
        for layer in layers {
            canvas.add_layer(layer)?;
        }
        Ok(canvas)
    }

    /// Gets the dimensions of the canvas
    pub fn dim(&self) -> PCoord {
        self.dimensions
    }

    /// Merges all the layers of the Canvas into a single [`Scene`] with an optional
    /// background-color
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
                blend_mode: self.layers[k].blend_mode,
            };
        }
        net_layer.scene
    }

    /// Gets the number of layers currently present in the Canvas
    pub fn num_layers(&self) -> u16 {
        u16::try_from(self.layers.len()).unwrap()
    }

    /// Consumes a Layer and pushes it to the Canvas
    /// 
    /// `Note`: This method may fail with the [`InconsistentDimensions`][id] or [`MaxLayers`][ml]
    /// error variants only.
    ///
    /// [id]: CanvasError::InconsistentDimensions
    /// [ml]: CanvasError::MaxLayers
    pub fn add_layer(&mut self, layer: Layer) -> Result<(), CanvasError> {
        use CanvasError::{ MaxLayers, InconsistentDimensions };

        if self.layers.len() <= usize::from(MAX_LAYERS) {
            if self.dimensions == layer.scene.dim() {
                self.layers.push(layer);
                Ok(())
            } else {
                Err(InconsistentDimensions(layer.scene.dim(), self.dimensions))
            }
        } else {
            Err(MaxLayers)
        }
    }

    /// Creates a new empty layer consistent with the canvas's dimensions and of a solid color, and
    /// appends it to the canvas
    ///
    /// `Note`: This method may fail with the [`MaxLayers`][ml] error variant only.
    ///
    /// [ml]: CanvasError::MaxLayers
    pub fn new_layer(&mut self, color: Option<Pixel>) -> Result<(), CanvasError> {
        use CanvasError::MaxLayers;

        if self.layers.len() <= usize::from(MAX_LAYERS) {
            self.layers.push(Layer::new_with_solid_color(self.dimensions, color));
            Ok(())
        } else {
            Err(MaxLayers)
        }
    }

    /// Gets and returns a reference to a Layer at a particular index in the Canvas
    ///
    /// `Note`: This method may fail with the [`IndexOutOfBounds`][ioob] error variant only.
    ///
    /// [ioob]: CanvasError::IndexOutOfBounds
    pub fn get_layer(&self, index: u16) -> Result<&Layer, CanvasError> {
        use CanvasError::{ IndexOutOfBounds };

        if usize::from(index) < self.layers.len() {
            Ok(&self.layers[usize::from(index)])
        } else {
            Err(IndexOutOfBounds(index.into(), self.layers.len()))
        }
    }

    /// Gets and returns a mutable reference to a Layer at a particular index in the Canvas
    ///
    /// `Note`: This method may fail with the [`IndexOutOfBounds`][ioob] error variant only.
    ///
    /// [ioob]: CanvasError::IndexOutOfBounds
    pub fn get_layer_mut(&mut self, index: u16) -> Result<&mut Layer, CanvasError> {
        use CanvasError::{ IndexOutOfBounds };

        if usize::from(index) < self.layers.len() {
            Ok(&mut self.layers[usize::from(index)])
        } else {
            Err(IndexOutOfBounds(index.into(), self.layers.len()))
        }
    }

    /// Deletes a returns the Layer at a particular index from the Canvas 
    ///
    /// `Note`: This method may fail with the [`IndexOutOfBounds`][ioob] error variant only.
    ///
    /// [ioob]: CanvasError::IndexOutOfBounds
    pub fn del_layer(&mut self, index: u16) -> Result<Layer, CanvasError> {
        use CanvasError::{ IndexOutOfBounds };

        if usize::from(index) < self.layers.len() {
            Ok(self.layers.remove(usize::from(index)))
        } else {
            Err(IndexOutOfBounds(index.into(), self.layers.len()))
        }
    }

    /// Duplicates a Layer at a particular index from the Canvas and places it at the next index,
    /// pushing all the succeeding layers by one
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

    /// Moves a Layer at a particular index from the Canvas and places it at a new index, cascading
    /// all other Layers appropriately & failing if any of the indexes are invalid
    ///
    /// `Note`: This method may fail with the [`IndexOutOfBounds`][ioob] error variant only.
    ///
    /// [ioob]: CanvasError::IndexOutOfBounds
    pub fn move_layer(&mut self, old_index: u16, new_index: u16) -> Result<(), CanvasError> {
        use CanvasError::{ IndexOutOfBounds };

        if usize::from(old_index) >= self.layers.len() {
            return Err(IndexOutOfBounds(usize::from(old_index), self.layers.len()));
        }
        else if usize::from(new_index) >= self.layers.len() {
            return Err(IndexOutOfBounds(usize::from(new_index), self.layers.len()));
        }
        let layer = self.layers.remove(usize::from(old_index));
        Ok(self.layers.insert(usize::from(new_index), layer))
    }

    pub fn resize(&mut self, _new_dim: PCoord) {
        todo!()
    }
}


// Error Types

/// Error enum to describe various errors returns by Canvas methods
#[derive(Debug)]
pub enum CanvasError {

    /// Error that occurs when trying to add a Layer of inconsistent dimensions to the Canvas
    InconsistentDimensions(PCoord, PCoord),

    /// Error that occurs when trying to add a Layer when Canvas already contains the maximum
    /// specified amount of layers
    MaxLayers,

    /// Error that occurs when trying to access a Layer at an index that is out of bounds for the
    /// given Canvas
    IndexOutOfBounds(usize, usize),
}

impl fmt::Display for CanvasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CanvasError::*;
        match self {
            InconsistentDimensions(layer_dim, canvas_dim) => write!(
                f,
                "cannot add this layer of dimensions {} to this canvas of dimensions {}",
                layer_dim,
                canvas_dim,
            ),
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
