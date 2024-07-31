use super::Pixel;
use serde::{Deserialize, Serialize};

use std::fmt;

/// A unit of indexed color, representing an 8-bit palette index
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Savefile)]
pub struct IndexedPixel(pub u8);

impl Pixel for IndexedPixel {
    fn empty() -> Self {
        Self(0)
    }
}

impl fmt::Display for IndexedPixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <3}", self.0)
    }
}
