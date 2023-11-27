use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::elements::layer::Scene;

pub struct Layer {
    pub scene: Scene,
    pub opacity: u8
}

impl Layer {
    pub fn merge(layers: Vec<&Layer>, blend_mode: BlendMode) -> Result<Layer, String> {
        if layers.len() <= 1 {
            return Err(format!("can only merge two or more layers. found {}", layers.len()));
        }
        let dim = layers[0].scene.dim;
        let mut resultant_scene = Scene::new(
            dim,
            vec![Some(Pixel::background()); dim.area() as usize]
        )?;
        for k in 0..layers.len() {
            for i in 0..resultant_scene.dim.x {
                for j in 0..resultant_scene.dim.y {
                    let coord = Coord{ x: i, y: j };
                    let mut bottom = Pixel::get_certain(resultant_scene.get_pixel(coord)?);
                    let mut top = Pixel::get_certain(layers[k].scene.get_pixel(coord)?);
                    top.a = ((((layers[k].opacity as u16) * (top.a as u16)) as f32)/255f32) as u8;
                    resultant_scene.set_pixel(
                        coord,
                        Some(blend_mode.merge_down(
                            top,
                            bottom
                        ))
                    )?;
                }
            }
        }
        Ok(Layer{ scene: resultant_scene, opacity: 255 })
    }
}
