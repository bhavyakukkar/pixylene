use std::rc::Rc;
use std::cell::RefCell;

use crate::types::Coord;
use crate::project::Project;
use crate::action::{ Action, ActionError, Change };

pub struct ChangeCameraRepeat {
    pub repeat_diff: Coord
}
impl Action for ChangeCameraRepeat {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        project.camera.set_repeat(project.camera.repeat.add(self.repeat_diff))?;
        println!("\n{}", self.repeat_diff);
        let mut change_camera_repeat_back = ChangeCameraRepeat {
            repeat_diff: self.repeat_diff.multiply(Coord{ x: -1, y: -1 }),
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(change_camera_repeat_back)))])
    }
}
