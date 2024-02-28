use crate::{
    types::{ Coord, Pixel, BlendMode, Cursor },
    project::{ Project },
    action::{ Action, ActionError, Change },
};

use std::rc::Rc;
use std::cell::RefCell;

/* 
 * Draw At One Cursor
 * An action that draws once at the specified `cursor with the specified `color
 *
 */ 

pub struct DrawAtOneCursor {
    pub cursor: Cursor,
    pub color: Option<Pixel>,
    pub blend_mode: BlendMode,
}
impl Action for DrawAtOneCursor {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let old_pixel = project.canvas.layers[self.cursor.layer].scene.get_pixel(self.cursor.coord)?;
        project.canvas.layers[self.cursor.layer].scene.set_pixel(
            self.cursor.coord,
            Some(self.blend_mode.merge_down(
                Pixel::get_certain(self.color),
                Pixel::get_certain(old_pixel)
            )),
        )?;
        let mut draw_once_back = DrawAtOneCursor {
            cursor: self.cursor,
            color: old_pixel,
            blend_mode: BlendMode::Overwrite,
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(draw_once_back)))])
    }
}