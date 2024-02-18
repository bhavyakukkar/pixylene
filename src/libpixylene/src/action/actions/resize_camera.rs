use std::rc::Rc;
use std::cell::RefCell;

use crate::types::Coord;
use crate::project::Project;
use crate::action::{ Action, ActionError, Change };

pub struct ResizeCamera {
    pub dim_incr: Coord
}
impl Action for ResizeCamera {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        project.camera.set_dim(project.camera.dim.add(self.dim_incr))?;
        let mut resize_camera_back = ResizeCamera {
            dim_incr: self.dim_incr.multiply(Coord{ x: -1, y: -1 }),
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(resize_camera_back)))])
    }
}
