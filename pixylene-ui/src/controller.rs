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
use pixylene_actions_lua::LuaActionManager;
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

    keymap: KeyMap,
    rev_keymap: ReqUiFnMap,
    every_frame: Vec<UiFn>,
    cmd_map: CommandMap,

    //Window boundaries
    b_console: Option<Rectangle>,
    b_camera: Option<Rectangle>,
    b_statusline: Option<Rectangle>,
    padding: u8,

    /// Manually set this to true if the user-interface has been started and we are not still in
    /// the CLI
    pub started: bool,
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
                            return err.to_string();
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
        if !config.new_keys {
            _ = <Config as Default>::default().keys.into_iter().map(|entry| {
                keymap.insert(Key::new(entry.k.code(), Some(entry.k.modifiers())), entry.f);
            }).collect::<Vec<()>>();
        }

        if !is_default {
            _ = config.keys.into_iter().map(|entry| {
                keymap.insert(Key::new(entry.k.code(), Some(entry.k.modifiers())), entry.f);
            }).collect::<Vec<()>>();
        }

        let rev_keymap = config.required_keys;

        let every_frame = config.every_frame;

        Ok(Self {
            target,

            sessions: Vec::new(),
            sel_session: 0,

            defaults,

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
                (String::from("showstatus"), UpdateStatusline),
            ]),

            b_console: None,
            b_camera: None,
            b_statusline: None,
            padding: PADDING,

            started: false,
        })
    }

    pub fn new_session(&mut self, start_type: &Option<StartType>) {
        if self.sessions.len() > 255 {
            if self.started {
                self.console_out("cannot have more than 256 sessions open", &LogType::Error);
            } else {
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
        )
            .unwrap_or_else(|err| panic!("Error in Lua Context:\n{}", err));

        let mut action_map: HashMap<String, ActionLocation> = HashMap::new();
        _ = lua_action_manager.list_actions().iter().map(|action_name| {
            action_map.insert(action_name.clone(), ActionLocation::Lua);
        }).collect::<Vec<()>>();

        add_my_native_actions(&mut action_map);

        match start_type {
            Some(StartType::New) => {
                let mut pixylene = Pixylene::new(&self.defaults);
                if let Some(b_camera) = self.b_camera {
                    pixylene.project.out_dim = b_camera.size;
                }

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
            Some(StartType::Open{ path }) => {
                match Pixylene::open(&path) {
                    Ok(mut pixylene) => {
                        if let Some(b_camera) = self.b_camera {
                            pixylene.project.out_dim = b_camera.size;
                        }
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
                        if self.started {
                            self.console_out(&format!(
                                "failed to open: {}",
                                err.to_string()
                            ), &LogType::Error);
                        } else {
                            eprintln!("failed to open: {}", err.to_string());
                            exit(1);
                        }
                    },
                }
            },
            Some(StartType::Import{ path }) => {
                match Pixylene::import(&path, &self.defaults) {
                    Ok(mut pixylene) => {
                        if let Some(b_camera) = self.b_camera {
                            pixylene.project.out_dim = b_camera.size;
                        }

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
                        if self.started {
                            self.console_out(&format!(
                                "failed to import: {}",
                                err.to_string()
                            ), &LogType::Error);
                        } else {
                            eprintln!("failed to import: {}", err.to_string());
                            exit(1);
                        }
                    },
                }
            },
            // This can never happen in case UI has already started
            None => {
                eprintln!("invalid command");
                exit(1);
            }
        }

        //start app if this is first session
        if !self.started {
            self.started = true;
            self.start();
        }
    }

    fn quit_session(&mut self) {
        self.sessions.remove(self.sel_session as usize - 1);
        if self.sessions.len() == 0 {
            self.target.borrow_mut().finalize();
            exit(0);
        }
        if usize::from(self.sel_session) > self.sessions.len() {
            self.sel_session -= 1;
        }
    }

    fn start(&mut self) {
        use UiFn::{ RunKey, ForceQuit };

        self.target.borrow_mut().initialize();

        let window_size = self.target.borrow().get_size();

        self.set_boundaries(self.compute_boundaries(&window_size));

        // case when started from new_session instead of start directly
        if self.sessions.len() == 1 {
            self.sessions[0].pixylene.borrow_mut().project.out_dim = self.b_camera.unwrap().size;
        }

        //clear entire screen
        let current_dim = self.target.borrow().get_size();
        self.target.borrow_mut().clear(&Rectangle{
            start: UCoord{ x: 0, y: 0 },
            size: current_dim,
        });

        for func in self.every_frame.clone() {
            self.perform_ui(&func);
        }

        loop {
            //sleep(Duration::from_millis(1));

            if !self.target.borrow_mut().refresh() {
                self.perform_ui(&ForceQuit);
            }
            let key_info = self.target.borrow().get_key();
            if let Some(key_info) = key_info {
                match key_info {
                    KeyInfo::Key(key) => {
                        self.perform_ui(&RunKey(key));
                    },
                    KeyInfo::UiFn(ui_fn) => {
                        self.perform_ui(&ui_fn);
                    },
                }
                for func in self.every_frame.clone() {
                    self.perform_ui(&func);
                }
            }
        }
    }

    fn perform_ui(&mut self, func: &UiFn) {
        use UiFn::*;

        match func {
            //Sessions
            New => {
                self.new_session(&Some(StartType::New));
            },
            Open => {
                let input = self.console_in("open: ");
                match input {
                    Some(input) => {
                        self.new_session(&Some(StartType::Open{path:input}));
                    },
                    None => (),
                }
            },
            Import => {
                let input = self.console_in("import: ");
                match input {
                    Some(input) => {
                        self.new_session(&Some(StartType::Import{path:input}));
                    },
                    None => (),
                }
            },
            Quit => {
                if self.sessions[self.sel_session as usize - 1].modified {
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
                match self.sel_session.checked_add(1) {
                    Some(new) => {
                        if usize::from(new) <= self.sessions.len() {
                            self.sel_session = new;
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
                match self.sel_session.checked_sub(1) {
                    Some(new) => {
                        if new > 0 {
                            self.sel_session = new;
                        } else {
                            self.console_out(
                                "this is the first session",
                                &LogType::Warning
                            );
                        }
                    },
                    None => {
                        self.console_out(
                            "there are no sessions open",
                            &LogType::Warning
                        );
                    },
                }
            },

            //File
            Save => {
                let mut did_save = false;
                match &self.sessions[self.sel_session as usize - 1].project_file_path {
                    Some(path) => {
                        match self.sessions[self.sel_session as usize - 1].pixylene.borrow()
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
                                match self.sessions[self.sel_session as usize - 1].pixylene
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
                            self.sessions[self.sel_session as usize - 1].project_file_path =
                                Some(path);
                        }
                    },
                }
                if did_save {
                    self.sessions[self.sel_session as usize - 1].modified = false;
                }
            },
            Export => {
                match self.console_in("export path: ") {
                    Some(path) => match self.console_in("scaling factor: ") {
                        Some(input) => match str::parse::<u16>(&input) {
                            Ok(scale_up) => {
                                self.console_out("exporting...",
                                                   &LogType::Info);
                                match self.sessions[self.sel_session as usize - 1].pixylene
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
                let PixyleneSession {
                    ref mut native_action_manager,
                    ref mut pixylene,
                    ..
                } = &mut self.sessions[self.sel_session as usize - 1];

                native_action_manager.undo(&mut pixylene.borrow_mut().project.canvas);
            },
            Redo => {
                let PixyleneSession {
                    ref mut native_action_manager,
                    ref mut pixylene,
                    ..
                } = &mut self.sessions[self.sel_session as usize - 1];

                native_action_manager.redo(&mut pixylene.borrow_mut().project.canvas);
            },

            RunKey(key) => {
                //special required keys
                if *key == self.rev_keymap.force_quit {
                    self.perform_ui(&ForceQuit);
                }
                else if *key == self.rev_keymap.start_command {
                    self.perform_ui(&RunCommandSpecify);
                }

                //other keys
                else {
                    if let Some(funcs) = self.keymap.get(&key) {
                        for func in (*funcs).clone() {
                            self.perform_ui(&func);
                        }
                    } else {
                        self.console_out(&format!("unmapped key: {:?}", key), &LogType::Warning);
                    }
                }
            },

            RunCommandSpecify => {
                self.console_clear();
                if let Some(cmd) = self.console_in(":") {
                    self.perform_ui(&RunCommand(cmd));
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
                    self.perform_ui(&func);
                } else {
                    self.console_out(&format!("command not found: '{}'", command), &LogType::Error);
                }
            },

            RunActionSpecify => {
                if let Some(action_name) = self.console_in("action: ") {
                    self.perform_ui(&RunAction(action_name));
                } else {
                    self.console_clear();
                }
            },
            RunAction(action_name) => {
                //use pixylene_actions_lua::mlua;
                let mut self_clone = Controller::new(self.target.clone()).unwrap();
                self_clone.set_boundaries((self.b_camera.unwrap(),
                                           self.b_statusline.unwrap(),
                                           self.b_console.unwrap()));

                let Self {
                    sessions,
                    target,
                    sel_session,
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
                } = &mut sessions[*sel_session as usize - 1];

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
                let session_index = self.sel_session as usize - 1;
                if let Some(action_name) = &self.sessions[session_index].last_action_name {
                    self.perform_ui(&RunAction(action_name.clone()));
                }
                else {
                    self.console_out("no previous action to repeat", &LogType::Warning);
                }
            },

            PreviewFocusLayer => {
                let session = &mut self.sessions[self.sel_session as usize - 1];
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
                let session = &mut self.sessions[self.sel_session as usize - 1];
                self.target.borrow_mut().draw_camera(
                    session.pixylene.borrow().project.out_dim,
                    session.pixylene.borrow().project.render(),
                    false,
                    &self.b_camera.unwrap(),
                );
                self.console_in("press ENTER to stop previewing project");
            },

            UpdateStatusline => {
                use colored::Colorize;

                self.target.borrow_mut().clear(&self.b_statusline.unwrap());

                let session = &self.sessions[self.sel_session as usize - 1];
                let mut statusline: Statusline = Vec::new();
                let padding = "     ".on_truecolor(60,60,60);

                statusline.push(session.name.on_truecolor(60,60,60).bright_white());
                statusline.push(padding.clone());

                statusline.push(
                    format!("Layer {} of {}",
                            session.pixylene.borrow().project.focus.1 + 1,
                            session.pixylene.borrow().project.canvas.num_layers())
                    .on_truecolor(60,60,60).bright_white()
                );
                statusline.push(padding.clone());

                statusline.push(
                    format!("Session {} of {}", self.sel_session, self.sessions.len())
                    .on_truecolor(60,60,60).bright_white()
                );
                statusline.push(padding.clone());

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
                statusline.push(padding.clone());

                statusline.push("Palette: ".on_truecolor(60,60,60).bright_white());
                let mut colors_summary = session.pixylene.borrow().project.canvas.palette.colors()
                    .map(|(a,b)| (a.clone(), b.clone())).take(16).collect::<Vec<(u8, Pixel)>>();
                colors_summary.sort_by_key(|(index, _)| *index);
                for (index, color) in colors_summary {
                    statusline.push(
                        format!(" {: <3}", index)
                        .on_truecolor(color.r, color.g, color.b).white()
                    );
                }

                self.target.borrow_mut().draw_statusline(&statusline,
                                                         &self.b_statusline.unwrap());
            }
        }
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
