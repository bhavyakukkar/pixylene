use crate::{
    types::{ self, UCoord, PCoord, Pixel, TruePixel, IndexedPixel, BlendMode },
    project::{ Scene, SceneError, Palette },
    utils::messages::U32TOUSIZE,
};

use serde::{ Deserialize, Serialize };



/// A [`Scene`](Scene) with additional information including an opacity, mute switch and a
/// [`BlendMode`](BlendMode).
#[derive(Debug, Serialize, Deserialize, PartialEq, Savefile, Clone)]
pub struct Layer<T=TruePixel>
where T: Pixel
{
    pub scene: Scene<T>,
    pub opacity: u8,
    pub mute: bool,
    pub blend_mode: BlendMode,
}

impl<T: Pixel> Layer<T> {
    /// Create a new layer with the given dimensions and single color
    pub fn new_with_solid_color(dimensions: PCoord, color: Option<T>) -> Layer<T> {
        Layer::<T> {
            scene: Scene::<T>::new(
                dimensions,
                vec![color; usize::try_from(dimensions.area()).expect(U32TOUSIZE)]
            ).unwrap(),
            opacity: 255,
            mute: false,
            blend_mode: BlendMode::Normal,
        }
    }
}

impl Layer<TruePixel> {
    /// Return the net merged layer as a result of merging two truecolor layers with a given
    /// blend-mode
    ///
    /// `Note`: This method does not use the respective layer's owned
    /// [`blend-modes`](Layer::blend_mode) in order to not make assumptions, however you may simply
    /// pass them externally.
    ///
    /// `Note`: This method may fail with the [`MergeError`][me] or [`BlendError`][be] error
    /// variants only.
    ///
    /// [me]: LayerError::MergeError
    /// [be]: LayerError::BlendError
    pub fn merge(
        dimensions: PCoord,
        top: &Layer<TruePixel>,
        bottom: &Layer<TruePixel>,
        blend_mode: BlendMode
    )
        -> Result<Scene<TruePixel>, LayerError>
    {
        use LayerError::{ MergeError, BlendError };
        let mut merged_scene_grid: Vec<Option<TruePixel>> = Vec::new();
        for i in 0..dimensions.x() {
            for j in 0..dimensions.y() {
                let coord = UCoord{ x: i, y: j };
                let top_p = if top.mute {
                    TruePixel::empty()
                } else {
                    match top.scene.get_pixel(coord) {
                        Ok(pixel) => pixel.unwrap_or(TruePixel::empty()).dissolve(top.opacity),
                        Err(scene_error) => {
                            return Err(MergeError(true, coord, scene_error));
                        }
                    }
                };
                let bottom_p = if bottom.mute {
                    Pixel::empty()
                } else {
                    match bottom.scene.get_pixel(coord) {
                        Ok(pixel) => pixel.unwrap_or(TruePixel::empty()).dissolve(bottom.opacity),
                        Err(scene_error) => {
                            return Err(MergeError(false, coord, scene_error));
                        }
                    }
                };
                merged_scene_grid.push(
                    Some(blend_mode.blend(top_p, bottom_p)
                        .map_err(|err| BlendError(UCoord{ x: i, y: j }, err))?)
                );
            }
        }
        Ok(Scene::<TruePixel>::new(dimensions, merged_scene_grid).unwrap())
    }
}

impl Layer<IndexedPixel> {
    pub fn to_true_layer(&self, palette: &Palette) -> Layer<TruePixel> {
        Layer::<TruePixel> {
            scene: Scene::<TruePixel>::new(
                self.scene.dim(),
                self.scene.grid()
                    .map(|index_maybe| match index_maybe {
                        Some(index) => palette.get_color(index.0)
                            .map(|true_pixel| Some(true_pixel.clone()))
                            .unwrap_or(None),
                        None => None,
                    })
                    .collect::<Vec<Option<TruePixel>>>()
            ).unwrap(), //cant fail because x.dim() is used to construct scene from x.grid()
                        //which are consistent
            opacity: self.opacity,
            mute: self.mute,
            blend_mode: self.blend_mode.clone(),
        }
    }
}


// Error Types

/// Error enum to describe various errors returns by Layer methods
#[derive(Debug)]
pub enum LayerError {

    /// Error that occurs when trying to merge inconsistently sized layers in
    /// [`merge`](Layer::merge) and coordinates valid for the passed dimensions turn out to be out
    /// of bounds for any of the two layers passed
    MergeError(bool, UCoord, SceneError),

    /// Error that is propagated when trying to blend using the blend-mode passed to
    /// [`merge`](Layer::merge)
    BlendError(UCoord, types::BlendError),
}

impl std::fmt::Display for LayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use LayerError::*;
        match self {
            MergeError(at_top_layer, coord, scene_error) => write!(
                f,
                "Getting pixel at coordinate {} on scene on the {} layer failed: {}",
                coord,
                if *at_top_layer { "top" } else { "bottom" },
                scene_error,
            ),
            BlendError(coord, err) => write!(
                f,
                "Blending pixels while merging failed at coordinate {}: {}",
                coord,
                err,
            ),
        }
    }
}
