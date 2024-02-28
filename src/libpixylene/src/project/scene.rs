use std::num::{ TryFromIntError };

use crate::{
    types::{ Coord, PCoord, UCoord, Pixel },
    utils::messages::U32TOUSIZE,
};

#[derive(Savefile, Clone)]
pub struct Scene {
    dim: PCoord,
    grid: Vec<Option<Pixel>>
}

impl Scene {
    pub fn new(dimensions: PCoord, flat_grid: Vec<Option<Pixel>>) -> Result<Self, SceneError> {
        use SceneError::{ DimensionMismatch };
        if flat_grid.len() != usize::try_from(dimensions.area()).expect(U32TOUSIZE) {
            Err(DimensionMismatch(flat_grid.len(), dimensions))
        } else {
            Ok(Self{ dim: dimensions, grid: flat_grid })
        }
    }
    pub fn get_pixel(&self, coord: UCoord) -> Result<Option<Pixel>, SceneError> {
        use SceneError::{ OutOfBoundCoordinates };
        if coord.x >= self.dim.x() || coord.y >= self.dim.y() {
            Err(OutOfBoundCoordinates(coord, self.dim))
        }
        else {
            let index = usize::from(coord.x) * usize::from(self.dim.y()) + usize::from(coord.y);
            Ok(self.grid[index])
        }
    }
    pub fn get_pixel_raw(&self, coord: Coord) -> Option<Option<Pixel>> {
        if coord.x < 0 || coord.y < 0 { None }
        else if coord.x >= i32::from(self.dim.x()) || coord.y >= i32::from(self.dim.y()) { None }
        else {
            let index = usize::try_from(coord.x).unwrap() * usize::from(self.dim.y()) +
                usize::try_from(coord.y).unwrap();
            Some(self.grid[index])
        }
    }
    pub fn set_pixel(&mut self, coord: UCoord, new_pixel: Option<Pixel>) -> Result<(), SceneError> {
        use SceneError::{ OutOfBoundCoordinates };
        if coord.x >= self.dim.x() || coord.y >= self.dim.y() {
            Err(OutOfBoundCoordinates(coord, self.dim))
        } else {
            let index = usize::from(coord.x) * usize::from(self.dim.y()) + usize::from(coord.y);
            self.grid[index] = new_pixel;
            Ok(())
        }
    }
    pub fn dim(&self) -> PCoord {
        self.dim
    }
}


// Error Types

#[derive(Debug)]
pub enum SceneError {
    DimensionMismatch(usize, PCoord),
    OutOfBoundCoordinates(UCoord, PCoord),
    TryFromIntError(PCoord, TryFromIntError),
}

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SceneError::*;
        match self {
            DimensionMismatch(length, dim) => write!(
                f,
                "Flattened grid found of length {} which does not match {} from product of given \
                dimensions {}",
                length,
                dim.area(),
                dim,
            ),
            OutOfBoundCoordinates(coord, dim) => write!(
                f,
                "Cannot get_pixel from out-of-bounds coordinates {} on scene of dimensions {}, \
                valid coordinates for this scene lie between {} and {} (inclusive)",
                coord,
                dim,
                UCoord{ x: 0, y: 0 },
                Coord::from(dim).add(Coord{ x: -1, y: -1 }),
            ),
            TryFromIntError(dim, err) => write!(
                f,
                "Cannot parse dimension area {} from given dimensions {} as usize due to your \
                architecture: {}",
                dim.area(),
                dim,
                err,
            ),
        }
    }
}

/*
impl From<TryFromIntError> for SceneError {
    fn from(item: TryFromIntError) {
        SceneError::TryFromIntError(item)
    }
}
*/
