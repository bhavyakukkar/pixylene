use libpixylene::{ types::{ UCoord, PCoord }, project::{ OPixel } };
use pixylene_actions::{ LogType };
use serde::{ Deserialize, Serialize };
use std::{ collections::HashMap, path::PathBuf };
use clap::Subcommand;


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
    Key(crossterm::event::KeyEvent),
    #[allow(dead_code)]
    UiFn(UiFn),
}


// needed to serialize Key since KeyMap doesn't implement Serialize
// all thanks to https://github.com/serde-rs/serde/issues/1316
mod string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>
    {
        String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
    }
}

/// A Real Key on a keyboard that can be mapped to trigger a [`UiFn`](crate::keybinds::UiFn) or
/// associate with a [`ReqUiFn`](crate::keybinds::ReqUiFn).
///
/// `Note`: This was made primarily with compatibility to [`crossterm`](crossterm) in mind and
/// hence is a wrapper around keymap-rs's [`KeyMap`](keymap::KeyMap) type (not to be confused with
/// this crate's [`KeyMap`](KeyMap)), allowing easy conversion from the Key events of the
/// [`crossterm`] & [`termion`] libraries.
///
/// Other target implementations require manual association.
#[derive(Eq, Hash, Deserialize, PartialEq, Debug)]
pub struct Key(
    keymap::KeyMap
);

impl std::str::FromStr for Key {
    type Err = pom::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        keymap::backend::parse(s).map(|k| Key(k))
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Clone for Key {
    fn clone(&self) -> Self {
        Key(keymap::backend::parse(&self.0.to_string()).unwrap())
        //KeySer(Key::from(crossterm::event::KeyEvent::from(self.0)))
    }
}

impl From<crossterm::event::KeyEvent> for Key {
    fn from(item: crossterm::event::KeyEvent) -> Key {
        Key(keymap::KeyMap::from(item))
    }
}

impl From<keymap::KeyMap> for Key {
    fn from(item: keymap::KeyMap) -> Key {
        Key(item)
    }
}

impl Into<keymap::KeyMap> for Key {
    fn into(self) -> keymap::KeyMap {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub start: UCoord,
    pub size: PCoord,
}

/// The Statusline
pub type Statusline = Vec<colored::ColoredString>;

/// The map of namespace-names to secondary maps of [`Keys`](Key) to the ordered
/// sequence of [`UiFns`](UiFn) they will execute when pressed
pub type KeyMap = HashMap<Option<String>, HashMap<Key, Vec<UiFn>>>;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize, Serialize, Subcommand)]
pub enum UiFn {
    #[serde(alias = "new")]
    //not needed: #[command(visible_alias = "new")]
    New{
        #[serde(alias = "w")]
        width: Option<u16>,
        #[serde(alias = "h")]
        height: Option<u16>,
        #[serde(alias = "i")]
        #[clap(long, short, action)]
        indexed: bool,
    },

    #[serde(alias = "e")]
    #[command(visible_alias = "e")]
    OpenCanvas{
        path: PathBuf
    },

    #[serde(alias = "E")]
    #[command(visible_alias = "E")]
    OpenCanvasSpecify,

    #[serde(alias = "ep")]
    #[command(visible_alias = "ep")]
    OpenProject{
        path: PathBuf
    },

    #[serde(alias = "Ep")]
    #[command(visible_alias = "Ep")]
    OpenProjectSpecify,

    #[serde(alias = "import")]
    //not needed: #[command(visible_alias = "import")]
    Import{
        path: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
    },

    //#[serde(alias = "Import")]
    //#[command(visible_alias = "Import")]
    //ImportSpecify,

    #[serde(alias = "q")]
    #[command(visible_alias = "q")]
    Quit,

    #[serde(alias = "q!")]
    #[command(visible_alias = "q!")]
    ForceQuit,

    #[serde(alias = "ses")]
    #[command(visible_alias = "ses")]
    GoToSession{
        index: u8
    },

    #[serde(alias = "nses")]
    #[command(visible_alias = "nses")]
    GoToNextSession,

    #[serde(alias = "pses")]
    #[command(visible_alias = "pses")]
    GoToPrevSession,

    #[serde(alias = "w")]
    #[command(visible_alias = "w")]
    SaveCanvas,

    #[serde(alias = "wp")]
    #[command(visible_alias = "wp")]
    SaveProject,

    #[serde(alias = "export")]
    //not needed: #[command(visible_alias = "export")]
    Export,

    #[serde(alias = "undo")]
    //not needed: #[command(visible_alias = "undo")]
    Undo,

    #[serde(alias = "redo")]
    //not needed: #[command(visible_alias = "redo")]
    Redo,

    #[serde(alias = "ns")]
    #[command(visible_alias = "ns")]
    EnterNamespace{
        #[serde(alias = "n")]
        name: Option<String>
    },

    #[serde(alias = "dns")]
    #[command(visible_alias = "dns")]
    EnterDefaultNamespace,

    #[serde(alias = "key")]
    #[command(visible_alias = "key")]
    RunKey{
        #[serde(with = "string")]
        key: Key,
    },

    #[serde(alias = "cmd")]
    #[command(visible_alias = "cmd")]
    RunCommand{
        cmd: String
    },

    #[serde(alias = "Cmd")]
    #[command(visible_alias = "Cmd")]
    RunCommandSpecify,

    #[serde(alias = "a")]
    #[command(visible_alias = "a")]
    RunAction{
        #[serde(alias = "n")]
        name: String,
    },

    #[serde(alias = "A")]
    #[command(visible_alias = "A")]
    RunActionSpecify,

    #[serde(alias = "la")]
    #[command(visible_alias = "la")]
    RunLastAction,

    #[serde(alias = "l")]
    #[command(visible_alias = "l")]
    RunLua{
        #[serde(alias = "s")]
        statement: String,
    },

    #[serde(alias = "L")]
    #[command(visible_alias = "L")]
    RunLuaSpecify,

    #[serde(alias = "dl")]
    #[command(visible_alias = "dl")]
    PreviewFocusLayer,

    #[serde(alias = "dp")]
    #[command(visible_alias = "dp")]
    PreviewProject,

    #[serde(alias = "ds")]
    #[command(visible_alias = "ds")]
    DrawStatusline,

    #[serde(alias = "pc")]
    #[command(visible_alias = "pc")]
    PrintCanvasJson,

    #[serde(alias = "ln")]
    #[command(visible_alias = "ln")]
    ListNamespaces,

    #[serde(alias = "lk")]
    #[command(visible_alias = "lk")]
    ListKeybindMap {
        #[serde(alias = "n")]
        namespace: Option<String>,
    },

    #[serde(alias = "lc")]
    #[command(visible_alias = "lc")]
    ListCommands,
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
