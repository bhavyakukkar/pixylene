use crate::ui::{ UserInterface, Key, Rectangle, Statusline };

use libpixylene::{ types::{ PCoord }, project::{ OPixel } };
use pixylene_actions::{ LogType };
use crossterm::{ event, cursor, terminal, style, queue, execute };
use std::io::{ Write };


/// Pixylene UI's Target for the [`crossterm`](crossterm) terminal manipulation library
/// [Crossterm repository](https://github.com/crossterm-rs/crossterm)
pub struct TargetCrossterm;

impl UserInterface for TargetCrossterm {

    fn initialize(&mut self) {
        use terminal::{
            enable_raw_mode,
            EnterAlternateScreen,
        };
        use cursor::{ Hide };
        let mut stdout = std::io::stdout();

        enable_raw_mode().unwrap();
        queue!(
            stdout,
            EnterAlternateScreen,
            Hide,
        ).unwrap();
        _ = stdout.flush();
    }

    fn finalize(&mut self) {
        use cursor::{ Show };
        use terminal::{ disable_raw_mode, LeaveAlternateScreen };
        let mut stdout = std::io::stdout();

        disable_raw_mode().unwrap();
        queue!(
            stdout,
            Show,
            LeaveAlternateScreen,
        ).unwrap();
        _ = stdout.flush();
    }

    // Crossterm blocks until read and requires no extra work between frames
    fn refresh(&mut self) -> bool { true }

    //fn set_camera_boundary(&mut self, boundary: Rectangle) {
    //    self.b_camera = boundary;
    //}
    //fn set_statusline_boundary(&mut self, boundary: Rectangle) {
    //    self.b_statusline = boundary;
    //}
    //fn set_console_boundary(&mut self, boundary: Rectangle) {
    //    self.b_console = boundary;
    //}

    fn draw_camera(&mut self, dim: PCoord, buffer: Vec<OPixel>, show_cursors: bool,
                   boundary: &Rectangle)
    {
        use cursor::{ MoveTo, MoveLeft, MoveDown };
        use style::{ SetBackgroundColor, SetForegroundColor, Color, Print, ResetColor };
        let mut stdout = std::io::stdout();

        queue!(
            stdout,
            ResetColor,
            MoveTo(boundary.start.y, boundary.start.x),
        ).unwrap();

        for i in 0..dim.x() {
            for j in 0..dim.y() {
                let o_pixel = &buffer.get(usize::from(i)*usize::from(dim.y()) + usize::from(j))
                    .unwrap();
                match o_pixel {
                    OPixel::Filled{ color, has_cursor, .. } => {
                        //if color.a != 255 {
                        //    panic!("alpha not 255, color: {} at ({},{})\n", color, i, j);
                        //}
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
                            Print(if show_cursors && *has_cursor { "╳" } else { " " }),
                        ).unwrap();
                    },
                    OPixel::Empty{ has_cursor, .. } => {
                        queue!(
                            stdout,
                            ResetColor,
                            Print(if show_cursors && *has_cursor { "╳" } else { " " }),
                        ).unwrap();
                    },
                    OPixel::OutOfScene => {
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
                MoveLeft(dim.y() as u16),
            ).unwrap();
        }
        queue!(stdout, ResetColor).unwrap();
        _ = stdout.flush();
    }

    fn get_key(&self) -> Option<Key> {
        use event::{ Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind, read };

        loop {
            //blocking read
            match read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        if let KeyCode::Char(c) = key_event.code {
                            return Some(KeyEvent::new(KeyCode::Char(c),
                                        key_event.modifiers.difference(KeyModifiers::SHIFT)));
                        } else {
                            return Some(key_event);
                        }
                    }
                },
                _ => (),
            }
        }
    }

    fn get_size(&self) -> PCoord {
        let (width, height) = terminal::size().unwrap();
        PCoord::new(height, width).unwrap()
    }

    fn draw_statusline(&mut self, statusline: &Statusline, boundary: &Rectangle) {
        use cursor::{ MoveTo };
        use style::{ Print, SetForegroundColor, SetBackgroundColor, ResetColor,
                     SetAttribute, Attribute };

        let mut stdout = std::io::stdout();

        queue!(
            stdout,
            MoveTo(boundary.start.y.try_into().unwrap(), boundary.start.x.try_into().unwrap()),
        ).unwrap();
        for colored_string in statusline.iter() {
            queue!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetBackgroundColor(MyColor(colored_string.bgcolor()).into()),
                SetForegroundColor(MyColor(colored_string.fgcolor()).into()),
                Print(colored_string),
                SetAttribute(Attribute::Reset),
                ResetColor,
            ).unwrap();
        }
        _ = stdout.flush();
    }

    /*
    fn draw_statusline(&mut self, project: &Project, _action_manager: &ActionManager, mode: &Mode,
                       session: &u8, boundary: &Rectangle) {
        use terminal::{ size, Clear, ClearType };
        use cursor::{ MoveTo };
        use style::{ Print, SetForegroundColor, SetBackgroundColor, Color, ResetColor };
        let mut stdout = std::io::stdout();

        let padding = "     ";
        queue!(
            stdout,
            MoveTo(0, boundary.start.x.try_into().unwrap()),
            SetBackgroundColor(Color::Rgb{r:50,g:50,b:50,}),
        ).unwrap();
        for _ in 0..size().unwrap().0 {
            queue!(stdout, Print(" ")).unwrap();
        }
        queue!(
            stdout,
            MoveTo(
                boundary.start.y.try_into().unwrap(),
                boundary.start.x.try_into().unwrap()
            ),
            Clear(ClearType::UntilNewLine),
            SetForegroundColor(Color::Rgb{r:255,g:255,b:255,}),
            Print(format!(
                "|{}|{}|layer {} of {}|{}|Session {}|{}|{}|{}|",
                mode,
                //match mode {
                //    Splash => "Splash",
                //    Command => "Command",
                //    Normal => "Normal",
                //    Preview => "Preview",
                //    GridSelect => "GridSelect",
                //    PointSelect => "PointSelect",
                //},
                padding,
                project.focus.1 + 1,
                project.canvas.num_layers(),
                padding,
                session,
                padding,
                project.focus.0,
                padding,
            )),
            SetForegroundColor(Color::Rgb{r:30,g:30,b:30}),
        ).unwrap();

        /*
        for i in 0..project.canvas.palette.colors.len() {
            if let Some(color) = project.canvas.palette.get_color(i+1).unwrap() {
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
        */

        queue!(
            stdout,
            ResetColor,
            SetBackgroundColor(Color::Rgb{r:50,g:50,b:50}),
            SetForegroundColor(Color::Rgb{r:255,g:255,b:255}),
            Print(format!(
                //"|{}(S:'{}' C:'{}'){}|",
                "|{}{}|",
                padding,
                //&action_manager.scene_lock.clone().unwrap_or(String::from("-")),
                //&action_manager.camera_lock.clone().unwrap_or(String::from("-")),
                padding,
                //match pixylene.project.cursors.len() {
                //    0 => String::from("No cursors"),
                //    1 => format!("1 cursor: {}", pixylene.project.cursors[0].coord),
                //    _ => format!("{} cursors", pixylene.project.cursors.len()).to_string(),
                //},
            )),
        ).unwrap();
        _ = stdout.flush();
    }
*/

    fn console_in(&mut self, message: &str, discard_key: &Key, boundary: &Rectangle) -> Option<String> {
        use terminal::{ Clear, ClearType };
        use cursor::{ MoveTo, MoveLeft, Show, Hide };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
        use event::{ Event, KeyEvent, KeyCode, read };
        let mut stdout = std::io::stdout();

        let out: Option<String>;

        execute!(
            stdout,
            ResetColor,
            MoveTo(boundary.start.y as u16, boundary.start.x as u16),
            //Clear(ClearType::UntilNewLine),
            SetForegroundColor(Color::Rgb{ r: 220, g: 220, b: 220 }),
            Print(&message),
            ResetColor,
            Show,
        ).unwrap();

        let mut input = String::new();
        loop {
            let event = read().unwrap();
            if let Event::Key(key) = event {
                if key == *discard_key {
                    out = None;
                    break;
                }
                let KeyEvent { code, .. } = key;
                match code {
                    KeyCode::Enter => {
                        out = Some(input);
                        break;
                    },
                    KeyCode::Backspace => {
                        if input.len() > 0 {
                            execute!(stdout, MoveLeft(1), Clear(ClearType::UntilNewLine)).unwrap();
                            input.pop();
                        }
                    },
                    KeyCode::Char(c) => {
                        execute!(stdout, Print(c)).unwrap();
                        input.push(c);
                    },
                    _ => {},
                }
            }
        }
        execute!(stdout, Hide).unwrap();
        out
    }

    fn console_out(&mut self, message: &str, log_type: &LogType, boundary: &Rectangle) {
        use cursor::{ MoveTo };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
        let mut stdout = std::io::stdout();

        execute!(
            stdout,
            ResetColor,
            MoveTo(boundary.start.y as u16, boundary.start.x as u16),
            //Clear(ClearType::UntilNewLine),
            SetForegroundColor(match log_type {
                LogType::Info => Color::Rgb{ r: 240, g: 240, b: 240 },
                LogType::Error => Color::Rgb{ r: 255, g: 70, b: 70 },
                LogType::Warning => Color::Rgb{ r: 70, g: 235, b: 235 },
                LogType::Success => Color::Rgb{ r: 70, g: 255, b: 70 },
            }),
            Print(&message[0..std::cmp::min(message.len(), usize::from(boundary.size.y()))]),
            ResetColor,
        ).unwrap();
    }

    fn draw_paragraph(&mut self, _paragraph: Vec<String>) {
    }

    fn clear(&mut self, boundary: &Rectangle) {
        use cursor::{ MoveTo };
        use style::{ Print };
        let mut stdout = std::io::stdout();

        queue!(
            stdout,
            MoveTo(boundary.start.y as u16, boundary.start.x as u16),
            //todo: dont clear past console boundary
        ).unwrap();
        for i in 0..boundary.size.x() {
            for _ in 0..boundary.size.y() {
                queue!(
                    stdout,
                    Print(' '),
                ).unwrap();
            }
            if i < boundary.size.x() - 1 {
                queue!(stdout, MoveTo(boundary.start.y, boundary.start.x + i+1)).unwrap();
            }
        }

        _ = stdout.flush();
    }

    fn clear_all(&mut self) {
        use terminal::{ Clear, ClearType };
        queue!(
            std::io::stdout(),
            Clear(ClearType::All),
        ).unwrap();
    }
}

struct MyColor(Option<colored::Color>);
impl From<MyColor> for style::Color {
    fn from(item: MyColor) -> style::Color {
        use style::Color::*;

        match item.0 {
            Some(color) => match color {
                colored::Color::Black => Black,
                colored::Color::Red => Red,
                colored::Color::Green => Green,
                colored::Color::Yellow => Yellow,
                colored::Color::Blue => Blue,
                colored::Color::Magenta => Magenta,
                colored::Color::Cyan => Cyan,
                colored::Color::White => White,
                colored::Color::BrightBlack => DarkGrey,
                colored::Color::BrightRed => DarkRed,
                colored::Color::BrightGreen => DarkGreen,
                colored::Color::BrightYellow => DarkYellow,
                colored::Color::BrightBlue => DarkBlue,
                colored::Color::BrightMagenta => DarkMagenta,
                colored::Color::BrightCyan => DarkCyan,
                colored::Color::BrightWhite => Grey,
                colored::Color::TrueColor { r, g, b } => Rgb { r, g, b },
            },
            None => Reset,
        }
    }
}
