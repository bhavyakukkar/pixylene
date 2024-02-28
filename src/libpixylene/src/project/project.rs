use crate::{
    types::{ Coord, PCoord, UCoord, Pixel, BlendMode },
    project::{ Palette, CameraPixel, Camera, Layer, Canvas, CanvasError },
};

use std::collections::HashMap;


#[derive(Clone)]
pub struct ProjectPixel {
    pub camera_pixel: CameraPixel,
    pub has_cursor: bool,
}

#[derive(Savefile)]
pub struct Project {
    pub canvas: Canvas,
    cursors: HashMap<(UCoord, u16), ()>,
    sel_cursor: Option<(UCoord, u16)>,
    pub camera: Camera,
    pub focus: (Coord, u16),
}

impl Project {
    pub fn new(
        dimensions: PCoord,
        camera: Camera,
        focus: (Coord, u16),
        palette: Palette
    ) -> Result<Project, ProjectError> {
        let canvas = Canvas::new(
            dimensions,
            palette,
        );
        let project = Project {
            canvas,
            cursors: HashMap::new(),
            sel_cursor: None,
            camera,
            focus,
        };
        Ok(project)
    }
    /*
    pub fn set_cursor(&mut self, index: usize, new_cursor: (UCoord, usize))
        -> Result<(), ProjectError> {
        use ProjectError::{ CursorCoordOutOfBounds, LayerOutOfBounds, CursorIndexOutOfBounds };
        match self.cursors.get_mut(index) {
            Some(cursor) => {
                let ( coord, layer ) = new_cursor;
                if coord.x >= self.canvas.dim().x() || coord.y >= self.canvas.dim().y() {
                    return Err(CursorCoordOutOfBounds(index, coord, self.canvas.dim()));
                }
                if layer >= self.canvas.num_layers() {
                    return Err(LayerOutOfBounds(layer, self.canvas.num_layers()));
                }
                cursor.0 = coord;
                cursor.1 = layer;
                Ok(())
            },
            None => Err(CursorIndexOutOfBounds(index, self.cursors.len())),
        }
    }
    */
    pub fn render_layer(&self) -> Result<Vec<ProjectPixel>, ProjectError> {
        let net_scene = Layer::merge(
            self.canvas.dim(),
            &self.canvas.get_layer(self.focus.1)?,
            &Layer::new_with_solid_color(self.canvas.dim(), Some(Pixel::background())),
            BlendMode::Normal
        ).unwrap();

        let mut project_pixels: Vec<ProjectPixel> = Vec::new();
        for camera_pixel in self.camera.render_scene(&net_scene, self.focus.0) {
            project_pixels.push(ProjectPixel {
                camera_pixel,
                has_cursor: match camera_pixel {
                    CameraPixel::Filled{ scene_coord, .. } |
                    CameraPixel::Empty{ scene_coord } => self.cursors
                        .get(&(scene_coord, self.focus.1)).is_some(),
                    CameraPixel::OutOfScene => { false }
                }
            });
        }
        Ok(project_pixels)
    }
    pub fn render(&self) -> Vec<CameraPixel> {
        self.camera.render_scene(
            &self.canvas.merged_scene(Some(Pixel::background())),
            self.focus.0
        )
    }
    fn toggle_cursor_at(&mut self, coord: UCoord, layer: u16) -> Result<(), ProjectError> {
        use ProjectError::{ CursorLayerOutOfBounds };
        if layer < self.canvas.num_layers() {
            if self.cursors.get(&(coord, layer)).is_some() {
                self.cursors.remove(&(coord, layer)).unwrap();
            } else {
                _ = self.cursors.insert((coord, layer), ());
            }
            Ok(())
        } else {
            Err(CursorLayerOutOfBounds(layer, self.canvas.num_layers()))
        }
    }
    pub fn resize(&mut self) {
        todo!()
    }
}


// Error Types

#[derive(Debug)]
pub enum ProjectError {
    CursorIndexOutOfBounds(usize, usize),
    CursorCoordOutOfBounds(usize, UCoord, PCoord),
    CanvasError(CanvasError),
    CursorLayerOutOfBounds(u16, u16),
}
impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ProjectError::*;
        match self {
            CursorIndexOutOfBounds(index, cursors_len) => write!(
                f,
                "index {} is out of bounds for cursors of length {} in the project",
                index,
                cursors_len,
            ),
            CursorCoordOutOfBounds(index, cursor_coord, dim) => write!(
                f,
                "cannot set cursor at index {} of cursors to coordinate {} since project \
                dimensions are {}, valid coordinates for this project lie between {} and \
                {} (inclusive)",
                index,
                cursor_coord,
                dim,
                UCoord::new(0,0),
                Coord::from(dim).add(Coord{ x: -1, y: -1 }),
            ),
            CanvasError(error) => write!(f, "{}", error),
            CursorLayerOutOfBounds(layer, layers_len) => write!(
                f,
                "layer index {} is out of bounds for the {} layers present in the project",
                layer,
                layers_len,
            ),
        }
    }
}
impl From<CanvasError> for ProjectError {
    fn from(item: CanvasError) -> ProjectError { ProjectError::CanvasError(item) }
}
