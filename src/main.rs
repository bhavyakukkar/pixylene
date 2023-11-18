mod utils;
mod layer;
mod session;
mod state;
mod stroke;

use crate::utils::{ Coord, Pixel };
use crate::session::{ SessionScene, SessionCamera, SessionLayers };
use crate::layer::{ Scene, Camera, Layer };
use crate::stroke::Stroke;
use std::collections::HashMap;
use colored::*;

fn initialize_strokes() -> HashMap<String, Box<dyn Stroke>> {
    let mut strokes: HashMap<String, Box<dyn Stroke>> = HashMap::new();

    struct Pencil;
    impl Stroke for Pencil {
        fn get_num_clicks(&self) -> u8 {
            1
        }
        fn perform(&mut self, _click: u8, scene: &mut Scene, focus: Coord, color: Pixel) {
            scene.set_pixel(focus, color).unwrap();
        }
    }
    let pencil = Pencil;
    strokes.insert(String::from("pencil"), Box::new(pencil));

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
                            scene.set_pixel(Coord{x: i, y: j}, color).unwrap();
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

fn display_camera_grid(camera: &Camera) {
    for i in 0..camera.dim.x {
        for j in 0..camera.dim.y {
            match camera.get_camera_pixel(Coord{x: i, y: j}) {
                Some(camera_pixel) => {
                    match camera_pixel.color {
                        Pixel::B8(_) => panic!("Expecting 24-bit pixel"),
                        Pixel::B24{r, g, b} => {
                            print!("{}", " ".on_truecolor(r, g, b));
                        }
                    }
                },
                None => {
                    print!(" ");
                }
            }
        }
        println!();
    }
    println!();
}

fn main() {
    /*
    let mut scene = Scene::new(
        Coord { x: 3, y: 3 },
        vec![Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9)],
        Pixel::B24{ r: 0, g: 0, b: 0 }
    ).unwrap();
    let mut camera = Camera::new(
        &scene,
        Coord{ x: 8, y: 16 },
        Coord{ x: 1, y: 1 },
        1,
        Coord{ x: 1, y: 2 }
    ).unwrap();

    if let Some(pixel) = scene.get_pixel(camera.focus) {
        if let Pixel::B24{r, g, b} = pixel {
            scene.set_pixel(camera.focus, Pixel::B8(r+1)).unwrap();
        } else {
            panic!("pixel on scene at camera focus is 24-bit");
        }
    } else {
        panic!("pixel on scene at camera focus is not defined");
    }

    */
    //layer.camera.set_focus(&layer.scene, layer.camera.focus.add(Coord{ x:1, y: 1})).unwrap();
    
    let session_scene = SessionScene {
        dim: Coord { x: 20, y: 20 },
        background: Pixel::B24{ r: 0, g: 0, b: 0 },
    };
    let session_camera = SessionCamera {
        dim: Coord { x: 32, y: 64 },
        focus: Coord { x: 8, y: 8 },
        mult: 2,
        repeat: Coord { x: 1, y: 2 }
    };
    let session_layers = SessionLayers {
        len: 8,
    };

    let scene = Scene::new (
        Coord { x: 16, y: 16 },
        vec![Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 103, g: 58, b: 183 },  Pixel::B24{ r: 156, g: 39, b: 176 },  Pixel::B24{ r: 240, g: 98, b: 146 },  Pixel::B24{ r: 255, g: 23, b: 68 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 63, g: 81, b: 181 },  Pixel::B24{ r: 103, g: 58, b: 183 },  Pixel::B24{ r: 156, g: 39, b: 176 },  Pixel::B24{ r: 240, g: 98, b: 146 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 3, g: 169, b: 244 },  Pixel::B24{ r: 33, g: 150, b: 243 },  Pixel::B24{ r: 63, g: 81, b: 181 },  Pixel::B24{ r: 103, g: 58, b: 183 },  Pixel::B24{ r: 156, g: 39, b: 176 },  Pixel::B24{ r: 240, g: 98, b: 146 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 150, b: 136 },  Pixel::B24{ r: 0, g: 188, b: 212 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 156, g: 39, b: 176 },  Pixel::B24{ r: 240, g: 98, b: 146 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 205, g: 220, b: 57 },  Pixel::B24{ r: 139, g: 195, b: 72 },  Pixel::B24{ r: 76, g: 175, b: 80 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 156, g: 39, b: 176 },  Pixel::B24{ r: 240, g: 98, b: 146 },  Pixel::B24{ r: 255, g: 23, b: 68 },  Pixel::B24{ r: 255, g: 87, b: 34 },  Pixel::B24{ r: 62, g: 39, b: 35 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 235, b: 59 },  Pixel::B24{ r: 205, g: 220, b: 57 },  Pixel::B24{ r: 139, g: 195, b: 72 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 103, g: 58, b: 183 },  Pixel::B24{ r: 156, g: 39, b: 176 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 87, b: 34 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 235, b: 59 },  Pixel::B24{ r: 205, g: 220, b: 57 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 63, g: 81, b: 181 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 235, b: 59 },  Pixel::B24{ r: 205, g: 220, b: 57 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 3, g: 169, b: 244 },  Pixel::B24{ r: 33, g: 150, b: 243 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 193, b: 7 },  Pixel::B24{ r: 255, g: 235, b: 59 },  Pixel::B24{ r: 205, g: 220, b: 57 },  Pixel::B24{ r: 139, g: 195, b: 72 },  Pixel::B24{ r: 76, g: 175, b: 80 },  Pixel::B24{ r: 0, g: 150, b: 136 },  Pixel::B24{ r: 0, g: 188, b: 212 },  Pixel::B24{ r: 3, g: 169, b: 244 },  Pixel::B24{ r: 33, g: 150, b: 243 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 156, g: 39, b: 176 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 244, g: 67, b: 54 },  Pixel::B24{ r: 255, g: 152, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 33, g: 150, b: 243 },  Pixel::B24{ r: 63, g: 81, b: 181 },  Pixel::B24{ r: 103, g: 58, b: 183 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 0, g: 0, b: 0 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 },  Pixel::B24{ r: 255, g: 255, b: 255 }],
        //vec![Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8)],
        Pixel::B24{ r: 0, g: 0, b: 0 }
    ).unwrap();
    let mut layer = Layer::new_from_scene(&session_camera, scene).unwrap();
    //let mut layer = Layer::new(&session_scene, &session_camera).unwrap();

    layer.camera.render(&layer.scene);
    display_camera_grid(&layer.camera);

    let mut strokes = initialize_strokes();
    /*
    for (stroke_name, mut stroke) in strokes {
        println!("Performing {} at {}", stroke_name, layer.camera.focus);
        stroke.perform(0, &mut layer.scene, layer.camera.focus, Pixel::B8(9));
    }
    */
    if let Some(rectangle_fill) = strokes.get_mut("rectangle_fill") {
        rectangle_fill.perform(0, &mut layer.scene, Coord{ x: 5, y: 5 }, Pixel::B24{r: 0, g: 0, b: 0});
    }

    /*
    if let Some(pencil) = strokes.get_mut("pencil") {
        pencil.perform(0, &mut layer.scene, layer.camera.focus, Pixel::B24{r: 0, g: 0, b: 0});
    }
    */

    if let Some(rectangle_fill) = strokes.get_mut("rectangle_fill") {
        rectangle_fill.perform(1, &mut layer.scene, Coord{ x: 8, y: 11}, Pixel::B24{r: 0, g: 0, b: 0});
    }

    layer.camera.render(&layer.scene);
    display_camera_grid(&layer.camera);
}
