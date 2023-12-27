use std::collections::HashMap;

use crate::project::Project;
use crate::action::{ Action, Change };
use crate::grammar::Decorate;


#[derive(Debug)]
enum UndoError {
    LockedAction(String),
    InvalidChangeStack(String),
    NothingToUndo(String),
}
impl std::fmt::Display for UndoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use UndoError::*;
        let decorate = |name: &str, desc: &String| -> String {
            Decorate::output(name.to_string(), None, Some(desc.to_string()))
        };
        match self {
            LockedAction(desc) => write!(f, "{}", decorate("LockedAction", desc)),
            InvalidChangeStack(desc) => write!(f, "{}", decorate("InvalidChangeStack", desc)),
            NothingToUndo(desc) => write!(f, "{}", decorate("NothingToUndo", desc)),
        }
    }
}

#[derive(Debug)]
enum PerformError {
    LockedAction(String),
    ActionNotFound(String),
}
impl std::fmt::Display for PerformError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PerformError::*;
        let decorate = |name: &str, desc: &String| -> String {
            Decorate::output(name.to_string(), None, Some(desc.to_string()))
        };
        match self {
            LockedAction(desc) => write!(f, "{}", decorate("LockedAction", desc)),
            ActionNotFound(desc) => write!(f, "{}", decorate("ActionNotFound", desc)),
        }
    }
}

#[derive(Debug)]
pub enum ActionManagerError {
    UndoError(UndoError),
    PerformError(PerformError),
    ActionFailedToPerform(String),
    CannotUntrackAction(String),
}
impl std::fmt::Display for ActionManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ActionManagerError::*;
        let decorate = |name: &str, desc: &String| -> String {
            Decorate::output(name.to_string(), None, Some(desc.to_string()))
        };
        match self {
            ActionManagerError::UndoError(error) => write!(f, "{}", Decorate::output(
                "UndoError".to_string(),
                None,
                Some(error.to_string())
            )),
            ActionManagerError::PerformError(error) => write!(f, "{}", Decorate::output(
                "PerformError".to_string(),
                None,
                Some(error.to_string())
            )),
            ActionFailedToPerform(desc) => write!(f, "{}", decorate("ActionFailedToPerform", desc)),
            CannotUntrackAction(desc) => write!(f, "{}", decorate("CannotUntrackAction", desc)),
        }
    }
}

pub struct ActionManager {
    pub actions: HashMap<String, Box<dyn Action>>,
    scene_lock: Option<String>,
    camera_lock: Option<String>,
    pub change_stack: Vec<Change>,
}
impl ActionManager {
    pub fn new(actions: HashMap<String, Box<dyn Action>>) -> Self {
        ActionManager {
            actions: actions,
            scene_lock: None,
            camera_lock: None,
            change_stack: Vec::new(),
        }
    }
    //currently very useless
    fn record(&mut self, changes: Vec<Change>) {
        for change in changes {
            match change {
                Change::Start => {
                    self.change_stack.push(Change::Start);
                },
                Change::End => {
                    self.change_stack.push(Change::End);
                }
                Change::StartEnd(action_rc) => {
                    self.change_stack.push(Change::StartEnd(action_rc));
                },
                Change::Untracked(action_rc) => {
                    self.change_stack.push(Change::Untracked(action_rc));
                },
            }
        }
    }
    pub fn perform(
        &mut self,
        project: &mut Project,
        action_name: String
    )
    -> Result<(), ActionManagerError> {
        use PerformError::*;
        use ActionManagerError::ActionFailedToPerform;

        let action = match self.actions.get_mut(&action_name) {
            Some(action) => { action },
            None => {
                return Err(ActionManagerError::PerformError(ActionNotFound(format!(
                    "action '{}' was not found in inserted actions",
                    action_name,
                ))));
            }
        };

        if let Some(scene_locked_action_name) = &self.scene_lock {
            if action.locks_scene() && action_name.ne(scene_locked_action_name) {
                return Err(ActionManagerError::PerformError(LockedAction(format!(
                    "cannot perform scene-locking action '{}' while action '{}' has locked the \
                     scene",
                    action_name,
                    scene_locked_action_name,
                ))));
            }
        }

        if let Some(camera_locked_action_name) = &self.camera_lock {
            if action.locks_camera() && action_name.ne(camera_locked_action_name) {
                return Err(ActionManagerError::PerformError(LockedAction(format!(
                    "cannot perform camera-locking action '{}' while action '{}' has locked the \
                     camera",
                    action_name,
                    camera_locked_action_name,
                ))));
            }
        }

        if action.locks_scene() { self.scene_lock = Some(action_name.clone()); }
        if action.locks_camera() { self.camera_lock = Some(action_name.clone()); }

        match action.perform_action(project) {
            Ok(changes) => {
                let num_changes = changes.len();
                if num_changes > 0 {
                    //todo: fix bad vec implementation with perform returning a VecDeque of changes instead of a Vec of changes
                    let mut i = 0;
                    let mut last_change: Change = Change::Start;
                    for change in changes {
                        if i == num_changes - 1 {
                            last_change = change;
                            break;
                        }
                        print!(" ({}{})", match &self.scene_lock { Some(_) => ":", None => " " }, match &self.camera_lock { Some(_) => ":", None => " " });
                        match &change {
                            Change::Start => print!(" S_ "),
                            Change::End => print!(" _E"),
                            Change::StartEnd(_) => print!(" SE"),
                            Change::Untracked(_) => print!(" UU"),
                        }
                        self.change_stack.push(change);
                        i += 1;
                    }

                    print!(" ({}{})", match &self.scene_lock { Some(_) => ":", None => " " }, match &self.camera_lock { Some(_) => ":", None => " " });
                    match &last_change {
                        Change::Start => print!(" S_ "),
                        Change::End => print!(" _E"),
                        Change::StartEnd(_) => print!(" SE"),
                        Change::Untracked(_) => print!(" UU"),
                    }
                    match last_change {
                        Change::Start => (),
                        Change::End |
                        Change::StartEnd(_) |
                        Change::Untracked(_) => {
                            if action.locks_scene() { self.scene_lock = None; }
                            if action.locks_camera() { self.camera_lock = None; }
                        }
                    }
                    self.change_stack.push(last_change);
                }
            },
            Err(error) => {
                return Err(ActionFailedToPerform(String::from(error)));
            }
        }
        Ok(())
    }
    pub fn undo(&mut self, project: &mut Project) -> Result<(), ActionManagerError> {
        use UndoError::*;
        use ActionManagerError::{ ActionFailedToPerform, CannotUntrackAction };

        let mut index: usize = self.change_stack.len() - 1;
        if index == 0 {
            return Err(ActionManagerError::UndoError(NothingToUndo(
                String::from("nothing to undo")
            )));
        }
        /*
        if self.lock {
            return Err(ActionManagerError::UndoError(LockedAction(
                String::from("Cannot undo while action-manager is locked")
            )));
        }
        */
        let mut was_locked = false;
        let mut reverted_changes: Vec<Change> = Vec::new();
        loop {
            if let Some(change) = self.change_stack.pop() {
                match change {
                    Change::Start => {
                        if was_locked {
                            reverted_changes.push(Change::End);
                            self.record(reverted_changes);
                            was_locked = false;
                            return Ok(());
                        } else {
                            return Err(ActionManagerError::UndoError(InvalidChangeStack(
                                format!(
                                    "invalid change_stack: Start change at index {} while no \
                                    locked End change pending resolution",
                                    index,
                                )
                            )));
                        }
                    },
                    Change::End => {
                        if was_locked {
                            return Err(ActionManagerError::UndoError(InvalidChangeStack(
                                format!(
                                    "invalid change_stack: End change at index {} while a locked \
                                    End change is pending resolution",
                                    index,
                                )
                            )));
                        } else {
                            was_locked = true;
                            reverted_changes.push(Change::Start);
                        }
                    },
                    Change::StartEnd(action_rc) => {
                        if was_locked {
                            return Err(ActionManagerError::UndoError(InvalidChangeStack(
                                format!(
                                    "invalid change_stack: StartEnd change at index {} while a \
                                    locked End change is pending resolution",
                                    index,
                                )
                            )));
                        } else {
                            let mut action = action_rc.borrow_mut();

                            match (*action).perform_action(project) {
                                Ok(changes) => {
                                    for change in changes {
                                        index += 1;
                                        match change.as_untracked() {
                                            Ok(change) => {
                                                reverted_changes.push(change);
                                            },
                                            Err(error) => {
                                                return Err(CannotUntrackAction(error));
                                            },
                                        }
                                    }
                                    return Ok(())
                                },
                                Err(error) => {
                                    return Err(ActionFailedToPerform(format!(
                                        "at index {} of change-stack: {}",
                                        index,
                                        error,
                                    )));
                                },
                            }
                        }
                    },
                    Change::Untracked(action_rc) => {
                        if was_locked {
                            let mut action = action_rc.borrow_mut();
                            match (*action).perform_action(project) {
                                Ok(changes) => {
                                    for change in changes {
                                        index += 1;
                                        match change.as_untracked() {
                                            Ok(change) => {
                                                reverted_changes.push(change);
                                            },
                                            Err(error) => {
                                                return Err(CannotUntrackAction(error));
                                            },
                                        }
                                    }
                                },
                                Err(error) => {
                                    return Err(ActionFailedToPerform(format!(
                                        "at index {} of change-stack: {}",
                                        index,
                                        error,
                                    )));
                                },
                            }
                        } else {
                            return Err(ActionManagerError::UndoError(InvalidChangeStack(
                                format!(
                                    "invalid change_stack: Untracked change at index {} while no \
                                    locked End change pending resolution",
                                    index,
                                )
                            )));
                        }
                    },
                }
            }
            index -= 1;
        }
    }
}
