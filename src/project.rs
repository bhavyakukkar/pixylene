use crate::elements::{
    common::{ Coord, Pixel, BlendMode },
    palette::Palette,
    layer::{ Scene, Camera, CameraPixel, Layer }
};

#[derive(Debug)]
pub enum ProjectError {
    NonNaturalDimensions(Coord),
    CursorIndexOutOfBounds(usize, usize),
    CursorCoordOutOfBounds(usize, Cursor, Coord),
    LayerOutOfBounds(usize, usize),
    LayerDimensionsMismatch(usize, Coord, Coord),
    MultipleCursorsUnderDevelopment,
}
impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ProjectError::*;
        match self {
            NonNaturalDimensions(dim) => write!(
                f,
                "cannot use non-natural project dimensions, found {}",
                dim,
            ),
            CursorIndexOutOfBounds(index, cursors_len) => write!(
                f,
                "index {} is out of bounds for cursors of length {} in the project",
                index,
                cursors_len,
            ),
            CursorCoordOutOfBounds(index, cursor, dim) => write!(
                f,
                "cannot set cursor at index {} of cursors to layer {} and coordinate {} since \
                project dimensions are {}, valid coordinates for this project lie between {} and \
                {} (inclusive)",
                index,
                cursor.layer,
                cursor.coord,
                dim,
                Coord{ x: 0, y: 0 },
                dim.add(Coord{ x: -1, y: -1 }),
            ),
            LayerOutOfBounds(layer, layers_len) => write!(
                f,
                "layer index {} is out of bounds for the {} layers present in the project",
                layer,
                layers_len,
            ),
            LayerDimensionsMismatch(index, layer_dim, project_dim) => write!(
                f,
                "cannot include layer at index {} of layers since its dimensions {} are not equal \
                to given project dimensions {}",
                index,
                layer_dim,
                project_dim,
            ),
            MultipleCursorsUnderDevelopment => write!(
                f,
                "multiple cursors are still currently under development",
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Savefile)]
pub struct Cursor {
    pub layer: usize,
    pub coord: Coord,
}

pub struct ProjectPixel {
    pub camera_pixel: CameraPixel,
    pub has_cursor: bool,
}

#[derive(Savefile)]
pub struct Project {
    pub dimensions: Coord,
    pub layers: Vec<Layer>,
    pub cursors: Vec<Cursor>,
    pub camera: Camera,
    pub focus: Cursor,
    pub palette: Palette,
}

impl Project {
    pub fn new(
        dimensions: Coord,
        layers: Vec<Layer>,
        cursors: Vec<Cursor>,
        camera: Camera,
        focus: Cursor,
        palette: Palette
    ) -> Result<Project, ProjectError> {
        use ProjectError::{ NonNaturalDimensions, LayerDimensionsMismatch, MultipleCursorsUnderDevelopment };
        if dimensions.x < 0 || dimensions.y < 0 {
            return Err(NonNaturalDimensions(dimensions));
        }
        for (index, layer) in layers.iter().enumerate() {
            let layer_dim = layer.scene.dim();
            if layer_dim.x != dimensions.x || layer_dim.y != dimensions.y {
                return Err(LayerDimensionsMismatch(index, layer_dim, dimensions));
            }
        }
        /*
        if cursors.len() > 1 {
            return Err(MultipleCursorsUnderDevelopment);
        }
        */
        let mut project = Project {
            dimensions: dimensions,
            layers: layers,
            cursors: vec![Cursor { layer: 0, coord: Coord { x: 0, y: 0 }}; cursors.len()],
            camera: camera,
            focus: focus,
            palette: palette,
        };
        for (index, cursor) in cursors.iter().enumerate() { project.set_cursor(index, *cursor)?; }
        Ok(project)
    }
    pub fn get_num_layers(&self) -> usize {
        self.layers.len()
    }
    pub fn merged_scene(&self) -> Scene {
        let mut net_layer = Layer::new_with_solid_color(self.dimensions, Some(Pixel::background()));
        for k in 0..self.layers.len() {
            if self.layers[k].mute { continue; }
            net_layer = Layer {
                scene: Layer::merge(
                    self.dimensions,
                    &self.layers[0],
                    &net_layer,
                    BlendMode::Normal
                ).unwrap(),
                opacity: 255,
                mute: false,
            };
        }
        net_layer.scene
    }
    pub fn set_cursor(&mut self, index: usize, new_cursor: Cursor) -> Result<(), ProjectError> {
        use ProjectError::{ CursorCoordOutOfBounds, LayerOutOfBounds, CursorIndexOutOfBounds };
        match self.cursors.get_mut(index) {
            Some(cursor) => {
                let Cursor { coord: coord, layer: layer } = new_cursor;
                if coord.x < 0 || coord.y < 0 || coord.x >= self.dimensions.x || coord.y >= self.dimensions.y {
                    return Err(CursorCoordOutOfBounds(index, new_cursor, self.dimensions));
                }
                if layer >= self.layers.len() {
                    return Err(LayerOutOfBounds(layer, self.layers.len()));
                }
                cursor.coord = new_cursor.coord;
                cursor.layer = new_cursor.layer;
                Ok(())
            },
            None => Err(CursorIndexOutOfBounds(index, self.cursors.len())),
        }
    }
    pub fn render_layer(&self) -> Result<Vec<ProjectPixel>, ProjectError> {
        use ProjectError::{ LayerOutOfBounds };
        let mut project_pixels: Vec<ProjectPixel> = Vec::new();
        for camera_pixel in self.camera.render_scene(
            &self.layers
                .get(self.focus.layer)
                .ok_or(LayerOutOfBounds(self.focus.layer, self.layers.len()))?
                .scene,
            self.focus.coord
        ) {
            project_pixels.push(ProjectPixel {
                camera_pixel,
                has_cursor: match camera_pixel {
                    CameraPixel::Filled{ scene_coord, .. } |
                    CameraPixel::Empty{ scene_coord } => {
                        self.in_cursors(Cursor{ layer: self.focus.layer, coord: scene_coord})
                    },
                    _ => { false }
                }
            });
        }
        Ok(project_pixels)
    }
    pub fn render(&self) -> Vec<CameraPixel> {
        self.camera.render_scene(&self.merged_scene(), self.focus.coord)
    }
    fn in_cursors(&self, cursor: Cursor) -> bool {
        for ex_cursor in &self.cursors {
            if ex_cursor.coord == cursor.coord && ex_cursor.layer == cursor.layer {
                return true;
            }
        }
        return false;
    }
}
