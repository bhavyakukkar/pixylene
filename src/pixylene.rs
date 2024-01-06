use crate::grammar::Decorate;
use crate::elements::{
    common::{ Coord, Pixel, BlendMode },
    layer::{ Scene, Camera, CameraPixel, Layer },
    palette::Palette,
};
use crate::file::{
    png_file::{ PngFile, PngFileError },
    project_file::{ ProjectFile, ProjectFileError }
};
use crate::project::Project;
use crate::action::{ Action, action_manager::{ ActionManagerError, ActionManager } };

use std::collections::HashMap;

pub trait PixyleneDisplay {
    fn display(&mut self);
}

#[derive(Debug)]
pub enum PixyleneError {
    ActionManagerError(ActionManagerError),
    ProjectFileError(ProjectFileError),
    PngFileError(PngFileError),
    NoLayersToExport,
}
//todo: fix
impl std::fmt::Display for PixyleneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PixyleneError::*;
        match self {
            ActionManagerError(action_manager_error) => write!(f, "{}", action_manager_error),
            ProjectFileError(project_file_error) => write!(f, "{}", project_file_error),
            PngFileError(png_file_error) => write!(f, "{}", png_file_error),
            NoLayersToExport => write!(f, "cannot export with 0 layers in the project"),
        }
    }
}

pub struct Pixylene {
    pub project: Project,
    pub action_manager: ActionManager,
    //defaults: Defaults,
}
impl Pixylene {
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
    pub fn import(path: &str) -> Result<Pixylene, PixyleneError> {
        let mut png_file = PngFile::read(String::from(path)).unwrap();
        let mut scene = png_file.to_scene().unwrap();
        let mut camera = Camera::new(
            Coord { x: 36, y: 72 }, //todo: dont use defalt
            scene.dim(),
            Coord {
                x: scene.dim().x.checked_div(2).unwrap(),
                y: scene.dim().y.checked_div(2).unwrap()
            },
            1,
            Coord{ x: 1, y: 2 }
        ).unwrap();
        let mut project = Project {
            dimensions: scene.dim(),
            layers: vec![Layer {
                scene: scene,
                opacity: 255,
                mute: false,
            }],
            selected_layer: 0,
            camera: camera,
            palette: Palette { colors: vec![
                Some(Pixel{r: 81, g: 87, b: 109, a: 255}),
                Some(Pixel{r: 231, g: 130, b: 132, a: 255}),
                Some(Pixel{r: 166, g: 209, b: 137, a: 255}),
                Some(Pixel{r: 229, g: 200, b: 144, a: 255}),
                Some(Pixel{r: 140, g: 170, b: 238, a: 255}),
                Some(Pixel{r: 244, g: 184, b: 228, a: 255}),
                Some(Pixel{r: 129, g: 200, b: 190, a: 255}),
                Some(Pixel{r: 181, g: 191, b: 226, a: 255}),
            ] },
        };

        Ok(Pixylene {
            project: project,
            action_manager: ActionManager::new(HashMap::new()),
        })
    }
    pub fn export(&self, path: &str) -> Result<(), PixyleneError> {
        use PixyleneError::{ NoLayersToExport, PngFileError };
        let merged_layer: Layer;

        match PngFile::from_scene(
            &self.project.merged_scene(),
            //todo: use from Pixylene struct instead of defaults
            png::ColorType::Rgba,
            png::BitDepth::Eight,
        ) {
            Ok(png_file) => match png_file.write(path.to_string()) {
                Ok(()) => Ok(()),
                Err(png_file_error) => Err(PngFileError(png_file_error)),
            },
            Err(png_file_error) => Err(PngFileError(png_file_error)),
        }
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

