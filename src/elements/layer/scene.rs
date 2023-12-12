use crate::elements::common::{Coord, Pixel};

#[derive(Savefile)]
pub struct Scene {
    pub dim: Coord,
    pub grid: Vec<Option<Pixel>>
}

impl Scene {
    pub fn new(dim: Coord, grid: Vec<Option<Pixel>>) -> Result<Self, String> {
        if grid.len() == (dim.x*dim.y).try_into().unwrap() {
            Ok(Self{ dim, grid })
        }
        else {
            Err(format!("Scene dimensions do not match pixel grid"))
        }
    }
    pub fn get_pixel(&self, coord: Coord) -> Result<Option<Pixel>, String> {
        if coord.x >= 0 && coord.x < self.dim.x && coord.y >= 0 && coord.y < self.dim.y {
            Ok(self.grid[(coord.x*self.dim.y + coord.y) as usize])
        } else {
            Err(format!("Cannot get pixel {} from scene of dimensions {}", coord, self.dim))
        }
    }
    pub fn set_pixel(&mut self, coord: Coord, new_pixel: Option<Pixel>) -> Result<(), String> {
        if coord.x >= 0 && coord.x < self.dim.x && coord.y >= 0 && coord.y < self.dim.y {
            self.grid[(coord.x*self.dim.y + coord.y) as usize] = new_pixel;
            Ok(())
        } else {
            Err(format!("cannot set_pixel at invalid {} on scene of dimensions {}", coord, self.dim))
        }
    }
}
