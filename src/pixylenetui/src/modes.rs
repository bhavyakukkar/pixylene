use libpixylene::types::Pixel;

#[derive(Clone, Copy)]
pub enum VimMode {
    Splash,
    Command,
    Normal,
    Preview,
    GridSelect,
    PointSelect,
}

pub enum EmacsMode {
    Normal,
    Layer,
    Command,
    Ooze{color: Pixel},
    Shape{shape: String},
    Eraser{shape: String},
}
