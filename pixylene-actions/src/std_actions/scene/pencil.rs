use crate::{ Console, memento, utils::OptionalTrueOrIndexed };
use super::Draw;

use libpixylene::{
    types::{ UCoord, IndexedPixel, BlendMode },
    project::{ CanvasType, Canvas, Project },
};


/// An action that extends Draw to dynamically use the project's color at a specificed
/// palette index and blend it normally with the existing color at each cursor, taking the equipped
/// pencil if index not specified
pub struct Pencil {
    palette_index: Option<u8>,
}

impl Pencil {
    pub fn new(palette_index: Option<u8>) -> Self {
        Pencil{ palette_index }
    }
}

//impl command::Action for Pencil {
//    fn perform(&mut self, project: &mut Project, console: &dyn Console)
//        -> command::ActionResult
//    {
//        let mut changes: Vec<command::Change> = vec![command::Change::Start];
//        let cursors = project.cursors().map(|a| a.clone())
//            .collect::<Vec<(UCoord, u16)>>();
//        for cursor in cursors {
//            if let Ok(draw) = (Draw::new(
//                cursor,
//                Some(match (&self).palette_index {
//                    Some(index) => *project.canvas.palette.get_color(index)?,
//                    None => *project.canvas.palette.get_equipped(),
//                }),
//                //Some(*project.canvas.palette.get_color((&self).palette_index)?),
//                BlendMode::Normal,
//            )).perform(project, console) {
//                for change in draw {
//                    changes.push(change.as_untracked()?);
//                }
//            }
//        }
//        changes.push(command::Change::End);
//        Ok(changes)
//    }
//}

impl memento::Action for Pencil {
    fn perform(&mut self, project: &mut Project, console: &dyn Console) -> memento::ActionResult {
        use OptionalTrueOrIndexed::*;

        let cursors = project.cursors().map(|a| a.clone())
            .collect::<Vec<(UCoord, u16)>>();
        for cursor in cursors {
            Draw::new(
                cursor,
                match project.canvas_mut() {
                    CanvasType::True(ref mut canvas) => True(Some(match (&self).palette_index {
                        Some(index) => *canvas.palette().get_color(index)?,
                        None => *canvas.palette().get_equipped(),
                    })),
                    CanvasType::Indexed(_) =>
                        Indexed(self.palette_index.map(|index| IndexedPixel(index))),
                },
                BlendMode::Normal,
            ).perform(project, console)?;
        }
        Ok(())
    }
}
