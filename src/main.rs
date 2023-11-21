mod utils;
mod layer;
mod session;
mod state;
mod stroke;

use crate::utils::{ Coord, Pixel };
use crate::session::{ SessionScene, SessionCamera, SessionLayers };
use crate::layer::{ Scene, Camera, CameraPixel, Layer };
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

fn display_camera_grid(camera: &Camera) {
    for i in 0..camera.dim.x {
        for j in 0..camera.dim.y {
            match camera.get_camera_pixel(Coord{x: i, y: j}).unwrap() {
                CameraPixel::Filled{ brush, color } => {
                    print!("{}", " ".on_truecolor(color.r, color.g, color.b));
                },
                CameraPixel::Empty => {
                    print!(" ");
                },
                CameraPixel::OutOfScene => {
                    print!(" ");
                }
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let session_scene = SessionScene {
        dim: Coord { x: 20, y: 20 },
        background: None
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
        vec![Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 23, b: 68, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 63, g: 81, b: 181, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 3, g: 169, b: 244, a: 255 }),  Some(Pixel{ r: 33, g: 150, b: 243, a: 255 }),  Some(Pixel{ r: 63, g: 81, b: 181, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 150, b: 136, a: 255 }),  Some(Pixel{ r: 0, g: 188, b: 212, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 139, g: 195, b: 72, a: 255 }),  Some(Pixel{ r: 76, g: 175, b: 80, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 23, b: 68, a: 255 }),  Some(Pixel{ r: 255, g: 87, b: 34, a: 255 }),  Some(Pixel{ r: 62, g: 39, b: 35, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 235, b: 59, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 139, g: 195, b: 72, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 87, b: 34, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 235, b: 59, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 63, g: 81, b: 181, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 235, b: 59, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 3, g: 169, b: 244, a: 255 }),  Some(Pixel{ r: 33, g: 150, b: 243, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 193, b: 7, a: 255 }),  Some(Pixel{ r: 255, g: 235, b: 59, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 139, g: 195, b: 72, a: 255 }),  Some(Pixel{ r: 76, g: 175, b: 80, a: 255 }),  Some(Pixel{ r: 0, g: 150, b: 136, a: 255 }),  Some(Pixel{ r: 0, g: 188, b: 212, a: 255 }),  Some(Pixel{ r: 3, g: 169, b: 244, a: 255 }),  Some(Pixel{ r: 33, g: 150, b: 243, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 244, g: 67, b: 54, a: 255 }),  Some(Pixel{ r: 255, g: 152, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 33, g: 150, b: 243, a: 255 }),  Some(Pixel{ r: 63, g: 81, b: 181, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 })]
        //vec![Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255), Pixel(9, 255), Pixel(0, 255), Pixel(1, 255), Pixel(2, 255), Pixel(3, 255), Pixel(4, 255), Pixel(5, 255), Pixel(6, 255), Pixel(7, 255), Pixel(8, 255)],
    ).unwrap();
    let mut layer = Layer::new_from_scene(&session_camera, scene).unwrap();
    //let mut layer = Layer::new(&session_scene, &session_camera).unwrap();

    layer.camera.render(&layer.scene);
    display_camera_grid(&layer.camera);

    let mut strokes = initialize_strokes();
    /*
    for (stroke_name, mut stroke) in strokes {
        println!("Performing {} at {}", stroke_name, layer.camera.focus);
        stroke.perform(0, &mut layer.scene, layer.camera.focus, Pixel(9, 255));
    }
    */
    if let Some(rectangle_fill) = strokes.get_mut("rectangle_fill") {
        rectangle_fill.perform(0, &mut layer.scene, Coord{ x: 5, y: 5 }, Pixel{r: 0, g: 0, b: 0, a: 255});
    }

    /*
    if let Some(pencil) = strokes.get_mut("pencil") {
        pencil.perform(0, &mut layer.scene, layer.camera.focus, Pixel{r: 0, g: 0, b: 0, a: 255});
    }
    */

    if let Some(rectangle_fill) = strokes.get_mut("rectangle_fill") {
        rectangle_fill.perform(1, &mut layer.scene, Coord{ x: 8, y: 11}, Pixel{r: 0, g: 0, b: 0, a: 255});
    }

    layer.camera.render(&layer.scene);
    display_camera_grid(&layer.camera);
}
