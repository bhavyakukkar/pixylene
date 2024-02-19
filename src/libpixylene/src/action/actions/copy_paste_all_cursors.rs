use crate::{
    types::{ Coord, Pixel, BlendMode },
    project::{ Project },
    action::{ Action, ActionError, Change, actions::draw_at_one_cursor::DrawAtOneCursor },
};

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{ min, max };


pub struct CopyPasteAllCursors {
    pub selected_pixels: Vec<Option<Pixel>>,
}
impl Action for CopyPasteAllCursors {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        use ActionError::OnlyNCursorsSupported;

        match self.selected_pixels.len() {
            0 => {
                self.selected_pixels = Vec::new();
                //first iteration: copying
                for i in 0..project.cursors.len() {
                    self.selected_pixels.push(
                        project
                            .layers[project.cursors[i].layer]
                            .scene.get_pixel(project.cursors[i].coord).unwrap(),
                    );
                }
                Ok(Vec::new())
            },
            _ => {
                //second iteration: pasting
                if self.selected_pixels.len() != project.cursors.len() {
                    return Err(OnlyNCursorsSupported(
                            format!(
                                "as many cursors as there were when this action was invoked first, \
                                i.e., {}",
                                self.selected_pixels.len(),
                            ),
                            project.cursors.len(),
                    ));
                }
                let mut changes: Vec<Change> = vec![Change::Start];
                for i in 0..self.selected_pixels.len() {
                    match self.selected_pixels[i] {
                        Some(color) => {
                            if let Ok(draw_at_one_cursor) = (DrawAtOneCursor {
                                cursor: project.cursors[i].clone(),
                                color: Some(color),
                                blend_mode: BlendMode::Overwrite,
                            }).perform_action(project) {
                                for change in draw_at_one_cursor {
                                    changes.push(change.as_untracked()?);
                                }
                            }
                        },
                        //if a copied cursor had no color, do not try pasting it back
                        None => (),
                    }
                }
                changes.push(Change::End);
                self.selected_pixels = Vec::new();
                Ok(changes)
            }
        }
    }
}
