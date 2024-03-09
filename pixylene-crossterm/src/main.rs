use libpixylene::{
    pixylene::{ Pixylene, PixyleneNewDefaults, PixyleneImportDefaults },
    types::{ Coord, Pixel },
    project::Palette,
};

mod pixylene_tui;
mod utils;
mod modes;
mod raw_actions;
mod tui_actions;

use pixylene_tui::{ PixyleneTUI, Console };
use modes::*;

use crossterm::{ queue, cursor, terminal, event };
use clap::{ Parser, Subcommand };
use std::collections::HashMap;

#[derive(Parser)]
#[command(arg_required_else_help = true, author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<StartType>,
}

#[derive(Subcommand)]
enum StartType {
    //New { dimensions: Option<Coord>, palette: Option<Palette> },
    //Open { path: String },
    //Import { path: String, palette: Option<Palette> },
    New,
    Open{ path: String },
    Import{ path: String },
}

enum Behavior {
    VimLike,
    EmacsLike,
}
impl Behavior {
    const VIM_DISCARD_KEY: event::KeyEvent = event::KeyEvent::new(
        event::KeyCode::Esc,
        event::KeyModifiers::empty()
    );

    const EMACS_DISCARD_KEY: event::KeyEvent = event::KeyEvent::new(
        event::KeyCode::Char('g'),
        event::KeyModifiers::CONTROL
    );

    fn discard_key(&self) -> event::KeyEvent {
        match self {
            Behavior::VimLike => Self::VIM_DISCARD_KEY,
            Behavior::EmacsLike => Self::EMACS_DISCARD_KEY,
        }
    }
}


/*
struct Echo;
impl pixylene_actions::Action for Echo {
    fn perform_action(
        &mut self,
        _project: &mut libpixylene::project::Project,
        console: &pixylene_ui::Console
    ) -> Result<Vec<pixylene_actions::Change>, pixylene_actions::ActionError> {
        if let Some(string) = (console.cmdin)(String::from(":echo ")) {
            (console.cmdout)(string, pixylene_ui::LogType::Info);
        }
        Ok(Vec::new())
    }
}

fn main() {
    use libpixylene::{
        Pixylene,
        pixylene::PixyleneNewDefaults, 
        project::{ Project, Palette },
        types::{ PCoord, Pixel },
    };
    use pixylene_actions::action_manager::ActionManager;
    use pixylene_ui::{ Console, LogType };

    use std::collections::HashMap;


    let pixylene_defaults = PixyleneNewDefaults {
        dim: PCoord::from((10, 10)),
        camera_dim: PCoord::from((10, 10)),
        camera_repeat: (1, 2),
        palette: Palette{
            colors: vec![Some(Pixel{r:255,g:255,b:255,a:255}),Some(Pixel{r:0,g:0,b:0,a:255})]
        },
    };
    let mut pixylene = Pixylene::new(&pixylene_defaults).unwrap();
    let mut project: &mut Project = &mut pixylene.project;

    let mut action_manager = ActionManager::new(HashMap::new());

    let console = Console {
        cmdin: |message: String| -> Option<String> {
            println!("{}?", message);
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            Some(buffer)
        },
        cmdout: |message: String, log_type: LogType| {
            println!("{:?} => {}", log_type, message);
        },
    };

    //^ all that, for these 2 lines:
    action_manager.add_action(String::from("echo"), Box::new(Echo));
    action_manager.perform(&mut project, &console, String::from("echo"));
}
*/


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
    let behavior = Behavior::VimLike;

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
        Some(Pixel{r: 0, g: 0, b: 0, a: 0}),
    ] };
    let mut pixylene: Pixylene;
    let cli = Cli::parse();

    match &cli.command {
        Some(StartType::New) => {
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
        Some(StartType::Open{ path }) => {
            pixylene = Pixylene::open(
                &path,
            ).unwrap();
            project_file_path = Some(path.clone());
        },
        Some(StartType::Import{ path }) => {
            let palette = default_palette.clone();
            let import_defaults = PixyleneImportDefaults {
                camera_dim,
                palette,
            };
            pixylene = Pixylene::import(
                &path,
                &import_defaults,
            ).unwrap();
        },
        None => {
            println!("new or open or import");
            return;
        }
    }

    let mut app = PixyleneTUI::new(
        /* console_corner */Coord{ x: 1+camera_dim.x+1+1, y: 0 },
        /* camera_corner: */Coord{ x: 1, y: 2 },
        /* statusline_corner: */Coord{ x: 1+camera_dim.x+1, y: 2 },
        /* info_corner: */Coord{ x: 2, y: 86 },
        Some(pixylene),
        project_file_path,
        behavior.discard_key().clone(),
    );
    let mut vim_mode = VimMode::Normal;
    let mut last_vim_mode = VimMode::Normal;
    let mut emacs_mode = EmacsMode::Normal;

    let mut stdout = std::io::stdout();
    enable_raw_mode().unwrap();
    queue!(
        stdout,
        EnterAlternateScreen,
        Hide,
    ).unwrap();

    match behavior {
        Behavior::VimLike => {
            loop {
                match &vim_mode {
                    VimMode::Splash => {
                        todo!()
                    },
                    VimMode::Command => {
                        app.draw_statusline(&vim_mode);
                        if let Some(command) = app.cmdin(":") {
                            match command.as_str() {
                                "undo" => { app.undo(); },
                                "redo" => { app.undo(); },
                                "w" => { app.save(); },
                                "ex" => { app.export(); }
                                "q" => { break; },
                                "" => { },
                                _ => { app.perform_action(&command); },
                            }
                        }
                        vim_mode = last_vim_mode;
                    },
                    VimMode::Normal => {
                        app.show();
                        app.draw_statusline(&vim_mode);
                        if let Some(key) = app.getkey() {
                            if let KeyCode::Left = key { app.perform_action("cursor_left"); }
                            if let KeyCode::Down = key { app.perform_action("cursor_down"); }
                            if let KeyCode::Up = key { app.perform_action("cursor_up"); }
                            if let KeyCode::Right = key { app.perform_action("cursor_right"); }
                            if let KeyCode::Char(c) = key {
                                match c {
                                    ':' => { vim_mode = VimMode::Command; },
                                    'P' => { vim_mode = VimMode::Preview; },
                                    'v' => { vim_mode = VimMode::GridSelect; },
                                    //todo: change to Ctrl+V instead
                                    'V' => { vim_mode = VimMode::PointSelect; },

                                    'h' => { app.perform_action("cursor_left"); },
                                    'j' => { app.perform_action("cursor_down"); },
                                    'k' => { app.perform_action("cursor_up"); },
                                    'l' => { app.perform_action("cursor_right"); },

                                    //'c' => { app.perform_action("toggle_cursor"); },
                                    'c' => { app.perform_action("circular_outline"); },

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

                                    '-' => { app.perform_action("move_one_layer_up"); },
                                    '+' => { app.perform_action("move_one_layer_down"); },

                                    ';' => { app.perform_prev_action(); },
                                    'u' => { app.undo(); },
                                    'r' => { app.redo(); },
                                    //'q' => { break; },
                                    _ => (),
                                }
                            }
                        }
                    },
                    VimMode::Preview => {
                        app.preview();
                        app.draw_statusline(&vim_mode);
                        //todo: add Esc
                        if let Some(key) = app.getkey() {
                            if let KeyCode::Char(c) = key {
                                match c {
                                    'P'|'q' => { vim_mode = VimMode::Normal; },
                                    ':' => { vim_mode = VimMode::Command; },
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
                    VimMode::GridSelect => {
                        todo!()
                    },
                    VimMode::PointSelect => {
                        todo!()
                    },
                }
            }
        },
        Behavior::EmacsLike => {
            loop {
                match &emacs_mode {
                    //Modes that use the equipped color
                    EmacsMode::Normal => {
                        /*
                        
                        C-s => emacs_mode = EmacsMode::Shape { Some(last_shape) },
                        C-S-s => emacs_mode = EmacsMode::Shape { None },

                        */
                    }
                    EmacsMode::Ooze => {
                    }
                    EmacsMode::Shape{ shape } => {
                    }

                    //Modes that do not use the equipped color
                    EmacsMode::Layer => {
                        /*

                        n => new layer
                        d => delete layer
                        r => rename layer
                        c => clone layer
                        - => go to lower layer
                        + => go to upper layer

                        */
                    }
                    EmacsMode::Preview => {
                    }
                    EmacsMode::Command => {
                    }
                    EmacsMode::Cursors => {
                    }
                }
            }
        },
    }

    disable_raw_mode().unwrap();
    queue!(
        stdout,
        Show,
        LeaveAlternateScreen,
    ).unwrap();

    //app.export("/home/bhavya/pictures/trash/snowbrick_export.png").unwrap();
}
