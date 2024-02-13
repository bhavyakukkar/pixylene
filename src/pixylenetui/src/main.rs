use libpixylene::{
    pixylene::{ Pixylene, PixyleneNewDefaults, PixyleneImportDefaults },
    common::{ Coord, Pixel },
    elements::palette::Palette,
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
                        if let Some(command) = app.console.cmdin(":") {
                            match command.as_str() {
                                "undo" => { app.undo(); },
                                "redo" => { app.undo(); },
                                "w" => { app.save(); },
                                "ex" => { app.export(); }
                                "q" => { break; },
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

                                    '-' => { app.perform_action("move_one_layer_up"); },
                                    '+' => { app.perform_action("move_one_layer_down"); },
                                    //'-' => { app.pixylene.as_mut().unwrap().project.focus.layer = app.pixylene.as_mut().unwrap().project.focus.layer.checked_sub(1).unwrap_or(0); },
                                    //'+' => { app.pixylene.as_mut().unwrap().project.focus.layer += 1; },

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
                    EmacsMode::Normal => {
                    }
                    EmacsMode::Layer => {
                    }
                    EmacsMode::Command => {
                    }
                    EmacsMode::Ooze{ color } => {
                    }
                    EmacsMode::Shape{ shape } => {
                    }
                    EmacsMode::Eraser{ shape } => {
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
