use crate::elements::common::{ Coord, Pixel };
use crate::elements::layer::Scene;

use std::collections::HashMap;

pub trait Stroke {
    fn get_num_clicks(&self) -> u8;
    fn perform(&mut self, click: u8, scene: &mut Scene, focus: Coord, color: Pixel);
}

impl dyn Stroke {
    pub fn initialize_strokes() -> HashMap<String, Box<dyn Stroke>> {
        let mut strokes: HashMap<String, Box<dyn Stroke>> = HashMap::new();

        struct Pencil;
        impl Stroke for Pencil {
            fn get_num_clicks(&self) -> u8 {
                1
            }
            fn perform(&mut self, _click: u8, scene: &mut Scene, focus: Coord, color: Pixel) {
                scene.set_pixel(focus, Some(color)).unwrap();
            }
        }
        let pencil = Pencil;
        strokes.insert(String::from("pencil"), Box::new(pencil));

        struct Eraser;
        impl Stroke for Eraser {
            fn get_num_clicks(&self) -> u8 {
                1
            }
            fn perform(&mut self, _click: u8, scene: &mut Scene, focus: Coord, _color: Pixel) {
                scene.set_pixel(focus, None).unwrap();
            }
        }
        let eraser = Eraser;
        strokes.insert(String::from("eraser"), Box::new(eraser));

        struct RectangleFill {
            start_corner: Coord,
        }
        impl Stroke for RectangleFill {
            fn get_num_clicks(&self) -> u8 {
                2
            }
            fn perform(&mut self, click: u8, scene: &mut Scene, focus: Coord, color: Pixel) {
                match click {
                    0 => {
                        self.start_corner = focus;
                    },
                    1 => {
                        for i in self.start_corner.x..(focus.x + 1) {
                            for j in self.start_corner.y..(focus.y + 1) {
                                scene.set_pixel(Coord{x: i, y: j}, Some(color)).unwrap();
                            }
                        }
                    },
                    _ => panic!(),
                }
            }
        }
        let rectangle_fill = RectangleFill{ start_corner: Coord{ x: 0, y: 0 }};
        strokes.insert(String::from("rectangle_fill"), Box::new(rectangle_fill));

        strokes
    }
}
