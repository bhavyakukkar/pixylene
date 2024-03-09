use libpixylene::{ project::Project };
use crate::{ Action, Console, helper::Result };

struct MoveCursorsUp;
impl Action for MoveCursorsUp {
    fn perform_action(&mut self, project: &mut Project, console: &Console) -> Result {
        Ok(Vec::new())
    }
}
