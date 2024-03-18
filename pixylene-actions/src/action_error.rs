use crate::{ command::ChangeError };

use libpixylene::{
    types::{ BlendError },
    project::{ SceneError, PaletteError, ProjectError, CanvasError }
};



#[derive(Debug)]
pub enum ActionError {
    SceneError(SceneError),
    PaletteError(PaletteError),
    ChangeError(ChangeError),
    ProjectError(ProjectError),
    CanvasError(CanvasError),
    BlendError(BlendError),
    OnlyNCursorsSupported(String, usize),

    // Custom Errors
    ArgsError(String),
    OperationError(Option<String>),
}
impl From<SceneError> for ActionError {
    fn from(item: SceneError) -> ActionError { ActionError::SceneError(item) }
}
impl From<PaletteError> for ActionError {
    fn from(item: PaletteError) -> ActionError { ActionError::PaletteError(item) }
}
impl From<ChangeError> for ActionError {
    fn from(item: ChangeError) -> ActionError { ActionError::ChangeError(item) }
}
impl From<ProjectError> for ActionError {
    fn from(item: ProjectError) -> ActionError { ActionError::ProjectError(item) }
}
impl From<CanvasError> for ActionError {
    fn from(item: CanvasError) -> ActionError { ActionError::CanvasError(item) }
}
impl From<BlendError> for ActionError {
    fn from(item: BlendError) -> ActionError { ActionError::BlendError(item) }
}
impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ActionError::*;
        match self {
            SceneError(scene_error) => write!(f, "{}", scene_error),
            PaletteError(palette_error) => write!(f, "{}", palette_error),
            ChangeError(change_error) => write!(f, "{}", change_error),
            ProjectError(project_error) => write!(f, "{}", project_error),
            CanvasError(canvas_error) => write!(f, "{}", canvas_error),
            BlendError(blend_error) => write!(f, "{}", blend_error),
            OnlyNCursorsSupported(supported, supplied) => write!(
                f,
                "this action only supports {} cursor/s, found {}",
                supported,
                supplied,
            ),
            ArgsError(desc) => write!(f, "{}", desc),
            OperationError(desc) => write!(f, "{}", desc.clone().unwrap_or(String::new())),
        }
    }
}
