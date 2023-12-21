use std::rc::Rc;
use std::cell::RefCell;

use crate::elements::common::Coord;
use crate::project::Project;
use crate::action::{ Action, Change };

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
