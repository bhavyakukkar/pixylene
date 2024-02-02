use libpixylene::{
    pixylene::{ Pixylene, PixyleneNewDefaults, PixyleneImportDefaults },
    common::{ Coord, Pixel },
    elements::palette::Palette,
};

mod pixylene_tui;
mod utils;
mod modes;
mod actions;

use pixylene_tui::{ PixyleneTUI, Console };
use modes::Mode;

use crossterm::{ queue, cursor, terminal, event };
use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[command(arg_required_else_help = true, author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    //New { dimensions: Option<Coord>, palette: Option<Palette> },
    //Open { path: String },
    //Import { path: String, palette: Option<Palette> },
    New,
    Open{ path: String },
    Import{ path: String },
}

fn main() {
    use terminal::{
        size,
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    };
    use cursor::{ Hide, Show };
    use event::{ KeyCode };

    let mut project_file_path: Option<String> = None;
    let camera_dim = Coord {
        x: size().unwrap().1 as isize - 5isize,
        y: size().unwrap().0 as isize - 4isize,
    };
    let default_palette = Palette { colors: vec![
        Some(Pixel::from_hex(String::from("1d2021")).unwrap()),
        Some(Pixel::from_hex(String::from("fb543f")).unwrap()),
        Some(Pixel::from_hex(String::from("95c085")).unwrap()),
        Some(Pixel::from_hex(String::from("fac03b")).unwrap()),
        Some(Pixel::from_hex(String::from("0d6678")).unwrap()),
        Some(Pixel::from_hex(String::from("8f4673")).unwrap()),
        Some(Pixel::from_hex(String::from("8ba59b")).unwrap()),
        Some(Pixel::from_hex(String::from("a89984")).unwrap()),
        /*
        Some(Pixel{r: 81, g: 87, b: 109, a: 255}),
        Some(Pixel{r: 231, g: 130, b: 132, a: 255}),
        Some(Pixel{r: 166, g: 209, b: 137, a: 255}),
        Some(Pixel{r: 229, g: 200, b: 144, a: 255}),
        Some(Pixel{r: 140, g: 170, b: 238, a: 255}),
        Some(Pixel{r: 244, g: 184, b: 228, a: 255}),
        Some(Pixel{r: 129, g: 200, b: 190, a: 255}),
        Some(Pixel{r: 181, g: 191, b: 226, a: 255}),
        */
        Some(Pixel{r: 0, g: 0, b: 0, a: 0}),
    ] };
    let mut pixylene: Pixylene;
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New) => {
            let palette = default_palette.clone();
            let new_defaults = PixyleneNewDefaults {
                dim: Coord{ x: 64, y: 64 },
                camera_dim,
                camera_repeat: Coord{ x: 1, y: 2 },
                palette,
            };
            pixylene = Pixylene::new(
                &new_defaults,
            ).unwrap();
        },
        Some(Commands::Open{ path }) => {
            //let path = String::from("/home/bhavya/pictures/trash/snowbrick.bin");
            pixylene = Pixylene::open(
                &path,
            ).unwrap();
            project_file_path = Some(path.clone());
        },
        Some(Commands::Import{ path }) => {
            let palette = default_palette.clone();
            let import_defaults = PixyleneImportDefaults {
                camera_dim,
                palette,
            };
            pixylene = Pixylene::import(
                //"/home/bhavya/pictures/trash/snowbrick_rgba.png",
                &path,
                &import_defaults,
            ).unwrap();
        },
        None => {
            println!("new or open or import");
            return;
        }
    }

    //pixylene.project.camera.set_dim(camera_dim).unwrap();

    let mut app = PixyleneTUI {
        camera_corner: Coord{ x: 1, y: 2 },
        console_corner: Coord{ x: 1+camera_dim.x+1+1, y: 0 },
        statusline_corner: Coord{ x: 1+camera_dim.x+1, y: 2 },
        info_corner: Coord{ x: 2, y: 86 },
        pixylene: Some(pixylene),
        console: Console,
        last_action_name: None,
        project_file_path,
    };
    let mut mode = Mode::Normal;
    let mut last_mode = Mode::Normal;

    let mut stdout = std::io::stdout();
    enable_raw_mode().unwrap();
    queue!(
        stdout,
        EnterAlternateScreen,
        Hide,
    ).unwrap();

    actions::add_my_actions(app.pixylene.as_mut().unwrap());

    // remove later
    //app.draw_grid_border();
    //app.draw_info();
    // statusline decoration

    loop {
        match &mode {
            Mode::Splash => {
                todo!()
            },
            Mode::Command => {
                //bring cmdin() logic here for input loop,
                //since Esc must be accounted for for discarding a cmd
                //also this will fix bug happening when incr camera_dim.x by 1
                app.draw_statusline(&mode);
                let command = app.cmdin(":");
                match command.as_str() {
                    "undo" => { app.undo(); },
                    "redo" => { app.undo(); },
                    "w" => { app.save(); },
                    "ex" => { app.export(); }
                    "q" => { break; },
                    _ => { app.perform_action(&command); },
                }
                mode = last_mode;
            },
            Mode::Normal => {
                app.show();
                app.draw_statusline(&mode);
                if let Some(key) = app.getkey() {
                    if let KeyCode::Left = key { app.perform_action("cursor_left"); }
                    if let KeyCode::Down = key { app.perform_action("cursor_down"); }
                    if let KeyCode::Up = key { app.perform_action("cursor_up"); }
                    if let KeyCode::Right = key { app.perform_action("cursor_right"); }
                    if let KeyCode::Char(c) = key {
                        match c {
                            ':' => { mode = Mode::Command; },
                            'P' => { mode = Mode::Preview; },
                            'v' => { mode = Mode::GridSelect; },
                            //todo: change to Ctrl+V instead
                            'V' => { mode = Mode::PointSelect; },

                            'h' => { app.perform_action("cursor_left"); },
                            'j' => { app.perform_action("cursor_down"); },
                            'k' => { app.perform_action("cursor_up"); },
                            'l' => { app.perform_action("cursor_right"); },

                            'c' => { app.perform_action("toggle_cursor"); },

                            '1' => { app.perform_action("pencil1"); },
                            '2' => { app.perform_action("pencil2"); },
                            '3' => { app.perform_action("pencil3"); },
                            '4' => { app.perform_action("pencil4"); },
                            '5' => { app.perform_action("pencil5"); },
                            '6' => { app.perform_action("pencil6"); },
                            '7' => { app.perform_action("pencil7"); },
                            '8' => { app.perform_action("pencil8"); },
                            'f' => { app.perform_action("rectangular_fill"); },
                            'e' => { app.perform_action("eraser"); },

                            'y' => { app.perform_action("copy_paste_all_cursors"); },
                            'p' => { app.perform_action("copy_paste_all_cursors"); },

                            '-' => { app.pixylene.as_mut().unwrap().project.focus.layer = app.pixylene.as_mut().unwrap().project.focus.layer.checked_sub(1).unwrap_or(0); },
                            '+' => { app.pixylene.as_mut().unwrap().project.focus.layer += 1; },

                            ';' => { app.perform_prev_action(); },
                            'u' => { app.undo(); },
                            'r' => { app.redo(); },
                            //'q' => { break; },
                            _ => (),
                        }
                    }
                }
            },
            Mode::Preview => {
                app.preview();
                app.draw_statusline(&mode);
                //todo: add Esc
                if let Some(key) = app.getkey() {
                    if let KeyCode::Char(c) = key {
                        match c {
                            'P'|'q' => { mode = Mode::Normal; },
                            ':' => { mode = Mode::Command; },
                            'h' => { app.perform_action("focus_left"); },
                            'j' => { app.perform_action("focus_down"); },
                            'k' => { app.perform_action("focus_up"); },
                            'l' => { app.perform_action("focus_right"); },
                            '+' => { app.perform_action("zoom_in"); },
                            '-' => { app.perform_action("zoom_out"); },
                            _ => (),
                        }
                    }
                }
            },
            Mode::GridSelect => {
                todo!()
            },
            Mode::PointSelect => {
                todo!()
            },
        }
    }

    disable_raw_mode().unwrap();
    queue!(
        stdout,
        Show,
        LeaveAlternateScreen,
    ).unwrap();

    //app.export("/home/bhavya/pictures/trash/snowbrick_export.png").unwrap();
}
