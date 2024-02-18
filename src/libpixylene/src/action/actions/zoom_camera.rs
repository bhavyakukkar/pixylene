use std::rc::Rc;
use std::cell::RefCell;

use crate::types::Coord;
use crate::project::Project;
use crate::action::{ Action, ActionError, Change };

pub struct ZoomCamera {
    pub mult_incr: isize
}
impl Action for ZoomCamera {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        project.camera.set_mult(project.camera.mult + self.mult_incr)?;
        let mut zoom_camera_back = ZoomCamera {
            mult_incr: self.mult_incr * -1,
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(zoom_camera_back)))])
    }
}
