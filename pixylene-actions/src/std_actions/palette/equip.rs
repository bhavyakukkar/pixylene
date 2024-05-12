use crate::{ Console, memento };

use libpixylene::{
    project::{ Project },
};


/// An action that equips the given index for the color palette
pub struct Equip {
    palette_index: u8,
}

impl Equip {
    pub fn new(palette_index: u8) -> Self {
        Equip{ palette_index }
    }
}


impl memento::Action for Equip {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        Ok(project.canvas_mut().inner_mut().palette().equip(self.palette_index)?)
    }
}
