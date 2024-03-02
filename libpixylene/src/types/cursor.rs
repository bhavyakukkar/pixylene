use crate::types::{ Coord };

#[derive(Clone, Copy, PartialEq, Debug, Savefile)]
pub struct Cursor {
    pub layer: usize,
    pub coord: Coord,
}
