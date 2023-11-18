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
                        Pixel::B24{..} => panic!("Expecting 8-bit pixel"),
                        Pixel::B8(c) => {
                            print!("{}", c);
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
    println!("\n");
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
        background: Pixel::B8(0)
    };
    let session_camera = SessionCamera {
        dim: Coord { x: 10, y: 10 },
        focus: Coord { x: 10, y: 10 },
        mult: 1,
        repeat: Coord { x: 1, y: 1 }
    };
    let session_layers = SessionLayers {
        len: 8,
    };

    let scene = Scene::new (
        Coord { x: 20, y: 20 },
        vec![Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8)],
        Pixel::B8(0)
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
        rectangle_fill.perform(0, &mut layer.scene, layer.camera.focus, Pixel::B8(9));

        layer.camera.set_focus(&layer.scene, layer.camera.focus.add(Coord{ x: 2, y: 2 })).unwrap();
        layer.camera.render(&layer.scene);
        display_camera_grid(&layer.camera);

        rectangle_fill.perform(1, &mut layer.scene, layer.camera.focus, Pixel::B8(9));
    }

    layer.camera.render(&layer.scene);
    display_camera_grid(&layer.camera);
}
