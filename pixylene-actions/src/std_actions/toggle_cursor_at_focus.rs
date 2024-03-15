use crate::{
    types::{ Coord, Pixel, Cursor },
    project::{ Project },
    action::{ Action, ActionError, Change, actions::toggle_cursor_at_cursor::* },
};

use std::rc::Rc;
use std::cell::RefCell;


pub struct ToggleCursorAtFocus;
impl Action for ToggleCursorAtFocus {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let mut changes: Vec<Change> = vec![Change::Start];
        let toggle_cursor_at_cursor = ToggleCursorAtCursor {
            cursor: project.focus
        }
            .perform_action(project)?;
        for change in toggle_cursor_at_cursor {
            changes.push(change.as_untracked()?);
        }
        Ok(changes)
    }
}
