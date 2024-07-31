use crate::{memento, ActionError, Console};

use libpixylene::{project::Project, types::UCoord};

pub struct GoToSingleCursor {
    to_x: Option<u16>,
    to_y: Option<u16>,
}

impl GoToSingleCursor {
    pub fn new(to_x: Option<u16>, to_y: Option<u16>) -> Self {
        Self { to_x, to_y }
    }
}

impl memento::Action for GoToSingleCursor {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        use ActionError::OnlyNCursorsSupported;

        if project.num_cursors() != 1 {
            return Err(OnlyNCursorsSupported(
                "1".to_owned(),
                project.num_cursors() as usize,
            ));
        }
        let dim = project.canvas.layers.dim();
        let old_cursor = project.cursors().next().unwrap().clone();
        project.toggle_cursor_at(&old_cursor).unwrap();
        project
            .toggle_cursor_at(&(
                UCoord {
                    x: self
                        .to_x
                        .map(|new_x| new_x.min(dim.x() - 1))
                        .unwrap_or(old_cursor.0.x),
                    y: self
                        .to_y
                        .map(|new_y| new_y.min(dim.y() - 1))
                        .unwrap_or(old_cursor.0.y),
                },
                old_cursor.1,
            ))
            .unwrap();

        Ok(())
    }
}
