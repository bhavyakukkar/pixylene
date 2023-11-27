use crate::elements::common::{ Coord, Pixel };

pub struct SessionScene {
    pub dim: Coord,
    pub background: Option<Pixel>
}

pub struct SessionCamera {
    pub dim: Coord,
    pub focus: Coord,
    pub mult: isize,
    pub repeat: Coord
}

pub struct SessionLayers {
    pub len: u8,
}
