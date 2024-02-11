use std::rc::Rc;
use std::cell::RefCell;

use crate::common::Coord;
use crate::project::{ Project, ProjectError, Cursor };
use crate::action::{ Action, ActionError, Change };

pub struct SetFocus {
    pub coord: Option<Coord>,
    pub layer: Option<usize>,
}
impl Action for SetFocus {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let Cursor { coord: old_coord, layer: old_layer } = project.focus;
        project.focus.coord = match self.coord {
            Some(coord) => coord,
            None => old_coord,
        };
        project.focus.layer = match self.layer {
            Some(layer) => layer,
            None => old_layer,
        };
        let mut set_focus_back = SetFocus {
            coord: Some(old_coord),
            layer: Some(old_layer),
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(set_focus_back)))])
    }
}
