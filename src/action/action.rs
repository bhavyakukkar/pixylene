use std::rc::Rc;
use std::cell::RefCell;

use crate::project::Project;

pub enum Change {
    Start,
    End,
    StartEnd(Rc<RefCell<dyn Action>>),
    Untracked(Rc<RefCell<dyn Action>>),
}
impl Change {
    pub fn as_untracked(self) -> Result<Self, String> {
        match self {
            Change::Start |
            Change::End => Err(format!(
                "cannot set a Start change as untracked. only completed changes (containing an \
                action) can be changed as untracked"
            )),
            Change::StartEnd(action_rc) |
            Change::Untracked(action_rc) => Ok(Change::Untracked(action_rc)),
        }
    }
}

/* 
 * ACTION
 * An Action is a convenient way to change a Project
 *
 * An action may be of three types:
 * 1. "Primitive Action" that mutates the project directly (performs no other Action) in 1
 *    step and returns a singleton vector of Change::StartEnd.
 * 2. "Complex Action" that mutates the project indirectly (performs only Primitive Actions) in 1
 *    or more steps and returns a vector of Change::Untracked.
 * 3. "Primitive Untracked Action" that is a Primitive Action populating Complex Actions and isn't
 *    tracked when a Complex Action is being undone. It must perform in 1 step and return a
 *    singleton vector of Change::Untracked.
 *
 * In order to implement a multi-step Primitive Action you must implement a Complex Action as well
 * as a Primitive Action or a Primitive Untracked Action whereby the Complex Action performs the
 * latter/s
 *
 * Change.as_untracked may be used to convert a Start or End change of a Primitive Action into as
 * if that of a Primitive Untracked Action
 *
*/

pub trait Action {
    //perform action, transform to reverted (for undo) action, and return as a Change
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String>;
    //whether action has been completely executed
}
