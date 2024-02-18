use std::rc::Rc;
use std::cell::RefCell;

use crate::types::{ Coord, Pixel };
use crate::project::{ Project, Cursor };
use crate::action::{ Action, ActionError, Change, actions::toggle_cursor_at_cursor::* };

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
