#![allow(warnings)]
mod elements;
mod session;
mod project;
mod action;
mod file;

use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::elements::layer::{ Scene, Camera, CameraPixel, Layer };
use crate::elements::Palette;
use crate::session::{ SessionScene, SessionCamera, SessionLayers };
use crate::project::{ Change, Project };
use crate::action::{ Action, Pencil, RectangleFill, RectangleOutline };
use crate::file::Png;

use colored::*;
use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

fn execute_action_completely(project: &mut Project, action: Rc<RefCell<dyn Action>>) {
    let action_in_stack = Rc::clone(&action);
    project.change_stack.push(Change::Halt(action_in_stack));
    let mut action = action.borrow_mut();
    while {
        (*action).perform_action(project).unwrap();
        project.camera.set_focus(&project.layers[0].scene, Coord{ x: 11, y: 11 }).unwrap();
        !(*action).end_action()
    } {}
}

fn undo_all_actions(project: &mut Project) {
    while(true) {
        match project.change_stack.pop() {
            Some(change) => {
                match change {
                    Change::Cascade(action_ref) => {
                        let mut action = action_ref.borrow_mut();
                        (*action).perform_action(project).unwrap();
                    },
                    Change::Halt(action_ref) => (),
                }
            },
            None => break,
        }
    }
}

fn display_camera_grid(grid: Vec<CameraPixel>, camera: &Camera) {
    for i in 0..camera.dim.x {
        for j in 0..camera.dim.y {
            match grid[i as usize*camera.dim.y as usize + j as usize] {
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
    let png = Png::open(String::from("/home/bhavya/pictures/trash/snowbrick_rgba.png")).unwrap();
    let mut scene = png.to_scene().unwrap();
    let mut camera = Camera::new(
        &scene,
        Coord{ x: 18, y: 36 },
        Coord{ x: 8, y: 8 },
        1,
        Coord{ x: 1, y: 2 }
    ).unwrap();
    let mut project = Project {
        layers: vec![Layer {
            scene: scene,
            opacity: 255
        }],
        selected_layer: 0,
        camera: camera,
        palette: Palette { colors: vec![Some(Pixel{r: 0, g: 0, b: 0, a: 191})] },
        change_stack: vec![]
    };

    println!("Completely executing a Pencil...");
    let pencil = Rc::new(RefCell::new(Pencil{ palette_index: 1, new_pixel: None }));
    execute_action_completely(&mut project, pencil);
    project.camera.set_focus(&project.layers[0].scene, Coord{ x: 8, y: 8 }).unwrap();
    let grid: Vec<CameraPixel> = project.camera.render(&project.layers[0].scene);
    display_camera_grid(grid, &project.camera);

    println!("Undoing all changes...");
    undo_all_actions(&mut project);
    project.camera.set_focus(&project.layers[0].scene, Coord{ x: 8, y: 8 }).unwrap();
    let grid: Vec<CameraPixel> = project.camera.render(&project.layers[0].scene);
    display_camera_grid(grid, &project.camera);

    println!("Completely executing a RectangleFill...");
    let rectangle_fill = Rc::new(RefCell::new(RectangleFill{
        palette_index: 1,
        start_corner: None,
    }));
    execute_action_completely(&mut project, rectangle_fill);
    project.camera.set_focus(&project.layers[0].scene, Coord{ x: 8, y: 8 }).unwrap();
    let grid: Vec<CameraPixel> = project.camera.render(&project.layers[0].scene);
    display_camera_grid(grid, &project.camera);

    println!("Undoing all changes...");
    undo_all_actions(&mut project);
    project.camera.set_focus(&project.layers[0].scene, Coord{ x: 8, y: 8 }).unwrap();
    let grid: Vec<CameraPixel> = project.camera.render(&project.layers[0].scene);
    display_camera_grid(grid, &project.camera);

    println!("Completely executing a RectangleOutline...");
    let rectangle_outline = Rc::new(RefCell::new(RectangleOutline{
        palette_index: 1,
        start_corner: None,
    }));
    execute_action_completely(&mut project, rectangle_outline);
    project.camera.set_focus(&project.layers[0].scene, Coord{ x: 8, y: 8 }).unwrap();
    let grid: Vec<CameraPixel> = project.camera.render(&project.layers[0].scene);
    display_camera_grid(grid, &project.camera);

    println!("Undoing all changes...");
    undo_all_actions(&mut project);
    project.camera.set_focus(&project.layers[0].scene, Coord{ x: 8, y: 8 }).unwrap();
    let grid: Vec<CameraPixel> = project.camera.render(&project.layers[0].scene);
    display_camera_grid(grid, &project.camera);
}
