use savefile::prelude::*;

use crate::project::Project;

pub struct Save {
    pub version: u32,
}
impl Save {
    pub fn read(&self, path: String) -> Result<Project, String> {
        //version 0
        match load_file(path, self.version) {
            Ok(project) => Ok(project),
            Err(error) => Err(format!("error loading project: {}", error)),
        }
    }
    pub fn write(&self, path: String, project: &Project) -> Result<(), String> {
        //version 0
        match save_file(path, self.version, project) {
            Ok(()) => Ok(()),
            Err(error) => Err(format!("error saving project: {}", error)),
        }
    }
}
