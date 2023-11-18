use crate::utils::{ Coord, Pixel };
use crate::layer::Scene;

pub trait Stroke {
    fn get_num_clicks(&self) -> u8;
    fn perform(&mut self, click: u8, scene: &mut Scene, focus: Coord, color: Pixel);
}

/*
pub struct Stroke {
    pub clicks: u8,
    pub perform: fn(Stroke, click: u8, scene: &mut Scene, focus: Coord, color: Pixel),
}
*/

/*
pub struct Strokes(Vec<Stroke>);
impl Strokes {
    fn initialize() -> Strokes {
        let strokes = vec![pencil, paintbrush, rectangle, rectangle_fill];
        Strokes( strokes )
    }
}

struct pencil;
impl Stroke for pencil {
    fn get_num_clicks(&self) -> u8 {
        1
    }
    fn perform(self, click: u8, scene: &mut Scene, focus: Coord, color: Pixel) {
        scene.set_pixel(focus, color);
    }
}


struct paintbrush {
    strength: u8,
}
impl Stroke for paintbrush {
    fn get_num_clicks(&self) -> u8 {
        1
    }
    fn perform(self, click: u8, scene: &mut Scene, focus: Coord, color: Pixel) {
        scene.set_pixel(focus, color);
        todo!();
    }
}


struct rectangle {
    start_corner: Coord,
}
impl Stroke for rectangle {
    fn get_num_clicks(&self) -> u8 {
        2
    }
    fn perform(self, click: u8, scene: &mut Scene, focus: Coord, color: Pixel) {
        todo!();
    }
}


struct rectangle_fill {
    start_corner: Coord,
}
impl Stroke for rectangle_fill {
    fn get_num_clicks(&self) -> u8 {
        2
    }
    fn perform(self, click: u8, scene: &mut Scene, focus: Coord, color: Pixel) {
        match click {
            0 => {
                self.start_corner = focus;
            },
            1 => {
                for i in self.start_corner.x..(focus.x + 1) {
                    for j in self.start_corner.y..(focus.y + 1) {
                        scene.set_pixel(Coord{x: i, y: j}, color);
                    }
                }
            }
        }
    }
}
*/
