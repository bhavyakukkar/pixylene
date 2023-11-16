use crate::utils::{Coord, Pixel};
use crate::layer::Scene;

#[derive(Copy, Clone)]
pub struct CameraPixel {
    pub brush: char,
    pub color: Pixel
}

#[derive(Default)]
pub struct Camera {
    pub dim: Coord,
    pub focus: Coord,
    pub mult: isize,
    pub repeat: Coord,
    grid: Vec<Option<CameraPixel>>
}

impl Camera {
    pub fn new(scene: &Scene, dim: Coord, focus: Coord, mult: isize, repeat: Coord) -> Result<Self, String> {
        let grid: Vec<Option<CameraPixel>> = vec![None;(dim.x * dim.y) as usize];
        let mut camera: Self = Self{ grid: grid, ..Default::default() };
        camera.set_dim(dim)?;
        camera.set_focus(scene, focus)?;
        camera.set_mult(mult)?;
        camera.set_repeat(repeat)?;
        Ok(camera)
    }
    fn set_dim(&mut self, new_dim: Coord) -> Result<(), String> {
        if new_dim.x > 0 && new_dim.y > 0 {
            self.dim = new_dim;
            Ok(())
        } else {
            Err(format!("cannot set dimensions to negative coordinates, found: {}", new_dim))
        }
    }
    pub fn set_focus(&mut self, scene: &Scene, new_focus: Coord) -> Result<(), String> {
        if new_focus.x >= 0 && new_focus.x < scene.dim.x && new_focus.y >= 0 && new_focus.y < scene.dim.y {
            self.focus = new_focus;
            Ok(())
        } else {
            Err(format!("cannot set focus to {} since image dimensions are {}", new_focus, scene.dim))
        }
    }
    pub fn set_mult(&mut self, new_mult: isize) -> Result<(), String> {
        if new_mult > 0 {
            self.mult = new_mult;
            Ok(())
        } else {
            Err(format!("cannot set multiplier to 0 or negative value, found {}", new_mult))
        }
    }
    fn set_repeat(&mut self, new_repeat: Coord) -> Result<(), String> {
        if new_repeat.x > 0 && new_repeat.y > 0 {
            self.repeat = new_repeat;
            Ok(())
        } else {
            Err(format!("cannot set repeat to negative coordinates, found: {}", new_repeat))
        }
    }
    fn decode(&self, pixel: Pixel) -> char {
        return 'O';
    }
    pub fn render(&mut self, scene: &Scene) {
        let mut render_pixel = |i: isize, j: isize, x: isize, y: isize| {
            for mi in 0..self.mult*self.repeat.x {
                for mj in 0..self.mult*self.repeat.y {
                    if (i+mi) >= 0 && (i+mi) < self.dim.x && (j+mj) >= 0 && (j+mj) < self.dim.y {
                        match scene.get_pixel(Coord {x, y}) {
                            Some(pixel) => {
                                self.grid[((i+mi)*self.dim.y + (j+mj)) as usize] = Some(CameraPixel{
                                    brush: ' ',
                                    color: pixel
                                });
                            },
                            None => {
                                self.grid[((i+mi)*self.dim.y + (j+mj)) as usize] = None;
                            }
                        }
                    }
                }
            }
        };
        let dim = self.dim;
        let focus = self.focus;
        let mult = self.mult;
        let repeat = self.repeat;
        let mid: Coord = Coord{ x: (dim.x - (mult*repeat.x))/2, y: (dim.y - (mult*repeat.y))/2 };

        let mut i = mid.x;
        let mut x = focus.x;
        while i > -1*mult*repeat.x {
            let mut j = mid.y;
            let mut y = focus.y;
            while j > -1*mult*repeat.y {
                render_pixel(i, j, x, y);
                j -= mult*repeat.y;
                y -= 1;
            }
            j = mid.y + mult*repeat.y;
            y = focus.y + 1;
            while j < dim.y {
                render_pixel(i, j, x, y);
                j += mult*repeat.y;
                y += 1;
            }
            i -= mult*repeat.x;
            x -= 1;
        }
        i = mid.x + mult*repeat.x;
        x = focus.x + 1;
        while i < dim.x {
            let mut j = mid.y;
            let mut y = focus.y;
            while j > -1*mult*repeat.y {
                render_pixel(i, j, x, y);
                j -= mult*repeat.y;
                y -= 1;
            }
            j = mid.y + mult*repeat.y;
            y = focus.y + 1;
            while j < dim.y {
                render_pixel(i, j, x, y);
                j += mult*repeat.y;
                y += 1;
            }
            i += mult*repeat.x;
            x += 1;
        }
    }
    pub fn get_camera_pixel(&self, coord: Coord) -> Option<CameraPixel> {
        self.grid[(coord.x*self.dim.y + coord.y) as usize]
    }
}
