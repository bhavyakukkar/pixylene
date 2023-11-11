pub mod coord;
pub mod pixel;
pub mod scene;
pub mod camera;
mod color_picker;
mod action;

use crate::coord::Coord;
use crate::pixel::Pixel;
use crate::scene::Scene;
use crate::camera::Camera;

fn displayCameraGrid(camera: &Camera) {
    for i in 0..camera.dim.x {
        for j in 0..camera.dim.y {
            match camera.get_pixel(Coord{x: i, y: j}) {
                Some((char, pixel)) => {
                    match pixel {
                        Pixel::B24{..} => panic!("Expecting 8-bit pixel"),
                        Pixel::B8(c) => {
                            print!("{}", c);
                        }
                    }
                },
                None => {
                    print!("0");
                }
            }
        }
        println!();
    }
}

fn main() {
    /*let p1 = Pixel::B24{r: 255, g: 0, b: 0};
    let p2 = Pixel::B8(32);
    println!("p1: {}, p2: {}", p1, p2);*/

    let scene = Scene::new(
        Coord { x: 3, y: 3 },
        vec![Pixel::B8(1), Pixel::B8(2), Pixel::B8(3), Pixel::B8(4), Pixel::B8(5), Pixel::B8(6), Pixel::B8(7), Pixel::B8(8), Pixel::B8(9)],
        Pixel::B8(0)
    );
    let mut camera = Camera::new(
        Coord{ x: 8, y: 16 },
        Coord{ x: 0, y: 0 },
        1,
        Coord{ x: 1, y: 2 },
        0
    );
    
    for z in 0..3 {
        camera.render(&scene);
        println!("focus is now at ({}, {})", camera.focus.x, camera.focus.y);
        displayCameraGrid(&camera);
        camera.set_focus(&scene, Coord{ x: (camera.focus.x + 1), y: (camera.focus.y + 1) }).unwrap();
        println!("\n");
    }
}
