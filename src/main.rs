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
use crate::action::{ Pencil, MoveCamera, RectangularFill };
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

fn main() {
    //let mut app = Pixylene::import("/home/bhavya/pictures/trash/snowbrick_rgba.png").unwrap();
    let mut app = Pixylene::open("/home/bhavya/pictures/trash/snowbrick.bin").unwrap();

    app.add_action("rectangular_fill", Box::new(RectangularFill{
        palette_index: 1,
        start_corner: None
    }));
    app.add_action("pencil", Box::new(Pencil{
        palette_index: 2,
        new_pixel: None,
    }));
    app.add_action("move_camera_up", Box::new(MoveCamera{
        focus_move: Coord{ x: -1, y: 0 },
    }));
    app.add_action("move_camera_down", Box::new(MoveCamera{
        focus_move: Coord{ x: 1, y: 0 },
    }));

    app.perform("move_camera_up").unwrap();
    app.perform("move_camera_up").unwrap();
    app.perform("pencil").unwrap();
    app.perform("move_camera_down").unwrap();
    app.perform("move_camera_down").unwrap();
    app.display();

    //app.save("/home/bhavya/pictures/trash/snowbrick.bin").unwrap();
}
