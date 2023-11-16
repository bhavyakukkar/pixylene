use crate::utils::{Coord, Pixel};

pub struct Scene {
    pub dim: Coord,
    pub grid: Vec<Pixel>,
    pub pixel_depth: Pixel
}

impl Scene {
    pub fn new(dim: Coord, grid: Vec<Pixel>, pixel_depth: Pixel) -> Result<Self, String> {
        if grid.len() == (dim.x*dim.y).try_into().unwrap() {
            Ok(Self{ dim, grid, pixel_depth })
        }
        else {
            Err(format!("Scene dimensions do not match pixel grid"))
        }
    }
    pub fn get_pixel(&self, coord: Coord) -> Option<Pixel> {
        if coord.x >= 0 && coord.x < self.dim.x && coord.y >= 0 && coord.y < self.dim.y {
            Some(self.grid[(coord.x*self.dim.y + coord.y) as usize])
        } else {
            None
        }
    }
    pub fn set_pixel(&mut self, coord: Coord, new_pixel: Pixel) -> Result<(), String> {
        if coord.x >= 0 && coord.x < self.dim.x && coord.y >= 0 && coord.y < self.dim.y {
            match new_pixel {
                Pixel::B24{..} => match self.pixel_depth {
                    Pixel::B24{..} => {
                        self.grid[(coord.x*self.dim.y + coord.y) as usize] = new_pixel;
                        Ok(())
                    },
                    Pixel::B8(_) => Err("Cannot set 24-bit pixel to 8-bit scene".to_string()),
                },
                Pixel::B8(_) => match self.pixel_depth {
                    Pixel::B8(_) => {
                        self.grid[(coord.x*self.dim.y + coord.y) as usize] = new_pixel;
                        Ok(())
                    },
                    Pixel::B24{..} => Err("Cannot set 8-bit pixel to 24-bit scene".to_string()),
                }
            }
        } else {
            Err(format!("cannot set to pixel that is negative or out-of-bounds on scene on scene of dimensions {}, found: {}", self.dim, coord))
        }
    }
}
