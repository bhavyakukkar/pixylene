use crate::{
    ui::{ UserInterface, Rectangle, Statusline, KeyInfo, Key, KeyMap, ReqUiFnMap, UiFn },
    config::{ Config },
    actions::{ ActionLocation, add_my_native_actions },
};

use libpixylene::{
    Pixylene, PixyleneDefaults,
    project::{ OPixel, Layer, Palette },
    types::{ UCoord, PCoord, Coord, Pixel },
};
use pixylene_actions::{ memento::{ ActionManager }, Console, LogType };
use pixylene_actions_lua::{ LuaActionManager, ErrorType };
use std::collections::HashMap;
use std::process::exit;
use std::cell::RefCell;
use std::rc::Rc;
use clap::{ Subcommand };

use dirs::config_dir;
use std::fs::read_to_string;


const PADDING: u8 = 1;

type CommandMap = HashMap<String, UiFn>;

#[derive(Subcommand)]
pub enum StartType {
    //New { dimensions: Option<Coord>, palette: Option<Palette> },
    //Open { path: String },
    //Import { path: String, palette: Option<Palette> },
    New,
    Open{ path: String },
    Import{ path: String },
}

pub struct PixyleneSession {
    name: String,
    pixylene: Rc<RefCell<Pixylene>>,
    last_action_name: Option<String>,
    project_file_path: Option<String>,
    modified: bool,

    action_map: HashMap<String, ActionLocation>,
    native_action_manager: ActionManager,
    lua_action_manager: LuaActionManager,
}

/*
pub struct ConsoleCell {
    pub b_console: Option<Rectangle>,
}

impl ConsoleCell {
    pub fn get_console<'a>(&'a self, target: Rc<RefCell<dyn Target + 'static>>,
                           rev_keymap: &ReqUiFnMap)
        -> Console
    {
        Console {
            cmdin: Box::new(|message: String| -> Option<String> {
                    target.borrow().console_in(&message,
                                               &rev_keymap.get(&ReqUiFn::DiscardCommand)
                                               .unwrap().clone(),
                                               &self.b_console.unwrap())
            }),
            cmdout: Box::new(|message: String, log_type: &LogType| {
                    target.borrow().console_out(&message, log_type, &self.b_console.unwrap());
            }),
        }
    }
}
*/

//pub struct Controller<T: UserInterface + Console> {
//    target: Rc<RefCell<T>>,
pub struct Controller {
    target: Rc<RefCell<dyn UserInterface>>,

    sessions: Vec<PixyleneSession>,
    //this index is 1-based
    sel_session: u8,

    defaults: PixyleneDefaults,

    possible_namespaces: HashMap<String, ()>,
    namespace: Option<String>,
    keymap: KeyMap,
    rev_keymap: ReqUiFnMap,
    every_frame: Vec<UiFn>,
    cmd_map: CommandMap,

    //Window boundaries
    b_console: Option<Rectangle>,
    b_camera: Option<Rectangle>,
    b_statusline: Option<Rectangle>,
    padding: u8,
}

impl Console for Controller {
    fn cmdin(&self, message: &str) -> Option<String> {
        self.console_in(message)
    }
    fn cmdout(&self, message: &str, log_type: &LogType) {
        self.console_out(message, log_type)
    }
}

//impl<T: UserInterface + Console> Controller<T> {
impl Controller {

    fn set_boundaries(&mut self, boundaries: (Rectangle, Rectangle, Rectangle)) {
        self.b_camera = Some(boundaries.0);
        self.b_statusline = Some(boundaries.1);
        self.b_console = Some(boundaries.2);
    }

    fn console_in(&self, message: &str) -> Option<String> {
        let input = self.target.borrow_mut().console_in(message,
                                                        &self.rev_keymap.discard_command,
                                                        &self.b_console.unwrap());
        self.console_clear();
        input
    }

    fn console_out(&self, message: &str, log_type: &LogType) {
        self.target.borrow_mut().console_out(message,
                                         log_type,
                                         &self.b_console.unwrap());
    }

    fn console_clear(&self) {
        self.target.borrow_mut().clear(&self.b_console.unwrap());
    }

    fn sel_session(&self) -> Result<usize, ()> {
        if self.sessions.len() == 0 {
            self.console_out("start a session to use that function. start with :new or :open or \
                             :import", &LogType::Warning);
            Err(())
        } else {
            Ok(usize::from(self.sel_session) - 1)
        }
    }

    pub fn new(target: Rc<RefCell<dyn UserInterface>>) -> Result<Self, String> {
        use UiFn::*;
        use colored::Colorize;
        let mut is_default = false;

        let config: Config = match config_dir() {
            Some(mut path) => {
                path.push("pixylene");
                path.push("config");
                path.set_extension("toml");
                match read_to_string(path) {
                    Ok(contents) => {
                        Config::from(&contents).map_err(|err| {
                            format!("Error in config.toml:\n {}", err.to_string())
                        })?
                    },
                    //config file not present
                    Err(_) => {
                        is_default = true;
                        Default::default()
                    },
                }
            },
            //config dir not configured on user's OS
            None => {
                is_default = true;
                Default::default()
            },
        };

        let defaults = PixyleneDefaults {
            dim: UCoord{
                x: config.defaults.dimensions.x,
                y: config.defaults.dimensions.y,
            }.try_into().map_err(|err| format!(
                    "{}{}{}",
                    "Config File Error: ".red().bold(),
                    "defaults.dimensions\n".yellow().italic(),
                    err,
            ))?,
            repeat: UCoord{
                x: config.defaults.repeat.x,
                y: config.defaults.repeat.y,
            }.try_into().map_err(|err| format!(
                    "{}{}{}",
                    "Config File Error: ".red().bold(),
                    "defaults.repeat\n".italic(),
                    err,
            ))?,
            palette: Palette::from(&config.defaults.palette.iter()
                                   .map(|entry| (entry.id, entry.c.as_str()))
                                   .collect::<Vec<(u8, &str)>>()).map_err(|err| format!(
                                           "{}{}{}",
                                           "Config File Error: ".red().bold(),
                                           "defaults.palette\n".italic(),
                                           err,
                                    ))?,
        };

        let mut keymap: KeyMap = HashMap::new();
        let mut possible_namespaces = HashMap::new();
        if !config.new_keys {
            _ = <Config as Default>::default().keys.into_iter()
                .map(|group| {
                    if let Some(ref namespace) = group.name {
                        possible_namespaces.insert(namespace.clone(), ());
                    }
                    let mut map = HashMap::new();
                    _ = group.keys.into_iter().map(|entry| {
                        map.insert(Key::new(entry.k.code(), Some(entry.k.modifiers())), entry.f);
                    }).collect::<Vec<()>>();
                    keymap.insert(group.name.clone(), map);
                })
                .collect::<Vec<()>>();
        }

        if !is_default {
            _ = config.keys.into_iter()
                .map(|group| {
                    if let Some(ref namespace) = group.name {
                        possible_namespaces.insert(namespace.clone(), ());
                    }
                    let mut map = HashMap::new();
                    _ = group.keys.into_iter().map(|entry| {
                        map.insert(Key::new(entry.k.code(), Some(entry.k.modifiers())), entry.f);
                    }).collect::<Vec<()>>();

                    if let Some(existing_group) = keymap.get_mut(&group.name) {
                        existing_group.extend(map);
                    } else {
                        keymap.insert(group.name.clone(), map);
                    }
                }).collect::<Vec<()>>();
        }

        let rev_keymap = config.required_keys;

        let every_frame = config.every_frame;

        Ok(Self {
            target,

            sessions: Vec::new(),
            sel_session: 0,

            defaults,

            possible_namespaces,
            namespace: None,

            keymap,
            rev_keymap,
            every_frame,
            cmd_map: CommandMap::from([
                (String::from("new"), New),
                (String::from("open"), Open),
                (String::from("import"), Import),
                (String::from("q"), Quit),
                (String::from("q!"), ForceQuit),
                (String::from("ns"), GoToNextSession),
                (String::from("ps"), GoToPrevSession),
                (String::from("w"), Save),
                (String::from("export"), Export),
                (String::from("undo"), Undo),
                (String::from("redo"), Redo),
                (String::from("a"), RunActionSpecify),
                (String::from("showlayer"), PreviewFocusLayer),
                (String::from("showproject"), PreviewProject),
                (String::from("canvasjson"), PrintCanvasJson),
                (String::from("showstatus"), UpdateStatusline),
            ]),

            b_console: None,
            b_camera: None,
            b_statusline: None,
            padding: PADDING,
        })
    }

    fn new_session(&mut self, start_type: &StartType, from_args: bool) {
        if self.sessions.len() > 255 {
            if !from_args {
                self.console_out("cannot have more than 256 sessions open", &LogType::Error);
            } else {
                self.target.borrow_mut().finalize();
                eprintln!("cannot have more than 256 sessions open");
                exit(1);
            }
        }

        let native_action_manager;

        let lua_action_manager = LuaActionManager::setup(
            &match config_dir() {
                Some(mut path) => {
                    path.push("pixylene");
                    path.push("actions");
                    path.set_extension("lua");
                    match read_to_string(path) {
                        Ok(contents) => contents,
                        //actions file not present
                        Err(_) => String::new(),
                    }
                },
                None => String::new(),
            }
        );

        if let Some(ref err) = lua_action_manager.error {
            match err {
                ErrorType::LuaError(err) => {
                    self.console_out(&format!("Critical Lua error, cannot create tables: {err}"),
                                     &LogType::Error);
                },
                ErrorType::ConfigError(err) => {
                    self.console_out(&format!("Lua error in user config: {err}"),
                                     &LogType::Error);
                }
            }
        }

        let mut action_map: HashMap<String, ActionLocation> = HashMap::new();
        _ = lua_action_manager.list_actions().iter().map(|action_name| {
            action_map.insert(action_name.clone(), ActionLocation::Lua);
        }).collect::<Vec<()>>();

        add_my_native_actions(&mut action_map);

        match start_type {
            StartType::New => {
                let mut pixylene = Pixylene::new(&self.defaults);
                pixylene.project.out_dim = self.b_camera.unwrap().size;

                let dim = pixylene.project.canvas.dim();

                //cant fail because this is 1st layer not 257th, and because dimensions of layer
                //directly copied from canvas
                pixylene.project.canvas.add_layer(Layer::new_with_solid_color(dim, None)).unwrap();

                //move focus to center
                pixylene.project.focus.0 = Coord {
                    x: i32::from(dim.x()).checked_div(2).unwrap(),
                    y: i32::from(dim.y()).checked_div(2).unwrap(),
                };

                //toggle 1 cursor at center
                pixylene.project.toggle_cursor_at(&(UCoord {
                    x: u16::from(dim.x()).checked_div(2).unwrap(),
                    y: u16::from(dim.y()).checked_div(2).unwrap(),
                }, 0)).unwrap(); //cant fail because x,y less than dim and we know there is at
                                 //least 1 layer because we created it

                native_action_manager = ActionManager::new(&pixylene.project.canvas);
                self.sessions.push(PixyleneSession {
                    name: String::from("new"),
                    pixylene: Rc::new(RefCell::new(pixylene)),
                    last_action_name: None,
                    project_file_path: None,
                    modified: false,
                    action_map,
                    native_action_manager,
                    lua_action_manager,
                });
                self.sel_session += 1;
            },
            StartType::Open{ path } => {
                match Pixylene::open(&path) {
                    Ok(mut pixylene) => {
                        pixylene.project.out_dim = self.b_camera.unwrap().size;
                        native_action_manager = ActionManager::new(&pixylene.project.canvas);
                        self.sessions.push(PixyleneSession {
                            name: path.clone(),
                            pixylene: Rc::new(RefCell::new(pixylene)),
                            last_action_name: None,
                            project_file_path: Some(path.clone()),
                            modified: false,
                            action_map,
                            native_action_manager,
                            lua_action_manager,
                        });
                        self.sel_session += 1;
                    }
                    Err(err) => {
                        if !from_args {
                            self.console_out(&format!(
                                "failed to open: {}",
                                err.to_string()
                            ), &LogType::Error);
                        } else {
                            self.target.borrow_mut().finalize();
                            eprintln!("failed to open: {}", err.to_string());
                            exit(1);
                        }
                    },
                }
            },
            StartType::Import{ path } => {
                match Pixylene::import(&path, &self.defaults) {
                    Ok(mut pixylene) => {
                        pixylene.project.out_dim = self.b_camera.unwrap().size;

                        let dim = pixylene.project.canvas.dim();

                        //move focus to center
                        pixylene.project.focus.0 = Coord {
                            x: i32::from(dim.x()).checked_div(2).unwrap(),
                            y: i32::from(dim.y()).checked_div(2).unwrap(),
                        };

                        //toggle 1 cursor at center
                        pixylene.project.toggle_cursor_at(&(UCoord {
                            x: u16::from(dim.x()).checked_div(2).unwrap(),
                            y: u16::from(dim.y()).checked_div(2).unwrap(),
                        }, 0)).unwrap(); //cant fail because x,y less than dim and we know there is at
                                         //least 1 layer because we created it

                        native_action_manager = ActionManager::new(&pixylene.project.canvas);
                        self.sessions.push(PixyleneSession {
                            name: path.clone(),
                            pixylene: Rc::new(RefCell::new(pixylene)),
                            last_action_name: None,
                            project_file_path: None,
                            modified: false,
                            action_map,
                            native_action_manager,
                            lua_action_manager,
                        });
                        self.sel_session += 1;
                    },
                    Err(err) => {
                        if !from_args {
                            self.console_out(&format!(
                                "failed to import: {}",
                                err.to_string()
                            ), &LogType::Error);
                        } else {
                            self.target.borrow_mut().finalize();
                            eprintln!("failed to import: {}", err.to_string());
                            exit(1);
                        }
                    },
                }
            },
        }
    }

    fn quit_session(&mut self) {
        if self.sessions.len() <= 1 {
            self.target.borrow_mut().finalize();
            exit(0);
        } else {
            self.sessions.remove(self.sel_session as usize - 1);
            if usize::from(self.sel_session) > self.sessions.len() {
                self.sel_session -= 1;
            }
        }
    }

    pub fn start(&mut self, start_type: &Option<StartType>) {
        use UiFn::{ RunKey, ForceQuit };

        self.target.borrow_mut().initialize();

        let window_size = self.target.borrow().get_size();

        self.set_boundaries(self.compute_boundaries(&window_size));

        //clear entire screen
        let current_dim = self.target.borrow().get_size();
        self.target.borrow_mut().clear(&Rectangle{
            start: UCoord{ x: 0, y: 0 },
            size: current_dim,
        });

        if let Some(start_type) = start_type {
            self.new_session(start_type, true);

            for func in self.every_frame.clone() {
                _ = self.perform_ui(&func);
            }
        }
        else {
            //splash screen
            self.console_out("Welcome to Pixylene", &LogType::Info);
        }

        loop {
            //sleep(Duration::from_millis(1));

            if !self.target.borrow_mut().refresh() {
                _ = self.perform_ui(&ForceQuit);
            }
            let key_info = self.target.borrow().get_key();
            if let Some(key_info) = key_info {
                match key_info {
                    KeyInfo::Key(key) => {
                        _ = self.perform_ui(&RunKey(key));
                    },
                    KeyInfo::UiFn(ui_fn) => {
                        _ = self.perform_ui(&ui_fn);
                    },
                }
                if self.sessions.len() > 0 {
                    for func in self.every_frame.clone() {
                        _ = self.perform_ui(&func);
                    }
                }
            }
        }
    }

    fn perform_ui(&mut self, func: &UiFn) -> Result<(), ()> {
        use UiFn::*;

        match func {
            //Sessions
            New => {
                self.new_session(&StartType::New, false);
            },
            Open => {
                let input = self.console_in("open: ");
                match input {
                    Some(input) => {
                        self.new_session(&StartType::Open{path:input}, false);
                    },
                    None => (),
                }
            },
            Import => {
                let input = self.console_in("import: ");
                match input {
                    Some(input) => {
                        self.new_session(&StartType::Import{path:input}, false);
                    },
                    None => (),
                }
            },
            Quit => {
                if self.sessions.len() > 0
                    && self.sessions[self.sel_session as usize - 1].modified {
                    self.console_out(
                        "project has been modified since last change, force quit (:q!) to discard \
                        modifications",
                        &LogType::Error,
                    );
                } else {
                    self.quit_session();
                }
            },
            ForceQuit => { 
                self.quit_session();
            },
            GoToSession(index) => {
                let num_sessions = u8::try_from(self.sessions.len()).unwrap();
                if num_sessions == 0 {
                    self.console_out("there are no sessions open", &LogType::Error);
                }
                if *index < num_sessions {
                    self.sel_session = *index;
                } else {
                    self.console_out(
                        &format!("only {} sessions open", num_sessions),
                        &LogType::Error
                    );
                }
            },
            GoToNextSession => {
                let s = self.sel_session()? + 1;
                match s.checked_add(1) {
                    Some(new) => {
                        if usize::from(new) <= self.sessions.len() {
                            self.sel_session = new.try_into().unwrap(); //cant fail because
                                                                        //sel_session can never be
                                                                        //increased past 256
                        } else {
                            self.console_out(
                                "this is the last session",
                                &LogType::Warning
                            );
                        }
                    },
                    None => {
                        self.console_out(
                            "this is the last session",
                            &LogType::Warning
                        );
                    },
                }
            },
            GoToPrevSession => {
                let s = self.sel_session()? + 1;
                if s - 1 > 0 {
                    self.sel_session = (s - 1).try_into().unwrap(); //cant fail because
                                                                //sel_session can never be
                                                                //increased past 256
                } else {
                    self.console_out(
                        "this is the first session",
                        &LogType::Warning
                    );
                }
            },

            //File
            Save => {
                let s = self.sel_session()?;
                let mut did_save = false;
                match &self.sessions[s].project_file_path {
                    Some(path) => {
                        match self.sessions[s].pixylene.borrow()
                            .save(&path)
                        {
                            Ok(()) => {
                                self.console_out(
                                    &format!("saved to {}", path),
                                    &LogType::Info
                                );
                                did_save = true;
                            },
                            Err(err) => {
                                self.console_out(
                                    &format!("failed to save: {}",
                                             err),
                                    &LogType::Error
                                );
                            }
                        }
                    },
                    None => {
                        let mut new_project_file_path: Option<String> = None;
                        match self.console_in("save path: ") {
                            Some(input) => {
                                self.console_out("saving...", &LogType::Info);
                                match self.sessions[s].pixylene
                                    .borrow().save(&input) {
                                    Ok(()) => {
                                        self.console_out(
                                            &format!("saved to {}", input),
                                            &LogType::Info
                                        );
                                        new_project_file_path = Some(input);
                                        did_save = true;
                                    },
                                    Err(err) => {
                                        self.console_out(
                                            &format!("failed to save: {}",
                                                     err),
                                            &LogType::Error
                                        );
                                    }
                                }
                                //self.console_out(
                                //    &format!("saved to {}", input),
                                //    &LogType::Info,
                                //);

                            },
                            None => (),
                        }
                        if let Some(path) = new_project_file_path {
                            self.sessions[s].project_file_path =
                                Some(path);
                        }
                    },
                }
                if did_save {
                    self.sessions[s].modified = false;
                }
            },
            Export => {
                let s = self.sel_session()?;
                match self.console_in("export path: ") {
                    Some(path) => match self.console_in("scaling factor: ") {
                        Some(input) => match str::parse::<u16>(&input) {
                            Ok(scale_up) => {
                                self.console_out("exporting...",
                                                   &LogType::Info);
                                match self.sessions[s].pixylene
                                    .borrow().export(&path,
                                                              scale_up) {
                                    Ok(()) => {
                                        self.console_out(
                                            &format!("exported to {}", path),
                                            &LogType::Info
                                        );
                                    },
                                    Err(err) => {
                                        self.console_out(
                                            &format!("failed to export: {}",
                                                    err),
                                            &LogType::Error
                                        );
                                    }
                                }
                            },
                            Err(_) => {
                                self.console_out(
                                    &format!("invalid scaling factor: '{}'",
                                             input),
                                    &LogType::Error
                                );
                            }
                        },
                        None => (),
                    },
                    None => (),
                }
            },

            //Undo/Redo
            Undo => {
                let s = self.sel_session()?;
                let PixyleneSession {
                    ref mut native_action_manager,
                    ref mut pixylene,
                    ..
                } = &mut self.sessions[s];

                native_action_manager.undo(&mut pixylene.borrow_mut().project.canvas);
            },
            Redo => {
                let s = self.sel_session()?;
                let PixyleneSession {
                    ref mut native_action_manager,
                    ref mut pixylene,
                    ..
                } = &mut self.sessions[s];

                native_action_manager.redo(&mut pixylene.borrow_mut().project.canvas);
            },

            EnterNamespace(name) => {
                if let Some(name) = name {
                    if let Some(_) = self.possible_namespaces.get(name) {
                        self.namespace = Some(name.clone());
                    } else {
                        self.console_out(&format!("namespace '{}' doesn't exist", name),
                                         &LogType::Error);
                    }
                } else {
                    self.namespace = None;
                }
            },
            EnterDefaultNamespace => {
                _ = self.perform_ui(&EnterNamespace(None));
            },
            RunKey(key) => {
                //special required keys
                if *key == self.rev_keymap.force_quit {
                    _ = self.perform_ui(&ForceQuit);
                }
                else if *key == self.rev_keymap.start_command {
                    _ = self.perform_ui(&RunCommandSpecify);
                }

                //other keys
                else {
                    if let Some(funcs) = self.keymap
                        .get(&self.namespace).unwrap() //wont fail because only source of
                                                       //modification for self.namespace is
                                                       //EnterNamespace which sets only from
                                                       //possible_namespaces corresponding to
                                                       //keymap
                        .get(&key) {
                        for func in (*funcs).clone() {
                            _ = self.perform_ui(&func);
                        }
                    } else {
                        self.console_out(&format!("unmapped key: {:?}", key), &LogType::Warning);
                    }
                }
            },

            RunCommandSpecify => {
                self.console_clear();
                if let Some(cmd) = self.console_in(":") {
                    _ = self.perform_ui(&RunCommand(cmd));
                } else {
                    self.console_clear();
                }
            },
            RunCommand(command) => {
                let mut func: Option<UiFn> = None;
                if let Some(mapped_func) = self.cmd_map.get(command) {
                    func = Some(mapped_func.clone());
                }
                if let Some(func) = func {
                    _ = self.perform_ui(&func);
                } else {
                    self.console_out(&format!("command not found: '{}'", command), &LogType::Error);
                }
            },

            RunActionSpecify => {
                _ = self.sel_session()?;
                if let Some(action_name) = self.console_in("action: ") {
                    _ = self.perform_ui(&RunAction(action_name));
                } else {
                    self.console_clear();
                }
            },
            RunAction(action_name) => {
                let s = self.sel_session()?;
                let mut self_clone = Controller::new(self.target.clone()).unwrap();
                self_clone.set_boundaries((self.b_camera.unwrap(),
                                           self.b_statusline.unwrap(),
                                           self.b_console.unwrap()));

                let Self {
                    sessions,
                    target,
                    b_console,
                    ..
                } = self;

                let PixyleneSession {
                    ref mut pixylene,
                    ref mut action_map,
                    ref mut native_action_manager,
                    ref mut lua_action_manager,
                    ref mut last_action_name,
                    ref mut modified,
                    ..
                } = &mut sessions[s];

                match action_map.get(&action_name.clone()) {
                    Some(action_location) => {
                        target.borrow_mut().clear(&b_console.unwrap());
                        match action_location {
                            ActionLocation::Lua => {
                                match lua_action_manager.invoke(&action_name, pixylene.clone(),
                                                                Rc::new(self_clone)) {
                                    Ok(()) => {
                                        if native_action_manager
                                            .commit(&pixylene.borrow().project.canvas)
                                        {
                                            *last_action_name = Some(action_name.clone());
                                            *modified = true;
                                        }
                                    },
                                    Err(err) => {
                                        target.borrow_mut().console_out(
                                            //&format!("failed to perform: {}",
                                            &format!("{}",
                                            //match err {
                                            //    //print only cause, not traceback
                                            //    mlua::Error::CallbackError{ cause, .. } => cause,
                                            //    mlua::Error::RuntimeError(msg) => msg,
                                            //    otherwise => otherwise,
                                            //}),

                                            //todo: better reporting
                                            err.to_string().lines().map(|s| s.to_string()
                                            .replace("\t", " ")).collect::<Vec<String>>().join(", ")),
                                            &LogType::Error,
                                            &b_console.unwrap()
                                        );
                                    }
                                }
                            },
                            ActionLocation::Native(action) => {
                                let performed = native_action_manager.perform(
                                    &mut pixylene.borrow_mut().project,
                                    &self_clone,
                                    action.clone()
                                );
                                match performed {
                                    Ok(()) => {
                                        if native_action_manager
                                            .commit(&pixylene.borrow().project.canvas)
                                        {
                                            *last_action_name = Some(action_name.clone());
                                            *modified = true;
                                        }
                                    },
                                    Err(err) => {
                                        target.borrow_mut().console_out(
                                            //&format!("failed to perform: {}", err.to_string()),
                                            &format!("{}", err.to_string()),
                                            &LogType::Error,
                                            &b_console.unwrap()
                                        );
                                    }
                                }
                            },
                        }
                    },
                    None => {
                        target.borrow_mut().console_out(
                            &format!("action '{}' was not found in actions inserted into the \
                                     action-manager", action_name),
                            &LogType::Error,
                            &b_console.unwrap()
                        );
                    }
                }

                /*
                match action_manager.perform(
                    &mut pixylene.borrow_mut().project,
                    &mut self_clone,
                    &mut Echo,
                ) {
                    Ok(()) => (),
                    Err(err) => {
                        target.borrow_mut().console_out(
                            &format!("failed to perform: {}", err.to_string()),
                            &LogType::Error,
                            &b_console.unwrap()
                        );
                    }
                }
                */
            },
            RunLastAction => {
                let s = self.sel_session()?;
                if let Some(action_name) = &self.sessions[s].last_action_name {
                    _ = self.perform_ui(&RunAction(action_name.clone()));
                }
                else {
                    self.console_out("no previous action to repeat", &LogType::Warning);
                }
            },

            PreviewFocusLayer => {
                let s = self.sel_session()?;
                let session = &mut self.sessions[s];
                self.target.borrow_mut().draw_camera(
                    session.pixylene.borrow().project.out_dim,
                    match session.pixylene.borrow().project.render_layer() {

                        //Focus is in the bounds of selected session's project's canvas
                        //Send the project-rendered pixels
                        Ok(o_pixels) => o_pixels,

                        //Focus is not in the bounds of selected session's project's canvas
                        //Send a dummy project pixel to indicate empty
                        Err(_) => vec![OPixel::OutOfScene;
                            session.pixylene.borrow().project.out_dim.area() as usize],
                    },
                    true,
                    &self.b_camera.unwrap(),
                );
            },

            PreviewProject => {
                let s = self.sel_session()?;
                let session = &mut self.sessions[s];
                self.target.borrow_mut().draw_camera(
                    session.pixylene.borrow().project.out_dim,
                    session.pixylene.borrow().project.render(),
                    false,
                    &self.b_camera.unwrap(),
                );
                self.console_in("press ENTER to stop previewing project");
            },

            PrintCanvasJson => {
                let s = self.sel_session()?;
                let session = &mut self.sessions[s];
                self.target.borrow_mut().draw_paragraph(
                    vec![session.pixylene.borrow().project.canvas.to_json()]
                );
            },

            UpdateStatusline => {
                use colored::Colorize;
                let s = self.sel_session()?;

                self.target.borrow_mut().clear(&self.b_statusline.unwrap());

                let session = &self.sessions[s];
                let mut statusline: Statusline = Vec::new();
                let padding = "     ".on_truecolor(60,60,60);
                let spacing = "  ".on_truecolor(60,60,60);
                let divider = "｜".on_truecolor(60,60,60).truecolor(100,100,100);

                {
                    //Namespace
                    statusline.push(divider.clone());
                    statusline.push(
                        self.namespace.clone().unwrap_or(String::from("Normal"))
                        .on_truecolor(60,60,60).bright_white()
                    );
                    statusline.push(divider.clone());
                }

                statusline.push(padding.clone());

                {
                    //Session name (new|.pi|.png)
                    statusline.push(divider.clone());
                    statusline.push(session.name.on_truecolor(60,60,60).bright_white());
                    statusline.push(spacing.clone());

                    //Session index
                    statusline.push(
                        format!("Session {}/{}", s + 1, self.sessions.len())
                        .on_truecolor(60,60,60).bright_white()
                    );
                    statusline.push(divider.clone());
                }

                statusline.push(padding.clone());

                {
                    //Layer index
                    statusline.push(divider.clone());
                    statusline.push(
                        format!("Layer {}/{}",
                                session.pixylene.borrow().project.focus.1 + 1,
                                session.pixylene.borrow().project.canvas.num_layers())
                        .on_truecolor(60,60,60).bright_white()
                    );
                    statusline.push(spacing.clone());

                    //Layer opacity
                    statusline.push(
                        format!("{:.2}%",
                            session.pixylene.borrow().project.canvas.get_layer(
                                session.pixylene.borrow().project.focus.1
                            )
                            .map(|layer| (f32::from(layer.opacity)/2.55))
                            .unwrap_or(0.0)
                        )
                        .on_truecolor(60,60,60).bright_white()
                    );

                    //Layer mute
                    if session.pixylene.borrow().project.canvas.get_layer(
                        session.pixylene.borrow().project.focus.1
                    )
                    .map(|layer| layer.mute)
                    .unwrap_or(false) {
                        statusline.push(spacing.clone());
                        statusline.push("muted".on_truecolor(60,60,60).bright_white());
                    }
                    statusline.push(divider.clone());
                }

                statusline.push(padding.clone());

                {
                    //Cursors status
                    statusline.push(divider.clone());
                    let num_cursors = session.pixylene.borrow().project.num_cursors();
                    statusline.push(
                        format!("{}", match num_cursors {
                            0 => String::from("0 cursors"),
                            1 => {
                                let cursor = session.pixylene.borrow().project.cursors()
                                    .collect::<Vec<&(UCoord, u16)>>()[0].clone();
                                format!("Cursor: {}, {}", cursor.1, cursor.0)
                            },
                            _ => format!("{} cursors", num_cursors),
                        })
                        .on_truecolor(60,60,60).bright_white()
                    );
                    statusline.push(divider.clone());
                }

                statusline.push(padding.clone());

                {
                    //Palette
                    statusline.push(divider.clone());
                    statusline.push("Palette: ".on_truecolor(60,60,60).bright_white());
                    let mut colors_summary = session.pixylene.borrow().project.canvas.palette.colors()
                        .map(|(a,b,c)| (a.clone(), b.clone(), c))
                        .take(16)
                        .collect::<Vec<(u8, Pixel, bool)>>();
                    colors_summary.sort_by_key(|(index, ..)| *index);
                    for (index, color, is_equipped) in colors_summary {
                        statusline.push(
                            if is_equipped {
                                format!(" {: <3}", index)
                                .on_truecolor(color.r, color.g, color.b).white().underline()
                            } else {
                                format!(" {: <3}", index)
                                .on_truecolor(color.r, color.g, color.b).white()
                            }
                        );
                    }
                    statusline.push(divider.clone());
                }

                self.target.borrow_mut().draw_statusline(&statusline,
                                                         &self.b_statusline.unwrap());
            }
        }
        Ok(())
    }

    // returns boundaries of camera, statusline and console respectively
    fn compute_boundaries(&self, window: &PCoord) -> (Rectangle, Rectangle, Rectangle) {
        (
        /* camera: */Rectangle {
            start: UCoord{ x: 0 + self.padding as u16, y: 0 + self.padding as u16 },
            size: PCoord::new(
                window.x() - 3 - 2*self.padding as u16,
                window.y() - 2*self.padding as u16
            ).unwrap()
        },
        /* statusline: */Rectangle {
            start: UCoord{ x: window.x() - 2 - self.padding as u16, y: 0 + self.padding as u16 },
            size: PCoord::new(1, window.y() - 2*self.padding as u16).unwrap()
        },
        /* console: */Rectangle {
            start: UCoord{ x: window.x() - 1 - self.padding as u16, y: 0 + self.padding as u16 },
            size: PCoord::new(1, window.y() - 2*self.padding as u16).unwrap()
        }
        )
    }
}

struct Echo;
impl pixylene_actions::memento::Action for Echo {
    fn perform(&mut self, _project: &mut libpixylene::project::Project, console: &dyn Console)
    -> pixylene_actions::memento::ActionResult {
        console.cmdout("heyyy :3 :3 :3", &LogType::Error);
        Ok(())
    }
    fn has_ended(&self) -> bool {
        true
    }
}
