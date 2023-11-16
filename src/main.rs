mod utils;
mod layer;
mod session;
mod state;

use crate::utils::{ Coord, Pixel };
use crate::session::{ SessionScene, SessionCamera, Session };
use crate::layer::{ Scene, Camera, Layer };
//use crate::scene_view::primitives::{Coord, Pixel};
//use crate::scene_view::{Scene, Camera};

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
    camera.render(&scene);
    display_camera_grid(&camera);

    if let Some(pixel) = scene.get_pixel(camera.focus) {
        if let Pixel::B24{r, g, b} = pixel {
            scene.set_pixel(camera.focus, Pixel::B8(r+1)).unwrap();
        } else {
            panic!("pixel on scene at camera focus is 24-bit");
        }
    } else {
        panic!("pixel on scene at camera focus is not defined");
    }
    //scene.set_pixel(Coord{ x: 2, y: 2 }, Pixel::B8(scene.get_pixel(Coord{x: 2, y: 2}))).unwrap();
            
    camera.render(&scene);
    display_camera_grid(&camera);
    */
    //for z in 0..3 {
    //    camera.render(&scene);
    //    println!("focus is now at ({}, {})", camera.focus.x, camera.focus.y);
    //    displayCameraGrid(&camera);
    //    camera.set_focus(&scene, Coord{ x: (camera.focus.x + 1), y: (camera.focus.y + 1) }).unwrap();
    //    println!("\n");
    //}
    
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
    /*
    let session = Session { scene: session_scene, camera: session_camera };
    let mut layer1 = Layer::new(&session);

    layer1.camera.render(&layer1.scene);
    display_camera_grid(&layer1.camera);

    layer1.scene.set_pixel(Coord { x: 10, y: 10 }, Pixel::B8(9));
    layer1.camera.render(&layer1.scene);
    display_camera_grid(&layer1.camera);
    */

    let scene = Scene::new (
        Coord { x: 20, y: 20 },
        vec![Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9), Pixel::B8(0), Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8)],
        Pixel::B8(0)
    ).unwrap();
    let mut layer2 = Layer::new_from_scene(&session_camera, scene);

    layer2.camera.render(&layer2.scene);
    display_camera_grid(&layer2.camera);

    layer2.scene.set_pixel(Coord { x: 10, y: 10 }, Pixel::B8(9)).unwrap();
    layer2.camera.render(&layer2.scene);
    display_camera_grid(&layer2.camera);
}
