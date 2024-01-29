use std::rc::Rc;
use std::cell::RefCell;

use crate::common::Coord;
use crate::project::{ Project, Cursor };
use crate::action::{ Action, ActionError, Change, actions::move_one_cursor::MoveOneCursor };

pub struct MoveAllCursors {
    pub displacement: Coord,
}
impl Action for MoveAllCursors {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let mut changes: Vec<Change> = vec![Change::Start];
        for index in 0..project.cursors.len() {
            if let Ok(move_one_cursor) = (MoveOneCursor {
                displacement: self.displacement,
                index: index,
            }).perform_action(project) {
                for change in move_one_cursor {
                    changes.push(change.as_untracked()?);
                }
            }
        }
        changes.push(Change::End);
        Ok(changes)
    }
}
