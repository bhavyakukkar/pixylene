use crate::{Console, ActionError, memento, utils::OptionalTrueOrIndexed};

use libpixylene::{
    types::{UCoord, Pixel, TruePixel, BlendMode},
    project::{CanvasType, Project},
};

 
/// An action that draws once at the specified `cursor with the specified `color and specified
/// `blend_mode
#[derive(Debug)]
pub struct Draw {
    cursor: (UCoord, u16),
    color: OptionalTrueOrIndexed,
    blend_mode: BlendMode,
}

impl Draw {
    pub fn new(cursor: (UCoord, u16), color: OptionalTrueOrIndexed, blend_mode: BlendMode) -> Self {
        Draw{ cursor, color, blend_mode }
    }
}

//impl command::Action for Draw {
//    fn perform(&mut self, project: &mut Project, _console: &dyn Console)
//        -> command::ActionResult
//    {
//        let old_pixel = project.canvas.get_layer(self.cursor.1)?.scene.get_pixel(self.cursor.0)?;
//        project.canvas.get_layer_mut(self.cursor.1)?.scene.set_pixel(
//            self.cursor.0,
//            Some(self.blend_mode.blend(
//                self.color.unwrap_or(Pixel::empty()),
//                old_pixel.unwrap_or(Pixel::empty())
//            )?),
//        )?;
//
//        // Command Pattern requires declaration of the reverted Command
//        let draw_back = Draw {
//            cursor: self.cursor,
//            color: old_pixel,
//            blend_mode: BlendMode::Overwrite,
//        };
//        Ok(vec![command::Change::StartEnd(Rc::new(RefCell::new(draw_back)))])
//    }
//}

impl memento::Action for Draw {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        use ActionError::InvalidCanvasType;
        use OptionalTrueOrIndexed::*;

        match (project.canvas_mut(), &self.color) {
            (CanvasType::True(ref mut canvas), True(ref new_pixel)) => {
                let old_pixel: Option<TruePixel> = canvas.layers()
                    .get_layer(self.cursor.1)?.scene.get_pixel(self.cursor.0)?;

                canvas.layers_mut().get_layer_mut(self.cursor.1)?.scene.set_pixel(
                    self.cursor.0,
                    Some(self.blend_mode.blend(
                        new_pixel.unwrap_or(TruePixel::empty()),
                        old_pixel.unwrap_or(TruePixel::empty())
                    )?),
                )?;
                Ok(())
            },
            (CanvasType::Indexed(ref mut canvas), Indexed(ref new_pixel)) => {
                canvas.layers_mut().get_layer_mut(self.cursor.1)?.scene.set_pixel(
                    self.cursor.0,
                    *new_pixel,
                )?;
                Ok(())
            },
            (CanvasType::True(_), Indexed(_)) =>
                Err(InvalidCanvasType{ expecting_indexed: false }),
            (CanvasType::Indexed(_), True(_)) =>
                Err(InvalidCanvasType{ expecting_indexed: true }),
        }
    }
}
