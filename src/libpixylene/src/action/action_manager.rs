use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::project::Project;
use crate::action::{ Action, ActionError, Change, ChangeError, UntrackError };
use crate::grammar::Decorate;

#[derive(Debug)]
enum ActionNameOrChangeIndex {
    ActionName(String),
    ChangeIndex(usize),
}

#[derive(Debug)]
pub enum ActionManagerError {
    ActionNotFound(String),
    ActionFailedToPerform(ActionNameOrChangeIndex, ActionError),
    LockedScene(String, String),
    LockedCamera(String, String),
    ChangeError(ChangeError), //this variant might be useless, Untrack Errors must directly panic
    NothingToUndo,
    NothingToRedo,
}
impl std::fmt::Display for ActionManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ActionManagerError::*;
        match self {
            ActionNotFound(action_name) => write!(
                f,
                "action '{}' was not found in actions inserted into the action-manager",
                action_name,
            ),
            ActionFailedToPerform(parameter, action_error) => write!(
                f,
                "{}: {}",
                match parameter {
                    ActionNameOrChangeIndex::ActionName(action_name) => format!(
                        "action '{}' failed to perform",
                        action_name
                    ),
                    ActionNameOrChangeIndex::ChangeIndex(change_index) => format!(
                        "action at change-index {} of action-manager's change-stack failed to \
                        perform",
                        change_index,
                    ),
                },
                action_error,
            ),
            LockedScene(action_name, scene_locked_action_name) => write!(
                f,
                "cannot perform scene-locking action '{}' while action '{}' has locked the scene",
                action_name,
                scene_locked_action_name,
            ),
            LockedCamera(action_name, camera_locked_action_name) => write!(
                f,
                "cannot perform camera-locking action '{}' while action '{}' has locked the camera",
                action_name,
                camera_locked_action_name,
            ),
            ChangeError(change_error) => write!(f, "{}", change_error),
            NothingToUndo => write!(f, "nothing to undo"),
            NothingToRedo => write!(f, "nothing to redo"),
        }
    }
}

pub struct ActionManager {
    actions: HashMap<String, Box<dyn Action>>,
    pub scene_lock: Option<String>, //todo: make private when better 'lock' purpose
    pub camera_lock: Option<String>, //todo: make private
    change_stack: Vec<Change>,
    change_index: usize,
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
        use ActionManagerError::{ ActionNotFound, LockedScene, LockedCamera, ActionFailedToPerform };
        use ActionNameOrChangeIndex::*;

        let action = match self.actions.get_mut(&action_name) {
            Some(action) => { action },
            None => {
                return Err(ActionNotFound(action_name));
            }
        };

        if let Some(scene_locked_action_name) = &self.scene_lock {
            if action.locks_scene() && action_name.ne(scene_locked_action_name) {
                return Err(LockedScene(action_name, scene_locked_action_name.to_string()));
            }
        }

        if let Some(camera_locked_action_name) = &self.camera_lock {
            if action.locks_camera() && action_name.ne(camera_locked_action_name) {
                return Err(LockedCamera(action_name, camera_locked_action_name.to_string()));
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
                        if self.change_index <= self.change_stack.len() {
                            //perform called at non-latest state
                            self.change_stack.drain((self.change_index - 1)..);
                        }
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
            Err(action_error) => {
                //recovery: unlock the scene that failed to perform
                if action.locks_scene() { self.scene_lock = None; }
                if action.locks_camera() { self.camera_lock = None; }
                return Err(ActionFailedToPerform(ActionName(action_name), action_error));
            }
        }
        Ok(())
    }
    pub fn undo(&mut self, project: &mut Project) -> Result<(), ActionManagerError> {
        use ActionManagerError::{ NothingToUndo, ActionFailedToPerform, ChangeError };
        use ActionNameOrChangeIndex::*;

        if self.change_index == 0 {
            return Err(NothingToUndo);
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
                            Err(action_error) => {
                                return Err(ActionFailedToPerform(
                                    ChangeIndex(self.change_index),
                                    action_error
                                ));
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
                                new_change = change.as_untracked().unwrap();
                            },
                            Err(action_error) => {
                                return Err(ActionFailedToPerform(
                                    ChangeIndex(self.change_index),
                                    action_error
                                ));
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
        use ActionManagerError::{ NothingToRedo, ActionFailedToPerform };
        use ActionNameOrChangeIndex::*;

        if self.change_stack.len() == 0 || self.change_index == self.change_stack.len() {
            return Err(NothingToRedo)
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
                            Err(action_error) => {
                                return Err(ActionFailedToPerform(
                                    ChangeIndex(self.change_index),
                                    action_error
                                ));
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
                                new_change = change.as_untracked().unwrap();
                            },
                            Err(action_error) => {
                                return Err(ActionFailedToPerform(
                                    ChangeIndex(self.change_index),
                                    action_error
                                ));
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
    pub fn add_action(&mut self, action_name: String, action: Box<dyn Action>) {
        self.actions.insert(action_name, action);
    }
    pub fn list_actions(&self) -> Vec<String> {
        (&self.actions).into_iter().map(|(action_name, _)| action_name.clone()).collect()
    }
}
