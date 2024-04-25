use crate::project::{ Project };

use savefile::prelude::*;
use std::path::PathBuf;


pub struct ProjectFile {
    pub version: u32,
}
impl ProjectFile {
    pub fn read(&self, path: &PathBuf) -> Result<Project, ProjectFileError> {
        use ProjectFileError::{ LoadingError };
        match load_file(path.clone(), self.version) {
            Ok(project) => Ok(project),
            Err(error) => Err(LoadingError(path.clone(), error)),
        }
    }
    pub fn write(&self, path: &PathBuf, project: &Project) -> Result<(), ProjectFileError> {
        use ProjectFileError::{ SavingError };
        match save_file(path.clone(), self.version, project) {
            Ok(()) => Ok(()),
            Err(error) => Err(SavingError(path.clone(), error)),
        }
    }
}


// Error Types

#[derive(Debug)]
pub enum ProjectFileError {
    LoadingError(PathBuf, SavefileError),
    SavingError(PathBuf, SavefileError),
}
impl std::fmt::Display for ProjectFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ProjectFileError::*;
        match self {
            LoadingError(path, savefile_error) => write!(
                f,
                "failed to load project file '{}': {}",
                path.display(),
                savefile_error,
            ),
            SavingError(path, savefile_error) => write!(
                f,
                "failed to save project file '{}': {}",
                path.display(),
                savefile_error,
            ),
        }
    }
}
