use super::ActionResult;
use crate::Console;

use libpixylene::project::Project;
use std::fmt;

pub trait Action {
    fn perform(&mut self, project: &mut Project, console: &dyn Console) -> ActionResult;
    fn has_ended(&self) -> bool {
        true
    }
    fn locks_canvas(&self) -> bool {
        false
    }
}

pub enum ActionError {}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "action-error")
    }
}
