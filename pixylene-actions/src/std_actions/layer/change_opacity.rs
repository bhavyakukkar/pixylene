use crate::{ Console, memento, ActionError };

use libpixylene::{
    project::{ CanvasType, Project },
};


#[derive(Debug)]
pub struct ChangeOpacity;

impl memento::Action for ChangeOpacity {
    fn perform(&mut self, project: &mut Project, console: &dyn Console) -> memento::ActionResult {
        use ActionError::{InputError, Discarded};

        let input = console.cmdin("opacity (0.0 to 1.0): ").ok_or(Discarded)?;
        let opacity: Result<u8, ActionError> = match input.parse::<f32>() {
            Ok(num) => if num >= 0.0 && num <= 1.0 {
                Ok((num*255.0).round() as u8)
            } else {
                Err(InputError(format!("expecting between 0.0 and 1.0, found '{}'", num)))
            },
            Err(err) => Err(InputError(err.to_string())),
        };

        //temporary solution (doing same exact thing shouldn't need match)
        let layer = project.focus.1;
        match project.canvas_mut() {
            CanvasType::True(ref mut canvas) => {
                canvas.layers_mut().get_layer_mut(layer)?.opacity = opacity?;
            },
            CanvasType::Indexed(ref mut canvas) => {
                canvas.layers_mut().get_layer_mut(layer)?.opacity = opacity?;
            },
        }
        Ok(())
    }
}
