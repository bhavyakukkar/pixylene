use crate::{Console, ActionError, memento::self};
use libpixylene::{
    project::{ Project },
};

pub struct Multiplier {
    pub zoom: i8,
}

impl Multiplier {
    pub fn new(zoom: i8) -> Self {
        Self{ zoom }
    }
}


impl memento::Action for Multiplier {
    fn perform(&mut self, project: &mut Project, _console: &dyn Console) -> memento::ActionResult {
        use ActionError::ArgsError;

        project.set_out_mul(
            u8::try_from(i16::from(project.get_out_mul()) + i16::from(self.zoom))
                .map_err(|_|
                    ArgsError(format!("can't zoom {}", if self.zoom > 0 { "in" } else { "out" })))?
        )
        .map_err(|_|
            ArgsError("can't zoom out to 0".to_owned()))?;

        Ok(())
    }
}
