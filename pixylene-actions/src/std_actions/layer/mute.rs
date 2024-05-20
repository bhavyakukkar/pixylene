use crate::{ Console, memento };

use libpixylene::{
    project::{ LayersType, Project },
};


#[derive(Debug)]
pub struct Mute;

impl memento::Action for Mute {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        let layer = project.focus.1;

        //temporary solution (doing same exact thing shouldn't need match)
        //i dont think i have a solution for that honestly
        match project.canvas.layers {
            LayersType::True(ref mut layers) => {
                layers.get_layer_mut(layer)?.mute =
                    !layers.get_layer(layer)?.mute;
            },
            LayersType::Indexed(ref mut layers) => {
                layers.get_layer_mut(layer)?.mute =
                    !layers.get_layer(layer)?.mute;
            },
        }
        Ok(())
    }
}
