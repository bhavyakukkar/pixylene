use savefile::prelude::*;

use crate::grammar::Decorate;
use crate::project::Project;

#[derive(Debug)]
pub enum ProjectFileError {
    LoadingError(SavefileError),
    SavingError(SavefileError),
}
impl std::fmt::Display for ProjectFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ProjectFileError::*;
        match self {
            LoadingError(error) => write!(f, "{}", Decorate::output(
                "ProjectFileError".to_string(),
                None,
                Some(error.to_string()),
            )),
            SavingError(error) => write!(f, "{}", Decorate::output(
                "ProjectFileError".to_string(),
                None,
                Some(error.to_string()),
            )),
        }
    }
}

pub struct ProjectFile {
    pub version: u32,
}
impl ProjectFile {
    pub fn read(&self, path: String) -> Result<Project, ProjectFileError> {
        //version 0
        match load_file(path, self.version) {
            Ok(project) => Ok(project),
            Err(error) => Err(ProjectFileError::LoadingError(error)),
        }
    }
    pub fn write(&self, path: String, project: &Project) -> Result<(), ProjectFileError> {
        //version 0
        match save_file(path, self.version, project) {
            Ok(()) => Ok(()),
            Err(error) => Err(ProjectFileError::SavingError(error)),
        }
    }
}
