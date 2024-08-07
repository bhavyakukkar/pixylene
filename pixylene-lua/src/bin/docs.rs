use pixylene_lua::values::{project, types};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use tealr::TypeWalker;

fn main() {
    //over here
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expecting only 1 positional argument\n");
        println!(
            "Usage: pixylene-lua-docs FILE\nGenerate Pixylene's Lua API JSON Documentation to \
                 file"
        );
        exit(1);
    } else if args[1].eq(&String::from("-h"))
        || args[1].eq(&String::from("--help"))
        || args[1].eq(&String::from("-help"))
    {
        println!(
            "Usage: pixylene-lua-docs FILE\nGenerate Pixylene's Lua API JSON Documentation to \
                 file"
        );
        exit(0);
    }

    let path = Path::new(&args[1]);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Invalid file path: {}", err);
            exit(1);
        }
    };

    let values = TypeWalker::new()
        .process_type::<types::Coord>()
        .process_type::<types::UCoord>()
        .process_type::<types::PCoord>()
        .process_type::<types::TruePixel>()
        .process_type::<types::IndexedPixel>()
        .process_type::<types::BlendMode>()
        .process_type::<project::TrueScene>()
        .process_type::<project::IndexedScene>()
        .process_type::<project::TrueLayer>()
        .process_type::<project::IndexedLayer>()
        .process_type::<project::Palette>()
        .process_type::<project::Canvas>()
        .process_type::<project::Project>();

    let json =
        serde_json::to_string_pretty(&values).expect("serde_json failed to serialize the data");

    file.write_all(json.as_bytes())
        .expect("failed to write to file");
}
