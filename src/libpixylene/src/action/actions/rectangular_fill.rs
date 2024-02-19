use crate::{
    types::{ Coord, Pixel, BlendMode, Cursor },
    project::{ Project },
    action::{ Action, ActionError, Change, actions::draw_at_one_cursor::DrawAtOneCursor },
};

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{ min, max };


pub struct RectangularFill {
    pub palette_index: usize,
    pub start_corner: Option<Coord>,
}
impl Action for RectangularFill {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        use ActionError::{ OnlyNCursorsSupported };
        if project.cursors.len() != 1 {
            return Err(OnlyNCursorsSupported(String::from("1"), project.cursors.len()));
        }
        if let Some(start_corner) = self.start_corner {
            let mut changes: Vec<Change> = Vec::new();
            changes.push(Change::Start);
            for i in min(start_corner.x, project.cursors[0].coord.x)..(max(
                start_corner.x,
                project.cursors[0].coord.x
            ) + 1) {
                for j in min(start_corner.y, project.cursors[0].coord.y)..(max(
                    start_corner.y,
                    project.cursors[0].coord.y
                ) + 1) {
                    let mut draw_at_one_cursor = DrawAtOneCursor {
                        cursor: Cursor {
                            layer: project.cursors[0].layer,
                            coord: Coord{ x: i, y: j }, 
                        },
                        color: project.palette.get_color((&self).palette_index)?,
                        blend_mode: BlendMode::Normal,
                    }
                        .perform_action(project)?;
                    for change in draw_at_one_cursor {
                        changes.push(change.as_untracked()?);
                    }
                }
            }
            changes.push(Change::End);
            self.start_corner = None;
            Ok(changes)
        } else {
            self.start_corner = Some(project.cursors[0].coord);
            Ok(Vec::new())
        }
    }
    fn end_action(&self) -> bool {
        match self.start_corner {
            Some(_) => false,
            None => true,
        }
    }
    fn locks_scene(&self) -> bool { true }
}
