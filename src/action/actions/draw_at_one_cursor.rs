use std::rc::Rc;
use std::cell::RefCell;

use crate::elements::common::{ Coord, Pixel };
use crate::project::{ Project, Cursor };
use crate::action::{ Action, ActionError, Change };

/* 
 * Draw At One Cursor
 * An action that draws once at the specified `cursor with the specified `color
 *
 */ 

pub struct DrawAtOneCursor {
    pub cursor: Cursor,
    pub color: Option<Pixel>,
}
impl Action for DrawAtOneCursor {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let old_pixel = project.layers[self.cursor.layer].scene.get_pixel(self.cursor.coord)?;
        project.layers[self.cursor.layer].scene.set_pixel(
            self.cursor.coord,
            self.color
        )?;
        let mut draw_once_back = DrawAtOneCursor {
            cursor: self.cursor,
            color: old_pixel
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(draw_once_back)))])
    }
}
