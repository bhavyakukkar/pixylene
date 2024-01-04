use crate::elements::common::{ Coord, Pixel };
use crate::elements::layer::Scene;
use crate::grammar::Decorate;

#[derive(Debug)]
enum CameraError {
    NonNaturalDimensions(String),
    FocusDoesntLieOnScene(String),
    NonNaturalMultiplier(String),
    NonNaturalRepeat(String),
}
use CameraError::*;

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use CameraError::*;
        let n = String::from("CameraError");
        match self {
            NonNaturalDimensions(desc) => Decorate::output(n, Some("NonNaturalDimensions"), Some(desc)),
            FocusDoesntLieOnScene(desc) => Decorate::output(n, Some("FocusDoesLieOnScene"), Some(desc)),
            NonNaturalMultiplier(desc) => Decorate::output(n, Some("NonNaturalMultiplier"), Some(desc)),
            NonNaturalRepeat(desc) => Decorate::output(n, Some("NonNaturalRepeat"), Some(desc)),
        }
    }
}

#[derive(Copy, Clone)]
pub enum CameraPixel {
    Filled {
        brush: char,
        color: Pixel,
        is_focus: bool,
    },
    Empty,
    OutOfScene
}

#[derive(Default, Savefile)]
pub struct Camera {
    pub dim: Coord,
    pub focus: Coord,
    pub mult: isize,
    pub repeat: Coord,
}

impl Camera {
    pub fn new(
        scene: &Scene,
        dimensions: Coord,
        focus: Coord,
        multiplier: isize,
        camera_pixels_per_scene_pixel: Coord
    ) -> Result<Self, CameraError> {
        let mut camera: Self = Self{ ..Default::default() };
        camera.set_dim(dimensions)?;
        camera.set_focus(scene, focus)?;
        camera.set_mult(multiplier)?;
        camera.set_repeat(camera_pixels_per_scene_pixel)?;
        Ok(camera)
    }
    pub fn set_dim(&mut self, new_dim: Coord) -> Result<(), CameraError> {
        if new_dim.x > 0 && new_dim.y > 0 {
            self.dim = new_dim;
            Ok(())
        } else {
            Err(NonNaturalDimensions(format!(
                "cannot set camera's dimensions to negative coordinates, found: {}",
                new_dim
            )))
        }
    }
    pub fn set_focus(&mut self, scene: &Scene, new_focus: Coord) -> Result<(), CameraError> {
        if new_focus.x >= 0 && new_focus.x < scene.dim.x && new_focus.y >= 0 && new_focus.y < scene.dim.y {
            self.focus = new_focus;
            Ok(())
        } else {
            Err(format!("cannot set camera's focus to {} since image dimensions are {}", new_focus, scene.dim))
        }
    }
    pub fn set_mult(&mut self, new_mult: isize) -> Result<(), CameraError> {
        if new_mult > 0 {
            self.mult = new_mult;
            Ok(())
        } else {
            Err(format!("cannot set camera's multiplier to 0 or negative value, found {}", new_mult))
        }
    }
    pub fn set_repeat(&mut self, new_repeat: Coord) -> Result<(), CameraError> {
        if new_repeat.x > 0 && new_repeat.y > 0 {
            self.repeat = new_repeat;
            Ok(())
        } else {
            Err(format!("cannot set camera's repeat to negative coordinates, found: {}", new_repeat))
        }
    }
    pub fn render(&mut self, scene: &Scene) -> Vec<CameraPixel> {
        let mut grid: Vec<CameraPixel> = vec![CameraPixel::OutOfScene; (self.dim.x * self.dim.y) as usize];
        let mut render_pixel = |i: isize, j: isize, x: isize, y: isize, is_focus: bool| {
            for mi in 0..self.mult*self.repeat.x {
                for mj in 0..self.mult*self.repeat.y {
                    if (i+mi) < 0 || (i+mi) >= self.dim.x || (j+mj) < 0 || (j+mj) >= self.dim.y {
                        continue;
                    }
                    match scene.get_pixel(Coord{x, y}) {
                        Ok(pixel_maybe) => {
                            match pixel_maybe {
                                Some(pixel) => {
                                    grid[((i+mi)*self.dim.y + (j+mj)) as usize] = CameraPixel::Filled{
                                        brush: ' ',
                                        color: pixel,
                                        is_focus: is_focus,
                                    };
                                },
                                None => {
                                    grid[((i+mi)*self.dim.y + (j+mj)) as usize] = CameraPixel::Empty;
                                }
                            }
                        },
                        Err(_) => {
                            grid[((i+mi)*self.dim.y + (j+mj)) as usize] = CameraPixel::OutOfScene;
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
                render_pixel(i, j, x, y, i == mid.x && j == mid.y);
                j -= mult*repeat.y;
                y -= 1;
            }
            j = mid.y + mult*repeat.y;
            y = focus.y + 1;
            while j < dim.y {
                render_pixel(i, j, x, y, i == mid.x && j == mid.y);
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
                render_pixel(i, j, x, y, false);
                j -= mult*repeat.y;
                y -= 1;
            }
            j = mid.y + mult*repeat.y;
            y = focus.y + 1;
            while j < dim.y {
                render_pixel(i, j, x, y, false);
                j += mult*repeat.y;
                y += 1;
            }
            i += mult*repeat.x;
            x += 1;
        }
        return grid;
    }
}
