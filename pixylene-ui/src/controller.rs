use crate::{
    utils::{parse_cmd, deparse},
    ui::{ UserInterface, Rectangle, Statusline, KeyInfo, Key, ReqUiFnMap, UiFn },
    config::Config,
    actions::{ ActionLocation, add_my_native_actions, add_my_lua_actions },
};

use libpixylene::{
    Pixylene,
    project::{ OPixel, Layer, LayersType },
    types::{ UCoord, PCoordContainer, PCoord, Coord, TruePixel },
};
use pixylene_actions::{ memento::{ ActionManager }, Console, LogType };
use pixylene_lua::LuaActionManager;
use dirs::config_dir;
use std::{
    fs::read_to_string,
    collections::{HashMap, hash_map::Iter},
    process::exit,
    cell::RefCell,
    rc::Rc,
    path::PathBuf,
};
use clap::Subcommand;


const SPLASH_LOGO: &str = r#"

    ____  _ ___  ____  _ _     _____ _      _____
   /  __\/ \\  \//\  \/// \   /  __// \  /|/  __/
   |  \/|| | \  /  \  / | |   |  \  | |\ |||  \  
   |  __/| | /  \  / /  | |_/\|  /_ | | \|||  /_ 
   \_/   \_//__/\\/_/   \____/\____\\_/  \|\____\


 Welcome to
 Pixylene,
 the extensible Pixel Art Editor


 type  :new 16 16     - to create a new 16x16 canvas
 type  :lc            - to list all commands
 type  :ln            - to list all namespaces
 type  :lk            - to list the required keys & keys in the default namespace
 type  :q             - to quit
"#;
// type  :help          - if you are new!
// type  :new = { w=16, h=16 }      - to create a new 16x16 canvas 
// type  :import foo.png            - to start editing 'foo.png'
// type  :e foo.json                - to edit a previously saved canvas file 'foo.json'
// type  :ep foo.pixylene           - to edit a previously saved project file 'foo.pixylene'


#[derive(Subcommand)]
pub enum StartType {
    //New { dimensions: Option<Coord>, palette: Option<Palette> },
    //Open { path: String },
    //Import { path: String, palette: Option<Palette> },
    New {
        width: Option<u16>,
        height: Option<u16>,
        #[clap(long, short, action)]
        indexed: bool,
        /*/*todo*/colorscheme: Option<Colorscheme>,*/
    },
    Canvas {
        path: PathBuf
    },
    Project {
        path: PathBuf
    },
    Import {
        path: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
        /*/*todo*/colorscheme: Option<Colorscheme>,*/
    },
}

pub struct PixyleneSession {
    name: String,
    pixylene: Rc<RefCell<Pixylene>>,
    last_action_name: Option<String>,

    canvas_file_path: Option<PathBuf>,
    project_file_path: Option<PathBuf>,

    modified: bool,

    action_map: HashMap<String, ActionLocation>,
    native_action_manager: ActionManager,
    lua_action_manager: Option<LuaActionManager>,
}

pub struct Controller {
    target: Rc<RefCell<dyn UserInterface>>,
    config: Config,
    namespace: Option<String>,

    //sessions
    sessions: Vec<PixyleneSession>,
    sel_session: u8, //1-based index

    //window boundaries
    b_console: Rectangle,
    b_camera: Rectangle,
    b_statusline: Rectangle,
}

struct ControllerLite {
    b_console: Rectangle,
    discard_command: Key,
    target: Rc<RefCell<dyn UserInterface>>,
}

impl Console for ControllerLite {
    fn cmdin(&self, message: &str) -> Option<String> {
        let input = self.target.borrow_mut()
            .console_in(message, &self.discard_command, &self.b_console);
        self.target.borrow_mut().clear(&self.b_console);
        return input;
    }

    fn cmdout(&self, message: &str, log_type: &LogType) {
        self.target.borrow_mut().console_out(message, log_type, &self.b_console);
    }
}

impl Controller {

    pub fn new(target: Rc<RefCell<dyn UserInterface>>, config: Config) -> Self {
        target.borrow_mut().initialize();

        let window_size = target.borrow().get_size();
        let (b_camera, b_statusline, b_console) = compute_boundaries(&window_size, config.padding);

        //clear entire screen
        let current_dim = target.borrow().get_size();
        target.borrow_mut().clear(&Rectangle {
            start: UCoord{ x: 0, y: 0 },
            size: current_dim,
        });

        Self {
            target,
            config,
            namespace: None,

            sessions: Vec::new(),
            sel_session: 0,

            b_console,
            b_camera,
            b_statusline,
        }
    }

    pub fn run(&mut self) {
        use UiFn::{ RunKey, ForceQuit };

        if self.sessions.len() > 0 {
            for func in self.config.every_frame.clone() {
                _ = self.perform_ui(&func);
            }
        } else {
            //welcome screen
            self.target.borrow_mut()
                .draw_paragraph(vec![String::from(SPLASH_LOGO).into()], &self.b_console);
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
                        _ = self.perform_ui(&RunKey{ key: Key::from(key).into() });
                    },
                    KeyInfo::UiFn(ui_fn) => {
                        _ = self.perform_ui(&ui_fn);
                    },
                }

                if self.sessions.len() > 0 {
                    for func in self.config.every_frame.clone() {
                        _ = self.perform_ui(&func);
                    }
                } else {
                    //welcome screen
                    self.target.borrow_mut()
                        .draw_paragraph(vec![String::from(SPLASH_LOGO).into()], &self.b_console);
                }
            }
        }
    }

    fn console_in(&self, message: &str) -> Option<String> {
        let input = self.target.borrow_mut()
            .console_in(message, &self.config.required_keys.discard_command, &self.b_console);
        self.console_clear();
        input
    }

    fn console_out(&self, message: &str, log_type: &LogType) {
        self.target.borrow_mut().console_out(message, log_type, &self.b_console);
    }

    fn console_clear(&self) {
        self.target.borrow_mut().clear(&self.b_console);
    }

    fn sel_session(&self) -> Result<usize, ()> {
        if self.sessions.len() == 0 {
            self.console_out("start a session to use that function. start with :new or :o or :op \
                             or :import", &LogType::Warning);
            Err(())
        } else {
            Ok(usize::from(self.sel_session) - 1)
        }
    }

    pub fn new_session(&mut self, start_type: &StartType, from_args: bool) {
        if self.sessions.len() > 255 {
            if !from_args {
                self.console_out("cannot have more than 256 sessions open", &LogType::Error);
            } else {
                self.target.borrow_mut().finalize();
                eprintln!("cannot have more than 256 sessions open");
                exit(1);
            }
        }

        let mut action_map: HashMap<String, ActionLocation> = HashMap::new();

        //Create the Lua Action-Manager
        let lua_action_manager = match LuaActionManager::new() {
            Ok(mut m) => {
                //Add Lua actions defined in source-code
                add_my_lua_actions(&mut m);

                //Add Lua actions defined in user-config
                let _ = m.load(&match config_dir() {
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
                })
                .map_err(|err| {
                    self.console_out(
                        &format!("Error in user lua config: {err}"),
                        &LogType::Error,
                    );
                });

                //Get names of all Lua actions and store them
                let _ = m.list_actions()
                    .iter()
                    .map(|action_name| {
                        action_map.insert(action_name.clone(), ActionLocation::Lua);
                    })
                    .collect::<()>();
                Some(m)
            },
            Err(err) => {
                self.console_out(
                    &format!("Critical Lua error, cannot create tables:\n{err}\nYou might not have \
                        enough memory. You can still use Pixylene but any Lua modules will not \
                        function."),
                    &LogType::Error,
                );
                None
            },
        };

        add_my_native_actions(&mut action_map);

        match start_type {
            StartType::New { width, height, indexed } => {
                let mut defaults = self.config.defaults.clone();
                if let Some(width) = width {
                    if let Some(height) = height {
                        if let Ok(dim) = PCoord::new(*height, *width) {
                            defaults.dim = dim;
                        } else {
                            if !from_args {
                                self.console_out("invalid dimensions, cannot have 0", 
                                                 &LogType::Error);
                                return;
                            } else {
                                self.target.borrow_mut().finalize();
                                eprintln!("invalid dimensions, cannot have 0");
                                exit(1);
                            }
                        }
                    }
                }
                let mut pixylene = Pixylene::new(&defaults, *indexed);
                pixylene.project.out_dim = self.b_camera.size;
                let dim = pixylene.project.canvas.layers.dim();
                match &mut pixylene.project.canvas.layers {
                    LayersType::True(ref mut layers) => {
                        layers.add_layer(Layer::new_with_solid_color(dim, None)).unwrap();
                    },
                    LayersType::Indexed(ref mut layers) => {
                        layers.add_layer(Layer::new_with_solid_color(dim, None)).unwrap();
                    },
                }
                initialize_project(&mut pixylene);

                let native_action_manager = ActionManager::new(&pixylene.project.canvas);
                self.sessions.push(PixyleneSession {
                    name: String::from("new"),
                    pixylene: Rc::new(RefCell::new(pixylene)),
                    last_action_name: None,
                    canvas_file_path: None,
                    project_file_path: None,
                    modified: false,
                    action_map,
                    native_action_manager,
                    lua_action_manager,
                });
                self.sel_session += 1;
            },
            StartType::Canvas{ path } => {
                match Pixylene::open_canvas(&path, &self.config.defaults) {
                    Ok(mut pixylene) => {
                        pixylene.project.out_dim = self.b_camera.size;
                        initialize_project(&mut pixylene);
                        let native_action_manager = ActionManager::new(&pixylene.project.canvas);
                        self.sessions.push(PixyleneSession {
                            name: path.display().to_string(),
                            pixylene: Rc::new(RefCell::new(pixylene)),
                            last_action_name: None,
                            canvas_file_path: Some(path.clone()),
                            project_file_path: None,
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
            StartType::Project{ path } => {
                match Pixylene::open_project(&path) {
                    Ok(mut pixylene) => {
                        pixylene.project.out_dim = self.b_camera.size;
                        let native_action_manager = ActionManager::new(&pixylene.project.canvas);
                        self.sessions.push(PixyleneSession {
                            name: path.display().to_string(),
                            pixylene: Rc::new(RefCell::new(pixylene)),
                            last_action_name: None,
                            canvas_file_path: None,
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
            StartType::Import{ path, width, height } => {
                let mut resize = None;
                if let Some(width) = width {
                    if let Some(height) = height {
                        if let Ok(dim) = PCoord::new(*height, *width) {
                            resize = Some(dim);
                        } else {
                            if !from_args {
                                self.console_out("invalid dimensions, cannot have 0", 
                                                 &LogType::Error);
                                return;
                            } else {
                                self.target.borrow_mut().finalize();
                                eprintln!("invalid dimensions, cannot have 0");
                                exit(1);
                            }
                        }
                    }
                }
                match Pixylene::import(&path, resize, &self.config.defaults) {
                    Ok(mut pixylene) => {
                        pixylene.project.out_dim = self.b_camera.size;
                        initialize_project(&mut pixylene);
                        let native_action_manager = ActionManager::new(&pixylene.project.canvas);
                        self.sessions.push(PixyleneSession {
                            name: path.display().to_string(),
                            pixylene: Rc::new(RefCell::new(pixylene)),
                            last_action_name: None,
                            canvas_file_path: None,
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


    fn perform_ui(&mut self, func: &UiFn) -> Result<(), ()> {
        use UiFn::*;

        match func {
            //Sessions
            New{ width, height, indexed } => {
                self.new_session(
                    &StartType::New{ width: *width, height: *height, indexed: *indexed },
                    false
                );
            },

            OpenCanvas{ path } => {
                self.new_session(&StartType::Canvas{ path: path.clone() }, false);
            },
            OpenCanvasSpecify => {
                let input = self.console_in("open canvas file: ");
                match input {
                    Some(input) => {
                        _ = self.perform_ui(&OpenCanvas{ path: PathBuf::from(input) });
                    }
                    None => (),
                }
            },
            OpenProject{ path } => {
                self.new_session(&StartType::Project{ path: path.clone() }, false);
            },
            OpenProjectSpecify => {
                let input = self.console_in("open project file: ");
                match input {
                    Some(input) => { 
                        _ = self.perform_ui(&OpenProject{ path: PathBuf::from(input) });
                    },
                    None => (),
                }
            },
            Import{ path, width, height } => {
                self.new_session(
                    &StartType::Import{ path: path.clone(), width: *width, height: *height },
                    false
                );
            },
            //ImportSpecify => {
            //    let input = self.console_in("import: ");
            //    match input {
            //        Some(input) => { 
            //            _ = self.perform_ui(&Import{ path: PathBuf::from(input) });
            //        },
            //        None => (),
            //    }
            //},

            Quit => {
                if self.sessions.len() > 0
                    && self.sessions[self.sel_session as usize - 1].modified {
                    self.console_out(
                        "canvas has been modified since last change, force quit (:q!) to discard \
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
            GoToSession{ index } => {
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
            SaveCanvas => {
                let s = self.sel_session()?;
                let mut did_save = false;
                match &self.sessions[s].canvas_file_path {
                    Some(path) => {
                        match self.sessions[s].pixylene.borrow().save_canvas(&path) {
                            Ok(()) => {
                                self.console_out(
                                    &format!("saved to {}", path.display()),
                                    &LogType::Info
                                );
                                did_save = true;
                            },
                            Err(err) => {
                                self.console_out(
                                    &format!("failed to save: {}", err),
                                    &LogType::Error
                                );
                            }
                        }
                    },
                    None => {
                        let mut new_canvas_file_path: Option<PathBuf> = None;
                        match self.console_in("save path (.json): ") {
                            Some(input) => {
                                self.console_out("saving...", &LogType::Info);
                                let mut path = PathBuf::from(input.clone());
                                path.set_extension("json");
                                match self.sessions[s].pixylene.borrow().save_canvas(&path) {
                                    Ok(()) => {
                                        self.console_out(
                                            &format!("saved to {}", path.display()),
                                            &LogType::Info
                                        );
                                        new_canvas_file_path = Some(path);
                                        did_save = true;
                                    },
                                    Err(err) => {
                                        self.console_out(
                                            &format!("failed to save: {}", err),
                                            &LogType::Error
                                        );
                                    }
                                }
                            },
                            None => (),
                        }
                        if let Some(path) = new_canvas_file_path {
                            self.sessions[s].canvas_file_path =
                                Some(path);
                        }
                    },
                }
                if did_save {
                    self.sessions[s].modified = false;
                }
            },
            SaveProject => {
                let s = self.sel_session()?;
                let mut did_save = false;
                match &self.sessions[s].project_file_path {
                    Some(path) => {
                        match self.sessions[s].pixylene.borrow()
                            .save_project(&path)
                        {
                            Ok(()) => {
                                self.console_out(
                                    &format!("saved to {}", path.display()),
                                    &LogType::Info
                                );
                                did_save = true;
                            },
                            Err(err) => {
                                self.console_out(
                                    &format!("failed to save: {}", err),
                                    &LogType::Error
                                );
                            }
                        }
                    },
                    None => {
                        let mut new_project_file_path: Option<PathBuf> = None;
                        match self.console_in("save path (.pixylene): ") {
                            Some(input) => {
                                self.console_out("saving...", &LogType::Info);
                                let mut path = PathBuf::from(input.clone());
                                path.set_extension("pixylene");
                                match self.sessions[s].pixylene
                                    .borrow().save_project(&path) {
                                    Ok(()) => {
                                        self.console_out(
                                            &format!("saved to {}", path.display()),
                                            &LogType::Info
                                        );
                                        new_project_file_path = Some(path);
                                        did_save = true;
                                    },
                                    Err(err) => {
                                        self.console_out(
                                            &format!("failed to save: {}", err),
                                            &LogType::Error
                                        );
                                    }
                                }
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
                //todo: instead of taking scaling factor, let user know canvas dimensions and then
                //ask for both export width and height
                match self.console_in("export path (.png): ") {
                    Some(path) => match self.console_in("scaling factor: ") {
                        Some(input) => match str::parse::<u16>(&input) {
                            Ok(scale_up) => {
                                let resize = PCoord::new(scale_up as u32, scale_up as u32)
                                    .map(|scale| PCoordContainer::<u32>::from(
                                        self.sessions[s].pixylene.borrow().project.canvas.layers
                                        .dim()
                                    ).0.mul(scale))
                                    .map_err(|_| self.console_out(
                                        "scaling factor cannot be 0",
                                        &LogType::Error
                                    ))?
                                    .map_err(|_| self.console_out(
                                        "scaling factor too large",
                                        &LogType::Error
                                    ))?;
                                let mut path = PathBuf::from(path.clone());
                                path.set_extension("png");
                                self.console_out("exporting...", &LogType::Info);
                                match self.sessions[s].pixylene.borrow().export(Some(resize),
                                                                                &path)
                                {
                                    Ok(()) => {
                                        self.console_out(
                                            &format!("exported to {}", path.display()),
                                            &LogType::Info
                                        );
                                    },
                                    Err(err) => {
                                        self.console_out(
                                            &format!("failed to export: {}", err),
                                            &LogType::Error
                                        );
                                    }
                                }
                            },
                            Err(_) => {
                                self.console_out(
                                    &format!("invalid scaling factor: '{}'", input),
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

            EnterNamespace{ name } => {
                if let Some(name) = name {
                    if let Some(_) = self.config.possible_namespaces.get(name) {
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
                _ = self.perform_ui(&EnterNamespace{ name: None });
            },
            RunKey{ key } => {
                //special required keys
                if *key == self.config.required_keys.force_quit {
                    _ = self.perform_ui(&ForceQuit);
                }
                else if *key == self.config.required_keys.start_command {
                    _ = self.perform_ui(&RunCommandSpecify);
                }

                //other keys
                else {
                    if let Some(funcs) = self.config.keymap
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
                    _ = self.perform_ui(&RunCommand{ cmd });
                } else {
                    self.console_clear();
                }
            },
            RunCommand{ cmd } => {
                //todo: ignore --help, -help, -h, & help
                //todo: exlude separator ',' when error of arguments not provided
                parse_cmd(cmd)
                    .map(|uifn| {
                        _ = self.perform_ui(&uifn);
                    })
                    .unwrap_or_else(|err| {
                        self.console_out(&format!("{}", err.lines()
                            .filter(|s| s.len() > 0)
                            .take_while(|s| !s.contains("--help") &&
                                        !s.contains("Commands:"))
                            .map(|s| s.trim().to_owned()).reduce(|a, b| a + ", " + &b)
                            .unwrap_or("".to_owned())
                        ), &LogType::Error);
                    });
            },

            RunActionSpecify => {
                _ = self.sel_session()?;
                if let Some(action_name) = self.console_in("action: ") {
                    _ = self.perform_ui(&RunAction{ name: action_name });
                } else {
                    self.console_clear();
                }
            },
            RunAction{ name } => {
                let s = self.sel_session()?;

                let visible_target = ControllerLite {
                    b_console: self.b_console,
                    discard_command: self.config.required_keys.discard_command.clone(),
                    target: self.target.clone(),
                };

                let Self {
                    sessions,
                    target,
                    b_console,
                    b_camera,
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

                match action_map.get(&name.clone()) {
                    Some(action_location) => {
                        target.borrow_mut().clear(&b_console);
                        match action_location {
                            ActionLocation::Lua => {
                                match lua_action_manager.as_mut().unwrap() //cant fail because if
                                                                            //lua_action_manager
                                                                            //doesn't exist, no
                                                                            //actions can have
                                                                            //location Lua (check
                                                                            //action_map in fn
                                                                            //new_session)
                                    .invoke_action(&name, pixylene.clone(), Rc::new(visible_target))
                                {
                                    Ok(()) => {
                                        if native_action_manager
                                            .commit(&pixylene.borrow().project.canvas)
                                        {
                                            *last_action_name = Some(name.clone());
                                            *modified = true;
                                        }
                                    },
                                    Err(err) => {
                                        use colored::Colorize;

                                        let error = format!(
                                            "{}",
                                            err.to_string().lines()
                                                .map(|s| s.to_string().replace("\t", " "))
                                                .collect::<Vec<String>>().join(", ")
                                        );
                                        if error.len() <= b_console.size.y().into() {
                                            target.borrow_mut().console_out(
                                                &error,
                                                &LogType::Error,
                                                &b_console
                                            );
                                        } else {
                                            target.borrow_mut().draw_paragraph(
                                                vec![error.red()],
                                                &b_camera
                                            );
                                            self.console_in("press ENTER to close error");
                                        }
                                    }
                                }
                            },
                            ActionLocation::Native(action) => {
                                let performed = native_action_manager.perform(
                                    &mut pixylene.borrow_mut().project,
                                    &visible_target,
                                    action.clone()
                                );
                                match performed {
                                    Ok(()) => {
                                        if native_action_manager
                                            .commit(&pixylene.borrow().project.canvas)
                                        {
                                            *last_action_name = Some(name.clone());
                                            *modified = true;
                                        }
                                    },
                                    Err(err) => {
                                        target.borrow_mut().console_out(
                                            //&format!("failed to perform: {}", err.to_string()),
                                            &format!("{}", err.to_string()),
                                            &LogType::Error,
                                            &b_console
                                        );
                                    }
                                }
                            },
                        }
                    },
                    None => {
                        target.borrow_mut().console_out(
                            &format!("action '{}' was not found in actions inserted into the \
                                     action-manager", name),
                            &LogType::Error,
                            &b_console
                        );
                    }
                }
            },
            RunLastAction => {
                let s = self.sel_session()?;
                if let Some(action_name) = &self.sessions[s].last_action_name {
                    _ = self.perform_ui(&RunAction{ name: action_name.clone() });
                }
                else {
                    self.console_out("no previous action to repeat", &LogType::Warning);
                }
            },

            RunLuaSpecify => {
                _ = self.sel_session()?;
                if let Some(statement) = self.console_in("lua statement: ") {
                    _ = self.perform_ui(&RunLua{ statement });
                } else {
                    self.console_clear();
                }
            },
            RunLua{ statement } => {
                let s = self.sel_session()?;
                let visible_target = ControllerLite {
                    b_console: self.b_console,
                    discard_command: self.config.required_keys.discard_command.clone(),
                    target: self.target.clone(),
                };

                let Self {
                    sessions,
                    target,
                    b_console,
                    b_camera,
                    ..
                } = self;

                let PixyleneSession {
                    ref mut pixylene,
                    ref mut native_action_manager,
                    ref mut lua_action_manager,
                    ref mut modified,
                    ..
                } = &mut sessions[s];

                _ = lua_action_manager.as_ref().ok_or_else(|| {
                    target.borrow_mut().console_out(
                        &format!("This command cannot be used as Lua modules are disabled."),
                        &LogType::Error,
                        &b_console,
                    );
                })?;

                target.borrow_mut().clear(&b_console);
                match lua_action_manager.as_mut().unwrap() //cant fail because checked like 2 lines ago
                    .invoke(statement, pixylene.clone(), Rc::new(visible_target))
                {
                    Ok(()) => {
                        if native_action_manager.commit(&pixylene.borrow().project.canvas) {
                            *modified = true;
                        }
                    },
                    Err(err) => {
                        use colored::Colorize;

                        let error = format!(
                            "{}",
                            err.to_string().lines()
                                .map(|s| s.to_string().replace("\t", " "))
                                .collect::<Vec<String>>().join(", ")
                        );
                        if error.len() <= b_console.size.y().into() {
                            target.borrow_mut().console_out(
                                &error,
                                &LogType::Error,
                                &b_console
                            );
                        } else {
                            target.borrow_mut().draw_paragraph(
                                vec![error.red()],
                                &b_camera
                            );
                            self.console_in("press ENTER to close error");
                        }
                    }
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
                    &self.b_camera,
                );
            },

            PreviewProject => {
                let s = self.sel_session()?;
                let session = &mut self.sessions[s];
                self.target.borrow_mut().draw_camera(
                    session.pixylene.borrow().project.out_dim,
                    session.pixylene.borrow().project.render(),
                    false,
                    &self.b_camera,
                );
                self.console_in("press ENTER to stop previewing project");
            },

            PrintCanvasJson => {
                let s = self.sel_session()?;
                let session = &mut self.sessions[s];
                self.target.borrow_mut().draw_paragraph(
                    vec![session.pixylene.borrow().project.canvas.to_json()
                        .unwrap_or_else(|err| err.to_string()).into()],
                    &self.b_camera
                );
                self.console_in("press ENTER to stop previewing canvas JSON");
            },

            ListNamespaces => {
                use colored::{ Colorize, ColoredString };
                let mut paragraph: Vec<ColoredString> = vec![
                    "".into(),
                    "Namespaces".underline().bright_yellow(),
                    format!(
                        " {:<20} : {} keybinds",
                        "Default".bright_magenta(),
                        self.config.keymap.get(&None).as_ref().unwrap().len(),
                    )
                    .into()
                ];
                let _ = self.config.keymap.iter()
                    .filter(|(namespace, _)| namespace.is_some())
                    .map(|(namespace, keys)| {
                        paragraph.push(
                            format!(
                                " {:<20} : {} keybinds",
                                namespace.as_ref().unwrap().to_owned().bright_magenta(),
                                keys.len(),
                            )
                            .into()
                        );
                    })
                    .collect::<()>();
                self.target.borrow_mut().clear_all();
                self.target.borrow_mut().draw_paragraph(paragraph, &self.b_camera);
                let _ = self.console_in("press ENTER to exit listing namespaces");
                self.target.borrow_mut().clear_all();
            },

            ListKeybindMap{ namespace } => {
                use colored::{ Colorize, ColoredString };
                let half_width = self.target.borrow().get_size().y() as usize/2;
                let print_namespace_simple =
                    |keys: Iter<Key, Vec<UiFn>>, paragraph: &mut Vec<ColoredString>| {
                        let _ = keys
                            .map(|(key, ui_fns)| {
                                paragraph.push(
                                    format!("{:<half_width$}",
                                        format!(
                                            "  {:<10} -> {}",
                                            key.to_string().bright_magenta(),
                                            if self.config.keymap_show_command_names {
                                                format!("{:?}", ui_fns)
                                            } else {
                                                deparse(ui_fns)
                                            }
                                        )
                                    )
                                    .into()
                                );
                            })
                            .collect::<()>();
                    };

                let print_namespace_compact =
                    |mut keys: Iter<Key, Vec<UiFn>>, paragraph: &mut Vec<ColoredString>| loop {
                        let mut line = String::new();
                        if let Some((key, ui_fns)) = keys.next() {
                            line.push_str(
                                &format!("{:<half_width$}",
                                    format!(
                                        "  {:<10} -> {}",
                                        key.to_string().bright_magenta(),
                                        if self.config.keymap_show_command_names { format!("{:?}", ui_fns) }
                                        else { deparse(ui_fns) }
                                    )));
                        } else {
                            break;
                        }

                        if let Some((key, ui_fns)) = keys.next() {
                            line.push_str(&format!(
                                "{:<10} -> {}",
                                key.to_string().bright_magenta(),
                                if self.config.keymap_show_command_names { format!("{:?}", ui_fns) }
                                else { deparse(ui_fns) }
                            ));
                            paragraph.push(line.into());
                        } else {
                            paragraph.push(line.into());
                            break;
                        }
                    };

                let mut paragraph: Vec<ColoredString> = Vec::new();

                self.target.borrow_mut().clear_all();

                match namespace {
                    //list required keys and keys in default namespace
                    None => {
                        //todo: refactor so later may be used separately in :help
                        //required keys
                        {
                            let ReqUiFnMap {
                                ref force_quit,
                                ref start_command,
                                ref discard_command } = self.config.required_keys;
                            paragraph.push("".into());
                            paragraph.push("Required Keys".underline().bright_yellow());
                            paragraph.push(format!(
                                "  Force Quit <- {},   Start Command <- {},   Discard Command <- {}",
                                force_quit.to_string().bright_magenta(),
                                start_command.to_string().bright_magenta(),
                                discard_command.to_string().bright_magenta(),
                            ).into());
                        }

                        //keys in default namespace
                        paragraph.push(ColoredString::from(""));
                        paragraph.push("Default Namespace".underline().bright_yellow());
                        print_namespace_compact(
                            self.config.keymap.get(&None).unwrap().iter(),
                            &mut paragraph
                        );
                    }

                    //list keys in requested namespace
                    Some(namespace) => {
                        paragraph.push(ColoredString::from(""));
                        paragraph.push(
                            format!("Namespace '{namespace}'").underline().bright_yellow());

                        if let Some(namespace) = self.config.keymap.get(&Some(namespace.to_string())) {
                            print_namespace_simple(namespace.iter(), &mut paragraph);
                        }
                    }
                }

                self.target.borrow_mut().draw_paragraph(paragraph,
                    &self.b_camera
                );
                self.console_in("press ENTER to exit listing keybindings");
                self.target.borrow_mut().clear_all();
            },

            ListCommands => {
                use colored::{ Colorize, ColoredString };
                self.target.borrow_mut().clear_all();
                self.target.borrow_mut().draw_paragraph(
                    vec![
                        vec!["Commands".underline().bright_yellow()],
                        parse_cmd("--help").unwrap_err().lines().skip(3)
                            .take_while(|line| !line.contains("  help"))
                            .map(|l| l.bright_magenta()).collect::<Vec<ColoredString>>()
                    ].into_iter().flatten().collect(),
                    &self.b_camera
                );
                self.console_in("press ENTER to exit listing commands");
                self.target.borrow_mut().clear_all();
            },

            DrawStatusline => {
                use colored::Colorize;
                let s = self.sel_session()?;

                self.target.borrow_mut().clear(&self.b_statusline);

                let session = &self.sessions[s];
                let mut statusline: Statusline = Vec::new();
                let padding = " ".on_truecolor(60,60,60);
                let spacing = "  ".on_truecolor(60,60,60);
                let divider = "｜".on_truecolor(60,60,60).truecolor(100,100,100);

                {
                    //Namespace
                    statusline.push(divider.clone());
                    statusline.push(
                        self.namespace.clone().unwrap_or(String::from("Default"))
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

                    //Session canvas-type (true|indexed)
                    statusline.push(match session.pixylene.borrow().project.canvas.layers {
                        LayersType::True(_) => "rgba",
                        LayersType::Indexed(_) => "indexed",
                    }.on_truecolor(60,60,60));
                    statusline.push(spacing.clone());

                    //Session dimensions
                    statusline.push(
                        format!("{}", session.pixylene.borrow().project.canvas.layers.dim())
                        .on_truecolor(60,60,60).bright_white()
                    );
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
                                session.pixylene.borrow().project.canvas.layers.len())
                        .on_truecolor(60,60,60).bright_white()
                    );
                    statusline.push(spacing.clone());

                    //Layer opacity
                    statusline.push(
                        format!("{:.2}%",
                            match &session.pixylene.borrow().project.canvas.layers {
                                LayersType::True(ref layers) => layers.get_layer(
                                    session.pixylene.borrow().project.focus.1)
                                    .map(|layer| (f32::from(layer.opacity)/2.55)),
                                LayersType::Indexed(ref layers) => layers.get_layer(
                                    session.pixylene.borrow().project.focus.1)
                                    .map(|layer| (f32::from(layer.opacity)/2.55)),
                            }
                            .unwrap_or(0.0),
                        )
                        .on_truecolor(60,60,60).bright_white()
                    );

                    //Layer mute
                    if match &session.pixylene.borrow().project.canvas.layers {
                        LayersType::True(ref layers) => layers.get_layer(
                            session.pixylene.borrow().project.focus.1)
                            .map(|layer| layer.mute),
                        LayersType::Indexed(ref layers) => layers.get_layer(
                            session.pixylene.borrow().project.focus.1)
                            .map(|layer| layer.mute),
                    }
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
                        .collect::<Vec<(u8, TruePixel, bool)>>();
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
                                                         &self.b_statusline);
            },
        }
        Ok(())
    }
}

// returns boundaries of camera, statusline and console respectively
fn compute_boundaries(window: &PCoord, padding: u8) -> (Rectangle, Rectangle, Rectangle) {
    (
    /* camera: */Rectangle {
        start: UCoord{ x: 0 + padding as u16, y: 0 + padding as u16 },
        size: PCoord::new(
            window.x() - 3 - 2*padding as u16,
            window.y() - 2*padding as u16
        ).unwrap()
    },
    /* statusline: */Rectangle {
        start: UCoord{ x: window.x() - 2 - padding as u16, y: 0 + padding as u16 },
        size: PCoord::new(1, window.y() - 2*padding as u16).unwrap()
    },
    /* console: */Rectangle {
        start: UCoord{ x: window.x() - 1 - padding as u16, y: 0 + padding as u16 },
        size: PCoord::new(1, window.y() - 2*padding as u16).unwrap()
    }
    )
}

fn initialize_project(pixylene: &mut Pixylene) {
    let dim = pixylene.project.canvas.layers.dim();

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
}
