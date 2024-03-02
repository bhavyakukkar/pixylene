use crate::{
    types::{ Coord, UCoord, PCoord, Pixel },
    project::{ Scene },
    utils::messages::U32TOUSIZE,
};

#[derive(Copy, Clone)]
pub enum CameraPixel {
    Filled {
        scene_coord: UCoord,
        brush: char,
        color: Pixel,
        is_focus: bool,
    },
    Empty {
        scene_coord: UCoord
    },
    OutOfScene
}

#[derive(Default, Savefile)]
pub struct Camera {
    pub dim: PCoord,
    mult: u8,
    pub repeat: (u8, u8),
}

impl Camera {
    pub fn new(
        dim: PCoord,
        multiplier: u8,
        repeat: (u8, u8),
    ) -> Result<Self, CameraError> {
        let mut camera = Camera{ dim, mult: 1, repeat };
        camera.set_mult(multiplier)?;
        Ok(camera)
    }
    pub fn get_mult(& self) -> u8 {
        self.mult
    }
    pub fn set_mult(&mut self, new_mult: u8) -> Result<(), CameraError> {
        use CameraError::{ ZeroMultiplier };
        if new_mult > 0 {
            self.mult = new_mult;
            Ok(())
        } else {
            Err(ZeroMultiplier)
        }
    }
    pub fn render_scene(&self, scene: &Scene, focus: Coord) -> Vec<CameraPixel> {
        use CameraPixel::*;
        let mut grid: Vec<CameraPixel> = vec![
            CameraPixel::OutOfScene;
            usize::try_from(self.dim.area()).expect(U32TOUSIZE)
        ];
        let mut render_pixel = |i: i64, j: i64, x: i32, y: i32, is_focus: bool| {
            for mi in 0..i64::from(u16::from(self.mult)*u16::from(self.repeat.0)) {
                for mj in 0..i64::from(u16::from(self.mult)*u16::from(self.repeat.1)) {
                    if (i+mi) < 0 || (i+mi) >= i64::from(self.dim.x()) ||
                       (j+mj) < 0 || (j+mj) >= i64::from(self.dim.y()) {
                        continue;
                    }
                    if let Some(pixel_maybe) = scene.get_pixel_raw(Coord{x, y}) {
                        if let Some(pixel) = pixel_maybe {
                            let index = usize::try_from(i+mi).unwrap() *
                                        usize::from(self.dim.y()) +
                                        usize::try_from(j+mj).unwrap();
                            grid[index] = Filled {
                                scene_coord: UCoord{
                                    x: u16::try_from(x).unwrap(),
                                    y: u16::try_from(y).unwrap()
                                },
                                brush: ' ',
                                color: pixel,
                                is_focus: is_focus,
                            };
                        } else {
                            let index = usize::try_from(i+mi).unwrap() *
                                        usize::from(self.dim.y()) +
                                        usize::try_from(j+mj).unwrap();
                            grid[index] = Empty {
                                scene_coord: UCoord{
                                    x: u16::try_from(x).unwrap(),
                                    y: u16::try_from(y).unwrap()
                                },
                            };
                        }
                    } else {
                        let index = usize::try_from(i+mi).unwrap() *
                                    usize::from(self.dim.y()) +
                                    usize::try_from(j+mj).unwrap();
                        grid[index] = OutOfScene;
                    }
                }
            }
        };
        let dim = self.dim;
        let mult = self.mult;
        let repeat = self.repeat;

        let focus_x = i32::from(focus.x);
        let focus_y = i32::from(focus.y);
        let mult_x = i64::from(u16::from(mult) * u16::from(repeat.0));
        let mult_y = i64::from(u16::from(mult) * u16::from(repeat.1));
        let mid_x = i64::from((dim.x() - u16::from(mult) * u16::from(repeat.0))
                              .checked_div(2).unwrap());
        let mid_y = i64::from((dim.y() - u16::from(mult) * u16::from(repeat.1))
                              .checked_div(2).unwrap());

        let mut i = mid_x;
        let mut x = focus_x;
        while i > -1*mult_x {
            let mut j = mid_y;
            let mut y = focus_y;
            while j > -1*mult_y {
                render_pixel(i, j, x, y, i == mid_x && j == mid_y);
                j -= mult_y;
                y -= 1;
            }
            j = mid_y + mult_y;
            y = focus_y + 1;
            while j < dim.y().into() {
                render_pixel(i, j, x, y, i == mid_x && j == mid_y);
                j += mult_y;
                y += 1;
            }
            i -= mult_x;
            x -= 1;
        }
        i = mid_x + mult_x;
        x = focus_x + 1;
        while i < dim.x().into() {
            let mut j = mid_y;
            let mut y = focus_y;
            while j > -1*mult_y {
                render_pixel(i, j, x, y, false);
                j -= mult_y;
                y -= 1;
            }
            j = mid_y + mult_y;
            y = focus_y + 1;
            while j < dim.y().into() {
                render_pixel(i, j, x, y, false);
                j += mult_y;
                y += 1;
            }
            i += mult_x;
            x += 1;
        }
        return grid;
    }
}


// Error Types

#[derive(Debug)]
pub enum CameraError {
    ZeroMultiplier,
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use CameraError::*;
        match self {
            ZeroMultiplier => write!(
                f,
                "cannot set camera's multiplier to 0",
            ),
        }
    }
}
