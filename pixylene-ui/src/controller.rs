use crate::{
    ui::{ UserInterface, Mode, Rectangle },
    keybinds::{ KeyMap, ReverseKeyMap, KeyFn, UiFn, get_keybinds },
};

use libpixylene::{
    Pixylene, PixyleneDefaults,
    project::{ OPixel, Palette }, 
    types::{ UCoord, PCoord, Coord },
};
use pixylene_actions::{ memento::{ Action, ActionManager }, Console, LogType };
use pixylene_actions_lua::LuaActionManager;
use std::collections::HashMap;
use std::process::exit;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use std::thread::{ sleep };
use std::time::Duration;
use clap::{ Subcommand };


type CommandMap = HashMap<String, UiFn>;

enum ActionLocation {
    Native(Rc<RefCell<dyn Action>>),
    Lua,
}

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
    pixylene: Rc<RefCell<Pixylene>>,
    last_action_name: Option<String>,
    project_file_path: Option<String>,
    modified: bool,
    mode: Mode,

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
                           rev_keymap: &ReverseKeyMap)
        -> Console
    {
        Console {
            cmdin: Box::new(|message: String| -> Option<String> {
                    target.borrow().console_in(&message,
                                               &rev_keymap.get(&KeyFn::DiscardCommand)
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
    sel_session: u8,

    defaults: PixyleneDefaults,

    //console: Console,
    actions_loader: fn(&mut ActionManager) -> (),

    keymap: KeyMap,
    rev_keymap: ReverseKeyMap,
    cmd_map: CommandMap,

    //Window boundaries
    b_console: Option<Rectangle>,
    b_camera: Option<Rectangle>,
    b_statusline: Option<Rectangle>,

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

    fn set_boundaries(&mut self, b_camera: Rectangle, b_statusline: Rectangle,
                      b_console: Rectangle) {
        self.b_camera = Some(b_camera);
        self.b_statusline = Some(b_statusline);
        self.b_console = Some(b_console);
    }

    fn console_in(&self, message: &str) -> Option<String> {
        self.target.borrow_mut().console_in(message,
                                        self.rev_keymap.get(&KeyFn::DiscardCommand).unwrap(),
                                        &self.b_console.unwrap())
    }

    fn console_out(&self, message: &str, log_type: &LogType) {
        self.target.borrow_mut().console_out(message,
                                         log_type,
                                         &self.b_console.unwrap());
    }

    fn console_clear(&self) {
        self.target.borrow_mut().console_clear(&self.b_console.unwrap());
    }

    //pub fn new(target: Rc<RefCell<T>>) -> Self {
    pub fn new(target: Rc<RefCell<dyn UserInterface>>) -> Self {
        use UiFn::*;

        //Go parse TOML config and get pixylene defaults
        let defaults = get_pixylene_defaults();

        //Go parse TOML config, get keybinds, go deserialize them into key-map and reverse key-map
        let ( keymap, rev_keymap ): (KeyMap, ReverseKeyMap) = get_keybinds();

        let actions_loader = |_action_manager: &mut ActionManager| {
            ()
        };
        //let target_clone: Rc<RefCell<dyn Target>> = Rc::clone(&target);

        Self {
            target,

            sessions: Vec::new(),
            sel_session: 0,

            defaults,

            actions_loader,

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
                (String::from("a"), RunAction),
                (String::from("showlayer"), PreviewFocusLayer),
                (String::from("showproject"), PreviewProject),
                (String::from("showstatus"), UpdateStatusline),
            ]),
            keymap,
            rev_keymap,

            b_console: None,
            b_camera: None,
            b_statusline: None,

            started: true,
        }
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

        //Load the UI-wide actions into the action_manager for this new session
        //let mut action_manager = ActionManager::new(HashMap::new());
        let native_action_manager;
        //(self.actions_loader)(&mut action_manager);

        let lua_action_manager = LuaActionManager::setup(
            Path::new("/home/bhavya/.config/pixylene.lua")
        ).unwrap();
        let mut action_map: HashMap<String, ActionLocation> = HashMap::new();
        lua_action_manager.list_actions().iter().map(|action_name| {
            action_map.insert(action_name.clone(), ActionLocation::Lua);
        }).collect::<Vec<()>>();

        match start_type {
            Some(StartType::New) => {
                let mut pixylene = Pixylene::new(&self.defaults);
                if let Some(b_camera) = self.b_camera {
                    pixylene.project.out_dim = b_camera.size;
                }
                native_action_manager = ActionManager::new(&pixylene.project.canvas);
                self.sessions.push(PixyleneSession {
                    pixylene: Rc::new(RefCell::new(pixylene)),
                    last_action_name: None,
                    project_file_path: None,
                    modified: false,
                    mode: Mode::Normal,
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
                            pixylene: Rc::new(RefCell::new(pixylene)),
                            last_action_name: None,
                            project_file_path: Some(path.clone()),
                            modified: false,
                            mode: Mode::Normal,
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
                        pixylene.project.focus.0 = Coord {
                            x: i32::from(dim.x()).checked_div(2).unwrap(),
                            y: i32::from(dim.y()).checked_div(2).unwrap(),
                        };
                        pixylene.project.out_repeat = PCoord::new(1,1).unwrap();

                        native_action_manager = ActionManager::new(&pixylene.project.canvas);
                        self.sessions.push(PixyleneSession {
                            pixylene: Rc::new(RefCell::new(pixylene)),
                            last_action_name: None,
                            project_file_path: Some(path.clone()),
                            modified: false,
                            mode: Mode::Normal,
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
        self.start();
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
        use UiFn::{ PreviewFocusLayer, UpdateStatusline };
        //let mut mode = Mode::Normal;

        self.target.borrow_mut().initialize();

        let window_size = self.target.borrow().get_size();

        let padding = 1;
        self.set_boundaries(
            /* camera: */Rectangle {
                start: UCoord{ x: 0 + padding, y: 0 + padding },
                size: PCoord::new(window_size.x() - 2 - 2*padding, window_size.y() - 2*padding)
                    .unwrap()
            },
            /* statusline: */Rectangle {
                start: UCoord{ x: window_size.x() - 2 - padding, y: 0 + padding },
                size: PCoord::new(1, window_size.y() - 2*padding).unwrap()
            },
            /* console: */Rectangle {
                start: UCoord{ x: window_size.x() - 1 - padding, y: 0 + padding },
                size: PCoord::new(1, window_size.y() - 2*padding).unwrap()
            }
        );

        // case when started from new_session instead of start directly
        if self.sessions.len() == 1 {
            self.sessions[0].pixylene.borrow_mut().project.out_dim = self.b_camera.unwrap().size;
        }

        self.perform_ui(&PreviewFocusLayer);
        //self.perform_ui(&UpdateStatusline);
        loop {
            //sleep(Duration::from_millis(1));

            if !self.target.borrow_mut().refresh() { break; }
            let key = self.target.borrow().get_key();
            if let Some(key) = key {
                if let Some(funcs) = self.keymap.get(&key) {
                    for func in (*funcs).clone() {
                        self.perform_ui(&func);
                    }
                } else {
                    self.console_out(&format!("unmapped key: {:?}", key), &LogType::Warning);
                }

                self.perform_ui(&PreviewFocusLayer);
                //self.perform_ui(&UpdateStatusline);
            }
            /*
            match &mode {
                Mode::Normal => {
                    /*
                    
                    C-s => emacs_mode = Mode::Shape { Some(last_shape) },
                    C-S-s => emacs_mode = Mode::Shape { None },

                    */
                }
                Mode::Ooze => {
                }
                Mode::Shape/*{ shape }*/ => {
                }

                //Modes that do not use the equipped color
                Mode::Layer => {
                    /*

                    n => new layer
                    d => delete layer
                    r => rename layer
                    c => clone layer
                    - => go to lower layer
                    + => go to upper layer

                    */
                }
                Mode::Command => {
                }
                Mode::Cursors => {
                }
            }
            */
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
                        &format!(
                            "project has been modified since last change, \
                            force quit ({:?}) to discard modifications",
                            self.rev_keymap.get(&KeyFn::ForceQuit), //todo: do reverse lookup on
                                                                    //this instead of using special
                                                                    //rev_keymap reversed for
                                                                    //operations that require
                                                                    //reverse mapping like
                                                                    //discard-key
                        ),
                        &LogType::Error,
                    );
                } else {
                    self.quit_session();
                }
            },
            ForceQuit => { 
                self.quit_session();
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
                                    },
                                    Err(err) => {
                                        self.console_out(
                                            &format!("failed to save: {}",
                                                     err),
                                            &LogType::Error
                                        );
                                    }
                                }
                                self.console_out(
                                    &format!("saved to {}", input),
                                    &LogType::Info,
                                );

                            },
                            None => (),
                        }
                    },
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
                                            &format!("exported to {}", input),
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

            //Command -> Recursive, translates string and calls this fn
            RunCommand => {
                if let Some(cmd) = self.console_in(":") {
                    let mut func: Option<UiFn> = None;
                    if let Some(mapped_func) = self.cmd_map.get(&cmd) {
                        func = Some(mapped_func.clone());
                    }
                    if let Some(func) = func {
                        self.perform_ui(&func);
                    } else {
                        self.console_out(&format!("command not found: '{}'", cmd), &LogType::Error);
                    }
                } else {
                    self.console_clear();
                }
            },

            RunAction => {
                if let Some(action_name) = self.console_in("a: ") {
                    let mut self_clone = Controller::new(self.target.clone());
                    self_clone.set_boundaries(self.b_camera.unwrap(),
                                              self.b_statusline.unwrap(),
                                              self.b_console.unwrap());

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
                        ..
                    } = &mut sessions[*sel_session as usize - 1];

                    match action_map.get(&action_name/*.clone()*/) {
                        Some(action_location) => match action_location {
                            ActionLocation::Lua => {
                                lua_action_manager.invoke(&action_name, pixylene.clone(),
                                                          Rc::new(self_clone)).unwrap();
                            },
                            ActionLocation::Native(action) => {
                                match native_action_manager.perform(
                                    &mut pixylene.borrow_mut().project,
                                    &self_clone,
                                    action.clone()
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
                            },
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
                } else {
                    self.console_clear();
                }
            },
            RunActionSpecify(action_name) => {
                let mut self_clone = Controller::new(self.target.clone());
                self_clone.set_boundaries(self.b_camera.unwrap(),
                                          self.b_statusline.unwrap(),
                                          self.b_console.unwrap());

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
                    ..
                } = &mut sessions[*sel_session as usize - 1];

                match action_map.get(&action_name.clone()) {
                    Some(action_location) => match action_location {
                        ActionLocation::Lua => {
                            match lua_action_manager.invoke(&action_name, pixylene.clone(),
                                                            Rc::new(self_clone)) {
                                Ok(()) => (()),
                                Err(err) => {
                                    target.borrow_mut().console_out(
                                        &format!("failed to perform: {}", err.to_string()),
                                        &LogType::Error,
                                        &b_console.unwrap()
                                    );
                                }
                            }
                        },
                        ActionLocation::Native(action) => {
                            match native_action_manager.perform(
                                &mut pixylene.borrow_mut().project,
                                &self_clone,
                                action.clone()
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
                        },
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
                todo!()
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
            },

            UpdateStatusline => {
                let session = &self.sessions[self.sel_session as usize - 1];
                self.target.borrow_mut().draw_statusline(&session.pixylene.borrow().project,
                                                         &session.native_action_manager,
                                                         &session.mode,
                                                         &self.sel_session,
                                                         &self.b_statusline.unwrap());
            }
        }
    }
}

fn get_pixylene_defaults(/*fallback: PixyleneFallback*/) -> PixyleneDefaults {
    PixyleneDefaults {
        dim: PCoord::new(64, 64).unwrap(),
        palette: Palette::from(&[(1, "#ffffff"), (2, "#000000"), (3, "#00000000")]).unwrap(),
    }
}

struct Echo;
impl pixylene_actions::memento::Action for Echo {
    fn perform(&mut self, project: &mut libpixylene::project::Project, console: &dyn Console)
    -> pixylene_actions::memento::ActionResult {
        console.cmdout("heyyy :3 :3 :3", &LogType::Error);
        Ok(())
    }
    fn has_ended(&self) -> bool {
        true
    }
}
