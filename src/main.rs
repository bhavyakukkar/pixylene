#![allow(warnings)]
mod elements;
mod project;
mod action;
mod file;

use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::elements::layer::{ Scene, Camera, CameraPixel, Layer };
use crate::elements::Palette;
use crate::project::Project;
use crate::action::{ Change, Action, ActionManager, DrawOnce, Pencil, RectangularFill };
use crate::file::Png;

use colored::*;
use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

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

fn display_change_stack(app: &AppState) {
    for change in &app.action_manager.change_stack {
        print!("{}->", match change {
            Change::Start => "s",
            Change::End => "e",
            Change::StartEnd(_) => "se",
            Change::Untracked(_) => "u",
        });
    }
    println!();
}

struct AppState {
    project: Project,
    action_manager: ActionManager,
}

fn main() {
    let mut png = Png::open(String::from("/home/bhavya/pictures/trash/snowbrick_rgba.png")).unwrap();
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
        palette: Palette { colors: vec![
            Some(Pixel{r: 0, g: 0, b: 0, a: 255}),
            Some(Pixel{r: 127, g: 0, b: 255, a: 255 }),
        ] },
    };

    let mut actions: HashMap<String, Box<dyn Action>> = HashMap::new();
    actions.insert(String::from("rectangular_fill"), Box::new(RectangularFill{
        palette_index: 1,
        start_corner: None
    }));
    actions.insert(String::from("draw_once"), Box::new(Pencil{
        palette_index: 2,
        new_pixel: None,
    }));
    let action_manager = ActionManager::new(actions);
    let mut app = AppState {
        project: project,
        action_manager: action_manager,
    };

    app.action_manager.perform(&mut app.project, String::from("rectangular_fill")).unwrap();
    display_change_stack(&app);
    
    {
        app.project.camera.set_focus(
            &app.project.layers[app.project.selected_layer].scene,
            Coord{ x: 8, y: 8 }
        )
            .unwrap();
        let grid: Vec<CameraPixel> = app.project.camera.render(&app.project.layers[0].scene);
        display_camera_grid(grid, &app.project.camera);
    }

    app.project.camera.set_focus(
        &app.project.layers[app.project.selected_layer].scene,
        Coord{ x: 10, y: 10 }
    )
        .unwrap();

    app.action_manager.perform(&mut app.project, String::from("rectangular_fill")).unwrap();
    display_change_stack(&app);

    {
        app.project.camera.set_focus(
            &app.project.layers[app.project.selected_layer].scene,
            Coord{ x: 8, y: 8 }
        )
            .unwrap();
        let grid: Vec<CameraPixel> = app.project.camera.render(&app.project.layers[0].scene);
        display_camera_grid(grid, &app.project.camera);
    }


    app.action_manager.perform(&mut app.project, String::from("draw_once")).unwrap();
    display_change_stack(&app);

    {
        app.project.camera.set_focus(
            &app.project.layers[app.project.selected_layer].scene,
            Coord{ x: 8, y: 8 }
        )
            .unwrap();
        let grid: Vec<CameraPixel> = app.project.camera.render(&app.project.layers[0].scene);
        display_camera_grid(grid, &app.project.camera);
    }

    png = Png::from_scene(
        &app.project.layers[app.project.selected_layer].scene,
        png::ColorType::Rgba,
        png::BitDepth::Eight,
    ).unwrap();
    png.save(String::from("/home/bhavya/pictures/trash/snowbrick_rgba_mod.png")).unwrap();
    /*
    app.action_manager.undo(&mut app.project).unwrap();
    display_change_stack(&app);

    {
        app.project.camera.set_focus(
            &app.project.layers[app.project.selected_layer].scene,
            Coord{ x: 8, y: 8 }
        )
            .unwrap();
        let grid: Vec<CameraPixel> = app.project.camera.render(&app.project.layers[0].scene);
        display_camera_grid(grid, &app.project.camera);
    }

    app.action_manager.undo(&mut app.project).unwrap();
    display_change_stack(&app);

    {
        app.project.camera.set_focus(
            &app.project.layers[app.project.selected_layer].scene,
            Coord{ x: 8, y: 8 }
        )
            .unwrap();
        let grid: Vec<CameraPixel> = app.project.camera.render(&app.project.layers[0].scene);
        display_camera_grid(grid, &app.project.camera);
    }

    app.action_manager.undo(&mut app.project).unwrap();
    display_change_stack(&app);
    */
}
