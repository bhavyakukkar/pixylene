use crate::ui::{ UiFn, Key, ReqUiFnMap, KeyMap };

use libpixylene::{PixyleneDefaults, types::{UCoord, PCoord}, project::Palette};
use crossterm::{event::{KeyEvent as K, KeyModifiers as KM, KeyCode::*}};
use dirs::config_dir;
use serde::{ Deserialize };
use toml::{ from_str, de::Error };
use std::{
    fs::read_to_string,
    collections::HashMap,
};


/// Configuration (parsed from Config Syntax and evaluated for logic errors)

pub struct Config {
    pub defaults: PixyleneDefaults,
    pub keymap_show_command_names: bool,
    pub possible_namespaces: HashMap<String, ()>,
    pub keymap: KeyMap,
    pub required_keys: ReqUiFnMap,
    pub every_frame: Vec<UiFn>,
    pub padding: u8,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            defaults: PixyleneDefaults {
                dim: PCoord::new(10, 10).unwrap(),
                palette: Palette::new(),
                repeat: PCoord::new(1, 2).unwrap(),
            },
            keymap_show_command_names: false,
            possible_namespaces: HashMap::new(),
            keymap: HashMap::new(),
            required_keys: ReqUiFnMap {
                start_command: K::new(Char(':'), KM::empty()).into(),
                discard_command: K::new(Esc, KM::empty()).into(),
                force_quit: K::new(Char('c'), KM::CONTROL).into(),
            },
            every_frame: Vec::new(),
            padding: 1,
        }
    }

    pub fn from_config_toml() -> Result<Self, String> {
        let config = match parse_config() {
            Some(c) => Some(c?),
            None => None,
        };
        let ConfigSyntax {
            mut required_keys,
            mut every_frame,
            mut keymap_show_command_names,
            mut padding,

            mut defaults,
            keys,
            ..
        } =
            ConfigSyntax::default();

        let (keymap, possible_namespaces) = get_keys_from_config(&config, keys);

        if let Some(config) = config {
            required_keys = config.required_keys;
            every_frame = config.every_frame;
            keymap_show_command_names = config.keymap_show_command_names;
            padding = config.padding;
            defaults = config.defaults;
        }

        let defaults = parse_defaults(defaults)?;

        Ok(Self {
            defaults,
            keymap_show_command_names,
            possible_namespaces,
            keymap,
            required_keys,
            every_frame,
            padding,
        })
    }
}



/// Config Syntax (deserialized config.toml)

type KUE = KeyXUiFnEntry;
type NKE = NamespaceXKeysEntry;
type PCE = PaletteColorEntry;


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
pub struct ConfigSyntax {
    pub required_keys: ReqUiFnMap,
    pub keys: Vec<NamespaceXKeysEntry>,
    pub new_keys: bool,
    pub every_frame: Vec<UiFn>,
    pub defaults: PixyleneDefaultsConfig,
    pub keymap_show_command_names: bool,
    pub padding: u8,
}

impl ConfigSyntax {
    pub fn from(toml: &String) -> Result<ConfigSyntax, Error> {
        from_str(toml)
    }
}

impl Default for ConfigSyntax {
    fn default() -> ConfigSyntax {
        ConfigSyntax {
            required_keys: ReqUiFnMap {
                start_command: K::new(Char(':'), KM::empty()).into(),
                discard_command: K::new(Esc, KM::empty()).into(),
                force_quit: K::new(Char('c'), KM::CONTROL).into(),
            },
            new_keys: false,
            every_frame: vec![UiFn::PreviewFocusLayer, UiFn::DrawStatusline],
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
                        KUE { k: K::new(Char('h'), KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_left") }] },
                        KUE { k: K::new(Char('j'), KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_down") }] },
                        KUE { k: K::new(Char('k'), KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_up") }] },
                        KUE { k: K::new(Char('l'), KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_right") }] },

                        KUE { k: K::new(Left, KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_left") }] },
                        KUE { k: K::new(Down, KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_down") }] },
                        KUE { k: K::new(Up, KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_up") }] },
                        KUE { k: K::new(Right, KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_right") }] },

                        KUE { k: K::new(Char('h'), KM::CONTROL).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_dup_left") }] },
                        KUE { k: K::new(Char('j'), KM::CONTROL).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_dup_down") }] },
                        KUE { k: K::new(Char('k'), KM::CONTROL).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_dup_up") }] },
                        KUE { k: K::new(Char('l'), KM::CONTROL).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_dup_right") }] },

                        KUE { k: K::new(Char('r'), KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("cursors_reset") }] },

                        KUE { k: K::new(Enter, KM::empty()).into(),
                              f: vec![UiFn::RunAction{ name: String::from("pencil") }] },

                        KUE { k: K::new(Char('u'), KM::empty()).into(),
                              f: vec![UiFn::Undo] },
                        KUE { k: K::new(Char('r'), KM::CONTROL).into(),
                              f: vec![UiFn::Redo] },

                        KUE { k: K::new(Char('c'), KM::CONTROL).into(),
                              f: vec![UiFn::ForceQuit] },
                    ]
                },
            ],
            keymap_show_command_names: true,
            padding: 1,
        }
    }
}

fn parse_config() -> Option<Result<ConfigSyntax, String>> {
    match config_dir() {
        Some(mut path) => {
            path.push("pixylene");
            path.push("config");
            path.set_extension("toml");
            match read_to_string(path) {
                //config file present
                Ok(contents) => Some(
                    ConfigSyntax::from(&contents)
                        .map_err(|err| format!("Error in config.toml:\n {}", err.to_string()))
                ),
                //config file not present
                Err(_) => None,
            }
        },
        //config dir not configured on user's OS
        None => None,
    }
}

fn parse_defaults(defaults: PixyleneDefaultsConfig) -> Result<PixyleneDefaults, String> {
    use colored::Colorize;

    Ok(PixyleneDefaults {
        dim: UCoord{
            x: defaults.dimensions.x,
            y: defaults.dimensions.y,
        }.try_into().map_err(|err| format!(
            "{}{}{}",
            "Config File Error: ".red().bold(),
            "defaults.dimensions\n".yellow().italic(),
            err,
        ))?,
        repeat: UCoord{
            x: defaults.repeat.x,
            y: defaults.repeat.y,
        }.try_into().map_err(|err| format!(
            "{}{}{}",
            "Config File Error: ".red().bold(),
            "defaults.repeat\n".italic(),
            err,
        ))?,
        palette: Palette::from(&defaults.palette.iter()
            .map(|entry| (entry.id, entry.c.as_str()))
            .collect::<Vec<(u8, &str)>>()).map_err(|err| format!(
                "{}{}{}",
                "Config File Error: ".red().bold(),
                "defaults.palette\n".italic(),
                err,
            ))?,
    })
}

fn get_keys_from_config(config: &Option<ConfigSyntax>, default_keys: Vec<NamespaceXKeysEntry>)
-> (KeyMap, HashMap<String, ()>)
{
    let mut keymap = HashMap::new();
    keymap.insert(None, HashMap::new());
    let mut possible_namespaces = HashMap::new();

    //if no user config or user config doesn't want new_keys
    if !(config.is_some() && config.as_ref().unwrap().new_keys) {
    //if !config.new_keys {
        //we are constructing a new default Config here, just because keymap::KeyMap doesn't
        //implement clone and we cannot clone it from an existing reference to a default config
        _ = default_keys.into_iter()
            .map(|group| {
                if let Some(ref namespace) = group.name {
                    possible_namespaces.insert(namespace.clone(), ());
                }
                let mut map = HashMap::new();
                _ = group.keys.into_iter().map(|entry| {
                    map.insert(entry.k, entry.f);
                }).collect::<Vec<()>>();
                keymap.insert(group.name.clone(), map);
            })
            .collect::<Vec<()>>();
    }

    if let Some(config) = config {
        _ = config.keys.iter()
            .map(|group| {
                if let Some(ref namespace) = group.name {
                    possible_namespaces.insert(namespace.clone(), ());
                }
                let mut map = HashMap::new();
                _ = group.keys.iter().map(|entry| {
                    map.insert(entry.k.clone(), entry.f.clone());
                }).collect::<Vec<()>>();

                if let Some(existing_group) = keymap.get_mut(&group.name) {
                    existing_group.extend(map);
                } else {
                    keymap.insert(group.name.clone(), map);
                }
            }).collect::<Vec<()>>();
    }

    (keymap, possible_namespaces)
}
