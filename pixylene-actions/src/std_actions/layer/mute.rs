use crate::{ Console, memento };

use libpixylene::{
    project::{ Project },
};


#[derive(Debug)]
pub struct Mute;

impl memento::Action for Mute {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        project.canvas.get_layer_mut(project.focus.1)?.mute =
            !project.canvas.get_layer(project.focus.1)?.mute;
        Ok(())
    }
}
