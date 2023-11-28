use crate::elements::common::{ Coord, Pixel };
use crate::elements::layer::Scene;

use std::collections::HashMap;

pub struct StrokeState {
    pub clicks_done: u8,
    pub clicks_required: u8
}

pub trait Stroke {
    fn get_num_clicks(&self) -> u8;
    fn perform_stroke(&mut self, click: u8, scene: &mut Scene, focus: Coord, color: Option<Pixel>) -> Result<(), String>;
}

impl dyn Stroke {
    pub fn initialize_strokes() -> HashMap<String, (Box<dyn Stroke>, StrokeState)> {
        let mut strokes: HashMap<String, (Box<dyn Stroke>, StrokeState)> = HashMap::new();

        struct Pencil;
        impl Stroke for Pencil {
            fn get_num_clicks(&self) -> u8 {
                1
            }
            fn perform_stroke(&mut self, _click: u8, scene: &mut Scene, focus: Coord, color: Option<Pixel>) -> Result<(), String> {
                scene.set_pixel(focus, color)?;
                Ok(())
            }
        }
        let pencil = Pencil;
        let clicks_required = pencil.get_num_clicks();
        strokes.insert(String::from("pencil"), (Box::new(pencil), StrokeState {
            clicks_done: 0,
            clicks_required: clicks_required
        }));

        //eraser is simple a Pencil that set_pixel's None
        struct Eraser;
        impl Stroke for Eraser {
            fn get_num_clicks(&self) -> u8 {
                1
            }
            fn perform_stroke(&mut self, _click: u8, scene: &mut Scene, focus: Coord, _color: Option<Pixel>) -> Result<(), String> {
                scene.set_pixel(focus, None)?;
                Ok(())
            }
        }
        let eraser = Eraser;
        let clicks_required = eraser.get_num_clicks();
        strokes.insert(String::from("eraser"), (Box::new(eraser), StrokeState {
            clicks_done: 0,
            clicks_required: clicks_required
        }));

        struct RectangleFill {
            start_corner: Coord,
        }
        impl Stroke for RectangleFill {
            fn get_num_clicks(&self) -> u8 {
                2
            }
            fn perform_stroke(&mut self, click: u8, scene: &mut Scene, focus: Coord, color: Option<Pixel>) -> Result<(), String> {
                match click {
                    0 => {
                        self.start_corner = focus;
                        Ok(())
                    },
                    1 => {
                        for i in self.start_corner.x..(focus.x + 1) {
                            for j in self.start_corner.y..(focus.y + 1) {
                                scene.set_pixel(Coord{x: i, y: j}, color)?;
                            }
                        }
                        Ok(())
                    },
                    _ => Err(String::from("invalid click-done parameter passed, expecting only 0 or 1")),
                }
            }
        }
        let rectangle_fill = RectangleFill{ start_corner: Coord{ x: 0, y: 0 }};
        let clicks_required = rectangle_fill.get_num_clicks();
        strokes.insert(String::from("rectangle_fill"), (Box::new(rectangle_fill), StrokeState {
            clicks_done: 0,
            clicks_required: clicks_required
        }));

        strokes
    }
}
