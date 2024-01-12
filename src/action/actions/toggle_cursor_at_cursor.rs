use std::rc::Rc;
use std::cell::RefCell;

use crate::elements::common::{ Coord, Pixel };
use crate::project::{ Project, Cursor };
use crate::action::{ Action, ActionError, Change };

pub struct ToggleCursorAtCursor {
    pub cursor: Cursor,
}
impl Action for ToggleCursorAtCursor {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        if let Some(matched_cursor) = project.cursors.iter().position(|&c| c == self.cursor) {
            project.cursors.remove(matched_cursor);
        } else {
            //temporary code that misbehaves when trying outside scene, fix after implemented
            //Project::add_cursor()
            project.cursors.push(Cursor{ layer: 0, coord: Coord::zero() });
            project.set_cursor(project.cursors.len() - 1, self.cursor)?;
        }
        let toggle_cursor_at_cursor = ToggleCursorAtCursor { cursor: self.cursor };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(toggle_cursor_at_cursor)))])
    }
}
