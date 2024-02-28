use crate::{
    types::{ Coord },
    project::{ Project },
    action::{ Action, ActionError, Change, helper::* },
};

use std::rc::Rc;
use std::cell::RefCell;


pub struct ZoomCamera {
    value: AbsOrRel<u8, i8>,
}
impl Action for ZoomCamera {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        if let Abs(new_mult) = self.value
        {
            project.camera.set_mult(new_mut)?;
        }
        else
        {
            project.camera.set_mult(project.camera.get_mult() + 
        }
        project.camera.set_mut(
        project.camera.set_mult(project.camera.mult + self.mult_incr)?;
        let mut zoom_camera_back = ZoomCamera {
            mult_incr: self.mult_incr * -1,
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(zoom_camera_back)))])
    }
}

fn make(
