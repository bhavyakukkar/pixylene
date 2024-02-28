use crate::{
    types::{ Coord, Pixel, BlendMode, Cursor },
    project::{ Project },
    action::{ Action, ActionError, Change, actions::draw_at_one_cursor::DrawAtOneCursor },
};

use std::rc::Rc;
use std::cell::RefCell;

/* 
 * Draw At All Cursors
 * An action that extends DrawAtOneCursor by performing it at all cursors present in the project
 *
 */ 

pub struct DrawAtAllCursors {
    pub color: Option<Pixel>,
    pub blend_mode: BlendMode,
}
impl Action for DrawAtAllCursors {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let mut changes: Vec<Change> = vec![Change::Start];
        for index in 0..project.cursors.len() {
            if let Ok(draw_at_one_cursor) = (DrawAtOneCursor {
                cursor: project.cursors[index].clone(),
                color: self.color,
                blend_mode: self.blend_mode.clone(),
            }).perform_action(project) {
                for change in draw_at_one_cursor {
                    changes.push(change.as_untracked()?);
                }
            }
        }
        changes.push(Change::End);
        Ok(changes)
    }
}
