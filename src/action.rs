#[warn(unused_assignments)]
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp::{ min, max };

use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::project::Project;
use crate::grammar::Decorate;

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

pub trait Action {
    //perform action, transform to reverted (for undo) action, and return as a Change
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String>;
    //whether action has been completely executed
    fn end_action(&self) -> bool;
}

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
    lock: bool,
    locked_action: Option<String>,
    change_stack: Vec<Change>,
}
impl ActionManager {
    pub fn new(actions: HashMap<String, Box<dyn Action>>) -> Self {
        ActionManager {
            actions: actions,
            lock: false,
            locked_action: None,
            change_stack: Vec::new(),
        }
    }
    fn record(&mut self, changes: Vec<Change>) {
        for change in changes {
            match change {
                Change::Start => {
                    self.lock = true;
                    self.change_stack.push(Change::Start);
                },
                Change::End => {
                    self.change_stack.push(Change::End);
                    self.lock = false;
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

        if let Some(action) = self.actions.get_mut(&action_name) {
            match &self.locked_action {
                Some(locked_action) => {
                    if locked_action.ne(&action_name) {
                        return Err(ActionManagerError::PerformError(LockedAction(format!(
                            "cannot perform action '{}' while action '{}' has locked the \
                            action-manager",
                            action_name,
                            locked_action,
                        ))));
                    } else {
                        match action.perform_action(project) {
                            Ok(changes) => {
                                self.record(changes);
                                return Ok(());
                            },
                            Err(error) => {
                                return Err(ActionFailedToPerform(String::from(error)));
                            },
                        }
                    }
                },
                None => {
                    match action.perform_action(project) {
                        Ok(changes) => {
                            self.record(changes);
                            return Ok(());
                        },
                        Err(error) => {
                            return Err(ActionFailedToPerform(String::from(error)));
                        },
                    }
                }
            }
        } else {
            return Err(ActionManagerError::PerformError(ActionNotFound(format!(
                "action '{}' was not found in inserted actions",
                action_name,
            ))));
        }
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
        if self.lock {
            return Err(ActionManagerError::UndoError(LockedAction(
                String::from("Cannot undo while action-manager is locked")
            )));
        }
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

pub struct ResizeCamera {
    pub dim_incr: Coord
}
impl Action for ResizeCamera {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String> {
        project.camera.set_dim(project.camera.dim.add(self.dim_incr))?;
        let mut resize_camera_back = ResizeCamera {
            dim_incr: self.dim_incr.multiply(Coord{ x: -1, y: -1 }),
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(resize_camera_back)))])
    }
    fn end_action(&self) -> bool {
        true
    }
}


pub struct MoveCamera {
    pub focus_move: Coord
}
impl Action for MoveCamera {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String> {
        let old_focus = project.camera.focus;
        project.camera.set_focus(
            &project.layers[project.selected_layer].scene,
            old_focus.add(self.focus_move)
        )?;
        let mut move_camera_back = MoveCamera {
            focus_move: self.focus_move.multiply(Coord{ x: -1, y: -1 }),
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(move_camera_back)))])
    }
    fn end_action(&self) -> bool {
        true
    }
}


pub struct ZoomCamera {
    pub mult_incr: isize
}
impl Action for ZoomCamera {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String> {
        project.camera.set_mult(project.camera.mult + self.mult_incr)?;
        let mut zoom_camera_back = ZoomCamera {
            mult_incr: self.mult_incr * -1,
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(zoom_camera_back)))])
    }
    fn end_action(&self) -> bool {
        true
    }
}


pub struct ChangeCameraRepeat {
    pub repeat_diff: Coord
}
impl Action for ChangeCameraRepeat {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String> {
        project.camera.set_repeat(project.camera.repeat.add(self.repeat_diff))?;
        let mut change_camera_repeat_back = ChangeCameraRepeat {
            repeat_diff: self.repeat_diff.multiply(Coord{ x: -1, y: -1 }),
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(change_camera_repeat_back)))])
    }
    fn end_action(&self) -> bool {
        true
    }
}


/* 
 * Draw Once
 * a "Primitive Action" that draws once on the set `layer at the set `focus with the set `color
 */ 
pub struct DrawOnce {
    pub layer: usize,
    pub focus: Coord,
    pub color: Option<Pixel>,
}
impl Action for DrawOnce {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String> {
        let old_pixel = project.layers[self.layer].scene.get_pixel(
            self.focus
        )?;
        project.layers[self.layer].scene.set_pixel(
            self.focus,
            self.color
        )?;
        let mut draw_once_back = DrawOnce {
            layer: self.layer,
            focus: self.focus,
            color: old_pixel
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(draw_once_back)))])
    }
    fn end_action(&self) -> bool { true }
}


pub struct Pencil {
    pub palette_index: usize,
    pub new_pixel: Option<Option<Pixel>>,
}
impl Action for Pencil {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String> {
        let mut changes: Vec<Change> = vec![Change::Start];
        let old_pixel = project.layers[project.selected_layer].scene.get_pixel(
            project.camera.focus
        )?;
        let mut draw_once = DrawOnce {
            layer: project.selected_layer,
            focus: project.camera.focus,
            color: Some(BlendMode::Normal.merge_down(
               Pixel::get_certain(project.palette.get_color((&self).palette_index)?),
               Pixel::get_certain(
                   project
                       .layers[project.selected_layer]
                       .scene.get_pixel(project.camera.focus)?
               )
           ))
        }
            .perform_action(project)?;
        for change in draw_once {
            changes.push(change.as_untracked()?);
        }
        changes.push(Change::End);
        Ok(changes)
    }
    fn end_action(&self) -> bool {
        true
    }
}

pub struct RectangularFill {
    pub palette_index: usize,
    pub start_corner: Option<Coord>,
}
impl Action for RectangularFill {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String> {
        if let Some(start_corner) = self.start_corner {
            let mut changes: Vec<Change> = Vec::new();
            for i in min(start_corner.x, project.camera.focus.x)..(max(
                start_corner.x,
                project.camera.focus.x
            ) + 1) {
                for j in min(start_corner.y, project.camera.focus.y)..(max(
                    start_corner.y,
                    project.camera.focus.y
                ) + 1) {
                    let mut draw_once = DrawOnce {
                        layer: project.selected_layer,
                        focus: Coord{ x: i, y: j },
                        color: Some(BlendMode::Normal.merge_down(
                            Pixel::get_certain(project.palette.get_color((&self).palette_index)?),
                            Pixel::get_certain(
                                project
                                    .layers[project.selected_layer]
                                    .scene.get_pixel(Coord{ x: i, y: j })?
                            )
                        ))
                    }
                        .perform_action(project)?;
                    for change in draw_once {
                        changes.push(change.as_untracked()?);
                    }
                }
            }
            changes.push(Change::End);
            Ok(changes)
        } else {
            self.start_corner = Some(project.camera.focus);
            Ok(vec![Change::Start])
        }
    }
    fn end_action(&self) -> bool {
        match self.start_corner {
            Some(_) => false,
            None => true,
        }
    }
}
