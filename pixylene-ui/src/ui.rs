use libpixylene::{ types::{ UCoord, PCoord }, project::{ OPixel } };
use pixylene_actions::{ LogType };


/// Trait needed for any User Interface Target to implement so that it can be controlled by
/// [`Controller`][c]
///
/// [c]: crate::Controller
pub trait UserInterface {
    fn initialize(&mut self);
    fn finalize(&mut self);

    /// Makes the target refresh between frames, returning whether target is still alive
    fn refresh(&mut self) -> bool;

    //fn set_camera_boundary(&mut self, boundary: Rectangle);
    //fn set_statusline_boundary(&mut self, boundary: Rectangle);
    //fn set_console_boundary(&mut self, boundary: Rectangle);

    /// Get the inputted key from the target
    ///
    /// Targets that block until key is received may always return Some(key), however targets that
    /// poll user-input may return None's until some key is received
    fn get_key(&self) -> Option<Key>;
    fn get_size(&self) -> PCoord;

    fn draw_camera(&mut self, dim: PCoord, buffer: Vec<OPixel>, show_cursors: bool,
                   boundary: &Rectangle);
    fn draw_paragraph(&mut self, paragraph: Vec<String>);

    fn draw_statusline(&mut self, statusline: &Statusline, boundary: &Rectangle);

    fn console_in(&mut self, message: &str, discard_key: &Key, boundary: &Rectangle) -> Option<String>;
    fn console_out(&mut self, message: &str, log_type: &LogType, boundary: &Rectangle);

    fn clear(&mut self, boundary: &Rectangle);
    fn clear_all(&mut self);
}

/// A Real Key on a keyboard that can be mapped to execute a [`UiFn`](crate::keybinds::UiFn) or
/// [`ReqUiFn`](crate::keybinds::ReqUiFn).
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

/// The Statusline
pub type Statusline = Vec<colored::ColoredString>;
