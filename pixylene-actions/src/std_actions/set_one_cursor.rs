use crate::{
    types::{ Coord, Cursor },
    project::{ ProjectError, Project },
    action::{ Action, ActionError, Change },
};

use std::rc::Rc;
use std::cell::RefCell;


pub struct SetOneCursor {
    pub index: usize,
    pub coord: Option<Coord>,
    pub layer: Option<usize>,
}
impl Action for SetOneCursor {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let Cursor { coord: old_coord, layer: old_layer } = project.get_cursor(self.index)?;
        project.set_cursor(self.index, Cursor{
            coord: match self.coord {
                Some(coord) => coord,
                None => old_coord,
            },
            layer: match self.layer {
                Some(layer) => layer,
                None => old_layer,
            }
        })?;
        let mut set_one_cursor_back = SetOneCursor {
            index: self.index,
            coord: Some(old_coord),
            layer: Some(old_layer),
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(set_one_cursor_back)))])
    }
}
