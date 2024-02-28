use crate::{
    types::{ Coord, PCoord },
    project::{ SceneError, Camera, CameraPixel, Palette, Project },
    file::{
        png_file::{ PngFile, PngFileError },
        project_file::{ ProjectFile, ProjectFileError }
    },
};


pub struct PixyleneNewDefaults {
    pub dim: PCoord,
    pub camera_dim: PCoord,
    pub camera_repeat: (u8, u8),
    pub palette: Palette,
}

pub struct PixyleneImportDefaults {
    pub camera_dim: PCoord,
    pub camera_repeat: (u8, u8),
    pub palette: Palette,
}

pub struct Pixylene {
    pub project: Project,
}

impl Pixylene {
    pub fn new(defaults: &PixyleneNewDefaults) -> Result<Pixylene, PixyleneError> {
        let project = Project::new(
            defaults.dim,
            /*
            vec![Layer {
                scene: Scene::new(
                    defaults.dim,
                    vec![None; defaults.dim.area()],
                )?,
                opacity: 255,
                mute: false,
            }],
            */
            /*
            vec![Cursor {
                layer: 0,
                coord: Coord {
                    x: defaults.dim.x.checked_div(2).unwrap(),
                    y: defaults.dim.y.checked_div(2).unwrap(),
                },
            }],
            */
            Camera::new(
                defaults.camera_dim,
                1,
                defaults.camera_repeat,
            ).unwrap(),
            //default focus is at the middle of the canvas dimensions
            (Coord {
                x: i32::from(defaults.dim.x().checked_div(2).unwrap()),
                y: i32::from(defaults.dim.y().checked_div(2).unwrap()),
            }, 0),
            defaults.palette.clone(),
        ).unwrap();

        Ok(Pixylene { project })
    }
    pub fn open(path: &str) -> Result<Pixylene, PixyleneError> {
        match (ProjectFile{ version: 0 }).read(path.to_string()) {
            Ok(project) => Ok(Pixylene { project }),
            Err(error) => Err(PixyleneError::ProjectFileError(error)),
        }
    }
    pub fn save(&self, path: &str) -> Result<(), PixyleneError> {
        match (ProjectFile{ version: 0 }).write(path.to_string(), &self.project) {
            Ok(()) => Ok(()),
            Err(error) => Err(PixyleneError::ProjectFileError(error)),
        }
    }
    pub fn import(path: &str, defaults: &PixyleneImportDefaults) -> Result<Pixylene, PixyleneError> {
        let png_file = PngFile::read(String::from(path)).unwrap();
        let scene = png_file.to_scene()?;
        let dim = scene.dim();
        //todo: add imported scene into project
        let project = Project::new(
            dim,
            Camera::new(
                defaults.camera_dim,
                1,
                defaults.camera_repeat,
            ).unwrap(),
            (Coord {
                x: i32::from(dim.x().checked_div(2).unwrap()),
                y: i32::from(dim.y().checked_div(2).unwrap()),
            }, 0),
            defaults.palette.clone(),
        ).unwrap();

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
    pub fn render(&self) -> Vec<CameraPixel> {
        self.project.render()
    }
}


// Error Types

#[derive(Debug)]
pub enum PixyleneError {
    SceneError(SceneError),
    ProjectFileError(ProjectFileError),
    PngFileError(PngFileError),
}

impl std::fmt::Display for PixyleneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PixyleneError::*;
        match self {
            SceneError(scene_error) => write!(f, "{}", scene_error),
            ProjectFileError(project_file_error) => write!(f, "{}", project_file_error),
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
    fn from(item: PngFileError) -> PixyleneError { PixyleneError::PngFileError(item) }
}
