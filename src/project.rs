use std::collections::HashMap;

use crate::elements::layer::{ Camera, Layer };
use crate::elements::Palette;
use crate::elements::stroke::{ StrokeState, Stroke };
use crate::action::Action;

pub struct Project {
    pub layers: Vec<Layer>,
    pub selected_layer: usize,
    pub camera: Camera,
    pub palette: Palette,
    pub strokes: HashMap<String, (Box<dyn Stroke>, StrokeState)>,
    pub selected_stroke: String,
    pub action_stack: Vec<Box<dyn Action>>,
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
