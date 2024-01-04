use crate::elements::common::{ Coord, Pixel };

#[derive(Savefile)]
pub struct Scene {
    pub dim: Coord,
    pub grid: Vec<Option<Pixel>>
}

impl Scene {
    pub fn new(dim: Coord, grid: Vec<Option<Pixel>>) -> Result<Self, &'static str> {
        if grid.len() as isize == (dim.x*dim.y) {
            Ok(Self{ dim, grid })
        }
        else {
            Err("Scene dimensions do not match pixel grid")
        }
    }
    pub fn get_pixel(&self, coord: Coord) -> Result<Option<Pixel>, String> {
        if coord.x >= 0 && coord.x < self.dim.x && coord.y >= 0 && coord.y < self.dim.y {
            Ok(self.grid[(coord.x*self.dim.y + coord.y) as usize])
        } else {
            Err(format!("Cannot get_pixel from invalid {} on scene of dimensions {}, \
                         valid values are between {} and {}", coord, self.dim, Coord{ x: 0, y: 0 }, self.dim.add(Coord{ x: -1, y: -1 })))
        }
    }
    pub fn set_pixel(&mut self, coord: Coord, new_pixel: Option<Pixel>) -> Result<(), String> {
        if coord.x >= 0 && coord.x < self.dim.x && coord.y >= 0 && coord.y < self.dim.y {
            self.grid[(coord.x*self.dim.y + coord.y) as usize] = new_pixel;
            Ok(())
        } else {
            Err(format!("cannot set_pixel at invalid {} on scene of dimensions {}, \
                         valid values are between {} and {}", coord, self.dim, Coord{ x: 0, y: 0 }, self.dim.add(Coord{ x: -1, y: -1 })))
        }
    }
}
