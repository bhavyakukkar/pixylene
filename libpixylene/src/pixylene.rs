use crate::{
    file::{PngFile, PngFileError, ProjectFile, ProjectFileError, CanvasFile, CanvasFileError},
    project::{Layer, Palette, TrueCanvas, IndexedCanvas, Project, SceneError},
    types::{BlendMode, PCoord},
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
        let mut project: Project;
        if indexed {
            project = Project::from(
                IndexedCanvas::new(defaults.dim, defaults.palette.clone()));
        } else {
            project = Project::from(
                TrueCanvas::new(defaults.dim, defaults.palette.clone()));
        }

        project.out_repeat = defaults.repeat;
        Self { project }
    }

    // To/Fro Canvas File
    pub fn open_canvas(path: &PathBuf, defaults: &PixyleneDefaults) -> Result<Pixylene, PixyleneError> {
        CanvasFile::read(path)
            .map(|canvas| {
                let mut p = Project::from(canvas);
                p.out_repeat = defaults.repeat;
                Pixylene { project: p }
            })
            .map_err(|error| PixyleneError::CanvasFileError(error))
    }
    pub fn save_canvas(&self, path: &PathBuf) -> Result<(), PixyleneError> {
        CanvasFile::write(path, self.project.canvas())
            .map_err(|err| PixyleneError::CanvasFileError(err))
    }

    //To/Fro Project File
    pub fn open_project(path: &PathBuf) -> Result<Pixylene, PixyleneError> {
        match (ProjectFile { version: 0 }).read(path) {
            Ok(project) => Ok(Pixylene { project }),
            Err(error) => Err(PixyleneError::ProjectFileError(error)),
        }
    }
    pub fn save_project(&self, path: &PathBuf) -> Result<(), PixyleneError> {
        (ProjectFile { version: 0 }).write(path, &self.project)
            .map_err(|err| PixyleneError::ProjectFileError(err))
    }

    //To/Fro PNG File
    pub fn import(path: &PathBuf, defaults: &PixyleneDefaults) -> Result<Pixylene, PixyleneError> {
        let png_file = PngFile::read(path)?;
        let scene = png_file.to_scene()?;
        let mut canvas = TrueCanvas::new(scene.dim(), defaults.palette.clone());
        canvas.layers_mut()
            .add_layer(Layer {
                scene,
                opacity: 255,
                mute: false,
                blend_mode: BlendMode::Normal,
            })
            .unwrap(); //cant fail, this is first layer, not 257th

        let mut project = Project::from(canvas);
        project.out_repeat = defaults.repeat;

        Ok(Pixylene { project })
    }

    pub fn export(&self, path: &PathBuf, scale_up: u16) -> Result<(), PixyleneError> {
        PngFile::from_scene(
            &self.project.canvas().inner().merged_scene(None),
            //todo: use from Pixylene struct instead of defaults
            png::ColorType::Rgba,
            png::BitDepth::Eight,
            scale_up,
        )?
        .write(path)?;
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
