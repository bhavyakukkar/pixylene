use std::rc::Rc;
use std::cell::RefCell;

use crate::types::Coord;
use crate::project::Project;
use crate::action::{ Action, ActionError, Change };

pub struct MoveFocus {
    pub displacement: Coord
}
impl Action for MoveFocus {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        project.focus.coord = project.focus.coord.add(self.displacement);
        let mut move_focus_back = MoveFocus {
            displacement: self.displacement.multiply(Coord{ x: -1, y: -1 }),
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(move_focus_back)))])
    }
}
