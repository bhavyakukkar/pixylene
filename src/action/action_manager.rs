use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

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
enum RedoError {
    LockedAction(String),
    InvalidChangeStack(String),
    NothingToRedo(String),
}
impl std::fmt::Display for RedoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use RedoError::*;
        let decorate = |name: &str, desc: &String| -> String {
            Decorate::output(name.to_string(), None, Some(desc.to_string()))
        };
        match self {
            LockedAction(desc) => write!(f, "{}", decorate("LockedAction", desc)),
            InvalidChangeStack(desc) => write!(f, "{}", decorate("InvalidChangeStack", desc)),
            NothingToRedo(desc) => write!(f, "{}", decorate("NothingToRedo", desc)),
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
    PerformError(PerformError),
    UndoError(UndoError),
    RedoError(RedoError),
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
            ActionManagerError::PerformError(error) => write!(f, "{}", Decorate::output(
                "PerformError".to_string(),
                None,
                Some(error.to_string())
            )),
            ActionManagerError::UndoError(error) => write!(f, "{}", Decorate::output(
                "UndoError".to_string(),
                None,
                Some(error.to_string())
            )),
            ActionManagerError::RedoError(error) => write!(f, "{}", Decorate::output(
                "RedoError".to_string(),
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
    pub change_stack: Vec<Change>, //todo: make private
    pub change_index: usize, //todo: make private
}
impl ActionManager {
    pub fn new(actions: HashMap<String, Box<dyn Action>>) -> Self {
        ActionManager {
            actions: actions,
            scene_lock: None,
            camera_lock: None,
            change_stack: Vec::new(),
            change_index: 0,
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
                        self.change_index += 1;
                        self.change_stack.push(change);
                        i += 1;
                    }

                    self.change_index += 1;
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

        if self.change_index == 0 {
            return Err(ActionManagerError::UndoError(NothingToUndo(
                String::from("nothing to undo")
            )));
        }
        let mut new_change: Change;
        loop {
            new_change = Change::End;
            if let Some(change) = self.change_stack.get_mut(self.change_index - 1) {
                match change {
                    Change::Start => {
                        new_change = Change::Start;
                        self.change_index -= 1;
                        break;
                    },
                    Change::End => {
                        new_change = Change::End;
                        self.change_index -= 1;
                    },
                    Change::StartEnd(action_rc) => {
                        let mut action = action_rc.borrow_mut();
                        match (*action).perform_action(project) {
                            Ok(changes) => for change in changes {
                                // assumes that action that returned
                                // StartEnd once will return StartEnd again
                                // and for-loop will only run for 1 iter
                                new_change = change;
                            },
                            Err(desc) => {
                                return Err(ActionManagerError::ActionFailedToPerform(desc));
                            }
                        }
                        self.change_index -= 1;
                        break;
                    },
                    Change::Untracked(action_rc) => {
                        let mut action = action_rc.borrow_mut();
                        match (*action).perform_action(project) {
                            Ok(changes) => for change in changes {
                                // assumes that action that returned
                                // Untracked once will return Untracked again
                                // and for-loop will only run for 1 iter
                                match change.as_untracked() {
                                    Ok(untracked_change) => {
                                        new_change = untracked_change;
                                    },
                                    Err(desc) => panic!("{}", desc),
                                }
                            },
                            Err(desc) => {
                                return Err(ActionManagerError::ActionFailedToPerform(desc));
                            }
                        }
                        self.change_index -= 1;
                    }
                }
                self.change_stack[self.change_index] = new_change;
            }
        }
        Ok(())
    }
    pub fn redo(&mut self, project: &mut Project) -> Result<(), ActionManagerError> {
        use RedoError::*;

        if self.change_stack.len() == 0 || self.change_index == self.change_stack.len() - 1 {
            return Err(ActionManagerError::RedoError(NothingToRedo(
                String::from("nothing to redo")
            )))
        }
        let mut new_change: Change;
        loop {
            new_change = Change::End;
            if let Some(change) = self.change_stack.get_mut(self.change_index) {
                match change {
                    Change::Start => {
                        new_change = Change::Start;
                        self.change_index += 1;
                    },
                    Change::End => {
                        new_change = Change::End;
                        self.change_index += 1;
                        break;
                    },
                    Change::StartEnd(action_rc) => {
                        let mut action = action_rc.borrow_mut();
                        match (*action).perform_action(project) {
                            Ok(changes) => for change in changes {
                                // assumes that action that returned
                                // StartEnd once will return StartEnd again
                                // and for-loop will only run for 1 iter
                                new_change = change;
                            },
                            Err(desc) => {
                                return Err(ActionManagerError::ActionFailedToPerform(desc));
                            }
                        }
                        self.change_index += 1;
                        break;
                    },
                    Change::Untracked(action_rc) => {
                        let mut action = action_rc.borrow_mut();
                        match (*action).perform_action(project) {
                            Ok(changes) => for change in changes {
                                // assumes that action that returned
                                // Untracked once will return Untracked again
                                // and for-loop will only run for 1 iter
                                match change.as_untracked() {
                                    Ok(change) => {
                                        new_change = change;
                                    },
                                    Err(desc) => {
                                        return Err(ActionManagerError::CannotUntrackAction(desc));
                                    }
                                }
                            },
                            Err(desc) => {
                                return Err(ActionManagerError::ActionFailedToPerform(desc));
                            }
                        }
                        self.change_index += 1;
                    }
                }
                self.change_stack[self.change_index - 1] = new_change;
            }
        }
        Ok(())
    }
}
