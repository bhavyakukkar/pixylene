use crate::layer::Layer;

struct Project {
    layers: Vec<Layer>,
    selected_layer: u8,
    palette: Palette
}

impl Project {
    fn new(layers) -> Result<Project, String> {
        todo!();
    }
}
