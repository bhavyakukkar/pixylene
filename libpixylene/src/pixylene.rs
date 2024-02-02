use crate::grammar::Decorate;
use crate::{
    common::{ Coord, Pixel, BlendMode },
    elements::{
        layer::{ Scene, SceneError, Camera, CameraPixel, Layer },
        palette::Palette,
    },
};
use crate::file::{
    png_file::{ PngFile, PngFileError },
    project_file::{ ProjectFile, ProjectFileError }
};
use crate::project::{ Project, Cursor };
use crate::action::{ Action, action_manager::{ ActionManagerError, ActionManager } };

use std::collections::HashMap;

pub trait PixyleneDisplay {
    fn display(&mut self);
}

#[derive(Debug)]
pub enum PixyleneError {
    SceneError(SceneError),
    ActionManagerError(ActionManagerError),
    ProjectFileError(ProjectFileError),
    PngFileError(PngFileError),
    NoLayersToExport,
}

impl std::fmt::Display for PixyleneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PixyleneError::*;
        match self {
            SceneError(scene_error) => write!(f, "{}", scene_error),
            ActionManagerError(action_manager_error) => write!(f, "{}", action_manager_error),
            ProjectFileError(project_file_error) => write!(f, "{}", project_file_error),
            PngFileError(png_file_error) => write!(f, "{}", png_file_error),
            NoLayersToExport => write!(f, "cannot export with 0 layers in the project"),
        }
    }
}

impl From<SceneError> for PixyleneError {
    fn from(item: SceneError) -> PixyleneError {
        use SceneError::{ InvalidDimensions };
        if let InvalidDimensions(_) = item {
            return PixyleneError::SceneError(item);
        } else {
            panic!("Pixylene methods should never be able to achieve these scene errors.")
        }
    }
}
impl From<PngFileError> for PixyleneError {
    fn from(item: PngFileError) -> PixyleneError { PixyleneError::PngFileError(item) }
}

pub struct PixyleneNewDefaults {
    pub dim: Coord,
    pub camera_dim: Coord,
    pub camera_repeat: Coord,
    pub palette: Palette,
}

pub struct PixyleneImportDefaults {
    pub camera_dim: Coord,
    pub palette: Palette,
}

pub struct Pixylene {
    pub project: Project,
    pub action_manager: ActionManager,
}
impl Pixylene {
    pub fn new(defaults: &PixyleneNewDefaults) -> Result<Pixylene, PixyleneError> {
        let project = Project::new(
            defaults.dim,
            vec![Layer {
                scene: Scene::new(
                    defaults.dim,
                    vec![None; defaults.dim.area().try_into().unwrap()],
                )?,
                opacity: 255,
                mute: false,
            }, Layer {
                scene: Scene::new(
                    defaults.dim,
                    vec![None; defaults.dim.area().try_into().unwrap()],
                )?,
                opacity: 255,
                mute: false,
            }],
            vec![Cursor {
                layer: 0,
                coord: Coord {
                    x: defaults.dim.x.checked_div(2).unwrap(),
                    y: defaults.dim.y.checked_div(2).unwrap(),
                },
            }],
            Camera::new(
                defaults.camera_dim,
                1,
                defaults.camera_repeat,
            ).unwrap(),
            Cursor {
                layer: 0,
                coord: Coord {
                    x: defaults.dim.x.checked_div(2).unwrap(),
                    y: defaults.dim.y.checked_div(2).unwrap(),
                },
            },
            defaults.palette.clone(),
        ).unwrap();

        Ok(Pixylene {
            project: project,
            action_manager: ActionManager::new(HashMap::new()),
        })
    }
    pub fn open(path: &str) -> Result<Pixylene, PixyleneError> {
        match (ProjectFile{ version: 0 }).read(path.to_string()) {
            Ok(project) => Ok(Pixylene {
                project: project,
                action_manager: ActionManager::new(HashMap::new()),
            }),
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
        let mut png_file = PngFile::read(String::from(path)).unwrap();
        let mut scene = png_file.to_scene()?;
        let mut scene2 = Scene::new(scene.dim(), vec![None; scene.dim().area().try_into().unwrap()])?;
        let dimensions = scene.dim();
        let mut project = Project::new(
            scene.dim(),
            vec![Layer {
                scene: scene,
                opacity: 255,
                mute: false,
            }, Layer {
                scene: scene2,
                opacity: 255,
                mute: false,
            }],
            vec![Cursor {
                layer: 0,
                coord: Coord {
                    x: dimensions.x.checked_div(2).unwrap(),
                    y: dimensions.y.checked_div(2).unwrap(),
                },
            }],
            Camera::new(
                defaults.camera_dim,
                //Coord { x: 36, y: 72 }, //todo: dont use default
                1,
                Coord{ x: 1, y: 2 }
            ).unwrap(),
            Cursor {
                layer: 0,
                coord: Coord {
                    x: dimensions.x.checked_div(2).unwrap(),
                    y: dimensions.y.checked_div(2).unwrap(),
                },
            },
            defaults.palette.clone(),
        ).unwrap();

        Ok(Pixylene {
            project: project,
            action_manager: ActionManager::new(HashMap::new()),
        })
    }
    pub fn export(&self, path: &str) -> Result<(), PixyleneError> {
        use PixyleneError::{ NoLayersToExport, PngFileError };
        let merged_layer: Layer;

        PngFile::from_scene(
            &self.project.merged_scene(),
            //todo: use from Pixylene struct instead of defaults
            png::ColorType::Rgba,
            png::BitDepth::Eight,
        )?
            .write(path.to_string())?;
        Ok(())
    }
    pub fn add_action(&mut self, action_name: &str, action: Box<dyn Action>) {
        self.action_manager.actions.insert(action_name.to_string(), action);
    }
    pub fn perform(&mut self, action_name: &str) -> Result<(), PixyleneError> {
        match self.action_manager.perform(&mut self.project, action_name.to_string()) {
            Ok(_) => Ok(()),
            Err(error) => Err(PixyleneError::ActionManagerError(error)),
        }
    }
    pub fn render(&self) -> Vec<CameraPixel> {
        self.project.render()
    }
    pub fn undo(&mut self) -> Result<(), PixyleneError> {
        match self.action_manager.undo(&mut self.project) {
            Ok(()) => Ok(()),
            Err(error) => Err(PixyleneError::ActionManagerError(error)),
        }
    }
    pub fn redo(&mut self) -> Result<(), PixyleneError> {
        match self.action_manager.redo(&mut self.project) {
            Ok(()) => Ok(()),
            Err(error) => Err(PixyleneError::ActionManagerError(error)),
        }
    }
}
