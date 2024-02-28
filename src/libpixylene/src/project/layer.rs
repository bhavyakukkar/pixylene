use crate::{
    types::{ UCoord, PCoord, Pixel, BlendMode },
    project::{ Scene, SceneError },
    utils::messages::U32TOUSIZE,
};

#[derive(Savefile, Clone)]
pub struct Layer {
    pub scene: Scene,
    pub opacity: u8,
    pub mute: bool,
}

impl Layer {
    pub fn new_with_solid_color(dimensions: PCoord, color: Option<Pixel>) -> Layer {
        Layer {
            scene: Scene::new(
                dimensions,
                vec![color; usize::try_from(dimensions.area()).expect(U32TOUSIZE)]
            ).unwrap(),
            opacity: 255,
            mute: false,
        }
    }
    pub fn merge(dimensions: PCoord, layer_top: &Layer, layer_bottom: &Layer, blend_mode: BlendMode)
    -> Result<Scene, LayerError> {
        use LayerError::{ MergeError };
        let mut merged_scene_grid: Vec<Option<Pixel>> = Vec::new();
        for i in 0..dimensions.x() {
            for j in 0..dimensions.y() {
                let coord = UCoord{ x: i, y: j };
                let top = Pixel::get_certain(
                    match layer_top.scene.get_pixel(coord) {
                        Ok(pixel) => pixel,
                        Err(scene_error) => {
                            return Err(MergeError(true, coord, scene_error));
                        }
                    }
                );
                let bottom = Pixel::get_certain(
                    match layer_bottom.scene.get_pixel(coord) {
                        Ok(pixel) => pixel,
                        Err(scene_error) => {
                            return Err(MergeError(false, coord, scene_error));
                        }
                    }
                );
                //todo: needs replacing
                //top.a = ((((layer_top.opacity as u16) * (top.a as u16)) as f32)/255f32) as u8;
                //bottom.a = ((((layer_bottom.opacity as u16) * (bottom.a as u16)) as f32)/255f32) as u8;
                merged_scene_grid.push(Some(blend_mode.merge_down(top, bottom)));
            }
        }
        Ok(Scene::new(dimensions, merged_scene_grid).unwrap())
    }
}


// Error Types

#[derive(Debug)]
pub enum LayerError {
    MergeError(bool, UCoord, SceneError),
}
impl std::fmt::Display for LayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use LayerError::*;
        match self {
            MergeError(at_top_layer, coord, scene_error) => write!(
                f,
                "Getting pixel at coordinate {} on scene of {} failed: {}",
                coord,
                if *at_top_layer { "layer_top" } else { "layer_bottom" },
                scene_error
            ),
        }
    }
}
