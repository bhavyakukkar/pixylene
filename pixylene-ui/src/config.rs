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
pub struct NamespaceXKeysEntry {
    pub name: Option<String>,
    pub keys: Vec<KeyXUiFnEntry>,
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
    pub keys: Vec<NamespaceXKeysEntry>,
    pub new_keys: bool,
    pub every_frame: Vec<UiFn>,
    pub defaults: PixyleneDefaultsConfig,
}

impl Config {
    pub fn from(toml: &String) -> Result<Config, Error> {
        from_str(toml)
    }
}

impl Default for Config {
    fn default() -> Config {
        use event::{ KeyCode::*, KeyModifiers };
        type KM = KeyModifiers;
        type KUE = KeyXUiFnEntry;
        type NKE = NamespaceXKeysEntry;
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
                NKE {
                    name: None,
                    keys: vec![
                        KUE { k: K::new(Char('h'), Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("cursors_left"))] },
                        KUE { k: K::new(Char('j'), Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("cursors_down"))] },
                        KUE { k: K::new(Char('k'), Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("cursors_up"))] },
                        KUE { k: K::new(Char('l'), Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("cursors_right"))] },

                        KUE { k: K::new(Left, Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("cursors_left"))] },
                        KUE { k: K::new(Down, Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("cursors_down"))] },
                        KUE { k: K::new(Up, Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("cursors_up"))] },
                        KUE { k: K::new(Right, Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("cursors_right"))] },

                        KUE { k: K::new(Char('h'), Some(KM::CONTROL)),
                              f: vec![UiFn::RunAction(String::from("cursors_dup_left"))] },
                        KUE { k: K::new(Char('j'), Some(KM::CONTROL)),
                              f: vec![UiFn::RunAction(String::from("cursors_dup_down"))] },
                        KUE { k: K::new(Char('k'), Some(KM::CONTROL)),
                              f: vec![UiFn::RunAction(String::from("cursors_dup_up"))] },
                        KUE { k: K::new(Char('l'), Some(KM::CONTROL)),
                              f: vec![UiFn::RunAction(String::from("cursors_dup_right"))] },

                        KUE { k: K::new(Char('r'), Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("cursors_reset"))] },

                        KUE { k: K::new(Enter, Some(KM::empty())),
                              f: vec![UiFn::RunAction(String::from("pencil"))] },

                        KUE { k: K::new(Char('u'), Some(KM::empty())),
                              f: vec![UiFn::Undo] },
                        KUE { k: K::new(Char('r'), Some(KM::CONTROL)),
                              f: vec![UiFn::Redo] },

                        KUE { k: K::new(Char('c'), Some(KM::CONTROL)),
                              f: vec![UiFn::ForceQuit] },
                    ]
                },
            ],
        }
    }
}
