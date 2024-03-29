use crate::{ Console, command, memento };
use super::Draw;

use libpixylene::{
    types::{ UCoord, Pixel, BlendMode },
    project::{ Project },
};
use std::rc::Rc;
use std::cell::RefCell;


/// An action that extends Draw to dynamically use the project's color at a specificed
/// palette index and blend it normally with the existing color at each cursor
pub struct Pencil {
    palette_index: u8,
}

impl Pencil {
    pub fn new(palette_index: u8) -> Self {
        Pencil{ palette_index }
    }
}

impl command::Action for Pencil {
    fn perform(&mut self, project: &mut Project, console: &dyn Console)
        -> command::ActionResult
    {
        let mut changes: Vec<command::Change> = vec![command::Change::Start];
        let cursors = project.cursors().map(|a| a.clone())
            .collect::<Vec<(UCoord, u16)>>();
        for cursor in cursors {
            if let Ok(draw) = (Draw::new(
                cursor,
                Some(*project.canvas.palette.get_color((&self).palette_index)?),
                BlendMode::Normal,
            )).perform(project, console) {
                for change in draw {
                    changes.push(change.as_untracked()?);
                }
            }
        }
        changes.push(command::Change::End);
        Ok(changes)
    }
}

impl memento::Action for Pencil {
    fn perform(&mut self, project: &mut Project, console: &dyn Console) -> memento::ActionResult {
        let cursors = project.cursors().map(|a| a.clone())
            .collect::<Vec<(UCoord, u16)>>();
        for cursor in cursors {
            Draw::new(
                cursor,
                Some(*project.canvas.palette.get_color((&self).palette_index)?),
                BlendMode::Normal,
            ).perform(project, console)?;
        }
        Ok(())
    }
}
