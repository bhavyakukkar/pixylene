use crate::{
    types::{ Coord, Pixel, BlendMode, Cursor },
    project::{ Palette, Scene, CameraPixel, Camera, Layer, Canvas },
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

#[derive(Clone)]
pub struct ProjectPixel {
    pub camera_pixel: CameraPixel,
    pub has_cursor: bool,
}

#[derive(Savefile)]
pub struct Project {
    pub canvas: Canvas,
    pub cursors: Vec<Cursor>,
    pub camera: Camera,
    pub focus: Cursor,
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
        let canvas = Canvas {
            dimensions,
            layers,
            palette,
        };
        let mut project = Project {
            canvas,
            cursors: vec![Cursor { layer: 0, coord: Coord { x: 0, y: 0 }}; cursors.len()],
            camera,
            focus,
        };
        for (index, cursor) in cursors.iter().enumerate() { project.set_cursor(index, *cursor)?; }
        Ok(project)
    }
    pub fn get_cursor(&self, index: usize) -> Result<Cursor, ProjectError> {
        use ProjectError::{ CursorIndexOutOfBounds };
        match self.cursors.get(index) {
            Some(cursor) => {
                Ok(cursor.clone())
            },
            None => Err(CursorIndexOutOfBounds(index, self.cursors.len())),
        }
    }
    pub fn set_cursor(&mut self, index: usize, new_cursor: Cursor) -> Result<(), ProjectError> {
        use ProjectError::{ CursorCoordOutOfBounds, LayerOutOfBounds, CursorIndexOutOfBounds };
        match self.cursors.get_mut(index) {
            Some(cursor) => {
                let Cursor { coord: coord, layer: layer } = new_cursor;
                if coord.x < 0 || coord.y < 0 ||
                    coord.x >= self.canvas.dimensions.x || coord.y >= self.canvas.dimensions.y {
                    return Err(CursorCoordOutOfBounds(index, new_cursor, self.canvas.dimensions));
                }
                if layer >= self.canvas.layers.len() {
                    return Err(LayerOutOfBounds(layer, self.canvas.layers.len()));
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
        let net_scene = Layer::merge(
            self.canvas.dimensions,
            self.canvas.layers
                .get(self.focus.layer)
                .ok_or(LayerOutOfBounds(self.focus.layer, self.canvas.get_num_layers()))?,
            &Layer::new_with_solid_color(self.canvas.dimensions, Some(Pixel::background())),
            BlendMode::Normal
        ).unwrap();

        let mut project_pixels: Vec<ProjectPixel> = Vec::new();
        for camera_pixel in self.camera.render_scene(
            &net_scene,
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
        self.camera.render_scene(&self.canvas.merged_scene(), self.focus.coord)
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
