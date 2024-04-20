use crate::project::Canvas;


pub struct CanvasFile;

impl Canvas {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
