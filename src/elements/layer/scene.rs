use crate::elements::common::{ Coord, Pixel };

#[derive(Debug)]
pub enum SceneError {
    DimensionMismatch(usize, Coord),
    NegativeCoordinates(Coord),
    OutOfBoundCoordinates(Coord, Coord),
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
            NegativeCoordinates(coord) => write!(
                f,
                "Cannot get_pixel from negative coordinates, found: {}",
                coord,
            ),
            OutOfBoundCoordinates(coord, dim) => write!(
                f,
                "Cannot get_pixel from out-of-bounds coordinates {} on scene of dimensions {}, \
                valid coordinates for this scene lie between {} and {} (inclusive)",
                coord,
                dim,
                Coord{ x: 0, y: 0 },
                dim.add(Coord{ x: -1, y: -1 }),
            ),
        }
    }
}

#[derive(Savefile)]
pub struct Scene {
    dim: Coord,
    grid: Vec<Option<Pixel>>
}

impl Scene {
    pub fn new(dimensions: Coord, flattened_grid: Vec<Option<Pixel>>) -> Result<Self, SceneError> {
        use SceneError::{ DimensionMismatch };
        if flattened_grid.len() as isize != (dimensions.x*dimensions.y) {
            Err(DimensionMismatch(flattened_grid.len(), dimensions))
        }
        else {
            Ok(Self{ dim: dimensions, grid: flattened_grid })
        }
    }
    pub fn get_pixel(&self, coord: Coord) -> Result<Option<Pixel>, SceneError> {
        use SceneError::{ NegativeCoordinates, OutOfBoundCoordinates };
        if coord.x < 0 || coord.y < 0 {
            Err(NegativeCoordinates(coord))
        }
        else if coord.x >= self.dim.x || coord.y >= self.dim.y {
            Err(OutOfBoundCoordinates(coord, self.dim))
        }
        else {
            Ok(self.grid[(coord.x*self.dim.y + coord.y) as usize])
        }
    }
    pub fn set_pixel(&mut self, coord: Coord, new_pixel: Option<Pixel>) -> Result<(), SceneError> {
        use SceneError::{ NegativeCoordinates, OutOfBoundCoordinates };
        if coord.x < 0 || coord.y < 0 {
            Err(NegativeCoordinates(coord))
        }
        else if coord.x >= self.dim.x || coord.y >= self.dim.y {
            Err(OutOfBoundCoordinates(coord, self.dim))
        } else {
            self.grid[(coord.x*self.dim.y + coord.y) as usize] = new_pixel;
            Ok(())
        }
    }
    pub fn dim(&self) -> Coord {
        self.dim
    }
}
