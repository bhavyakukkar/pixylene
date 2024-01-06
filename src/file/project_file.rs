use savefile::prelude::*;

use crate::grammar::Decorate;
use crate::project::Project;

#[derive(Debug)]
pub enum ProjectFileError {
    LoadingError(String, SavefileError),
    SavingError(String, SavefileError),
}
impl std::fmt::Display for ProjectFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ProjectFileError::*;
        match self {
            LoadingError(path, savefile_error) => write!(
                f,
                "failed to load project file '{}': {}",
                path,
                savefile_error,
            ),
            SavingError(path, savefile_error) => write!(
                f,
                "failed to save project file '{}': {}",
                path,
                savefile_error,
            ),
        }
    }
}

pub struct ProjectFile {
    pub version: u32,
}
impl ProjectFile {
    pub fn read(&self, path: String) -> Result<Project, ProjectFileError> {
        use ProjectFileError::{ LoadingError };
        match load_file(path.clone(), self.version) {
            Ok(project) => Ok(project),
            Err(error) => Err(LoadingError(path, error)),
        }
    }
    pub fn write(&self, path: String, project: &Project) -> Result<(), ProjectFileError> {
        use ProjectFileError::{ SavingError };
        match save_file(path.clone(), self.version, project) {
            Ok(()) => Ok(()),
            Err(error) => Err(SavingError(path, error)),
        }
    }
}
