use std::rc::Rc;
use std::cell::RefCell;

use crate::elements::common::Coord;
use crate::project::{ Project, ProjectError, Cursor };
use crate::action::{ Action, ActionError, Change };

pub struct MoveOneCursor {
    pub displacement: Coord,
    pub index: usize,
}
impl Action for MoveOneCursor {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        use ProjectError::{ CursorIndexOutOfBounds };
        let mut new_cursor = match project.cursors.get(self.index) {
            Some(cursor) => cursor.clone(),
            None => {
                return Err(ActionError::ProjectError(
                    CursorIndexOutOfBounds(self.index, project.cursors.len())
                ));
            },
        };
        new_cursor.coord = new_cursor.coord.add(self.displacement);
        project.set_cursor(self.index, new_cursor)?;

        let mut move_one_cursor_back = MoveOneCursor {
            displacement: self.displacement.multiply(Coord{ x: -1, y: -1 }),
            index: self.index,
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(move_one_cursor_back)))])
    }
}
