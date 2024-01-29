use std::rc::Rc;
use std::cell::RefCell;

use crate::common::{ Coord, Pixel };
use crate::project::{ Project, Cursor };
use crate::action::{ Action, ActionError, Change, actions::draw_at_one_cursor::DrawAtOneCursor };

/* 
 * Draw At All Cursors
 * An action that extends DrawAtOneCursor by performing it at all cursors present in the project
 *
 */ 

pub struct DrawAtAllCursors {
    pub color: Option<Pixel>,
}
impl Action for DrawAtAllCursors {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let mut changes: Vec<Change> = vec![Change::Start];
        for index in 0..project.cursors.len() {
            if let Ok(draw_at_one_cursor) = (DrawAtOneCursor {
                cursor: project.cursors[index].clone(),
                color: self.color,
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
