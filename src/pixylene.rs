use crate::grammar::Decorate;
use crate::elements::common::{ Coord, Pixel };
use crate::elements::layer::{ Camera, Layer };
use crate::elements::Palette;
use crate::file::{ png_file::*, project_file::* };
use crate::project::Project;
use crate::action::*;

use std::collections::HashMap;

pub trait PixyleneDisplay {
    fn display(&mut self);
}

#[derive(Debug)]
pub enum PixyleneError {
    ActionManagerError(ActionManagerError),
    ProjectNotOpenError(String),
    ProjectFileError(ProjectFileError),
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
            ProjectNotOpenError(desc) => write!(f, "{}", Decorate::output(
                "PixyleneError".to_string(),
                None,
                Some(desc.to_string()),
            )),
            ProjectFileError(error) => write!(f, "{}", Decorate::output(
                "PixyleneError".to_string(),
                None,
                Some(error.to_string()),
            )),
        }
    }
}

pub struct Pixylene {
    pub project: Project,
    action_manager: ActionManager,
    //defaults: Defaults,
}
impl Pixylene {
    pub fn import(path: &str) -> Result<Pixylene, PixyleneError> {
        let mut png_file = PngFile::open(String::from(path)).unwrap();
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
                Some(Pixel{r: 0, g: 0, b: 0, a: 255}),
                Some(Pixel{r: 127, g: 0, b: 255, a: 255 }),
            ] },
        };

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
    pub fn add_action(&mut self, action_name: &str, action: Box<dyn Action>) {
        self.action_manager.actions.insert(action_name.to_string(), action);
    }
    pub fn perform(&mut self, action_name: &str) -> Result<(), PixyleneError> {
        match self.action_manager.perform(&mut self.project, action_name.to_string()) {
            Ok(_) => Ok(()),
            Err(error) => Err(PixyleneError::ActionManagerError(error)),
        }
    }
}

