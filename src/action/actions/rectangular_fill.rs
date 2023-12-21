use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{ min, max };

use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::project::Project;
use crate::action::{ Action, Change };
use crate::action::actions::draw_once::DrawOnce;

pub struct RectangularFill {
    pub palette_index: usize,
    pub start_corner: Option<Coord>,
}
impl Action for RectangularFill {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String> {
        if let Some(start_corner) = self.start_corner {
            let mut changes: Vec<Change> = Vec::new();
            for i in min(start_corner.x, project.camera.focus.x)..(max(
                start_corner.x,
                project.camera.focus.x
            ) + 1) {
                for j in min(start_corner.y, project.camera.focus.y)..(max(
                    start_corner.y,
                    project.camera.focus.y
                ) + 1) {
                    let mut draw_once = DrawOnce {
                        layer: project.selected_layer,
                        focus: Coord{ x: i, y: j },
                        color: Some(BlendMode::Normal.merge_down(
                            Pixel::get_certain(project.palette.get_color((&self).palette_index)?),
                            Pixel::get_certain(
                                project
                                    .layers[project.selected_layer]
                                    .scene.get_pixel(Coord{ x: i, y: j })?
                            )
                        ))
                    }
                        .perform_action(project)?;
                    for change in draw_once {
                        changes.push(change.as_untracked()?);
                    }
                }
            }
            changes.push(Change::End);
            Ok(changes)
        } else {
            self.start_corner = Some(project.camera.focus);
            Ok(vec![Change::Start])
        }
    }
    fn end_action(&self) -> bool {
        match self.start_corner {
            Some(_) => false,
            None => true,
        }
    }
}
