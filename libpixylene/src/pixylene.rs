use crate::{
    file::{CanvasFile, CanvasFileError, PngFile, PngFileError, ProjectFile, ProjectFileError},
    project::{Canvas, Layers, LayersType, Palette, Project, SceneError},
    types::{IndexedPixel, PCoord, TruePixel},
};
use std::path::PathBuf;

#[derive(Clone)]
pub struct PixyleneDefaults {
    pub dim: PCoord,
    pub palette: Palette,
    pub repeat: PCoord,
}

pub struct Pixylene {
    pub project: Project,
}

impl Pixylene {
    /// Creates a new empty Project containing an empty true-color Canvas if `indexed` is false or
    /// an empty indexed-color Canvas if `indexed` is true.
    pub fn new(defaults: &PixyleneDefaults, indexed: bool) -> Self {
        let mut project = Project::new(Canvas {
            palette: defaults.palette.clone(),
            layers: if indexed {
                LayersType::Indexed(Layers::<IndexedPixel>::new(defaults.dim))
            } else {
                LayersType::True(Layers::<TruePixel>::new(defaults.dim))
            },
        });
        project.out_repeat = defaults.repeat;
        Self { project }
    }

    // To/Fro Canvas File
    pub fn open_canvas(path: &PathBuf, defaults: &PixyleneDefaults) -> Result<Self, PixyleneError> {
        CanvasFile::read(path)
            .map(|canvas| {
                let mut project = Project::new(canvas);
                project.out_repeat = defaults.repeat;
                Self { project }
            })
            .map_err(|error| PixyleneError::CanvasFileError(error))
    }
    pub fn save_canvas(&self, path: &PathBuf) -> Result<(), PixyleneError> {
        CanvasFile::write(path, &self.project.canvas)
            .map_err(|err| PixyleneError::CanvasFileError(err))
    }

    //To/Fro Project File
    pub fn open_project(path: &PathBuf) -> Result<Self, PixyleneError> {
        match (ProjectFile { version: 0 }).read(path) {
            Ok(project) => Ok(Pixylene { project }),
            Err(error) => Err(PixyleneError::ProjectFileError(error)),
        }
    }
    pub fn save_project(&self, path: &PathBuf) -> Result<(), PixyleneError> {
        (ProjectFile { version: 0 })
            .write(path, &self.project)
            .map_err(|err| PixyleneError::ProjectFileError(err))
    }

    //To/Fro PNG File
    pub fn import(
        path: &PathBuf,
        resize: Option<PCoord<u32>>,
        defaults: &PixyleneDefaults,
    ) -> Result<Pixylene, PixyleneError> {
        let mut png = PngFile::read(path)?;
        if let Some(resize) = resize {
            png.resize(resize)?;
        }
        let mut project = Project::new(png.to_canvas()?);
        if matches!(project.canvas.layers, LayersType::True(_)) {
            project.canvas.palette = defaults.palette.clone();
        }
        project.out_repeat = defaults.repeat;

        Ok(Pixylene { project })
    }

    pub fn export(&self, resize: Option<PCoord<u32>>, path: &PathBuf) -> Result<(), PixyleneError> {
        let mut png = PngFile::from_canvas(&self.project.canvas)?;
        if let Some(resize) = resize {
            png.resize(resize)?;
        }
        png.write(path)?;
        Ok(())
    }
}

// Error Types

#[derive(Debug)]
pub enum PixyleneError {
    SceneError(SceneError),
    ProjectFileError(ProjectFileError),
    CanvasFileError(CanvasFileError),
    PngFileError(PngFileError),
}

impl std::fmt::Display for PixyleneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PixyleneError::*;
        match self {
            SceneError(scene_error) => write!(f, "{}", scene_error),
            ProjectFileError(project_file_error) => write!(f, "{}", project_file_error),
            CanvasFileError(canvas_file_error) => write!(f, "{}", canvas_file_error),
            PngFileError(png_file_error) => write!(f, "{}", png_file_error),
        }
    }
}

impl From<SceneError> for PixyleneError {
    fn from(item: SceneError) -> PixyleneError {
        PixyleneError::SceneError(item)
    }
}

impl From<PngFileError> for PixyleneError {
    fn from(item: PngFileError) -> PixyleneError {
        PixyleneError::PngFileError(item)
    }
}
