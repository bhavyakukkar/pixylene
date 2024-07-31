use crate::{memento, utils::Direction, ActionError, Console};

use libpixylene::{
    project::Project,
    types::{Coord, UCoord},
};

pub struct DuplicateCursors {
    direction: Direction,
    amount: u16,
}

impl DuplicateCursors {
    pub fn new(direction: Direction, amount: u16) -> Self {
        DuplicateCursors { direction, amount }
    }
}

impl memento::Action for DuplicateCursors {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        if self.amount == 0 {
            return Err(ActionError::ArgsError(String::from("given amount 0")));
        }
        let dim = project.canvas.layers.dim();
        let cursors = project
            .cursors()
            .map(|cursor| cursor.clone())
            .collect::<Vec<(UCoord, u16)>>();
        for cursor in cursors {
            let dup_cursor = Coord::from(&cursor.0).add(
                Coord {
                    x: i32::from(self.amount),
                    y: i32::from(self.amount),
                }
                .mul(self.direction.unit()),
            );
            if dup_cursor.x < 0
                || dup_cursor.y < 0
                || dup_cursor.x >= dim.x() as i32
                || dup_cursor.y >= dim.y() as i32
            {
                ()
            } else {
                let dup_ucoord = UCoord {
                    x: dup_cursor.x.try_into().unwrap(),
                    y: dup_cursor.y.try_into().unwrap(),
                };
                if !project.is_cursor_at(&(dup_ucoord, cursor.1))? {
                    project.toggle_cursor_at(&(dup_ucoord, cursor.1))?;
                }
            }
        }
        Ok(())
    }
}
