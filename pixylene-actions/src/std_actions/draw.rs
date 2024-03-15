use crate::{ Console, command, memento };

use libpixylene::{
    types::{ UCoord, Pixel, BlendMode },
    project::{ Project },
};
use std::rc::Rc;
use std::cell::RefCell;

 
/// An action that draws once at the specified `cursor with the specified `color and specified
/// `blend_mode
pub struct Draw {
    pub cursor: (UCoord, u16),
    pub color: Option<Pixel>,
    pub blend_mode: BlendMode,
}

impl command::Action for Draw {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console)
        -> command::ActionResult
    {
        let old_pixel = project.canvas.get_layer(self.cursor.1)?.scene.get_pixel(self.cursor.0)?;
        project.canvas.get_layer_mut(self.cursor.1)?.scene.set_pixel(
            self.cursor.0,
            Some(self.blend_mode.blend(
                self.color.unwrap_or(Pixel::empty()),
                old_pixel.unwrap_or(Pixel::empty())
            )?),
        )?;

        // Command Pattern requires declaration of the reverted Command
        let draw_back = Draw {
            cursor: self.cursor,
            color: old_pixel,
            blend_mode: BlendMode::Overwrite,
        };
        Ok(vec![command::Change::StartEnd(Rc::new(RefCell::new(draw_back)))])
    }
}

impl memento::Action for Draw {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        let old_pixel = project.canvas.get_layer(self.cursor.1)?.scene.get_pixel(self.cursor.0)?;
        project.canvas.get_layer_mut(self.cursor.1)?.scene.set_pixel(
            self.cursor.0,
            Some(self.blend_mode.blend(
                self.color.unwrap_or(Pixel::empty()),
                old_pixel.unwrap_or(Pixel::empty())
            )?),
        )?;
        Ok(())
    }
}
