use crate::{ Console, memento };

use libpixylene::{
    types::{ UCoord },
    project::{ Project },
};


pub struct ResetCursors;

impl memento::Action for ResetCursors {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        let dim = project.canvas.dim();
        _ = project.clear_cursors();
        project.toggle_cursor_at(&(UCoord {
            x: u16::from(dim.x()).checked_div(2).unwrap(),
            y: u16::from(dim.y()).checked_div(2).unwrap(),
        }, project.focus.1))?;
        Ok(())
    }
}
