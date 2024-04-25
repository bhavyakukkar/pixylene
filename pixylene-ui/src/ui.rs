use libpixylene::{ types::{ UCoord, PCoord }, project::{ OPixel } };
use pixylene_actions::{ LogType };
use serde::{ Deserialize };
use std::collections::HashMap;
use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use std::path::PathBuf;


/// Trait needed for any User Interface Target to implement so that it can be controlled by
/// [`Controller`][c]
///
/// [c]: crate::Controller
pub trait UserInterface {
    fn initialize(&mut self);
    fn finalize(&mut self);

    /// Makes the target refresh between frames, returning whether target is still alive
    fn refresh(&mut self) -> bool;

    /// Get the inputted key from the target
    ///
    /// Targets that block until key is received may always return Some(key), however targets that
    /// poll user-input may return None's until some key is received
    fn get_key(&self) -> Option<KeyInfo>;
    fn get_size(&self) -> PCoord;

    fn draw_camera(&mut self, dim: PCoord, buffer: Vec<OPixel>, show_cursors: bool,
                   boundary: &Rectangle);
    fn draw_paragraph(&mut self, paragraph: Vec<colored::ColoredString>, boudnary: &Rectangle);

    fn draw_statusline(&mut self, statusline: &Statusline, boundary: &Rectangle);

    fn console_in(&mut self, message: &str, discard_key: &Key, boundary: &Rectangle) -> Option<String>;
    fn console_out(&mut self, message: &str, log_type: &LogType, boundary: &Rectangle);

    fn clear(&mut self, boundary: &Rectangle);
    fn clear_all(&mut self);
}

pub enum KeyInfo {
    #[allow(dead_code)]
    Key(Key),
    #[allow(dead_code)]
    UiFn(UiFn),
}

/// A Real Key on a keyboard that can be mapped to execute a [`UiFn`](crate::keybinds::UiFn) or
/// [`ReqUiFn`](crate::keybinds::ReqUiFn).
///
/// `Note`: This was made primarily with compatibility to [`crossterm`](crossterm) in mind and
/// hence is simply a type alias to crossterm's [`KeyEvent`](crossterm::event::KeyEvent).
///
/// Other target implementations require manual association.
//pub type Key = crossterm::event::KeyEvent;
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Key {
    #[serde(alias = "c")]
    code: KeyCode,
    #[serde(alias = "m")]
    modifiers: Option<KeyModifiers>,
}

impl Key {
    pub fn new(code: KeyCode, modifiers: Option<KeyModifiers>) -> Key {
        let key_event = KeyEvent::new(code, modifiers.unwrap_or(KeyModifiers::empty()));
        Key {
            code: key_event.code,
            modifiers: Some(key_event.modifiers),
        }
    }

    pub fn code(&self) -> KeyCode {
        self.code.clone()
    }

    pub fn modifiers(&self) -> KeyModifiers {
        self.modifiers.unwrap_or(KeyModifiers::empty()).clone()
    }
}

impl From<KeyEvent> for Key {
    fn from(item: KeyEvent) -> Key {
        Key { code: item.code, modifiers: Some(item.modifiers) }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //let code = toml::to_string(&self.code).map_err(|err| panic!("{:?}", self.code)).unwrap();
        write!(
            f,
            "{}{}",
            self.modifiers.unwrap_or(KeyModifiers::empty()).iter_names()
                .map(|x| x.0.to_owned()).reduce(|a,b| format!("{a}|{b}"))
                .map(|s| s + " ").unwrap_or("".to_owned()),
            if let KeyCode::Char(c) = self.code {
                String::from(c)
            } else if let KeyCode::F(u) = self.code {
                format!("F{}", u)
            } else if let KeyCode::Media(m) = self.code {
                format!("{:?}", m)
            } else if let KeyCode::Modifier(m) = self.code {
                format!("{:?}", m)
            } else {
                format!("{:?}", self.code)
            }
                //    toml::to_string(&KeyEvent::new(self.code, KeyModifiers::empty()))
                //        .unwrap()
                //        .lines()
                //        .take(1)
                //        .map(|s| s.to_owned())
                //        .fold(String::from(""), |a,b| a.to_owned() + &b)[7..]
                //        .to_string()
                //}
        )
    }
}

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub start: UCoord,
    pub size: PCoord,
}

/// The Statusline
pub type Statusline = Vec<colored::ColoredString>;

/// The map of namespace-names to secondary maps of [`Keys`](super::target::Key) to the ordered
/// sequence of [`UiFns`](UiFn) they will execute when pressed
pub type KeyMap = HashMap<Option<String>, HashMap<Key, Vec<UiFn>>>;

#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum UiFn {
    #[serde(alias = "new")]
    New{
        #[serde(alias = "w")]
        width: Option<u16>,
        #[serde(alias = "h")]
        height: Option<u16>
    },

    #[serde(alias = "e")]
    OpenCanvas{
        path: PathBuf
    },
    #[serde(alias = "E")]
    OpenCanvasSpecify,

    #[serde(alias = "ep")]
    OpenProject{
        path: PathBuf
    },
    #[serde(alias = "Ep")]
    OpenProjectSpecify,

    #[serde(alias = "import")]
    Import{
        path: String
    },
    ImportSpecify,

    #[serde(alias = "q")]
    Quit,

    #[serde(alias = "q!")]
    ForceQuit,

    GoToSession{
        index: u8
    },
    GoToNextSession,
    GoToPrevSession,

    SaveCanvas,
    SaveProject,
    Export,

    Undo,
    Redo,

    EnterNamespace{
        #[serde(alias = "n")]
        name: Option<String>
    },
    EnterDefaultNamespace,
    RunKey{
        key: Key
    },

    RunCommand{
        cmd: String
    },
    RunCommandSpecify,

    #[serde(alias = "a")]
    RunAction{
        #[serde(alias = "n")]
        name: String,
    },
    RunActionSpecify,
    RunLastAction,

    PreviewFocusLayer,
    PreviewProject,
    PrintCanvasJson,

    #[serde(alias = "keymap")]
    PrintKeybindMap,

    UpdateStatusline,
}

/// The mapping of [`Keys`](Key) to functions mandatorily required by the app. 
#[derive(Debug, Deserialize)]
pub struct ReqUiFnMap {
    pub force_quit: Key,
    pub start_command: Key,
    pub discard_command: Key,
}

/// Generic color type to be used in targets, wrapper around colored's [`Color`](colored::Color)
pub struct Color(pub Option<colored::Color>);

impl From<Color> for crossterm::style::Color {
    fn from(item: Color) -> crossterm::style::Color {
        use crossterm::style::Color::*;

        match item.0 {
            Some(color) => match color {
                colored::Color::Black => Black,
                colored::Color::Red => Red,
                colored::Color::Green => Green,
                colored::Color::Yellow => Yellow,
                colored::Color::Blue => Blue,
                colored::Color::Magenta => Magenta,
                colored::Color::Cyan => Cyan,
                colored::Color::White => White,
                colored::Color::BrightBlack => DarkGrey,
                colored::Color::BrightRed => DarkRed,
                colored::Color::BrightGreen => DarkGreen,
                colored::Color::BrightYellow => DarkYellow,
                colored::Color::BrightBlue => DarkBlue,
                colored::Color::BrightMagenta => DarkMagenta,
                colored::Color::BrightCyan => DarkCyan,
                colored::Color::BrightWhite => Grey,
                colored::Color::TrueColor { r, g, b } => Rgb { r, g, b },
            },
            None => Reset,
        }
    }
}

impl From<Color> for u32 {
    fn from(item: Color) -> u32 {
        match item.0 {
            Some(color) => match color {
                colored::Color::Black => 0x000000,
                colored::Color::Red => 0xFF0000,
                colored::Color::Green => 0x00FF00,
                colored::Color::Yellow => 0xFFFF00,
                colored::Color::Blue => 0x0000FF,
                colored::Color::Magenta => 0xFF00FF,
                colored::Color::Cyan => 0x00FFFF,
                colored::Color::White => 0xFFFFFF,
                colored::Color::BrightBlack => 0x808080,
                colored::Color::BrightRed => 0x800000,
                colored::Color::BrightGreen => 0x008000,
                colored::Color::BrightYellow => 0x808000,
                colored::Color::BrightBlue => 0x000080,
                colored::Color::BrightMagenta => 0x800080,
                colored::Color::BrightCyan => 0x008080,
                colored::Color::BrightWhite => 0xC0C0C0,
                colored::Color::TrueColor { r, g, b } => {
                    u32::from(r)*256*256 + u32::from(g)*256 + u32::from(b)
                },
            },
            None => 0x000000,
        }
    }
}
