use libpixylene::{ types::{ UCoord, PCoord }, project::{ Project, OPixel } };
use pixylene_actions::{ action_manager::ActionManager, LogType };

use std::fmt;

pub trait Target {
    fn initialize(&mut self);
    fn finalize(&mut self);

    //fn set_camera_boundary(&mut self, boundary: Rectangle);
    //fn set_statusline_boundary(&mut self, boundary: Rectangle);
    //fn set_console_boundary(&mut self, boundary: Rectangle);

    fn get_key(&self) -> Key;
    fn get_size(&self) -> Rectangle;

    fn draw_camera(&self, dim: PCoord, buffer: Vec<OPixel>, show_cursors: bool,
                   boundary: &Rectangle);
    fn draw_statusline(&self, project: &Project, action_manager: &ActionManager, mode: &Mode,
                       session: &u8, boundary: &Rectangle);
    fn draw_paragraph(&self, paragraph: Vec<String>);

    fn console_clear(&self, boundary: &Rectangle);
    fn console_in(&self, message: &str, discard_key: &Key, boundary: &Rectangle) -> Option<String>;
    fn console_out(&self, message: &str, log_type: LogType, boundary: &Rectangle);

}

//leaving this here
//Cursors similar to helix where clone_cursor_left clones
//the leftmost cursor/s, and vice versa wrt up, down & right


/// A Real Key on a keyboard that can be mapped to execute a [`KeyFn`](KeyFn).
///
/// `Note:` This was made primarily with compatibility to [`crossterm`](crossterm) in mind and
/// hence is simply a type alias to crossterm's [`KeyEvent`](crossterm::event::KeyEvent).
///
/// Other target implementations require manual association.
pub type Key = crossterm::event::KeyEvent;


#[derive(Copy, Clone)]
pub struct Rectangle {
    pub start: UCoord,
    pub size: PCoord,
}

pub enum Mode {
    Normal,
    Ooze,
    Shape,
    Layer,
    Command,
    Cursors,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Mode::*;
        match self {
            Normal => write!(f, "{}", "Normal"),
            Ooze => write!(f, "{}", "Ooze"),
            Shape => write!(f, "{}", "Shape"),
            Layer => write!(f, "{}", "Layer"),
            Command => write!(f, "{}", "Command"),
            Cursors => write!(f, "{}", "Cursors"),
        }
    }
}
