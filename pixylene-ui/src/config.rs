use crate::ui::{ UiFn, Key, ReqUiFnMap };

use crossterm::{ event };
use serde::{ Deserialize };
use toml::{ from_str, de::Error };


#[derive(Debug, Deserialize)]
pub struct KeyXUiFnEntry {
    pub k: Key,
    pub f: Vec<UiFn>,
}

#[derive(Debug, Deserialize)]
pub struct UCoordEntry {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Deserialize)]
pub struct PaletteColorEntry {
    pub id: u8,
    pub c: String,
}

#[derive(Debug, Deserialize)]
pub struct PixyleneDefaultsConfig {
    pub dimensions: UCoordEntry,
    pub repeat: UCoordEntry,
    pub palette: Vec<PaletteColorEntry>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub required_keys: ReqUiFnMap,
    pub keys: Vec<KeyXUiFnEntry>,
    pub new_keys: bool,
    pub every_frame: Vec<UiFn>,
    pub defaults: PixyleneDefaultsConfig,
}

pub fn generate_config() -> Option<Result<Config, Error>> {
    let toml_str = r##"
    new_keys = false
    keys = [
        # { k = { c = { Char = ':' }, m = "CONTROL" }, f = "RunCommandSpecify" },
        { k = { c = { Char = 'x' }, m = "ALT" }, f = [ "RunCommandSpecify" ] },
        { k = { c = { Char = ';' } }, f = [ "RunLastAction" ] },
    ]
    every_frame = [ "PreviewFocusLayer", "UpdateStatusline" ]

    [required_keys]
    # discard_command = { c = "Esc" }
    discard_command = { c = { Char = 'g' }, m = "CONTROL" }
    start_command = { c = { Char = 'x' }, m = "ALT" }
    force_quit = { c = { Char = 'c' }, m = "CONTROL" }
    
    [defaults]
    dimensions = { x = 32, y = 32 }
    repeat = { x = 1, y = 2 }
    palette = [
        { id = 1 , c = "#140c1c" },
        { id = 2 , c = "#442434" },
        { id = 3 , c = "#30346d" },
        { id = 4 , c = "#4e4a4e" },
        { id = 5 , c = "#854c30" },
        { id = 6 , c = "#346524" },
        { id = 7 , c = "#d04648aa" },
        { id = 8 , c = "#757161" },
        { id = 9 , c = "#597dce" },
        { id = 10, c = "#d27d2c" },
        { id = 11, c = "#8595a1" },
        { id = 12, c = "#6daa2c" },
        { id = 13, c = "#d2aa99" },
        { id = 14, c = "#6dc2ca" },
        { id = 15, c = "#dad45e" },
        { id = 16, c = "#deeed6" },
    ]
    "##;

    if let Err(x) = toml::from_str::<Config>(toml_str) {
        println!("{}", x);
    }
    Some(from_str(toml_str))
}

impl Default for Config {
    fn default() -> Config {
        use event::{ KeyCode::*, KeyModifiers };
        type KM = KeyModifiers;
        type KUE = KeyXUiFnEntry;
        type PCE = PaletteColorEntry;
        type K = Key;

        Config {
            required_keys: ReqUiFnMap {
                start_command: K::new(Char(':'), None),
                discard_command: K::new(Esc, Some(KM::empty())),
                force_quit: K::new(Char('c'), Some(KM::CONTROL)),
            },
            new_keys: false,
            every_frame: vec![UiFn::PreviewFocusLayer, UiFn::UpdateStatusline],
            defaults: PixyleneDefaultsConfig {
                dimensions: UCoordEntry { x: 32, y: 32 },
                repeat: UCoordEntry { x: 1, y: 2 },
                palette: vec![
                    PCE { id: 1 , c: String::from("#140c1c") },
                    PCE { id: 2 , c: String::from("#442434") },
                    PCE { id: 3 , c: String::from("#30346d") },
                    PCE { id: 4 , c: String::from("#4e4a4e") },
                    PCE { id: 5 , c: String::from("#854c30") },
                    PCE { id: 6 , c: String::from("#346524") },
                    PCE { id: 7 , c: String::from("#d04648") },
                    PCE { id: 8 , c: String::from("#757161") },
                    PCE { id: 9 , c: String::from("#597dce") },
                    PCE { id: 10, c: String::from("#d27d2c") },
                    PCE { id: 11, c: String::from("#8595a1") },
                    PCE { id: 12, c: String::from("#6daa2c") },
                    PCE { id: 13, c: String::from("#d2aa99") },
                    PCE { id: 14, c: String::from("#6dc2ca") },
                    PCE { id: 15, c: String::from("#dad45e") },
                    PCE { id: 16, c: String::from("#deeed6") },
                ],
            },
            keys: vec![
                KUE { k: K::new(Char(':'), Some(KM::empty())), f: vec![UiFn::RunCommandSpecify] },
                KUE { k: K::new(Esc, Some(KM::empty())), f: vec![UiFn::Quit] },

                KUE { k: K::new(Char('h'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("cl"))] },
                KUE { k: K::new(Char('j'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("cd"))] },
                KUE { k: K::new(Char('k'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("cu"))] },
                KUE { k: K::new(Char('l'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("cr"))] },

                KUE { k: K::new(Char('h'), Some(KM::CONTROL)),
                      f: vec![UiFn::RunAction(String::from("cdl"))] },
                KUE { k: K::new(Char('j'), Some(KM::CONTROL)),
                      f: vec![UiFn::RunAction(String::from("cdd"))] },
                KUE { k: K::new(Char('k'), Some(KM::CONTROL)),
                      f: vec![UiFn::RunAction(String::from("cdu"))] },
                KUE { k: K::new(Char('l'), Some(KM::CONTROL)),
                      f: vec![UiFn::RunAction(String::from("cdr"))] },

                KUE { k: K::new(Char('r'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("crc"))] },

                KUE { k: K::new(Char('1'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil1"))] },
                KUE { k: K::new(Char('2'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil2"))] },
                KUE { k: K::new(Char('3'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil3"))] },
                KUE { k: K::new(Char('4'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil4"))] },
                KUE { k: K::new(Char('5'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil5"))] },
                KUE { k: K::new(Char('6'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil6"))] },
                KUE { k: K::new(Char('7'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil7"))] },
                KUE { k: K::new(Char('8'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil8"))] },
                KUE { k: K::new(Char('9'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil9"))] },
                KUE { k: K::new(Char('0'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("pencil10"))] },

                KUE { k: K::new(Char('u'), Some(KM::empty())),
                      f: vec![UiFn::Undo] },
                KUE { k: K::new(Char('r'), Some(KM::CONTROL)),
                      f: vec![UiFn::Redo] },

                KUE { k: K::new(Char('i'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("zoomin"))] },
                KUE { k: K::new(Char('o'), Some(KM::empty())),
                      f: vec![UiFn::RunAction(String::from("zoomout"))] },
            ],
        }
    }
}
