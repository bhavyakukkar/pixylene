use crate::ui::{Key, KeyMap, ReqUiFnMap, UiFn};

use crossterm::event::{KeyCode::*, KeyEvent as K, KeyModifiers as KM};
use dirs::config_dir;
use libpixylene::{project::Palette, types::UCoord, PixyleneDefaults};
use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string};
use toml::{de::Error, from_str};

/// Configuration (parsed from Config Syntax and evaluated for logic errors)

pub struct Config {
    pub defaults: PixyleneDefaults,
    pub default_namespace: String,
    pub keymap_show_command_names: bool,
    pub possible_namespaces: HashMap<String, ()>,
    pub keymap: KeyMap,
    pub required_keys: ReqUiFnMap,
    pub every_frame: Vec<UiFn>,
    pub padding: u8,
}

impl Config {
    //todo: document what this function is doing
    pub fn from_config_toml() -> Result<Self, String> {
        let config = match parse_config() {
            Some(c) => Some(c?),
            None => None,
        };

        let ConfigSyntax {
            mut required_keys,
            mut default_namespace,
            mut every_frame,
            mut keymap_show_command_names,
            mut padding,

            mut defaults,
            keys,
            ..
        } = ConfigSyntax::default();

        let (keymap, possible_namespaces) = get_keys_from_config(&config, keys);

        if let Some(config) = config {
            required_keys = config.required_keys;
            default_namespace = config.default_namespace;
            every_frame = config.every_frame;
            keymap_show_command_names = config.keymap_show_command_names;
            padding = config.padding;
            defaults = config.defaults;
        }

        let defaults = parse_defaults(defaults)?;

        Ok(Self {
            defaults,
            default_namespace,
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

type PCE = PaletteColorEntry;

#[derive(Debug, Deserialize)]
pub struct KeyXUiFnEntry {
    pub k: Key,
    pub f: Vec<UiFn>,
}

type KeyXUiFnEntries = HashMap<Key, Vec<UiFn>>;
type NamespaceXKeysEntries = HashMap<String, KeyXUiFnEntries>;

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
#[serde(rename_all = "kebab-case")]
pub struct ConfigSyntax {
    pub required_keys: ReqUiFnMap,
    pub default_namespace: String,
    pub keys: NamespaceXKeysEntries,
    //TODO over here
    //pub overlay_keys: ,
    pub clear_all_keybinds: bool,
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
            default_namespace: "Main".to_owned(),
            clear_all_keybinds: false,
            every_frame: vec![UiFn::PreviewFocusLayer, UiFn::DrawStatusline],
            defaults: PixyleneDefaultsConfig {
                dimensions: UCoordEntry { x: 32, y: 32 },
                repeat: UCoordEntry { x: 1, y: 2 },
                palette: vec![
                    PCE {
                        id: 1,
                        c: String::from("#140c1c"),
                    },
                    PCE {
                        id: 2,
                        c: String::from("#442434"),
                    },
                    PCE {
                        id: 3,
                        c: String::from("#30346d"),
                    },
                    PCE {
                        id: 4,
                        c: String::from("#4e4a4e"),
                    },
                    PCE {
                        id: 5,
                        c: String::from("#854c30"),
                    },
                    PCE {
                        id: 6,
                        c: String::from("#346524"),
                    },
                    PCE {
                        id: 7,
                        c: String::from("#d04648"),
                    },
                    PCE {
                        id: 8,
                        c: String::from("#757161"),
                    },
                    PCE {
                        id: 9,
                        c: String::from("#597dce"),
                    },
                    PCE {
                        id: 10,
                        c: String::from("#d27d2c"),
                    },
                    PCE {
                        id: 11,
                        c: String::from("#8595a1"),
                    },
                    PCE {
                        id: 12,
                        c: String::from("#6daa2c"),
                    },
                    PCE {
                        id: 13,
                        c: String::from("#d2aa99"),
                    },
                    PCE {
                        id: 14,
                        c: String::from("#6dc2ca"),
                    },
                    PCE {
                        id: 15,
                        c: String::from("#dad45e"),
                    },
                    PCE {
                        id: 16,
                        c: String::from("#deeed6"),
                    },
                ],
            },
            keys: NamespaceXKeysEntries::from([(
                "Main".to_owned(),
                KeyXUiFnEntries::from([
                    (
                        K::new(Char('h'), KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_left"),
                        }],
                    ),
                    (
                        K::new(Char('j'), KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_down"),
                        }],
                    ),
                    (
                        K::new(Char('k'), KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_up"),
                        }],
                    ),
                    (
                        K::new(Char('l'), KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_right"),
                        }],
                    ),
                    (
                        K::new(Left, KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_left"),
                        }],
                    ),
                    (
                        K::new(Down, KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_down"),
                        }],
                    ),
                    (
                        K::new(Up, KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_up"),
                        }],
                    ),
                    (
                        K::new(Right, KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_right"),
                        }],
                    ),
                    (
                        K::new(Char('h'), KM::CONTROL).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_dup_left"),
                        }],
                    ),
                    (
                        K::new(Char('j'), KM::CONTROL).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_dup_down"),
                        }],
                    ),
                    (
                        K::new(Char('k'), KM::CONTROL).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_dup_up"),
                        }],
                    ),
                    (
                        K::new(Char('l'), KM::CONTROL).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_dup_right"),
                        }],
                    ),
                    (
                        K::new(Left, KM::CONTROL).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_dup_left"),
                        }],
                    ),
                    (
                        K::new(Down, KM::CONTROL).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_dup_down"),
                        }],
                    ),
                    (
                        K::new(Up, KM::CONTROL).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_dup_up"),
                        }],
                    ),
                    (
                        K::new(Right, KM::CONTROL).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_dup_right"),
                        }],
                    ),
                    (
                        K::new(Char('r'), KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("cursors_reset"),
                        }],
                    ),
                    (
                        K::new(Enter, KM::empty()).into(),
                        vec![UiFn::RunAction {
                            name: String::from("pencil"),
                        }],
                    ),
                    (K::new(Char('u'), KM::empty()).into(), vec![UiFn::Undo]),
                    (K::new(Char('r'), KM::CONTROL).into(), vec![UiFn::Redo]),
                    (K::new(Char('c'), KM::CONTROL).into(), vec![UiFn::ForceQuit]),
                ]),
            )]),
            keymap_show_command_names: true,
            padding: 1,
        }
    }
}

fn parse_config() -> Option<Result<ConfigSyntax, String>> {
    use colored::Colorize;

    match config_dir() {
        Some(mut path) => {
            path.push("pixylene");
            path.push("config");
            path.set_extension("toml");
            match read_to_string(path) {
                //config file present
                Ok(contents) => match ConfigSyntax::from(&contents) {
                    Ok(config) => {
                        if config.keys.len() == 0 {
                            return Some(Err(format!(
                                "{}{}\n{}",
                                "Config File Error: ".red().bold(),
                                "keys".italic(),
                                "At-least one namespace of keys is required",
                            )));
                        }

                        if let None = config.keys.get(&config.default_namespace) {
                            return Some(Err(format!(
                                "{}{}\n{}",
                                "Config File Error: ".red().bold(),
                                "default_namespace".italic(),
                                format!(
                                    "Provided default namespace {} has not been defined in \
                                    the config-file",
                                    config.default_namespace,
                                ),
                            )));
                        }

                        Some(Ok(config))
                    }
                    Err(err) => Some(Err(format!(
                        "Error parsing config.toml:\n {}",
                        err.to_string(),
                    ))),
                },
                //config file not present
                Err(_) => None,
            }
        }
        //config dir not configured on user's OS
        None => None,
    }
}

fn parse_defaults(defaults: PixyleneDefaultsConfig) -> Result<PixyleneDefaults, String> {
    use colored::Colorize;

    Ok(PixyleneDefaults {
        dim: UCoord {
            x: defaults.dimensions.x,
            y: defaults.dimensions.y,
        }
        .try_into()
        .map_err(|err| {
            format!(
                "{}{}\n{}",
                "Config File Error: ".red().bold(),
                "defaults.dimensions".yellow().italic(),
                err,
            )
        })?,
        repeat: UCoord {
            x: defaults.repeat.x,
            y: defaults.repeat.y,
        }
        .try_into()
        .map_err(|err| {
            format!(
                "{}{}\n{}",
                "Config File Error: ".red().bold(),
                "defaults.repeat".italic(),
                err,
            )
        })?,
        palette: Palette::from(
            &defaults
                .palette
                .iter()
                .map(|entry| (entry.id, entry.c.as_str()))
                .collect::<Vec<(u8, &str)>>(),
        )
        .map_err(|err| {
            format!(
                "{}{}\n{}",
                "Config File Error: ".red().bold(),
                "defaults.palette".italic(),
                err,
            )
        })?,
    })
}

fn get_keys_from_config(
    config: &Option<ConfigSyntax>,
    default_keys: NamespaceXKeysEntries,
) -> (KeyMap, HashMap<String, ()>) {
    let mut keymap = HashMap::new();
    keymap.insert(None, HashMap::new());
    let mut possible_namespaces = HashMap::new();

    //if no user config or user config doesn't want new_keys
    if !(config.is_some() && config.as_ref().unwrap().clear_all_keybinds) {
        //we are constructing a new default Config here, just because keymap::KeyMap doesn't
        //implement clone and we cannot clone it from an existing reference to a default config
        _ = default_keys
            .into_iter()
            .map(|(namespace, keys)| {
                possible_namespaces.insert(namespace.clone(), ());
                let mut map = HashMap::new();
                _ = keys
                    .into_iter()
                    .map(|(key, fns)| {
                        map.insert(key, fns);
                    })
                    .collect::<Vec<()>>();
                keymap.insert(Some(namespace), map);
            })
            .collect::<Vec<()>>();
    }

    if let Some(config) = config {
        _ = config
            .keys
            .iter()
            .map(|(namespace, keys)| {
                possible_namespaces.insert(namespace.clone(), ());
                let mut map = HashMap::new();
                _ = keys
                    .iter()
                    .map(|(key, fns)| {
                        map.insert(key.clone(), fns.clone());
                    })
                    .collect::<Vec<()>>();

                if let Some(existing_group) = keymap.get_mut(&Some(namespace.clone())) {
                    existing_group.extend(map);
                } else {
                    keymap.insert(Some(namespace.to_string()), map);
                }
            })
            .collect::<Vec<()>>();
    }

    (keymap, possible_namespaces)
}
