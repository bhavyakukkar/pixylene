use std::rc::Rc;
use std::cell::RefCell;

use crate::{
    project::{ Project, ProjectError, Cursor },
    elements::{ palette::PaletteError, layer::{ SceneError, CameraError } }
};

#[derive(Debug)]
pub enum UntrackError {
    NotDefined(Change),
}
impl std::fmt::Display for UntrackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use UntrackError::*;
        match self {
            NotDefined(change) => write!(
                f,
                "cannot set given change {} as untracked, only completed changes that contain an \
                action, like Change::StartEnd or another Change::Untracked, can be changed to \
                Change::Untracked",
                change,
            ),
        }
    }
}

#[derive(Debug)]
pub enum ChangeError {
    UntrackError(UntrackError),
}
impl std::fmt::Display for ChangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ChangeError::*;
        match self {
            UntrackError(untrack_error) => write!(f, "{}", untrack_error),
        }
    }
}
//#[derive(Debug)]
pub enum Change {
    Start,
    End,
    StartEnd(Rc<RefCell<dyn Action>>),
    Untracked(Rc<RefCell<dyn Action>>),
}
impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Change::*;
        match self {
            Start => write!(f, "Change::Start"),
            End => write!(f, "Change::End"),
            StartEnd(_) => write!(f, "Change::StartEnd"),
            Untracked(_) => write!(f, "Change::Untracked"),
        }
    }
}
impl std::fmt::Debug for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Change::*;
        match self {
            Start => write!(f, "S"),
            End => write!(f, "E"),
            StartEnd(_) => write!(f, "SE"),
            Untracked(_) => write!(f, "U"),
        }
    }
}
impl Change {
    pub fn as_untracked(self) -> Result<Self, ChangeError> {
        use UntrackError::*;
        match self {
            Change::Start |
            Change::End => Err(ChangeError::UntrackError(NotDefined(self))),
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
 * p.s. make sure in perform_action that any results are processed before changing the action's
 * state, such that in case action fails, it is still in the same state as it was before failing
*/

#[derive(Debug)]
pub enum ActionError {
    SceneError(SceneError),
    CameraError(CameraError),
    PaletteError(PaletteError),
    ChangeError(ChangeError),
    ProjectError(ProjectError),
    OnlyNCursorsSupported(String, usize),
    InputsError(String),
}
impl From<SceneError> for ActionError {
    fn from(item: SceneError) -> ActionError { ActionError::SceneError(item) }
}
impl From<CameraError> for ActionError {
    fn from(item: CameraError) -> ActionError { ActionError::CameraError(item) }
}
impl From<PaletteError> for ActionError {
    fn from(item: PaletteError) -> ActionError { ActionError::PaletteError(item) }
}
impl From<ChangeError> for ActionError {
    fn from(item: ChangeError) -> ActionError { ActionError::ChangeError(item) }
}
impl From<ProjectError> for ActionError {
    fn from(item: ProjectError) -> ActionError { ActionError::ProjectError(item) }
}
impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ActionError::*;
        match self {
            SceneError(scene_error) => write!(f, "{}", scene_error),
            CameraError(camera_error) => write!(f, "{}", camera_error),
            PaletteError(palette_error) => write!(f, "{}", palette_error),
            ChangeError(change_error) => write!(f, "{}", change_error),
            ProjectError(project_error) => write!(f, "{}", project_error),
            OnlyNCursorsSupported(supported, supplied) => write!(
                f,
                "this action only supports {} cursor/s, found {}",
                supported,
                supplied,
            ),
            InputsError(desc) => write!(f, "{}", desc),
        }
    }
}

pub trait Action {
    //perform action, transform to reverted (for undo) action, and return as a Change
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError>;
    fn end_action(&self) -> bool { true }

    // these methods must be overridden only for a complex Action, i.e.,
    // one that takes 2 or more calls to perform_action to complete
    fn locks_scene(&self) -> bool { false }
    fn locks_camera(&self) -> bool { false }
}
impl std::fmt::Debug for dyn Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[An Action that {} the scene, {} the camera & has {} performing]",
            if self.locks_scene() { "locks" } else { "doesn't lock" },
            if self.locks_camera() { "locks" } else { "doesn't lock" },
            if self.end_action() { "finished" } else { "not finished" },
        )
    }
}

pub fn include(mut action: Box<dyn Action>, project: &mut Project, changes: &mut Vec<Change>)
    -> Result<(), ActionError> {
    for change in (*action).perform_action(project)? {
        changes.push(change.as_untracked()?);
    }
    Ok(())
}
