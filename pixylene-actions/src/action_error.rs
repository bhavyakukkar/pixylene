use crate::command::ChangeError;

use libpixylene::{
    project::{LayersError, PaletteError, ProjectError, SceneError},
    types::{BlendError, TruePixelError},
};

#[derive(Debug)]
pub enum ActionError {
    TruePixelError(TruePixelError),
    SceneError(SceneError),
    PaletteError(PaletteError),
    ChangeError(ChangeError),
    ProjectError(ProjectError),
    LayersError(LayersError),
    BlendError(BlendError),
    OnlyNCursorsSupported(String, usize),

    // Custom Errors
    ExpectingCanvasType { expecting_indexed: bool },
    InvalidCanvasType { expecting_indexed: bool },
    ArgsError(String),
    InputError(String),
    OperationError(Option<String>),
    Discarded,
}
impl From<TruePixelError> for ActionError {
    fn from(item: TruePixelError) -> ActionError {
        ActionError::TruePixelError(item)
    }
}
impl From<SceneError> for ActionError {
    fn from(item: SceneError) -> ActionError {
        ActionError::SceneError(item)
    }
}
impl From<PaletteError> for ActionError {
    fn from(item: PaletteError) -> ActionError {
        ActionError::PaletteError(item)
    }
}
impl From<ChangeError> for ActionError {
    fn from(item: ChangeError) -> ActionError {
        ActionError::ChangeError(item)
    }
}
impl From<ProjectError> for ActionError {
    fn from(item: ProjectError) -> ActionError {
        ActionError::ProjectError(item)
    }
}
impl From<LayersError> for ActionError {
    fn from(item: LayersError) -> ActionError {
        ActionError::LayersError(item)
    }
}
impl From<BlendError> for ActionError {
    fn from(item: BlendError) -> ActionError {
        ActionError::BlendError(item)
    }
}
impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ActionError::*;
        match self {
            TruePixelError(pixel_error) => write!(f, "{}", pixel_error),
            SceneError(scene_error) => write!(f, "{}", scene_error),
            PaletteError(palette_error) => write!(f, "{}", palette_error),
            ChangeError(change_error) => write!(f, "{}", change_error),
            ProjectError(project_error) => write!(f, "{}", project_error),
            LayersError(layers_error) => write!(f, "{}", layers_error),
            BlendError(blend_error) => write!(f, "{}", blend_error),
            OnlyNCursorsSupported(supported, supplied) => write!(
                f,
                "this action only supports {} cursor/s, found {}",
                supported, supplied,
            ),
            ExpectingCanvasType { expecting_indexed } => write!(
                f,
                "{}",
                if *expecting_indexed {
                    "This action was expecting arguments pertaining to indexed-color but found \
                    those pertaining to true-color"
                } else {
                    "This action was expecting arguments pertaining to true-color but found those \
                    pertaining to indexed-color"
                },
            ),
            InvalidCanvasType { expecting_indexed } => write!(
                f,
                "{}",
                if *expecting_indexed {
                    "This action was inferred to operate using indexed-color but the canvas is \
                    true-color"
                } else {
                    "This action was inferred to operate using true-color but the canvas is \
                    indexed-color"
                },
            ),
            ArgsError(desc) => write!(f, "{}", desc),
            InputError(desc) => write!(f, "{}", desc),
            OperationError(desc) => write!(f, "{}", desc.clone().unwrap_or(String::new())),
            Discarded => write!(f, "this action was discarded by the user"),
        }
    }
}
