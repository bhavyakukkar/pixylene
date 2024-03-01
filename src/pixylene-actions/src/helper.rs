use crate::{ Action, ActionError, Change, Console };

use libpixylene::project::Project;


pub fn include(mut action: Box<dyn Action>, project: &mut Project, console: &Console, changes: &mut Vec<Change>)
    -> Result<(), ActionError> {
    for change in (*action).perform_action(project, console)? {
        changes.push(change.as_untracked()?);
    }
    Ok(())
}

pub enum AbsOrRel<A,B> {
    Abs(A),
    Rel(B),
}
pub use AbsOrRel::*;
