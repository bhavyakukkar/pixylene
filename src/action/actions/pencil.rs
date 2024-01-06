use std::rc::Rc;
use std::cell::RefCell;

use crate::elements::common::{ Pixel, BlendMode };
use crate::project::Project;
use crate::action::{ Action, ActionError, Change, actions::draw_once::DrawOnce };

pub struct Pencil {
    pub palette_index: usize,
    pub new_pixel: Option<Option<Pixel>>,
}
impl Action for Pencil {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let mut changes: Vec<Change> = vec![Change::Start];
        let old_pixel = project.layers[project.selected_layer].scene.get_pixel(
            project.camera.focus
        )?;
        let mut draw_once = DrawOnce {
            layer: project.selected_layer,
            focus: project.camera.focus,
            color: Some(BlendMode::Normal.merge_down(
               Pixel::get_certain(project.palette.get_color((&self).palette_index)?),
               Pixel::get_certain(
                   project
                       .layers[project.selected_layer]
                       .scene.get_pixel(project.camera.focus)?
               )
           ))
        }
            .perform_action(project)?;
        for change in draw_once {
            changes.push(change.as_untracked()?);
        }
        changes.push(Change::End);
        Ok(changes)
    }
    fn locks_scene(&self) -> bool { true }
}
