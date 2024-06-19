use crate::types::{PCoord, TruePixel, IndexedPixel, Pixel};
use super::{Palette, Layer};

use serde::{Serialize, Deserialize};
use std::{fmt, ops::{Index, IndexMut}, slice::{Iter, IterMut}};


/// The maximum number of Layers that a Canvas is allowed to have
pub const MAX_LAYERS: u16 = u16::MAX;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Savefile)]
pub struct Layers<T: Pixel> {
    layers: Vec<Layer<T>>,
    dimensions: PCoord,
}

impl<T: Pixel> Index<u16> for Layers<T> {
    type Output = Layer<T>;
    fn index(&self, index: u16) -> &Layer<T> {
        &self.layers[index as usize]
    }
}

impl<T: Pixel> IndexMut<u16> for Layers<T> {
    fn index_mut(&mut self, index: u16) -> &mut Layer<T> {
        &mut self.layers[index as usize]
    }
}

impl<T: Pixel> TryFrom<Vec<Layer<T>>> for Layers<T> {
    type Error = LayersError;

    fn try_from(item: Vec<Layer<T>>) -> Result<Layers<T>, LayersError> {
        let mut layer_iter = item.iter();
        let first_layer = layer_iter.next().ok_or(LayersError::NoDimensionInformation)?;
        let mut layers = Layers::<T>::new(first_layer.scene.dim());
        layers.add_layer(first_layer.clone()).unwrap(); //cant fail, first_layer has same dim as layers
        while let Some(layer) = layer_iter.next() {
            layers.add_layer(layer.clone())?;
        }
        Ok(layers)
    }
}

impl<T: Pixel> Layers<T> {

    pub fn new(dimensions: PCoord) -> Layers<T> {
        Self { layers: Vec::new(), dimensions }
    }

    /// Gets the number of layers currently present in the Canvas
    pub fn len(&self) -> u16 {
        u16::try_from(self.layers.len()).unwrap()
    }

    pub fn dim(&self) -> PCoord {
        self.dimensions
    }

    /// Consumes a Layer and pushes it to the Canvas
    /// 
    /// `Note`: This method may fail with the [`InconsistentDimensions`][id] or [`MaxLayers`][ml]
    /// error variants only.
    ///
    /// [id]: LayersError::InconsistentDimensions
    /// [ml]: LayersError::MaxLayers
    pub fn add_layer(&mut self, layer: Layer<T>) -> Result<(), LayersError> {
        use LayersError::{ MaxLayers, InconsistentDimensions };

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
    /// appends it to the canvas, returning its resultant index in the canvas
    ///
    /// `Note`: This method may fail with the [`MaxLayers`][ml] error variant only.
    ///
    /// [ml]: LayersError::MaxLayers
    pub fn new_layer(&mut self, color: Option<T>) -> Result<u16, LayersError> {
        use LayersError::MaxLayers;

        if self.layers.len() <= usize::from(MAX_LAYERS) {
            self.layers.push(Layer::<T>::new_with_solid_color(self.dimensions, color));
            Ok(u16::try_from(self.layers.len()).unwrap() - 1) //shouldn't fail because all sources
                                                              //of len increasing do not let
                                                              //self.layers exceed 256
        } else {
            Err(MaxLayers)
        }
    }

    /// Gets and returns a reference to a Layer at a particular index in the Canvas
    ///
    /// `Note`: This method may fail with the [`IndexOutOfBounds`][ioob] error variant only.
    ///
    /// [ioob]: LayersError::IndexOutOfBounds
    pub fn get_layer(&self, index: u16) -> Result<&Layer<T>, LayersError> {
        use LayersError::{ IndexOutOfBounds };

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
    /// [ioob]: LayersError::IndexOutOfBounds
    pub fn get_layer_mut(&mut self, index: u16) -> Result<&mut Layer<T>, LayersError> {
        use LayersError::{ IndexOutOfBounds };

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
    /// [ioob]: LayersError::IndexOutOfBounds
    pub fn del_layer(&mut self, index: u16) -> Result<Layer<T>, LayersError> {
        use LayersError::{ IndexOutOfBounds };

        if usize::from(index) < self.layers.len() {
            Ok(self.layers.remove(usize::from(index)))
        } else {
            Err(IndexOutOfBounds(index.into(), self.layers.len()))
        }
    }

    /// Duplicates a Layer at a particular index from the Canvas and places it at the next index,
    /// pushing all the succeeding layers by one
    pub fn duplicate_layer(&mut self, index: u16) -> Result<(), LayersError> {
        use LayersError::{ IndexOutOfBounds, MaxLayers };

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
    /// [ioob]: LayersError::IndexOutOfBounds
    pub fn move_layer(&mut self, old_index: u16, new_index: u16) -> Result<(), LayersError> {
        use LayersError::{ IndexOutOfBounds };

        if usize::from(old_index) >= self.layers.len() {
            return Err(IndexOutOfBounds(usize::from(old_index), self.layers.len()));
        }
        else if usize::from(new_index) >= self.layers.len() {
            return Err(IndexOutOfBounds(usize::from(new_index), self.layers.len()));
        }
        let layer = self.layers.remove(usize::from(old_index));
        Ok(self.layers.insert(usize::from(new_index), layer))
    }

    pub fn layers(&self) -> Iter<Layer<T>> {
        self.layers.iter()
    }

    pub fn layers_mut(&mut self) -> IterMut<Layer<T>> {
        self.layers.iter_mut()
    }

    fn _resize(&mut self, _new_dim: PCoord) {
        todo!()
    }
}

impl Layers<IndexedPixel> {
    /// Converts these indexed-color Layers to true-color Layers using the provided Palette from
    /// which to extract true-color pixels corresponding to indexed-color indexes
    pub fn to_true_layers(&self, palette: &Palette) -> Layers<TruePixel> {
        Layers::<TruePixel>::try_from(self.layers.iter()
            .map(|layer_indexed| layer_indexed.to_true_layer(palette))
            .collect::<Vec<Layer<TruePixel>>>())
        .unwrap() //cant fail because layers extracted from Layers which can only exist with
                  //consistent-dimensions layers
    }
}


// Error Types

/// Error enum to describe various errors returned by Layers methods
#[derive(Debug)]
pub enum LayersError {
    /// Error that occurs when trying to convert an empty vector of [`Layer`]s to a [`Layers`],
    /// in which case no dimensions information is able to be inferred
    NoDimensionInformation,

    /// Error that occurs when trying to add a Layer of inconsistent dimensions to the Canvas
    InconsistentDimensions(PCoord, PCoord),

    /// Error that occurs when trying to add a Layer when Canvas already contains the maximum
    /// specified amount of layers
    MaxLayers,

    /// Error that occurs when trying to access a Layer at an index that is out of bounds for the
    /// given Canvas
    IndexOutOfBounds(usize, usize),
}

impl fmt::Display for LayersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LayersError::*;
        match self {
            NoDimensionInformation => write!(
                f,
                "could not get information of dimensions while trying to construct layers",
            ),
            InconsistentDimensions(layer_dim, layers_dim) => write!(
                f,
                "cannot add this layer of dimensions {} to layers of dimensions {}",
                layer_dim,
                layers_dim,
            ),
            MaxLayers => write!(
                f,
                "cannot add a layer as the maximum allowed number of layers has reached: {}",
                MAX_LAYERS,
            ),
            IndexOutOfBounds(index, length) => write!(
                f,
                "cannot access layer at given index {} as layers only contains {} layers",
                index,
                length,
            ),
        }
    }
}
