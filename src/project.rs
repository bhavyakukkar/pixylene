use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::elements::layer::{ Camera, Layer };
use crate::elements::Palette;
use crate::action::Action;

struct DrawOnce {
    layer_index: usize,
    focus: Coord,
    new_pixel: Option<Pixel>,
}
impl Action for DrawOnce {
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String> {
        let old_pixel = project.layers[self.layer_index].scene.get_pixel(
            self.focus
        )?;
        project.layers[self.layer_index].scene.set_pixel(
            self.focus,
            self.new_pixel
        )?;
        self.new_pixel = old_pixel;
        Ok(())
    }
    fn end_action(&self) -> bool { true }
}

pub enum Change {
    Cascade(Rc<RefCell<dyn Action>>),
    Halt(Rc<RefCell<dyn Action>>),
}

pub struct Project {
    pub layers: Vec<Layer>,
    pub selected_layer: usize,
    pub camera: Camera,
    pub palette: Palette,
    pub change_stack: Vec<Change>,
}

impl Project {
    fn new(layers: Vec<Layer>) -> Result<Project, String> {
        todo!();
    }
    pub fn get_num_layers(&self) -> usize {
        self.layers.len()
    }
    pub fn draw_pixel(
        &mut self,
        layer_index: Option<usize>,
        focus: Coord,
        color: Option<Pixel>,
        blend_mode: BlendMode
    ) -> Result<(), String> {
        let layer_num = if let Some(layer) = layer_index {
            layer
        } else {
            self.selected_layer
        };
        if layer_num < 0 || layer_num >= self.layers.len() {
            return Err(format!(
                "cannot draw to invalid layer {} of project containing {} layers",
                layer_num,
                self.layers.len()
            ));
        }
        let mut draw_once = DrawOnce {
            layer_index: layer_num,
            focus: focus,
            new_pixel: Some(blend_mode.merge_down(
                Pixel::get_certain(color),
                Pixel::get_certain(self.layers[layer_num].scene.get_pixel(focus)?)
            ))
        };
        let old_focus = self.camera.focus;
        self.camera.set_focus(&self.layers[layer_num].scene, focus)?;
        draw_once.perform_action(self);
        self.camera.set_focus(&self.layers[layer_num].scene, old_focus)?;
        self.change_stack.push(Change::Cascade(
            Rc::new(RefCell::new(draw_once))
        ));
        Ok(())
    }
    /*
    pub let read_pixel = |focus: Coord| -> Result<Option<Pixel>, String> {
        return self.layers[self.selected_layer].scene.get_pixel(self.camera.focus)?;
    }
    */
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
