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
    fn import_rgb_eight() {
        let png = file::PngFile::read(
            &std::path::PathBuf::from("../assets/images/rgb_8bit_16x16.png")).unwrap();
        let canvas = png.to_canvas().unwrap();
        if let project::LayersType::True(ref layers) = &canvas.layers {
            assert_eq!(layers.len(), 1);
            println!("{}", canvas.merged_true_scene(None));
            png.write(&std::path::PathBuf::from("/tmp/rgb_8bit_16x16_new.png")).unwrap();
        } else {
            assert!(false);
        }
    }

    #[test]
    fn import_indexed_eight() {
        let png = file::PngFile::read(
            &std::path::PathBuf::from("../assets/images/indexed_8bit_33x33.png")).unwrap();
        let canvas = png.to_canvas().unwrap();
        if let project::LayersType::Indexed(layers) = canvas.layers {
            assert_eq!(layers.len(), 1);
            let project::Layer { scene, .. } = &layers[0];
            println!("{}", scene);
            png.write(&std::path::PathBuf::from("/tmp/indexed_8bit_33x33_new.png")).unwrap();
        } else {
            assert!(false);
        }
    }

    #[test]
    fn export_true_canvas() {
        import_rgb_eight();
        let canvas = file::PngFile::read(
            &std::path::PathBuf::from("../assets/images/rgb_8bit_16x16.png")).unwrap().to_canvas().unwrap();
        let png = file::PngFile::from_canvas(&canvas).unwrap();
        png.write(&std::path::PathBuf::from("/tmp/rgb_8bit_16x16_new2.png")).unwrap();
    }

    #[test]
    fn export_indexed_canvas() {
        import_indexed_eight();
        let canvas = file::PngFile::read(
            &std::path::PathBuf::from("../assets/images/indexed_8bit_33x33.png")).unwrap().to_canvas().unwrap();
        let png = file::PngFile::from_canvas(&canvas).unwrap();
        png.write(&std::path::PathBuf::from("/tmp/indexed_8bit_33x33_new2.png")).unwrap();
    }
}
