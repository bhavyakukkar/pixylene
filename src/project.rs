use std::collections::HashMap;

use crate::elements::layer::{ Camera, Layer };
use crate::elements::Stroke;

pub enum StrokeVisibility {
    NotSelected,
    Selected,
    Ongoing{ step: u8, clicks: u8 }
}

pub struct Project {
    pub layers: Vec<Layer>,
    pub camera: Camera,
    pub selected_layer: u8,
    //palette: Palette,
    pub strokes: HashMap<String, (Box<dyn Stroke>, StrokeVisibility)>,
    //action_stack: Vec<Action>,
}

impl Project {
    fn new(layers: Vec<Layer>) -> Result<Project, String> {
        todo!();
    }
    /*
    fn move_camera(&mut self, to: Coord) -> Box<dyn FnMut(&mut Project)> {
        let from: Coord = self.camera.focus;
        self.camera.focus = to;
        Box::new(|project| {
            self.camera.focus = from;
        })
    }
    */
    /*
    fn save_project(&self);
    fn open_project();
    fn export_png();
    fn import_png();
    fn save_color_palette();
    fn load_color_palette();
    */
}
