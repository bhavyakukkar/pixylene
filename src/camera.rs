use crate::coord::Coord;
use crate::pixel::Pixel;
use crate::scene::Scene;

pub struct Camera {
    pub dim: Coord,
    pub focus: Coord,
    pub mult: isize,
    pub repeat: Coord,
    grid: Vec<Option<(char,Pixel)>>,
    pixel_render_cascade_delay: u8
}

impl Camera {
    pub fn new(dim: Coord, focus: Coord, mult: isize, repeat: Coord, pixel_render_cascade_delay: u8) -> Self {
        let grid: Vec<Option<(char, Pixel)>> = vec![None;(dim.x * dim.y) as usize];
        Self{ dim, focus, mult, repeat, grid, pixel_render_cascade_delay }
    }
    pub fn set_focus(&mut self, scene: &Scene, new_focus: Coord) -> Result<(), String> {
        if new_focus.x >= 0 && new_focus.x < scene.dim.x && new_focus.y >= 0 && new_focus.y < scene.dim.y {
            self.focus = new_focus;
            Ok(())
        } else {
            Err(format!("cannot set focus to {} since image dimensions are {}", new_focus, scene.dim))
        }
    }
    pub fn set_mult(&mut self, new_mult: isize) {
        if new_mult > 0 {
            self.mult = new_mult;
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
                                self.grid[((i+mi)*self.dim.y + (j+mj)) as usize] = Some((' ', pixel));
                            },
                            None => {
                                self.grid[((i+mi)*self.dim.y + (j+mj)) as usize] = None;
                            }
                        }
                        //self.grid[((i+mi)*self.dim.y + (j+mj)) as usize] = Some((' ', scene.get_pixel(Coord{x, y})));
                        //render_to_screen_pixel(i+mi, j+mj, ' ', scene.get_pixel(Coord{x, y}));
                        /*match scene.get_pixel(Coord{x, y}) {
                            Some(pixel) => render_to_screen_pixel(i+mi, j+mj, ' ', pixel),
                            None => render_to_screen_pixel(i+mi, j+mj, ' ', Pixel())
                        }*/
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
    pub fn get_pixel(&self, coord: Coord) -> Option<(char,Pixel)> {
        self.grid[(coord.x*self.dim.y + coord.y) as usize]
    }
}
