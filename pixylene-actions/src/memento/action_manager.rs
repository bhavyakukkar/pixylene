use crate::Console;
use super::{ Action, ActionResult };

use libpixylene::project::{ Project, CanvasType };
use undo::{ Edit, History };
use std::rc::Rc;
use std::cell::RefCell;


/// A stored edit to the Canvas
struct TransformCanvas(
    /// The old canvas
    pub CanvasType,
    /// The new canvas
    pub CanvasType,
);

impl Edit for TransformCanvas {
    type Target = CanvasType;
    type Output = ();

    fn edit(&mut self, canvas: &mut CanvasType) {
        *canvas = self.1.clone();
    }
    
    fn undo(&mut self, canvas: &mut CanvasType) {
        *canvas = self.0.clone();
    }
}

pub struct ActionManager {
    canvas_state: CanvasType,
    canvas_history: History<TransformCanvas>,
}

impl ActionManager {

    /// Creates a new ActionManager and uses the Canvas passed to create the initial commit
    pub fn new(canvas: &CanvasType) -> ActionManager {
        ActionManager {
            canvas_state: canvas.clone(),
            canvas_history: History::new(),
        }
    }

    pub fn perform(&mut self, project: &mut Project, console: &dyn Console,
                   action: Rc<RefCell<dyn Action>>)
    -> ActionResult {

        action.borrow_mut().perform(project, console)?;
        Ok(())
    }

    /// Commits the Canvas state only if it has changed, returning whether the Canvas has changed
    pub fn commit(&mut self, canvas: &CanvasType) -> bool {
        let Self { ref mut canvas_state, ref mut canvas_history } = self;
        if *canvas != *canvas_state {
            let transform = TransformCanvas(canvas_state.clone(), canvas.clone());
            canvas_history.edit(canvas_state, transform);
            true
        } else {
            false
        }
    }

    pub fn undo(&mut self, canvas: &mut CanvasType) {
        let Self { ref mut canvas_state, ref mut canvas_history } = self;
        canvas_history.undo(canvas_state);
        *canvas = canvas_state.clone();
    }

    pub fn redo(&mut self, canvas: &mut CanvasType) {
        let Self { ref mut canvas_state, ref mut canvas_history } = self;
        canvas_history.redo(canvas_state);
        *canvas = canvas_state.clone();
    }

    //todo: add methods to go to next/previous branch
}
