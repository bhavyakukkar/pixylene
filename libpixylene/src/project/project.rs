use crate::{
    project::{Canvas, Layer, LayersError, LayersType, OPixel},
    types::{BlendMode, Coord, PCoord, TruePixel, UCoord},
};

use std::collections::HashSet;
use std::iter::Iterator;

/// The absolute state of a Pixel Art project at any given instance and a manager for the
/// [`Canvas`].
///
/// The `Project` as opposed to the `Canvas` contains data that does not directly influence how
/// a Pixel Art project looks, including `Cursors` and the data responsible for the rendering
/// of the `Canvas` on a real screen where the Pixel Art project is being visualized.
#[derive(Savefile)]
pub struct Project {
    /// The [`Canvas`] composed into the Project.
    pub canvas: Canvas,

    /// The dimensions of the rendered output.
    pub out_dim: PCoord,

    /// The coordinate of a [`Scene`][s] (first field) on a [`Layer`] (second field) in the Canvas
    /// that will be mapped to the center of the rendered output.
    ///
    /// `Note`: This focus differs from the `focus` parameter passed to [`Scene::render`][r] by
    /// having an extra layer field.
    ///
    /// [s]: crate::project::Scene
    /// [r]: crate::project::Scene::render
    pub focus: (Coord, u16),

    /// see [Scene::render](crate::project::Scene::render)
    out_mul: u8,

    /// see [Scene::render](crate::project::Scene::render)
    pub out_repeat: PCoord,

    cursors: HashSet<(UCoord, u16)>,
    num_cursors: u64,
    sel_cursor: Option<(UCoord, u16)>,
}

impl Project {
    /// Creates a new empty Project containing the provided [`Canvas`]
    pub fn new(canvas: Canvas) -> Self {
        Self {
            canvas,
            focus: (Coord { x: 0, y: 0 }, 0),
            out_dim: PCoord::new(10, 10).unwrap(), //shouldn't fail
            out_mul: 1,
            out_repeat: PCoord::new(1, 1).unwrap(), //shouldn't fail
            cursors: HashSet::new(),
            num_cursors: 0,
            sel_cursor: None,
        }
    }

    /// Gets the output multiplier of the Project
    ///
    /// Setters and getters are required for this attribute because out_mul is not allowed to be 0
    pub fn get_out_mul(&self) -> u8 {
        self.out_mul
    }

    /// Sets the out_dim of the Project, fails if given zero
    ///
    /// Setters and getters are required for this attribute because out_dim is not allowed to be 0
    pub fn set_out_mul(&mut self, new_mul: u8) -> Result<(), ProjectError> {
        use ProjectError::ZeroMultiplier;
        if new_mul > 0 {
            self.out_mul = new_mul;
            Ok(())
        } else {
            Err(ZeroMultiplier)
        }
    }

    /// Renders the [`Scene`][s] at the focussed [`Layer`] of the [`Canvas`] at the Layer specified
    /// by the Project's [`focus.1`][f] field, mapping the center of the output to the coordinate
    /// on the Scene specified by the Project's [`focus.0`][f] field, with the output's scaling
    /// determined by the Project's [`out_dim`][od], [`out_mul`][om] and [`out_repeat`][or] fields,
    /// returning a flattened vector of the output [`OPixels`](OPixel)
    ///
    /// `Note`: This method may fail with the [`LayersError`][ce] error variant only.
    ///
    /// [s]: crate::project::Scene
    /// [f]: #structfield.focus
    /// [od]: #structfield.out_dim
    /// [om]: Project::get_out_mul
    /// [or]: #structfield.out_repeat
    /// [ce]: ProjectError::LayersError
    pub fn render_layer(&self) -> Result<Vec<OPixel>, ProjectError> {
        let net_scene = match &self.canvas.layers {
            LayersType::True(layers) => {
                Layer::merge(
                    layers.dim(),
                    &Layer {
                        opacity: 255,
                        mute: false,
                        ..layers
                            .get_layer(self.focus.1)
                            .map_err(|err| ProjectError::LayersError(err))?
                            .clone()
                    },
                    &Layer::new_with_solid_color(layers.dim(), Some(TruePixel::BLACK)),
                    BlendMode::Normal,
                )
                .unwrap() //cant fail because dimensions and layers taken from same Layers which
                          //cannot exist with inconsistent-dimension layers
            }
            LayersType::Indexed(layers) => {
                Layer::merge(
                    layers.dim(),
                    &Layer {
                        opacity: 255,
                        mute: false,
                        ..layers
                            .get_layer(self.focus.1)
                            .map(|layer_indexed| layer_indexed.to_true_layer(&self.canvas.palette))
                            .map_err(|err| ProjectError::LayersError(err))?
                            .clone()
                    },
                    &Layer::new_with_solid_color(layers.dim(), Some(TruePixel::BLACK)),
                    BlendMode::Normal,
                )
                .unwrap() //cant fail because dimensions and layers taken from same Layers which
                          //cannot exist with inconsistent-dimension layers
            }
        };

        let out_pixels =
            net_scene.render(self.out_dim, self.out_mul, self.out_repeat, self.focus.0);

        Ok(out_pixels
            .iter()
            .map(|out_pixel| match out_pixel {
                OPixel::Filled {
                    scene_coord,
                    color,
                    is_focus,
                    ..
                } => OPixel::Filled {
                    scene_coord: *scene_coord,
                    color: *color,
                    is_focus: *is_focus,
                    has_cursor: self.cursors.contains(&(*scene_coord, self.focus.1)),
                },
                OPixel::Empty { scene_coord, .. } => OPixel::Empty {
                    scene_coord: *scene_coord,
                    has_cursor: self.cursors.contains(&(*scene_coord, self.focus.1)),
                },
                OPixel::OutOfScene => OPixel::OutOfScene,
            })
            .collect::<Vec<OPixel>>())
    }

    /// Renders the [`Scene`][s] obtained by merging all the [`Layers`](Layer) of the [`Canvas`],
    /// mapping the center of the output to the coordinate on the Scene specified by the Project's
    /// [`focus.0`][f] field, with the output's scaling determined by the Project's
    /// [`out_dim`][od], [`out_mul`][om] and [`out_repeat`][or] fields, returning a flattened
    /// vector of the output [`OPixels`](OPixel)
    ///
    /// [s]: crate::project::Scene
    /// [f]: #structfield.focus
    /// [od]: #structfield.out_dim
    /// [om]: Project::get_out_mul
    /// [or]: #structfield.out_repeat
    pub fn render(&self) -> Vec<OPixel> {
        self.canvas
            .merged_true_scene(Some(TruePixel::BLACK))
            .render(self.out_dim, self.out_mul, self.out_repeat, self.focus.0)
    }

    /// Returns whether there is a cursor present pointing at the specified coordinate on the
    /// specified [`Layer`] in the [`Canvas`]
    ///
    /// `Note`: This method may fail with the [`CursorLayerOutOfBounds`][cloob] &
    /// [`CursorCoordOutOfBounds`][cioob] error variants only.
    ///
    /// [cloob]: ProjectError::CursorLayerOutOfBounds
    /// [cioob]: ProjectError::CursorCoordOutOfBounds
    pub fn is_cursor_at(&self, cursor: &(UCoord, u16)) -> Result<bool, ProjectError> {
        use ProjectError::{CursorCoordOutOfBounds, CursorLayerOutOfBounds};

        if cursor.1 < self.canvas.layers.len() {
            if cursor.0.x < self.canvas.layers.dim().x()
                && cursor.0.y < self.canvas.layers.dim().y()
            {
                Ok(self.cursors.contains(cursor))
            } else {
                Err(CursorCoordOutOfBounds(
                    cursor.0.clone(),
                    self.canvas.layers.dim(),
                ))
            }
        } else {
            Err(CursorLayerOutOfBounds(cursor.1, self.canvas.layers.len()))
        }
    }

    /// Toggles a cursor to point at the specified coordinate on the specified [`Layer`] in the
    /// [`Canvas`], unsetting it if there was one already pointing
    ///
    /// `Note`: This method may fail with the [`CursorLayerOutOfBounds`][cloob] &
    /// [`CursorCoordOutOfBounds`][cioob] error variants only.
    ///
    /// [cloob]: ProjectError::CursorLayerOutOfBounds
    /// [cioob]: ProjectError::CursorCoordOutOfBounds
    pub fn toggle_cursor_at(&mut self, cursor: &(UCoord, u16)) -> Result<(), ProjectError> {
        use ProjectError::{CursorCoordOutOfBounds, CursorLayerOutOfBounds};

        if cursor.1 < self.canvas.layers.len() {
            if cursor.0.x < self.canvas.layers.dim().x()
                && cursor.0.y < self.canvas.layers.dim().y()
            {
                if self.cursors.remove(&cursor) {
                    self.num_cursors -= 1;
                } else {
                    _ = self.cursors.insert(cursor.clone());
                    self.num_cursors += 1;
                }
                Ok(())
            } else {
                Err(CursorCoordOutOfBounds(cursor.0, self.canvas.layers.dim()))
            }
        } else {
            Err(CursorLayerOutOfBounds(cursor.1, self.canvas.layers.len()))
        }
    }

    pub fn cursors(&self) -> impl Iterator<Item = &(UCoord, u16)> {
        self.cursors.iter()
    }

    pub fn num_cursors(&self) -> u64 {
        self.num_cursors
    }

    pub fn clear_cursors(&mut self) -> impl Iterator<Item = (UCoord, u16)> + '_ {
        self.num_cursors = 0;
        self.cursors.drain()
    }

    pub fn resize(&mut self) {
        todo!()
    }
}

//impl From<CanvasType> for Project {
//    fn from(canvas: CanvasType) -> Project {
//        Project {
//            indexed: if let CanvasType::Indexed(_) = canvas { true } else { false },
//            canvas,
//            focus: (Coord{x: 0, y: 0}, 0),
//            out_dim: PCoord::new(10, 10).unwrap(), //shouldn't fail
//            out_mul: 1,
//            out_repeat: PCoord::new(1, 1).unwrap(), //shouldn't fail
//            cursors: HashMap::new(),
//            num_cursors: 0,
//            sel_cursor: None,
//        }
//    }
//}

//impl From<TrueCanvas> for Project {
//    fn from(canvas: TrueCanvas) -> Project {
//        Project::from(CanvasType::True(canvas))
//    }
//}

//impl From<IndexedCanvas> for Project {
//    fn from(canvas: IndexedCanvas) -> Project {
//        Project::from(CanvasType::Indexed(canvas))
//    }
//}

// Error Types

/// Error enum to describe various errors returns by Canvas methods
#[derive(Debug)]
pub enum ProjectError {
    /// Error that occurs when trying to access a Cursor using a Layer index that is out of bounds
    /// for the Canvas
    CursorLayerOutOfBounds(u16, u16),

    /// Error that occurs when trying to access a Cursor using a coordinate that is out of bounds
    /// for the Canvas
    CursorCoordOutOfBounds(UCoord, PCoord),

    /// Error that is propagated in [`render_layer`](Project::render_layer) when trying to access a
    /// Layer set by the [`focus`](Project#structfield.focus) that is out of bounds for the Canvas
    LayersError(LayersError),

    /// Error that occurs when trying to set the output multipler out_mul to 0
    ZeroMultiplier,
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ProjectError::*;
        match self {
            CursorLayerOutOfBounds(layer, layers_len) => write!(
                f,
                "layer index {} is out of bounds for the {} layers present in the canvas",
                layer, layers_len,
            ),
            CursorCoordOutOfBounds(coord, canvas_dim) => write!(
                f,
                "cannot set cursor to coordinate {} since canvas dimensions are {}, valid \
                coordinates for this project lie between {} and {} (inclusive)",
                coord,
                canvas_dim,
                UCoord { x: 0, y: 0 },
                Coord::from(canvas_dim).add(Coord { x: -1, y: -1 }),
            ),
            LayersError(error) => write!(f, "{}", error),
            ZeroMultiplier => write!(f, "cannot set output multiplier to 0",),
        }
    }
}

impl From<LayersError> for ProjectError {
    fn from(item: LayersError) -> ProjectError {
        ProjectError::LayersError(item)
    }
}
