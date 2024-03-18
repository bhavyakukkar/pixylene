use crate::{ Console, ActionError, command, memento };

use libpixylene::{
    types::{ Coord, UCoord },
    project::{ Project },
};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;


pub struct MoveAllCursors {
    displacement: Coord,
}

impl MoveAllCursors {
    pub fn new(displacement: Coord) -> Self {
        MoveAllCursors{ displacement }
    }
}

impl memento::Action for MoveAllCursors {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        let mut new_cursors: HashMap<(UCoord, u16), ()> = HashMap::new();
        let dim = project.canvas.dim();
        let cursors = project.cursors().map(|cursor| cursor.clone());
        for cursor in cursors {
            let displaced_cursor = Coord::from(&cursor.0).add(self.displacement);
            if displaced_cursor.x < 0 || displaced_cursor.y < 0 
                || displaced_cursor.x >= dim.x() as i32 || displaced_cursor.y >= dim.y() as i32
            {
                return Err(ActionError::OperationError(None));
            } else {
                new_cursors.insert((UCoord {
                    x: displaced_cursor.x.try_into().unwrap(),
                    y: displaced_cursor.y.try_into().unwrap(),
                }, cursor.1), ());
            }
        }
        _ = project.clear_cursors();
        for cursor in new_cursors {
            project.toggle_cursor_at(cursor.0)?;
        }
        Ok(())
    }
}
