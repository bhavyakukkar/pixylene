use crate::{
    types::{ Coord, PCoord, UCoord, Pixel, BlendMode },
    project::{ Palette, Layer, OPixel, Canvas, CanvasError },
    utils::messages::{ PCOORD_NOTFAIL },
};

use std::collections::HashMap;


#[derive(Savefile)]
pub struct Project {
    pub canvas: Canvas,
    cursors: HashMap<(UCoord, u16), ()>,
    sel_cursor: Option<(UCoord, u16)>,
    pub focus: (Coord, u16),
    pub out_dim: PCoord,
    out_mul: u8,
    pub out_repeat: PCoord,
}

impl Project {
    pub fn new(
        dimensions: PCoord,
        palette: Palette
    ) -> Project {
        let canvas = Canvas::new(
            dimensions,
            palette,
        );
        Project {
            canvas,
            cursors: HashMap::new(),
            sel_cursor: None,
            focus: (Coord{x: 0, y: 0}, 0),
            out_dim: PCoord::new(10, 10).expect(PCOORD_NOTFAIL),
            out_mul: 1,
            out_repeat: PCoord::new(1, 2).expect(PCOORD_NOTFAIL),
        }
    }

    pub fn get_out_mul(&self) -> u8 {
        self.out_mul
    }

    pub fn set_out_mul(&mut self, new_mul: u8) -> Result<(), ProjectError> {
        use ProjectError::{ ZeroMultiplier };
        if new_mul > 0 {
            self.out_mul = new_mul;
            Ok(())
        } else {
            Err(ZeroMultiplier)
        }
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

    /// dont forget to set out_dim, out_mul and out_repeat before using this
    pub fn render_layer(&self) -> Result<Vec<OPixel>, ProjectError> {
        let net_scene = Layer::merge(
            self.canvas.dim(),
            &self.canvas.get_layer(self.focus.1)?,
            &Layer::new_with_solid_color(self.canvas.dim(), Some(Pixel::black())),
            BlendMode::Normal
        ).unwrap();

        let out_pixels: Vec<OPixel> = net_scene.render(
            self.out_dim, self.out_mul, self.out_repeat, self.focus.0
        );

        Ok(out_pixels.iter().map(|out_pixel| match out_pixel {
            OPixel::Filled { scene_coord, color, is_focus, .. } =>
                OPixel::Filled {
                    scene_coord: *scene_coord, color: *color, is_focus: *is_focus,
                    has_cursor: self.cursors.get(&(*scene_coord, self.focus.1)).is_some(),
                },
            OPixel::Empty { scene_coord, .. } =>
                OPixel::Empty {
                    scene_coord: *scene_coord,
                    has_cursor: self.cursors.get(&(*scene_coord, self.focus.1)).is_some(),
                },
            OPixel::OutOfScene => OPixel::OutOfScene,
        }).collect::<Vec<OPixel>>())
    }

    /// dont forget to set out_dim, out_mul and out_repeat before using this
    pub fn render(&self) -> Vec<OPixel> {
        self.canvas.merged_scene(Some(Pixel::black())).render(
            self.out_dim, self.out_mul, self.out_repeat, self.focus.0
        )
    }

    pub fn toggle_cursor_at(&mut self, coord: UCoord, layer: u16) -> Result<(), ProjectError> {
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
    ZeroMultiplier,
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
                UCoord{ x: 0, y: 0 },
                Coord::from(dim).add(Coord{ x: -1, y: -1 }),
            ),
            CanvasError(error) => write!(f, "{}", error),
            CursorLayerOutOfBounds(layer, layers_len) => write!(
                f,
                "layer index {} is out of bounds for the {} layers present in the project",
                layer,
                layers_len,
            ),
            ZeroMultiplier => write!(
                f,
                "cannot set camera's multiplier to 0",
            ),
        }
    }
}
impl From<CanvasError> for ProjectError {
    fn from(item: CanvasError) -> ProjectError { ProjectError::CanvasError(item) }
}
