use crate::grammar::Decorate;
use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::elements::layer::{ Camera, Layer };
use crate::elements::Palette;
use crate::file::{
    png_file::{ PngFile, PngFileError },
    project_file::{ ProjectFile, ProjectFileError }
};
use crate::project::Project;
use crate::action::Action;
use crate::action::action_manager::{ ActionManagerError, ActionManager };

use std::collections::HashMap;

pub trait PixyleneDisplay {
    fn display(&mut self);
}

#[derive(Debug)]
pub enum PixyleneError {
    ActionManagerError(ActionManagerError),
    ProjectFileError(ProjectFileError),
    PngFileError(PngFileError),
}
impl std::fmt::Display for PixyleneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PixyleneError::*;
        match self {
            ActionManagerError(error) => write!(f, "{}", Decorate::output(
                "PixyleneError".to_string(),
                None,
                Some(error.to_string()),
            )),
            ProjectFileError(error) => write!(f, "{}", Decorate::output(
                "PixyleneError".to_string(),
                None,
                Some(error.to_string()),
            )),
            PngFileError(error) => write!(f, "{}", Decorate::output(
                "PixyleneError".to_string(),
                None,
                Some(error.to_string()),
            )),
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
            &scene,
            Coord{ x: 36, y: 72 },
            Coord{ x: 8, y: 8 },
            1,
            Coord{ x: 1, y: 2 }
            ).unwrap();
        let mut project = Project {
            layers: vec![Layer {
                scene: scene,
                opacity: 255
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
        let mut layer_refs: Vec<&Layer> = Vec::new();
        for layer in &self.project.layers {
            layer_refs.push(layer);
        }
        match PngFile::from_scene(
            //todo: remove unwrap when layer error literature
            &Layer::merge(layer_refs, BlendMode::Normal).unwrap().scene,
            png::ColorType::Rgba,
            png::BitDepth::Eight,
        ) {
            Ok(png_file) => match png_file.write(path.to_string()) {
                Ok(()) => Ok(()),
                Err(error) => Err(PixyleneError::PngFileError(error)),
            },
            Err(error) => Err(PixyleneError::PngFileError(error)),
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

