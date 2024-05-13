use crate::types::{PCoord, TruePixel, IndexedPixel};
use super::{GenericCanvas, Palette, Layers, Scene};
use serde::{Serialize, Deserialize};
use std::boxed::Box;


#[derive(Clone, Serialize, Deserialize, PartialEq, Savefile)]
pub enum CanvasType {
    True(GenericCanvas<TruePixel>),
    Indexed(GenericCanvas<IndexedPixel>),
}

impl CanvasType {
    pub fn inner(&self) -> &dyn Canvas {
        use CanvasType::*;
        match self {
            True(c) => c,
            Indexed(c) => c,
        }
    }

    pub fn inner_mut(&mut self) -> &mut dyn Canvas {
        use CanvasType::*;
        match self {
            True(c) => { return c; },
            Indexed(c) => { return c; },
        }
    }
}


pub trait Canvas {
    fn layers_true(&self) -> Layers<TruePixel>;
    fn layers_indexed(&self) -> Layers<IndexedPixel>;

    //fn layers_true<'a>(&self) -> &'a Layers<TruePixel>;
    //fn layers_true_mut<'a>(&'a mut self) -> &'a mut Layers<TruePixel>;

    //fn layers_indexed<'a>(&'a self) -> &'a Layers<IndexedPixel>;
    //fn layers_indexed_mut<'a>(&'a mut self) -> &'a mut Layers<IndexedPixel>;

    fn palette<'a>(&'a mut self) -> &'a mut Palette;
    fn merged_scene(&self, background: Option<TruePixel>) -> Scene;
    fn dim(&self) -> PCoord;
    fn num_layers(&self) -> u16;
}
