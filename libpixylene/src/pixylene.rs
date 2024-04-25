use crate::{
    file::{PngFile, PngFileError, ProjectFile, ProjectFileError, CanvasFile, CanvasFileError},
    project::{Canvas, Layer, Palette, Project, SceneError},
    types::{BlendMode, PCoord},
};
use std::path::PathBuf;


pub struct PixyleneDefaults {
    pub dim: PCoord,
    pub palette: Palette,
    pub repeat: PCoord,
}

pub struct Pixylene {
    pub project: Project,
}

impl Pixylene {
    //From new
    pub fn new(defaults: &PixyleneDefaults) -> Pixylene {
        Pixylene {
            project: Project::new(Canvas::new(defaults.dim, defaults.palette.clone()),
                defaults.repeat)
        }
    }

    //To/Fro Canvas File
    pub fn open_canvas(path: &PathBuf, defaults: &PixyleneDefaults) -> Result<Pixylene, PixyleneError> {
        CanvasFile::read(path)
            .map(|canvas| Pixylene { project: Project::new(canvas, defaults.repeat) })
            .map_err(|error| PixyleneError::CanvasFileError(error))
    }
    pub fn save_canvas(&self, path: &PathBuf) -> Result<(), PixyleneError> {
        CanvasFile::write(path, &self.project.canvas)
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
    pub fn import(path: &str, defaults: &PixyleneDefaults) -> Result<Pixylene, PixyleneError> {
        let png_file = PngFile::read(String::from(path))?;
        let scene = png_file.to_scene()?;
        let dim = scene.dim();
        let mut project = Project::new(Canvas::new(dim, defaults.palette.clone()), defaults.repeat);
        project
            .canvas
            .add_layer(Layer {
                scene,
                opacity: 255,
                mute: false,
                blend_mode: BlendMode::Normal,
            })
            .unwrap(); //cant fail, this is first layer, not 257th

        Ok(Pixylene { project })
    }
    pub fn export(&self, path: &str, scale_up: u16) -> Result<(), PixyleneError> {
        PngFile::from_scene(
            &self.project.canvas.merged_scene(None),
            //todo: use from Pixylene struct instead of defaults
            png::ColorType::Rgba,
            png::BitDepth::Eight,
            scale_up,
        )?
        .write(path.to_string())?;
        Ok(())
    }
    /*
    pub fn render(&self) -> Vec<OPixel> {
        self.project.render()
    }
    */
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
