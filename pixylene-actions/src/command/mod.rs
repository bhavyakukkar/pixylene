mod action;
pub use self::action::Action;

mod change;
pub use self::change::{Change, ChangeError, UntrackError};

mod action_manager;
pub use action_manager::{ActionManager, ActionManagerError, ActionNameOrChangeIndex};

pub type ActionResult = Result<Vec<Change>, crate::ActionError>;

pub fn include(
    mut action: Box<dyn Action>,
    project: &mut libpixylene::project::Project,
    console: &dyn crate::Console,
    changes: &mut Vec<Change>,
) -> Result<(), crate::ActionError> {
    for change in (*action).perform(project, console)? {
        changes.push(change.as_untracked()?);
    }
    Ok(())
}
