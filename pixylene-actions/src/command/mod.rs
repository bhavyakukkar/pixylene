mod action;
pub use self::action::{ Action };

mod change;
pub use self::change::{ Change, UntrackError, ChangeError };

mod action_manager;
pub use action_manager::{ ActionNameOrChangeIndex, ActionManager, ActionManagerError };


pub type ActionResult = Result<Vec<Change>, crate::ActionError>;

pub fn include(
    mut action: Box<dyn Action>,
    project: &mut libpixylene::project::Project,
    console: &dyn crate::Console,
    changes: &mut Vec<Change>
) -> Result<(), crate::ActionError> {

    for change in (*action).perform(project, console)? {
        changes.push(change.as_untracked()?);
    }
    Ok(())
}

pub enum AbsOrRel<A, B> {
    Abs(A),
    Rel(B),
}
pub use AbsOrRel::*;
