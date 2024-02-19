use crate::{
    types::{ Pixel, BlendMode },
    project::{ Project },
    action::{ Action, ActionError, Change, actions::draw_at_one_cursor::DrawAtOneCursor },
};

use std::rc::Rc;
use std::cell::RefCell;


/*
 * Pencil
 * An action that extends DrawAtOneCursor to dynamically use the project's color at a specificed
 * palette index and blend it with the existing color at each cursor
 *
 */

pub struct Pencil {
    pub palette_index: usize,
}
impl Action for Pencil {
    fn perform_action(&mut self, project: &mut Project) -> Result<Vec<Change>, ActionError> {
        let mut changes: Vec<Change> = vec![Change::Start];
        for index in 0..project.cursors.len() {
            if let Ok(draw_at_one_cursor) = (DrawAtOneCursor {
                cursor: project.cursors[index].clone(),
                color: project.palette.get_color((&self).palette_index)?,
                blend_mode: BlendMode::Normal,
            }).perform_action(project) {
                for change in draw_at_one_cursor {
                    changes.push(change.as_untracked()?);
                }
            }
        }
        changes.push(Change::End);
        Ok(changes)
    }
    fn locks_scene(&self) -> bool { true }
}
