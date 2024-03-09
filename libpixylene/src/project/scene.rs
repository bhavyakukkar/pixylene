use crate::{
    types::{ Coord, PCoord, UCoord, Pixel },
    utils::messages::U32TOUSIZE,
};


/// A two-dimensional grid of pixels.
///
/// `Note`: Each item of the grid is an [`Option<Pixel>`] rather than simply a [`Pixel`].
#[derive(Savefile, Clone)]
pub struct Scene {
    dim: PCoord,
    grid: Vec<Option<Pixel>>
}

impl Scene {

    /// Tries to create a new scene with given dimensions and buffer of optional [`Pixels`][Pixel]
    ///
    /// This method may fail with the [`DimensionMismatch`][dm] error variant only.
    ///
    /// [dm]: SceneError::DimensionMismatch
    pub fn new(dimensions: PCoord, buffer: Vec<Option<Pixel>>) -> Result<Self, SceneError> {
        use SceneError::{ DimensionMismatch };

        if buffer.len() != usize::try_from(dimensions.area()).expect(U32TOUSIZE) {
            Err(DimensionMismatch(buffer.len(), dimensions))
        } else {
            Ok(Self{ dim: dimensions, grid: buffer })
        }
    }

    /// Tries to get the item at the given coordinatel & fails with context if coordinate is out of
    /// bounds for this scene
    ///
    /// This method may fail with the [`OutOfBoundCoordinates`][oobc] error variant only.
    ///
    /// [oobc]: SceneError::OutOfBoundCoordinates
    pub fn get_pixel(&self, coord: UCoord) -> Result<Option<Pixel>, SceneError> {
        use SceneError::{ OutOfBoundCoordinates };

        if coord.x >= self.dim.x() || coord.y >= self.dim.y() {
            Err(OutOfBoundCoordinates(coord, self.dim))
        }
        else {
            let index = usize::from(coord.x) * usize::from(self.dim.y()) + usize::from(coord.y);
            Ok(self.grid[index])
        }
    }

    /// Helper function similar to [`get_pixel`](#method.get_pixel) that gets the item at the given
    /// coordinate but returns `None` with no additional information if coordinate is out of bounds
    /// or coordinate is invalid (negative number)
    ///
    /// This method is left here for compatiblity with the [render](#method.render)
    /// method.
    pub fn get_pixel_raw(&self, coord: Coord) -> Option<Option<Pixel>> {
        if coord.x < 0 || coord.y < 0 { None }
        else if coord.x >= i32::from(self.dim.x()) || coord.y >= i32::from(self.dim.y()) { None }
        else {
            let index = usize::try_from(coord.x).unwrap() * usize::from(self.dim.y()) +
                usize::try_from(coord.y).unwrap();
            Some(self.grid[index])
        }
    }

    /// Tries to set an item at the given coordinate & fails with context if coordinate is out of
    /// bounds for this scene
    ///
    /// This method may fail with the [`OutOfBoundCoordinates`][oobc] error variant only.
    ///
    /// [oobc]: SceneError::OutOfBoundCoordinates
    pub fn set_pixel(&mut self, coord: UCoord, new_pixel: Option<Pixel>) -> Result<(), SceneError> {
        use SceneError::{ OutOfBoundCoordinates };

        if coord.x >= self.dim.x() || coord.y >= self.dim.y() {
            Err(OutOfBoundCoordinates(coord, self.dim))
        } else {
            let index = usize::from(coord.x) * usize::from(self.dim.y()) + usize::from(coord.y);
            self.grid[index] = new_pixel;
            Ok(())
        }
    }

    /// Gets the dimensions of this scene
    pub fn dim(&self) -> PCoord {
        self.dim
    }

    /// Renders a given Scene with the coordinate to be rendered at the center
    ///
    /// Rendering is the process of previewing any [`Scene`] by taking the following inputs:
    /// - dimensions, i.e, `the size of the output`
    /// - mult, i.e., `the number of [OPixels][op] on the output (in the x and y directions)
    /// corresponding to a single [Pixel] on the [Scene]`
    /// - focus, i.e. `the coordinate of the Scene that will be mapped to the center of the output`
    ///
    /// `Note`: the focus can be out of the scene; this function merely looks for the scene around
    /// the passed focus coordinate for the duration of its dimensions, placing [`OutOfScene`][oos]
    /// where the scene doesn't exist.
    ///
    /// [op]: OPixel
    /// [oos]: OPixel::OutOfScene
    pub fn render(&self, dim: PCoord, mul: u8, repeat: PCoord, focus: Coord) -> Vec<OPixel> {
        use OPixel::*;
        let mut grid: Vec<OPixel> = vec![
            OutOfScene;
            usize::try_from(dim.area()).expect(U32TOUSIZE)
        ];

        let mut render_pixel = |i: i64, j: i64, x: i32, y: i32, is_focus: bool| {
            for mi in 0..i64::from(u16::from(mul)*u16::from(repeat.x())) {
                for mj in 0..i64::from(u16::from(mul)*u16::from(repeat.y())) {
                    if (i+mi) < 0 || (i+mi) >= i64::from(dim.x()) ||
                       (j+mj) < 0 || (j+mj) >= i64::from(dim.y()) {
                        continue;
                    }
                    if let Some(pixel_maybe) = self.get_pixel_raw(Coord{x, y}) {
                        if let Some(pixel) = pixel_maybe {
                            let index = usize::try_from(i+mi).unwrap() *
                                        usize::from(dim.y()) +
                                        usize::try_from(j+mj).unwrap();
                            grid[index] = Filled {
                                scene_coord: UCoord{
                                    x: u16::try_from(x).unwrap(),
                                    y: u16::try_from(y).unwrap()
                                },
                                color: pixel,
                                is_focus: is_focus,
                                has_cursor: false,
                            };
                        } else {
                            let index = usize::try_from(i+mi).unwrap() *
                                        usize::from(dim.y()) +
                                        usize::try_from(j+mj).unwrap();
                            grid[index] = Empty {
                                scene_coord: UCoord{
                                    x: u16::try_from(x).unwrap(),
                                    y: u16::try_from(y).unwrap()
                                },
                                has_cursor: false,
                            };
                        }
                    } else {
                        let index = usize::try_from(i+mi).unwrap() *
                                    usize::from(dim.y()) +
                                    usize::try_from(j+mj).unwrap();
                        grid[index] = OutOfScene;
                    }
                }
            }
        };

        let focus_x = i32::from(focus.x);
        let focus_y = i32::from(focus.y);
        let mul_x = i64::from(u16::from(mul) * u16::from(repeat.x()));
        let mul_y = i64::from(u16::from(mul) * u16::from(repeat.y()));
        let mid_x = i64::from((dim.x() - u16::from(mul) * u16::from(repeat.x()))
                              .checked_div(2).unwrap());
        //apparently this line still isn't safe and fails with "attempt to subtract with overflow"
        let mid_y = i64::from((dim.y() - u16::from(mul) * u16::from(repeat.y()))
                              .checked_div(2).unwrap());

        let mut i = mid_x;
        let mut x = focus_x;
        while i > -1*mul_x {
            let mut j = mid_y;
            let mut y = focus_y;
            while j > -1*mul_y {
                render_pixel(i, j, x, y, i == mid_x && j == mid_y);
                j -= mul_y;
                y -= 1;
            }
            j = mid_y + mul_y;
            y = focus_y + 1;
            while j < dim.y().into() {
                render_pixel(i, j, x, y, i == mid_x && j == mid_y);
                j += mul_y;
                y += 1;
            }
            i -= mul_x;
            x -= 1;
        }
        i = mid_x + mul_x;
        x = focus_x + 1;
        while i < dim.x().into() {
            let mut j = mid_y;
            let mut y = focus_y;
            while j > -1*mul_y {
                render_pixel(i, j, x, y, false);
                j -= mul_y;
                y -= 1;
            }
            j = mid_y + mul_y;
            y = focus_y + 1;
            while j < dim.y().into() {
                render_pixel(i, j, x, y, false);
                j += mul_y;
                y += 1;
            }
            i += mul_x;
            x += 1;
        }

        return grid;
    }
}

/// An `O`utput `Pixel` as rendered by [`render`](Scene::render)
///
/// [`Pixel`] represents a virtual pixel contained in a piece of pixel art, whereas `OPixel`
/// represents an actual pixel on the application being used to edit the piece of pixel art.
#[derive(Clone)]
pub enum OPixel {

    /// An OPixel pointing to a Pixel on the Scene that is filled with some color
    ///
    /// Associated with a `Some(Some(Pixel))` variant received by [`get_pixel_raw`][gpr]
    ///
    /// [gpr]: Scene::get_pixel_raw
    Filled {
        scene_coord: UCoord,
        color: Pixel,
        is_focus: bool,
        has_cursor: bool,
    },

    /// An OPixel pointing to a Pixel on the Scene that is left empty or hasn't been filled in yet
    ///
    /// Associated with a `Some(None)` variant received by [`get_pixel_raw`][gpr]
    ///
    /// [gpr]: Scene::get_pixel_raw
    Empty {
        scene_coord: UCoord,
        has_cursor: bool,
    },

    /// An OPixel pointing to somewhere outside the bounds of the Scene
    ///
    /// Associated with a `None` variant received by [`get_pixel_raw`][gpr]
    ///
    /// [gpr]: Scene::get_pixel_raw
    OutOfScene,
}


// Error Types

/// Error enum to describe various errors returns by Scene methods
#[derive(Debug)]
pub enum SceneError {

    /// Error that occurs when the buffer length and dimensions passed to [`Scene::new`] do not
    /// match
    DimensionMismatch(usize, PCoord),

    /// Error that occurs when trying to access a coordinate that is out of bounds for the current
    /// scene
    OutOfBoundCoordinates(UCoord, PCoord),
}

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SceneError::*;
        match self {
            DimensionMismatch(length, dim) => write!(
                f,
                "Flattened grid found of length {} which does not match {} from product of given \
                dimensions {}",
                length,
                dim.area(),
                dim,
            ),
            OutOfBoundCoordinates(coord, dim) => write!(
                f,
                "Cannot get_pixel from out-of-bounds coordinates {} on scene of dimensions {}, \
                valid coordinates for this scene lie between {} and {} (inclusive)",
                coord,
                dim,
                UCoord{ x: 0, y: 0 },
                Coord::from(dim).add(Coord{ x: -1, y: -1 }),
            ),
        }
    }
}
