//let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

pub type ContextExpired<T> = Result<T, ()>;
pub const LAYER_GONE: &str = "cannot use methods on this Layer or its children (Scene, Opacity, \
                              Mute, BlendMode) since its context no longer exists on the project";

pub type CanvasMismatch<T> = Result<T, ()>;
pub const CANVAS_MISMATCH_TRUE: &str =
    "cannot make use of Indexed type where True type is required";
pub const CANVAS_MISMATCH_INDEXED: &str =
    "cannot make use of True type where Indexed type is required";

pub static BOXED_ERROR: fn(&str) -> Box<dyn std::error::Error + Send + Sync> =
    |s: &str| Box::from(s);
