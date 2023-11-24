mod utils;
mod layer;
mod session;
mod state;
mod stroke;

use crate::utils::{ Coord, Pixel, BlendMode };
use crate::session::{ SessionScene, SessionCamera, SessionLayers };
use crate::layer::{ Scene, Camera, CameraPixel, Layer };
use crate::stroke::Stroke;
use colored::*;

fn display_camera_grid(camera: &Camera) {
    for i in 0..camera.dim.x {
        for j in 0..camera.dim.y {
            match camera.get_camera_pixel(Coord{x: i, y: j}).unwrap() {
                CameraPixel::Filled{ brush, color } => {
                    if color.a != 255 {
                        panic!("alpha not 255");
                    }
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
    /*
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
    */

    let layer0 = Layer { scene: Scene::new(Coord{ x: 3, y: 3 }, vec![
                                           Some(Pixel{r:255,g:255,b:255,a:255}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
    ]).unwrap(), opacity: 255 };

    let layer1 = Layer { scene: Scene::new(Coord{ x: 3, y: 3 }, vec![
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:255,g:255,b:255,a:255}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
    ]).unwrap(), opacity: 255 };

    let layer2 = Layer { scene: Scene::new(Coord{ x: 3, y: 3 }, vec![
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:255,g:255,b:255,a:255}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
    ]).unwrap(), opacity: 255 };

    let layer3 = Layer { scene: Scene::new(Coord{ x: 3, y: 3 }, vec![
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:255,g:255,b:255,a:255}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
    ]).unwrap(), opacity: 255 };

    let layer4 = Layer { scene: Scene::new(Coord{ x: 3, y: 3 }, vec![
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:255,g:255,b:255,a:255}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
    ]).unwrap(), opacity: 255 };

    let layer5 = Layer { scene: Scene::new(Coord{ x: 3, y: 3 }, vec![
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:255,g:255,b:255,a:255}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
    ]).unwrap(), opacity: 255 };

    let layer6 = Layer { scene: Scene::new(Coord{ x: 3, y: 3 }, vec![
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:255,g:255,b:255,a:255}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
    ]).unwrap(), opacity: 255 };

    let layer7 = Layer { scene: Scene::new(Coord{ x: 3, y: 3 }, vec![
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:255,g:255,b:255,a:255}),Some(Pixel{r:0,g:0,b:0,a:0}),
    ]).unwrap(), opacity: 191 };

    let layer8 = Layer { scene: Scene::new(Coord{ x: 3, y: 3 }, vec![
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),
                                           Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:0,g:0,b:0,a:0}),Some(Pixel{r:255,g:255,b:255,a:255}),
    ]).unwrap(), opacity: 128 };


    let layer_vec = vec![&layer0, &layer1, &layer2, &layer3, &layer4, &layer5, &layer6, &layer7, &layer8];
    let layer_merged = Layer::merge(layer_vec, BlendMode::Normal).unwrap();

    let mut camera = Camera::new(
        &layer1.scene,
        Coord { x: 6, y: 12 },
        Coord { x: 1, y: 1 },
        2,
        Coord { x: 1, y: 2}
    ).unwrap();

    camera.render(&layer_merged.scene);
    display_camera_grid(&camera);


    /* STROKES EXAMPLE (WIP)
     *
    let mut scene = Scene::new(
        Coord { x: 16, y: 16 },
        vec![Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 23, b: 68, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 63, g: 81, b: 181, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 3, g: 169, b: 244, a: 255 }),  Some(Pixel{ r: 33, g: 150, b: 243, a: 255 }),  Some(Pixel{ r: 63, g: 81, b: 181, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 150, b: 136, a: 255 }),  Some(Pixel{ r: 0, g: 188, b: 212, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 139, g: 195, b: 72, a: 255 }),  Some(Pixel{ r: 76, g: 175, b: 80, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 240, g: 98, b: 146, a: 255 }),  Some(Pixel{ r: 255, g: 23, b: 68, a: 255 }),  Some(Pixel{ r: 255, g: 87, b: 34, a: 255 }),  Some(Pixel{ r: 62, g: 39, b: 35, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 235, b: 59, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 139, g: 195, b: 72, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 87, b: 34, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 235, b: 59, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 63, g: 81, b: 181, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 235, b: 59, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 3, g: 169, b: 244, a: 255 }),  Some(Pixel{ r: 33, g: 150, b: 243, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 193, b: 7, a: 255 }),  Some(Pixel{ r: 255, g: 235, b: 59, a: 255 }),  Some(Pixel{ r: 205, g: 220, b: 57, a: 255 }),  Some(Pixel{ r: 139, g: 195, b: 72, a: 255 }),  Some(Pixel{ r: 76, g: 175, b: 80, a: 255 }),  Some(Pixel{ r: 0, g: 150, b: 136, a: 255 }),  Some(Pixel{ r: 0, g: 188, b: 212, a: 255 }),  Some(Pixel{ r: 3, g: 169, b: 244, a: 255 }),  Some(Pixel{ r: 33, g: 150, b: 243, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 156, g: 39, b: 176, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 244, g: 67, b: 54, a: 255 }),  Some(Pixel{ r: 255, g: 152, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 33, g: 150, b: 243, a: 255 }),  Some(Pixel{ r: 63, g: 81, b: 181, a: 255 }),  Some(Pixel{ r: 103, g: 58, b: 183, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 0, g: 0, b: 0, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 }),  Some(Pixel{ r: 255, g: 255, b: 255, a: 255 })]
    ).unwrap();
    let mut camera = Camera::new(
        &scene,
        Coord { x: 32, y: 64 },
        Coord { x: 8, y: 8 },
        2,
        Coord { x: 1, y: 2}
    ).unwrap();
    let mut strokes = <dyn Stroke>::initialize_strokes();
    
    for (stroke_name, mut stroke) in strokes {
        println!("Performing {} at {}", stroke_name, layer.camera.focus);
        stroke.perform(0, &mut layer.scene, layer.camera.focus, Pixel(9, 255));
    }
    if let Some(rectangle_fill) = strokes.get_mut("rectangle_fill") {
        rectangle_fill.perform(0, &mut scene, Coord{ x: 5, y: 5 }, Pixel{r: 0, g: 0, b: 0, a: 255});
    }
    if let Some(pencil) = strokes.get_mut("pencil") {
        pencil.perform(0, &mut layer.scene, layer.camera.focus, Pixel{r: 0, g: 0, b: 0, a: 255});
    }
    if let Some(rectangle_fill) = strokes.get_mut("rectangle_fill") {
        rectangle_fill.perform(1, &mut scene, Coord{ x: 8, y: 11}, Pixel{r: 0, g: 0, b: 0, a: 255});
    }
    */
}
