use libpixylene::{
    Pixylene,
    types::Coord,
    project::{ CameraPixel, ProjectPixel },
    action,
};
use crate::{
    utils::LogType,
    modes::*,
    tui_actions::add_tui_actions,
    raw_actions::add_raw_actions,
};

use crossterm::{
    execute,
    queue,
    terminal,
    style,
    cursor,
    event
};
use std::collections::HashMap;
use std::rc::Rc;


pub struct Console {
    console_corner: Coord,
    discard_key: event::KeyEvent,
}
impl Console {
    /*
    pub fn new(console_corner: Coord) -> Console {
        Console { console_corner }
    }
    */
    pub fn cmdin(&self, message: &str) -> Option<String> {
        use terminal::{ Clear, ClearType };
        use cursor::{ MoveTo, MoveRight, MoveLeft, Show, Hide };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
        use event::{ Event, KeyEvent, KeyCode, read };

        let mut out: Option<String> = None;

        execute!(
            std::io::stdout(),
            ResetColor,
            MoveTo(self.console_corner.y as u16, self.console_corner.x as u16),
            Clear(ClearType::UntilNewLine),
            SetForegroundColor(Color::Rgb{ r: 220, g: 220, b: 220 }),
            Print(&message),
            ResetColor,
            Show,
        ).unwrap();

        let mut input = String::new();
        loop {
            let event = read().unwrap();
            if let Event::Key(key) = event {
                if key == self.discard_key {
                    execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap();
                    out = None;
                    break;
                }
                let KeyEvent { code, .. } = key;
                match code {
                    KeyCode::Enter => {
                        execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap();
                        out = Some(input);
                        break;
                    },
                    KeyCode::Esc => {
                    },
                    KeyCode::Backspace => {
                        if input.len() > 0 {
                            execute!(std::io::stdout(), MoveLeft(1), Clear(ClearType::UntilNewLine)).unwrap();
                            input.pop();
                        }
                    },
                    KeyCode::Char(c) => {
                        execute!(std::io::stdout(), Print(c)).unwrap();
                        input.push(c);
                    },
                    _ => {},
                }
            }
        }
        execute!(std::io::stdout(), Hide).unwrap();
        out
    }
    pub fn cmdout(&self, message: &str, log_type: LogType) {
        use terminal::{ Clear, ClearType };
        use cursor::{ MoveTo };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
        let corner = self.console_corner.add(Coord{ x: 0, y: 0 });
        execute!(
            std::io::stdout(),
            ResetColor,
            MoveTo(corner.y as u16, corner.x as u16),
            Clear(ClearType::UntilNewLine),
            SetForegroundColor(match log_type {
                LogType::Info => Color::Rgb{ r: 240, g: 240, b: 240 },
                LogType::Error => Color::Rgb{ r: 255, g: 70, b: 70 },
                LogType::Warning => Color::Rgb{ r: 70, g: 235, b: 235 },
            }),
            Print(&message),
            ResetColor,
        ).unwrap();
    }
}

pub struct PixyleneTUI {
    console: Rc<Console>,
    camera_corner: Coord,
    statusline_corner: Coord,
    info_corner: Coord,
    pixylene: Option<Pixylene>,
    last_action_name: Option<String>,
    project_file_path: Option<String>,
    actions: HashMap<String, Box<dyn action::Action>>,
    discard_key: event::KeyEvent,
}
impl PixyleneTUI {
    pub fn new(
        console_corner: Coord,
        camera_corner: Coord,
        statusline_corner: Coord,
        info_corner: Coord,
        pixylene: Option<Pixylene>,
        project_file_path: Option<String>,
        discard_key: event::KeyEvent,
    ) -> PixyleneTUI {
        let console = Console{ console_corner, discard_key };
        let mut pixylene_tui = PixyleneTUI {
            console: Rc::new(console),
            camera_corner,
            statusline_corner,
            info_corner,
            pixylene,
            last_action_name: None,
            project_file_path,
            actions: HashMap::new(),
            discard_key,
        };
        add_raw_actions(&mut pixylene_tui.actions);
        add_tui_actions(&mut pixylene_tui.actions, &pixylene_tui.console);
        pixylene_tui.dispatch_actions();
        return pixylene_tui;
    }
    /*
    pub fn create_console(console_corner: Coord) -> Console {
        Console{ console_corner }
    }
    */
    pub fn cmdin(&self, message: &str) -> Option<String> {
        self.console.cmdin(message)
    }
    pub fn cmdout(&self, message: &str, log_type: LogType) {
        self.console.cmdout(message, log_type)
    }
    pub fn preview(&mut self) {
        use cursor::{ MoveTo, MoveLeft, MoveDown };
        use style::{ SetBackgroundColor, Color, Print, ResetColor };
        use std::io::Write;

        let PixyleneTUI { pixylene: p, camera_corner: corner, .. } = self;
        let p: &mut Pixylene = p.as_mut().unwrap();
        let mut stdout = std::io::stdout();

        queue!(stdout, ResetColor).unwrap();
        queue!(
            stdout,
            MoveTo(corner.y as u16, corner.x as u16)
        ).unwrap();
        let grid: Vec<CameraPixel> = p.project.render();

        for i in 0..p.project.camera.dim.x {
            for j in 0..p.project.camera.dim.y {
                match grid[i as usize*p.project.camera.dim.y as usize + j as usize] {
                    CameraPixel::Filled{ color, .. } => {
                        if color.a != 255 {
                            panic!("alpha not 255");
                        }
                        queue!(
                            stdout,
                            SetBackgroundColor(Color::Rgb{
                                r: color.r,
                                g: color.g,
                                b: color.b,
                            }),
                            Print(" "),
                        ).unwrap();
                    },
                    CameraPixel::Empty{ .. } => {
                        queue!(
                            stdout,
                            ResetColor,
                            Print(" "),
                        ).unwrap();
                    },
                    CameraPixel::OutOfScene => {
                        queue!(
                            stdout,
                            ResetColor,
                            Print(" "),
                        ).unwrap();
                    }
                }
            }
            queue!(
                stdout,
                MoveDown(1),
                MoveLeft(p.project.camera.dim.y as u16),
            ).unwrap();
        }
        queue!(stdout, ResetColor).unwrap();
        stdout.flush();
    }
    pub fn show(&mut self) {
        use cursor::{ MoveTo, MoveLeft, MoveDown };
        use style::{ SetBackgroundColor, SetForegroundColor, Color, Print, ResetColor };
        use std::io::Write;

        let PixyleneTUI { pixylene: p, camera_corner: corner, .. } = self;
        let p: &mut Pixylene = p.as_mut().unwrap();
        let mut stdout = std::io::stdout();

        queue!(
            stdout,
            ResetColor,
            MoveTo(corner.y as u16, corner.x as u16),
        ).unwrap();

        let grid: Vec<ProjectPixel> = p.project.render_layer().unwrap_or(vec![
            ProjectPixel{ camera_pixel: CameraPixel::OutOfScene, has_cursor: false };
            p.project.camera.dim.area().try_into().unwrap()
        ]);
        for i in 0..p.project.camera.dim.x {
            for j in 0..p.project.camera.dim.y {
                let project_pixel = &grid[i as usize*p.project.camera.dim.y as usize + j as usize];
                match project_pixel.camera_pixel {
                    CameraPixel::Filled{ color, .. } => {
                        if color.a != 255 {
                            panic!("alpha not 255, color: {} at ({},{})\n", color, i, j);
                        }
                        queue!(
                            stdout,
                            SetBackgroundColor(Color::Rgb{
                                r: color.r,
                                g: color.g,
                                b: color.b,
                            }),
                            SetForegroundColor(Color::Rgb{
                                r: 255 - color.r,
                                g: 255 - color.g,
                                b: 255 - color.b,
                            }),
                            Print(if project_pixel.has_cursor { "╳" } else { " " }),
                        ).unwrap();
                    },
                    CameraPixel::Empty{ .. } => {
                        queue!(
                            stdout,
                            ResetColor,
                            Print(if project_pixel.has_cursor { "╳" } else { " " }),
                        ).unwrap();
                    },
                    CameraPixel::OutOfScene => {
                        queue!(
                            stdout,
                            ResetColor,
                            Print(" "),
                        ).unwrap();
                    }
                }
            }
            queue!(
                stdout,
                MoveDown(1),
                MoveLeft(p.project.camera.dim.y as u16),
            ).unwrap();
        }
        queue!(stdout, ResetColor).unwrap();
        stdout.flush();
    }
    pub fn draw_info(&mut self) {
        use cursor::{ MoveTo, MoveToNextLine, MoveRight };
        use style::{ Print };

        execute!(
            std::io::stdout(),
            MoveTo(self.info_corner.y as u16, self.info_corner.x as u16),
            Print("Available Actions:"),
            MoveTo(self.info_corner.y as u16, self.info_corner.x as u16 + 2),
        ).unwrap();

        for action_name in &self.pixylene.as_mut().unwrap().action_manager.list_actions() {
            execute!(
                std::io::stdout(),
                Print(&action_name),
                MoveToNextLine(1),
                MoveRight(self.info_corner.y as u16),
            ).unwrap();
        }
    }
    pub fn getkey(&self) -> Option<event::KeyCode> {
        use event::{ Event, read };

        match read().unwrap() {
            Event::Key(key_event) => Some(key_event.code),
            _ => None,
        }
    }
    pub fn dispatch_actions(&mut self) {
        for (action_name, action) in self.actions.drain() {
            self.pixylene.as_mut().unwrap().add_action(&action_name, action);
        }
    }
    pub fn draw_statusline(&mut self, mode: &VimMode) {
        use terminal::{ size, Clear, ClearType };
        use cursor::{ MoveTo };
        use style::{ Print, SetForegroundColor, SetBackgroundColor, Color, ResetColor };
        use std::io::Write;
        use VimMode::*;

        let PixyleneTUI { pixylene: p, statusline_corner: corner, .. } = self;
        let p: &mut Pixylene = p.as_mut().unwrap();
        let padding = "       ";
        let mut stdout = std::io::stdout();
        queue!(
            stdout,
            MoveTo(0, corner.x.try_into().unwrap()),
            SetBackgroundColor(Color::Rgb{r:50,g:50,b:50,}),
        ).unwrap();
        for _ in 0..size().unwrap().0 {
            queue!(stdout, Print(" ")).unwrap();
        }
        queue!(
            stdout,
            MoveTo(corner.y.try_into().unwrap(), corner.x.try_into().unwrap()),
            Clear(ClearType::UntilNewLine),
            SetForegroundColor(Color::Rgb{r:255,g:255,b:255,}),
            Print(format!(
                "|{}|{}|layer {} of {}|{}|{}|{}|",
                match mode {
                    Splash => "Splash",
                    Command => "Command",
                    Normal => "Normal",
                    Preview => "Preview",
                    GridSelect => "GridSelect",
                    PointSelect => "PointSelect",
                },
                padding,
                p.project.focus.layer + 1,
                p.project.canvas.get_num_layers(),
                padding,
                p.project.focus.coord,
                padding,
            )),
            SetForegroundColor(Color::Rgb{r:30,g:30,b:30}),
        ).unwrap();

        for i in 0..p.project.canvas.palette.colors.len() {
            if let Some(color) = p.project.canvas.palette.get_color(i+1).unwrap() {
                queue!(
                    stdout,
                    SetBackgroundColor(Color::Rgb{r: color.r, g: color.g, b: color.b}),
                ).unwrap();
            }
            queue!(
                stdout,
                Print(format!(" {} ", i+1)),
            ).unwrap();
        }

        queue!(
            stdout,
            ResetColor,
            SetBackgroundColor(Color::Rgb{r:50,g:50,b:50,}),
            SetForegroundColor(Color::Rgb{r:255,g:255,b:255,}),
            Print(format!(
                "|{}(S:'{}' C:'{}'){}|{}|",
                padding,
                &p.action_manager.scene_lock.clone().unwrap_or(String::from("-")),
                &p.action_manager.camera_lock.clone().unwrap_or(String::from("-")),
                padding,
                match p.project.cursors.len() {
                    0 => String::from("No cursors"),
                    1 => format!("1 cursor: {}", p.project.cursors[0].coord),
                    _ => format!("{} cursors", p.project.cursors.len()).to_string(),
                },
            )),
        ).unwrap();
        stdout.flush();
    }
    pub fn perform_prev_action(&mut self) {
        if let Some(action_name) = &self.last_action_name {
            self.perform_action(&action_name.clone());
        }
    }
    pub fn perform_action(&mut self, action_name: &str) {
        match self.pixylene.as_mut().unwrap().perform(&action_name) {
            Ok(()) => {
                self.last_action_name = Some(String::from(action_name));
            },
            Err(desc) => self.cmdout(&desc.to_string(), LogType::Error),
        }
    }
    pub fn undo(&mut self) {
        match self.pixylene.as_mut().unwrap().undo() {
            Ok(()) => (),
            Err(desc) => self.cmdout(&desc.to_string(), LogType::Error),
        }
    }
    pub fn redo(&mut self) {
        match self.pixylene.as_mut().unwrap().redo() {
            Ok(()) => (),
            Err(desc) => self.cmdout(&desc.to_string(), LogType::Error),
        }
    }
    pub fn save(&mut self) {
        match &self.project_file_path {
            Some(path) => match self.pixylene.as_mut().unwrap().save(&path) {
                Ok(()) => {
                    let message = format!("project saved to {}", path);
                    self.cmdout(&message, LogType::Info);
                },
                Err(desc) => self.cmdout(&desc.to_string(), LogType::Error),
            },
            None => {
                if let Some(path) = self.cmdin("save project as: ") {
                    match self.pixylene.as_mut().unwrap().save(&path) {
                        Ok(()) => {
                            let message = format!("project saved to {}", path);
                            self.project_file_path = Some(path.clone());
                            self.cmdout(&message, LogType::Info);
                        },
                        Err(desc) => self.cmdout(&desc.to_string(), LogType::Error),
                    }
                }
            }
        }
    }
    pub fn export(&mut self) {
        if let Some(path) = self.cmdin("export project as: ") {
            match self.pixylene.as_mut().unwrap().export(&path) {
                Ok(()) => {
                    let message = format!("project exported to {}", path);
                    self.cmdout(&message, LogType::Info);
                },
                Err(desc) => self.cmdout(&desc.to_string(), LogType::Error),
            }
        }
    }
}
