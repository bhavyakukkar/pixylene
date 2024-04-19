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

    #[test]
    fn main(){
        let mut pixylene = Pixylene::new(&PixyleneDefaults{
            dim: types::PCoord::new(4,4).unwrap(),
            palette: project::Palette::from(&vec![
                (1 , "#140c1c"),
                (2 , "#442434"),
                (3 , "#30346d"),
                (4 , "#4e4a4e"),
                (5 , "#854c30"),
                (6 , "#346524"),
                (7 , "#d04648"),
                (8 , "#757161"),
                (9 , "#597dce"),
                (10, "#d27d2c"),
                (11, "#8595a1"),
                (12, "#6daa2c"),
                (13, "#d2aa99"),
                (14, "#6dc2ca"),
                (15, "#dad45e"),
                (16, "#deeed6"),
            ]).unwrap(),
            repeat: types::PCoord::new(1,1).unwrap(),
        });
        pixylene.project.canvas.new_layer(Some(types::Pixel{ r: 47, g: 128, b: 255, a: 255 }))
            .unwrap();
        let json = serde_json::to_string(&pixylene.project.canvas).unwrap();
        println!("{}", json);
    }
}
