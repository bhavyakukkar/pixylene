use pixylene_ui::{
    Cli, config::Config, controller::Controller,
    ui::{ UserInterface, Key, KeyInfo, Rectangle, Statusline, Color },
};

use libpixylene::{ types::{ PCoord }, project::{ OPixel } };
use pixylene_actions::{ LogType };
use crossterm::{ event, cursor, terminal, style, queue, execute };
use std::io::{ Write };
use std::rc::Rc;
use std::cell::RefCell;
use clap::Parser;


/// Pixylene UI's Target for the [`crossterm`](crossterm) terminal manipulation library
/// [Crossterm repository](https://github.com/crossterm-rs/crossterm)
struct TargetCrossterm;

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
        stdout.flush().unwrap();
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
        stdout.flush().unwrap();
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
        stdout.flush().unwrap();
    }

    fn get_key(&self) -> Option<KeyInfo> {
        use event::{ Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind, read };

        loop {
            //blocking read
            match read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        if let KeyCode::Char(c) = key_event.code {
                            return Some(KeyInfo::Key(KeyEvent::new(
                                KeyCode::Char(c),
                                key_event.modifiers.difference(KeyModifiers::SHIFT)
                            )));
                        } else {
                            return Some(KeyInfo::Key(key_event.into()));
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
                SetBackgroundColor(Color(colored_string.bgcolor()).into()),
                SetForegroundColor(Color(colored_string.fgcolor()).into()),
                Print(colored_string),
                SetAttribute(Attribute::Reset),
                ResetColor,
            ).unwrap();
        }
        stdout.flush().unwrap();
    }

    fn draw_paragraph(&mut self, paragraph: Vec<colored::ColoredString>, _boundary: &Rectangle) {
        //todo: use boundary instead of full-screen
        use cursor::{ MoveTo, MoveToNextLine };
        use style::{ Print, SetForegroundColor, SetBackgroundColor, ResetColor,
                     SetAttribute, Attribute };
        let mut stdout = std::io::stdout();

        queue!(
            stdout,
            MoveTo(1, 1),
        ).unwrap();
        for line in paragraph.iter() {
            queue!(
                stdout,
                SetBackgroundColor(Color(line.bgcolor()).into()),
                SetForegroundColor(Color(line.fgcolor()).into()),
            ).unwrap();
            for char in line.to_string().chars() {
                if char == '\n' {
                    queue!(stdout, MoveToNextLine(1)).unwrap();
                } else {
                    queue!(stdout, Print(char)).unwrap();
                }
            }
            queue!(
                stdout,
                SetAttribute(Attribute::Reset),
                ResetColor,
                MoveToNextLine(1),
            ).unwrap();
        }
        stdout.flush().unwrap();
    }

    fn console_in(&mut self, message: &str, discard_key: &Key, boundary: &Rectangle)
        -> Option<String>
    {
        use terminal::{ Clear, ClearType };
        use cursor::{ MoveTo, MoveLeft, Show, Hide };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
        use event::{ Event, KeyEvent, KeyCode, KeyEventKind, read };
        let mut stdout = std::io::stdout();

        let out: Option<String>;

        queue!(
            stdout,
            ResetColor,
            MoveTo(boundary.start.y as u16, boundary.start.x as u16),
            //Clear(ClearType::UntilNewLine),
            SetForegroundColor(Color::Rgb{ r: 220, g: 220, b: 220 }),
            Print(&message),
            ResetColor,
            Show,
        ).unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        loop {
            let event = read().unwrap();
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    if Key::from(key.clone()) == *discard_key {
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
        }
        execute!(stdout, Hide).unwrap();
        out
    }

    fn console_out(&mut self, message: &str, log_type: &LogType, boundary: &Rectangle) {
        use cursor::{ MoveTo };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
        let mut stdout = std::io::stdout();

        queue!(
            stdout,
            ResetColor,
            MoveTo(boundary.start.y as u16, boundary.start.x as u16),
            //Clear(ClearType::UntilNewLine),
            SetForegroundColor(match log_type {
                LogType::Info => Color::White,
                LogType::Error => Color::Red,
                LogType::Warning => Color::Yellow,
                LogType::Success => Color::Green,
            }),
            Print(&message[0..std::cmp::min(message.len(), usize::from(boundary.size.y()))]),
            ResetColor,
        ).unwrap();
        stdout.flush().unwrap();
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

        stdout.flush().unwrap();
    }

    fn clear_all(&mut self) {
        use terminal::{ Clear, ClearType };
        queue!(
            std::io::stdout(),
            Clear(ClearType::All),
        ).unwrap();
    }
}


fn main() -> Result<(), ()> {
    let cli = Cli::parse();
    let target = TargetCrossterm;
    let config = Config::from_config_toml()
        .map_err(|err| eprintln!("{}", err))?;

    let mut pixylene_tui = Controller::new(Rc::new(RefCell::new(target)), config);
    if let Some(command) = cli.command {
        pixylene_tui.new_session(&command, true);
    }
    pixylene_tui.run();
    Ok(())
}
