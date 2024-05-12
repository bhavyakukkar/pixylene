use crate::{ Console, memento };

use libpixylene::{
    project::{ CanvasType, Project },
};


#[derive(Debug)]
pub struct Mute;

impl memento::Action for Mute {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        let layer = project.focus.1;

        //temporary solution (doing same exact thing shouldn't need match)
        match project.canvas_mut() {
            CanvasType::True(ref mut canvas) => {
                canvas.layers_mut().get_layer_mut(layer)?.mute =
                    !canvas.layers().get_layer(layer)?.mute;
            },
            CanvasType::Indexed(ref mut canvas) => {
                canvas.layers_mut().get_layer_mut(layer)?.mute =
                    !canvas.layers().get_layer(layer)?.mute;
            },
        }
        Ok(())
    }
}
