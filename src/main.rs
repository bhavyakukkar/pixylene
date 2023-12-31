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

    for i in 0..8 {
        app.add_action(&format!("{}", i+1), Box::new(actions::pencil::Pencil{
            palette_index: i+1,
            new_pixel: None,
        }));
    }
    app.add_action("f", Box::new(actions::rectangular_fill::RectangularFill{
        palette_index: 1,
        start_corner: None
    }));

    app.add_action("k", Box::new(actions::move_camera::MoveCamera{
        focus_move: Coord{ x: -1, y: 0 },
    }));
    app.add_action("j", Box::new(actions::move_camera::MoveCamera{
        focus_move: Coord{ x: 1, y: 0 },
    }));
    app.add_action("h", Box::new(actions::move_camera::MoveCamera{
        focus_move: Coord{ x: 0, y: -1 },
    }));
    app.add_action("l", Box::new(actions::move_camera::MoveCamera{
        focus_move: Coord{ x: 0, y: 1 },
    }));

    //app.project.palette.change_color_to(1, "00000000ff".to_string()).unwrap();

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = (&line[..(line.len() - 1)]).to_string();
        if line.eq(&String::from("u")) {
            match app.undo() {
                Ok(()) => (),
                Err(error) => println!("!!! {} !!!", error),
            }
        }
        else if line.eq(&String::from("r")) {
            match app.redo() {
                Ok(()) => (),
                Err(error) => println!("!!! {} !!!", error),
            }
        }
        else {
            match app.perform(&line) {
                Ok(()) => (),
                Err(error) => println!("!!! {} !!!", error),
            }
        }
        app.display();
    }

    //app.save("/home/bhavya/pictures/trash/snowbrick.bin").unwrap();
    //app.export("/home/bhavya/pictures/trash/snowbrick_export.png").unwrap();
}
