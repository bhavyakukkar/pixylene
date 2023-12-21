use std::rc::Rc;
use std::cell::RefCell;

use crate::elements::common::{ Coord, Pixel };
use crate::project::Project;
use crate::action::{ Action, Change };

/* 
 * Draw Once
 * a "Primitive Action" that draws once on the set `layer at the set `focus with the set `color
 */ 
pub struct DrawOnce {
    pub layer: usize,
    pub focus: Coord,
    pub color: Option<Pixel>,
}
impl Action for DrawOnce {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, String> {
        let old_pixel = project.layers[self.layer].scene.get_pixel(
            self.focus
        )?;
        project.layers[self.layer].scene.set_pixel(
            self.focus,
            self.color
        )?;
        let mut draw_once_back = DrawOnce {
            layer: self.layer,
            focus: self.focus,
            color: old_pixel
        };
        Ok(vec![Change::StartEnd(Rc::new(RefCell::new(draw_once_back)))])
    }
    fn end_action(&self) -> bool { true }
}
