extern crate savefile;
#[macro_use]
extern crate savefile_derive;

pub mod utils;

pub mod types;

pub mod project;

pub mod file;

mod pixylene;
pub use pixylene::*;

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn true_canvas(){
    //    let mut pixylene = Pixylene::new(&PixyleneDefaults{
    //        dim: types::PCoord::new(4,4).unwrap(),
    //        palette: project::Palette::from(&vec![
    //            (1 , "#140c1c"),
    //            (2 , "#442434"),
    //            (3 , "#30346d"),
    //            (4 , "#4e4a4e"),
    //            (5 , "#854c30"),
    //            (6 , "#346524"),
    //            (7 , "#d04648"),
    //            (8 , "#757161"),
    //            (9 , "#597dce"),
    //            (10, "#d27d2c"),
    //            (11, "#8595a1"),
    //            (12, "#6daa2c"),
    //            (13, "#d2aa99"),
    //            (14, "#6dc2ca"),
    //            (15, "#dad45e"),
    //            (16, "#deeed6"),
    //        ]).unwrap(),
    //        repeat: types::PCoord::new(1,1).unwrap(),
    //    }, false);
    //    pixylene.project.canvas_mut().inner_mut()
    //        .layers_true_mut()
    //        .new_layer(Some(types::TruePixel{ r: 47, g: 128, b: 255, a: 255 }))
    //        .unwrap();
    //    let json = serde_json::to_string(&pixylene.project.canvas()).unwrap();
    //    println!("{}", json);
    //}

    //#[test]
    //fn indexed_canvas(){
    //    let mut pixylene = Pixylene::new(&PixyleneDefaults{
    //        dim: types::PCoord::new(4,4).unwrap(),
    //        palette: project::Palette::from(&vec![
    //            (1 , "#140c1c"),
    //            (2 , "#442434"),
    //            (3 , "#30346d"),
    //            (4 , "#4e4a4e"),
    //            (5 , "#854c30"),
    //            (6 , "#346524"),
    //            (7 , "#d04648"),
    //            (8 , "#757161"),
    //            (9 , "#597dce"),
    //            (10, "#d27d2c"),
    //            (11, "#8595a1"),
    //            (12, "#6daa2c"),
    //            (13, "#d2aa99"),
    //            (14, "#6dc2ca"),
    //            (15, "#dad45e"),
    //            (16, "#deeed6"),
    //        ]).unwrap(),
    //        repeat: types::PCoord::new(1,1).unwrap(),
    //    }, true);
    //    pixylene.project.canvas_mut().inner_mut()
    //        .layers_indexed_mut()
    //        .new_layer(Some(types::IndexedPixel(0)))
    //        .unwrap();
    //    let json = serde_json::to_string(&pixylene.project.canvas()).unwrap();
    //    println!("{}", json);
    //}

    #[test]
    fn indexed_eight_to_scene() {
        let png = file::PngFile::read(
            &std::path::PathBuf::from("../assets/images/indexed_8bit_33x33.png")).unwrap();
        let canvas = png.to_scene().unwrap();
        if let project::LayersType::Indexed(layers) = canvas.layers {
            assert_eq!(layers.len(), 1);
            let project::Layer { scene, .. } = &layers[0];
            println!("{}", scene);
        } else {
            assert!(false);
        }
    }
}
