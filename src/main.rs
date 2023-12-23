#![allow(warnings)]
extern crate savefile;
#[macro_use]
extern crate savefile_derive;

mod grammar;
mod elements;
mod project;
mod action;
mod file;
mod pixylene;

use crate::elements::common::Coord;
use crate::elements::layer::CameraPixel;
use crate::action::{ Change, actions };
use crate::pixylene::{ Pixylene, PixyleneDisplay };

use colored::*;
use std::collections::HashMap;

impl PixyleneDisplay for Pixylene {
    fn display(&mut self) {
        let mut grid: Vec<crate::elements::layer::CameraPixel> = self.project.camera.render(
            &self.project.layers[0].scene
        );
        for i in 0..self.project.camera.dim.x {
            for j in 0..self.project.camera.dim.y {
                match grid[i as usize*self.project.camera.dim.y as usize + j as usize] {
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
}

impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match &self {
            action::Change::Start => write!(f, "{}", "S(__"),
            action::Change::End => write!(f, "{}", "__)E"),
            action::Change::StartEnd(_) => write!(f, "{}", "S()E"),
            action::Change::Untracked(_) => write!(f, "{}", "_--_"),
        }
    }
}

fn display_change_stack(change_stack: &Vec<Change>) {
    if change_stack.len() == 0 {
        println!("<empty>");
        return;
    }
    for change in change_stack {
        print!("{} ", change);
    }
    println!();
}

fn main() {
    let mut app = Pixylene::import("/home/bhavya/pictures/trash/snowbrick_rgba.png").unwrap();
    //let mut app = Pixylene::open("/home/bhavya/pictures/trash/snowbrick.bin").unwrap();

    app.add_action("rectangular_fill", Box::new(actions::rectangular_fill::RectangularFill{
        palette_index: 7,
        start_corner: None
    }));
    app.add_action("pencil1", Box::new(actions::pencil::Pencil{
        palette_index: 1,
        new_pixel: None,
    }));
    app.add_action("pencil2", Box::new(actions::pencil::Pencil{
        palette_index: 2,
        new_pixel: None,
    }));
    app.add_action("pencil3", Box::new(actions::pencil::Pencil{
        palette_index: 3,
        new_pixel: None,
    }));
    app.add_action("pencil4", Box::new(actions::pencil::Pencil{
        palette_index: 4,
        new_pixel: None,
    }));
    app.add_action("pencil5", Box::new(actions::pencil::Pencil{
        palette_index: 5,
        new_pixel: None,
    }));
    app.add_action("pencil6", Box::new(actions::pencil::Pencil{
        palette_index: 6,
        new_pixel: None,
    }));
    app.add_action("pencil7", Box::new(actions::pencil::Pencil{
        palette_index: 7,
        new_pixel: None,
    }));
    app.add_action("pencil8", Box::new(actions::pencil::Pencil{
        palette_index: 8,
        new_pixel: None,
    }));
    app.add_action("move_camera_up", Box::new(actions::move_camera::MoveCamera{
        focus_move: Coord{ x: -1, y: 0 },
    }));
    app.add_action("move_camera_down", Box::new(actions::move_camera::MoveCamera{
        focus_move: Coord{ x: 1, y: 0 },
    }));
    app.add_action("move_camera_left", Box::new(actions::move_camera::MoveCamera{
        focus_move: Coord{ x: 0, y: -1 },
    }));
    app.add_action("move_camera_right", Box::new(actions::move_camera::MoveCamera{
        focus_move: Coord{ x: 0, y: 1 },
    }));
    //app.perform("move_camera_left").unwrap();
    //app.perform("move_camera_up").unwrap();
    app.display();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    /*
    app.perform("pencil1").unwrap();
    app.display();
    display_change_stack(&app.action_manager.change_stack);
    app.perform("move_camera_right").unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    app.perform("pencil2").unwrap();
    app.display();
    display_change_stack(&app.action_manager.change_stack);
    app.perform("move_camera_right").unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    app.perform("pencil3").unwrap();
    app.display();
    display_change_stack(&app.action_manager.change_stack);
    app.perform("move_camera_right").unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    app.perform("pencil4").unwrap();
    app.display();
    display_change_stack(&app.action_manager.change_stack);
    app.perform("move_camera_right").unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    app.perform("pencil5").unwrap();
    app.display();
    display_change_stack(&app.action_manager.change_stack);
    app.perform("move_camera_right").unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    app.perform("pencil6").unwrap();
    app.display();
    display_change_stack(&app.action_manager.change_stack);
    app.perform("move_camera_right").unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    app.perform("pencil7").unwrap();
    app.display();
    display_change_stack(&app.action_manager.change_stack);
    app.perform("move_camera_right").unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    app.perform("pencil8").unwrap();
    app.display();
    display_change_stack(&app.action_manager.change_stack);
    app.perform("move_camera_right").unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    */

    //app.perform("move_camera_down").unwrap();
    //app.perform("move_camera_left").unwrap();
    app.perform("rectangular_fill").unwrap();
    for _ in 0..2 {
        app.perform("move_camera_down").unwrap();
        app.perform("move_camera_left").unwrap();
    }
    app.perform("rectangular_fill").unwrap();

    app.display();
    display_change_stack(&app.action_manager.change_stack);
    std::io::stdin().read_line(&mut line).unwrap();

    app.undo();
    /*
    app.perform("move_camera_up").unwrap(); app.perform("move_camera_up").unwrap(); app.perform("move_camera_up").unwrap();
    app.perform("move_camera_right").unwrap(); app.perform("move_camera_right").unwrap(); app.perform("move_camera_right").unwrap();
    app.perform("rectangular_fill").unwrap();

    app.perform("move_camera_down").unwrap(); app.perform("move_camera_down").unwrap(); app.perform("move_camera_down").unwrap();
    app.perform("move_camera_left").unwrap(); app.perform("move_camera_left").unwrap(); app.perform("move_camera_left").unwrap();
    app.perform("rectangular_fill").unwrap();
    */
    app.display();
    display_change_stack(&app.action_manager.change_stack);

    //app.save("/home/bhavya/pictures/trash/snowbrick.bin").unwrap();
    //app.export("/home/bhavya/pictures/trash/snowbrick_export.png").unwrap();
}
