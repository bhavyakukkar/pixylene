use libpixylene::{
    Pixylene,
    common::Coord, 
    elements::layer::CameraPixel,
    project::ProjectPixel,
};
use crate::{ utils::LogType, modes::Mode };

use crossterm::{
    execute,
    queue,
    terminal,
    style,
    cursor,
    event
};

pub struct PixyleneTUI {
    pub camera_corner: Coord,
    pub console_corner: Coord,
    pub statusline_corner: Coord,
    pub info_corner: Coord,
    pub pixylene: Option<Pixylene>,
    pub console: Console,
    pub last_action_name: Option<String>,
    pub project_file_path: Option<String>,
    //pub state: State,
}
impl PixyleneTUI {
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
                            panic!("alpha not 255");
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

        //let action_names = &self.pixylene.as_mut().unwrap().action_manager.actions.collect();
        for (action_name, _) in &self.pixylene.as_mut().unwrap().action_manager.actions {
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
    pub fn cmdin(&mut self, message: &str) -> String {
        use terminal::{ disable_raw_mode, enable_raw_mode };

        disable_raw_mode().unwrap();
        let input = self.console.ask(message.to_string(), self.console_corner);
        enable_raw_mode().unwrap();
        input
    }
    pub fn cmdout(&mut self, message: &str, log_type: LogType) {
        self.console.log(message.to_string(), log_type, self.console_corner.add(Coord{ x: 0, y: 0 }));
    }
    pub fn draw_statusline(&mut self, mode: &Mode) {
        use terminal::{ size, Clear, ClearType };
        use cursor::{ MoveTo };
        use style::{ Print, SetForegroundColor, SetBackgroundColor, Color, ResetColor };
        use std::io::Write;
        use Mode::*;

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
                p.project.layers.len(),
                padding,
                p.project.focus.coord,
                padding,
            )),
            SetForegroundColor(Color::Rgb{r:30,g:30,b:30}),
        ).unwrap();

        for i in 0..p.project.palette.colors.len() {
            if let Some(color) = p.project.palette.get_color(i+1).unwrap() {
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
                let path = self.cmdin("save project as: ");
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
    pub fn export(&mut self) {
        let path = self.cmdin("export project as: ");
        match self.pixylene.as_mut().unwrap().export(&path) {
            Ok(()) => {
                let message = format!("project exported to {}", path);
                self.cmdout(&message, LogType::Info);
            },
            Err(desc) => self.cmdout(&desc.to_string(), LogType::Error),
        }
    }
}

pub struct Console;
impl Console {
    pub fn ask(&self, message: String, corner: Coord) -> String {
        use terminal::{ Clear, ClearType };
        use cursor::{ MoveTo, MoveRight };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
        use event::{ Event, KeyEvent, KeyCode, read };
        execute!(
            std::io::stdout(),
            ResetColor,
            MoveTo(corner.y as u16, corner.x as u16),
            Clear(ClearType::UntilNewLine),
            SetForegroundColor(Color::Rgb{ r: 220, g: 220, b: 220 }),
            Print(&message),
            ResetColor,
            MoveRight(1),
        ).unwrap();

        let mut input = String::new();
        while let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
            match code {
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char(c) => {
                    input.push(c);
                }
                _ => {}
            }
        }
        input
    }
    pub fn log(&self, message: String, log_type: LogType, corner: Coord) {
        use terminal::{ Clear, ClearType };
        use cursor::{ MoveTo };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
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