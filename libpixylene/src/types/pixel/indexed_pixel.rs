use super::Pixel;
use serde::{Serialize, Deserialize};

/// A unit of indexed color, representing an 8-bit palette index
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Savefile)]
pub struct IndexedPixel(pub u8);

impl Pixel for IndexedPixel {
    fn empty() -> Self { Self(0) }
}
