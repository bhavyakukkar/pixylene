use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::elements::layer::{ self, Scene };

#[derive(Debug)]
pub enum MergeError {
    SceneError(usize, Coord, layer::SceneError),
    NotEnoughLayers(usize),
}

impl std::fmt::Display for MergeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use MergeError::*;
        match self {
            SceneError(k, coord, scene_error) => write!(
                f,
                "Getting pixel at coordinate {} on scene of layer at index {} of layers failed: {}",
                coord,
                k,
                scene_error
            ),
            NotEnoughLayers(num_layers) => write!(
                f,
                "can only merge two or more layers. found {}",
                num_layers,
            ),
        }
    }
}

#[derive(Debug)]
pub enum LayerError {
    MergeError(MergeError),
}
impl std::fmt::Display for LayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use LayerError::*;
        match self {
            MergeError(merge_error) => write!(f, "{}", merge_error),
        }
    }
}

#[derive(Savefile)]
pub struct Layer {
    pub scene: Scene,
    pub opacity: u8
}

impl Layer {
    pub fn merge(layers: Vec<&Layer>, blend_mode: BlendMode) -> Result<Layer, LayerError> {
        use MergeError::*;
        use LayerError::MergeError as Error;
        if layers.len() <= 1 {
            return Err(Error(NotEnoughLayers(layers.len())));
        }
        let dim = layers[0].scene.dim();
        let mut resultant_scene = Scene::new(
            dim,
            vec![Some(Pixel::background()); dim.area() as usize]
        ).unwrap();
        for k in 0..layers.len() {
            for i in 0..dim.x {
                for j in 0..dim.y {
                    let coord = Coord{ x: i, y: j };
                    let bottom = Pixel::get_certain(resultant_scene.get_pixel(coord).unwrap());
                    let mut top = Pixel::get_certain(
                        match layers[k].scene.get_pixel(coord) {
                            Ok(pixel) => pixel,
                            Err(scene_error) => {
                                return Err(
                                    Error(SceneError(k, coord, scene_error))
                                );
                            }
                        }
                    );
                    top.a = ((((layers[k].opacity as u16) * (top.a as u16)) as f32)/255f32) as u8;
                    resultant_scene.set_pixel(
                        coord,
                        Some(blend_mode.merge_down(
                            top,
                            bottom
                        ))
                    ).unwrap();
                }
            }
        }
        Ok(Layer{ scene: resultant_scene, opacity: 255 })
    }
}
