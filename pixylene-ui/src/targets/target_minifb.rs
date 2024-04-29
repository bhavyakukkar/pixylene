use pixylene_ui::{
    Cli, controller::Controller,
    ui::{ UserInterface, self, Rectangle, Statusline, Color, KeyInfo },
};

use libpixylene::{ types::{ PCoord }, project::OPixel };
use pixylene_actions::{ LogType };
use crossterm::event::{ KeyEvent, KeyCode, KeyModifiers };
use minifb::{ Window, WindowOptions, KeyRepeat, Scale };
use minifb_fonts::{ font5x8 };
use std::rc::Rc;
use std::cell::RefCell;
use clap::Parser;


const NOWIN: &str = "No Minifb Window found in Target, something is wrong.";
const WIDTH: u16 = 720;
const HEIGHT: u16 = 360;
const PIXELFACTOR: u16 = 8;

const FONT_WIDTH: u8 = 5;
const FONT_HEIGHT: u8 = 8;

pub struct TargetMinifb(/*window*/Option<Window>, /*buffer*/Vec<u32>);

impl TargetMinifb {
    pub fn new() -> Self {
        TargetMinifb(None, Vec::new())
    }

    /// Converts vector returned by [`get_keys_pressed`](minifb::Window::get_keys_pressed) into a
    /// single [`KeyEvent`](crossterm::event::KeyEvent)
    ///
    /// This method only resolves a single key along with Ctrl/Shift/Alt modifiers
    fn key_to_crossterm(mut ksd: Vec<minifb::Key>, ksp: Vec<minifb::Key>) -> Option<KeyEvent> {
        use minifb::Key::*;
        let mut m = KeyModifiers::empty();
        let mut shift: bool = false;

        //Shift -> only changes symbol of remaining keys, isn't added to modifiers
        if let Some(i) = ksd.iter().position(|&k| k == LeftShift) {
            //m.insert(KeyModifiers::SHIFT);
            shift = true;
            ksd.remove(i);
        }
        if let Some(i) = ksd.iter().position(|&k| k == RightShift) {
            //m.insert(KeyModifiers::SHIFT);
            shift = true;
            ksd.remove(i);
        }

        //Ctrl
        if let Some(i) = ksd.iter().position(|&k| k == LeftCtrl) {
            m.insert(KeyModifiers::CONTROL);
            ksd.remove(i);
        }
        if let Some(i) = ksd.iter().position(|&k| k == RightCtrl) {
            m.insert(KeyModifiers::CONTROL);
            ksd.remove(i);
        }

        //Alt
        if let Some(i) = ksd.iter().position(|&k| k == LeftAlt) {
            m.insert(KeyModifiers::ALT);
            ksd.remove(i);
        }
        if let Some(i) = ksd.iter().position(|&k| k == RightAlt) {
            m.insert(KeyModifiers::ALT);
            ksd.remove(i);
        }

        // handle remaining keys until single convertible key passed
        for k in ksp {
            if k >= Key0 && k <= Key9 {
                return Some(KeyEvent::new(if !shift {
                        //numbers
                        KeyCode::Char( (k as u8 + 48) as char )
                    } else {
                        //symbols above numbers
                        match k {
                            Key0 => KeyCode::Char(')'),
                            Key1 => KeyCode::Char('!'),
                            Key2 => KeyCode::Char('@'),
                            Key3 => KeyCode::Char('#'),
                            Key4 => KeyCode::Char('$'),
                            Key5 => KeyCode::Char('%'),
                            Key6 => KeyCode::Char('^'),
                            Key7 => KeyCode::Char('&'),
                            Key8 => KeyCode::Char('*'),
                            Key9 => KeyCode::Char('('),
                            _ => panic!(), //wont reach here
                        }
                    }, m));
            }

            if k >= A && k <= Z {
                return Some(KeyEvent::new(if !shift {
                        //small letters
                        KeyCode::Char( ((k as u8 - 10) + 97) as char)
                    } else {
                        //capital letters
                        KeyCode::Char( ((k as u8 - 10) + 65) as char )
                    }, m));
            }

            //Fn keys
            if k >= F1 && k <= F15 {
                return Some(if !shift {
                        KeyEvent::new(KeyCode::F(k as u8 - 35), m)
                    } else {
                        KeyEvent::new(KeyCode::F(k as u8 - 35), m.union(KeyModifiers::SHIFT))
                    });
            }

            //Numpad
            if k >= NumPad0/*86*/ && k <= NumPad9/*95*/ {
                return Some(if !shift {
                        KeyEvent::new(KeyCode::Char(((k as u8 - 86) + 48) as char), m)
                    } else {
                        KeyEvent::new(KeyCode::Char(((k as u8 - 86) + 48) as char),
                                 m.union(KeyModifiers::SHIFT))
                    });
            }

            return Some(match (k, shift) {
                (Down, false) => KeyEvent::new(KeyCode::Down, m),
                (Down, true) => KeyEvent::new(KeyCode::PageDown, m),
                (Left, false) => KeyEvent::new(KeyCode::Left, m),
                (Left, true) => KeyEvent::new(KeyCode::Home, m),
                (Right, false) => KeyEvent::new(KeyCode::Right, m),
                (Right, true) => KeyEvent::new(KeyCode::End, m),
                (Up, false) => KeyEvent::new(KeyCode::Up, m),
                (Up, true) => KeyEvent::new(KeyCode::PageUp, m),

                (Apostrophe, false) => KeyEvent::new(KeyCode::Char('\''), m),
                (Apostrophe, true) => KeyEvent::new(KeyCode::Char('"'), m),
                (Backquote, false) => KeyEvent::new(KeyCode::Char('`'), m),
                (Backquote, true) => KeyEvent::new(KeyCode::Char('~'), m),
                (Backslash, false) => KeyEvent::new(KeyCode::Char('\\'), m),
                (Backslash, true) => KeyEvent::new(KeyCode::Char('|'), m),
                (Comma, false) => KeyEvent::new(KeyCode::Char(','), m),
                (Comma, true) => KeyEvent::new(KeyCode::Char('<'), m),
                (Equal, false) => KeyEvent::new(KeyCode::Char('='), m),
                (Equal, true) => KeyEvent::new(KeyCode::Char('+'), m),
                (LeftBracket, false) => KeyEvent::new(KeyCode::Char('['), m),
                (LeftBracket, true) => KeyEvent::new(KeyCode::Char('{'), m),
                (Minus, false) => KeyEvent::new(KeyCode::Char('-'), m),
                (Minus, true) => KeyEvent::new(KeyCode::Char('_'), m),
                (Period, false) => KeyEvent::new(KeyCode::Char('.'), m),
                (Period, true) => KeyEvent::new(KeyCode::Char('>'), m),
                (RightBracket, false) => KeyEvent::new(KeyCode::Char(']'), m),
                (RightBracket, true) => KeyEvent::new(KeyCode::Char('}'), m),
                (Semicolon, false) => KeyEvent::new(KeyCode::Char(';'), m),
                (Semicolon, true) => KeyEvent::new(KeyCode::Char(':'), m),
                (Slash, false) => KeyEvent::new(KeyCode::Char('/'), m),
                (Slash, true) => KeyEvent::new(KeyCode::Char('?'), m),
                (Backspace, false) => KeyEvent::new(KeyCode::Backspace, m),
                (Backspace, true) => KeyEvent::new(KeyCode::Backspace, m.union(KeyModifiers::SHIFT)),
                (Delete, false) => KeyEvent::new(KeyCode::Delete, m),
                (Delete, true) => KeyEvent::new(KeyCode::Delete, m.union(KeyModifiers::SHIFT)),
                (End, false) => KeyEvent::new(KeyCode::End, m),
                (End, true) => KeyEvent::new(KeyCode::End, m.union(KeyModifiers::SHIFT)),
                (Enter, false) => KeyEvent::new(KeyCode::Enter, m),
                (Enter, true) => KeyEvent::new(KeyCode::Enter, m.union(KeyModifiers::SHIFT)),
                (Escape, false) => KeyEvent::new(KeyCode::Esc, m),
                (Escape, true) => KeyEvent::new(KeyCode::Esc, m.union(KeyModifiers::SHIFT)),
                (Home, false) => KeyEvent::new(KeyCode::Home, m),
                (Home, true) => KeyEvent::new(KeyCode::Home, m.union(KeyModifiers::SHIFT)),
                (Insert, false) => KeyEvent::new(KeyCode::Insert, m),
                (Insert, true) => KeyEvent::new(KeyCode::Insert, m.union(KeyModifiers::SHIFT)),
                (Menu, false) => KeyEvent::new(KeyCode::Menu, m),
                (Menu, true) => KeyEvent::new(KeyCode::Menu, m.union(KeyModifiers::SHIFT)),
                (PageDown, false) => KeyEvent::new(KeyCode::PageDown, m),
                (PageDown, true) => KeyEvent::new(KeyCode::PageDown, m.union(KeyModifiers::SHIFT)),
                (PageUp, false) => KeyEvent::new(KeyCode::PageUp, m),
                (PageUp, true) => KeyEvent::new(KeyCode::PageUp, m.union(KeyModifiers::SHIFT)),
                (Pause, false) => KeyEvent::new(KeyCode::Pause, m),
                (Pause, true) => KeyEvent::new(KeyCode::Pause, m.union(KeyModifiers::SHIFT)),
                (Space, false) => KeyEvent::new(KeyCode::Char(' '), m),
                (Space, true) => KeyEvent::new(KeyCode::Char(' '), m.union(KeyModifiers::SHIFT)),
                (Tab, false) => KeyEvent::new(KeyCode::Tab, m),
                (Tab, true) => KeyEvent::new(KeyCode::Tab, m.union(KeyModifiers::SHIFT)),
                (NumLock, false) => KeyEvent::new(KeyCode::NumLock, m),
                (NumLock, true) => KeyEvent::new(KeyCode::NumLock, m.union(KeyModifiers::SHIFT)),

                //todo: manage capitalization
                (CapsLock, false) => KeyEvent::new(KeyCode::CapsLock, m),
                (CapsLock, true) => KeyEvent::new(KeyCode::CapsLock, m.union(KeyModifiers::SHIFT)),

                (ScrollLock, false) => KeyEvent::new(KeyCode::ScrollLock, m),
                (ScrollLock, true) => KeyEvent::new(KeyCode::ScrollLock, m.union(KeyModifiers::SHIFT)),

                (LeftShift|RightShift, _) => { return None; },
                (LeftCtrl|RightCtrl, false) => { return None; },
                (LeftCtrl|RightCtrl, true) => { return None; },
                (LeftAlt|RightAlt, false) => { return None; },
                (LeftAlt|RightAlt, true) => { return None; },
                (LeftSuper|RightSuper, false) => { return None; },
                (LeftSuper|RightSuper, true) => { return None; },

                (NumPadDot, false) => KeyEvent::new(KeyCode::Char('.'), m),
                (NumPadDot, true) => KeyEvent::new(KeyCode::Char('.'), m.union(KeyModifiers::SHIFT)),
                (NumPadSlash, false) => KeyEvent::new(KeyCode::Char('/'), m),
                (NumPadSlash, true) => KeyEvent::new(KeyCode::Char('/'), m.union(KeyModifiers::SHIFT)),
                (NumPadAsterisk, false) => KeyEvent::new(KeyCode::Char('*'), m),
                (NumPadAsterisk, true) => KeyEvent::new(KeyCode::Char('*'),
                                                   m.union(KeyModifiers::SHIFT)),
                (NumPadMinus, false) => KeyEvent::new(KeyCode::Char('-'), m),
                (NumPadMinus, true) => KeyEvent::new(KeyCode::Char('-'), m.union(KeyModifiers::SHIFT)),
                (NumPadPlus, false) => KeyEvent::new(KeyCode::Char('+'), m),
                (NumPadPlus, true) => KeyEvent::new(KeyCode::Char('+'), m.union(KeyModifiers::SHIFT)),
                (NumPadEnter, false) => KeyEvent::new(KeyCode::Enter, m),
                (NumPadEnter, true) => KeyEvent::new(KeyCode::Enter, m.union(KeyModifiers::SHIFT)),
                (Unknown, false) => KeyEvent::new(KeyCode::Null, m),
                (Unknown, true) => KeyEvent::new(KeyCode::Null, m.union(KeyModifiers::SHIFT)),
                otherwise => panic!("{:?}", otherwise) //Everything else has been accounted for
                                                       //already
            });
        }
        None
    }
}

impl UserInterface for TargetMinifb {
    fn initialize(&mut self) {
        let mut window = Window::new(
            "Pixylene",
            usize::from(WIDTH),
            usize::from(HEIGHT),
            WindowOptions {
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        ).unwrap();
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // 60 FPS
        let buffer = vec![0; usize::from(WIDTH)*usize::from(HEIGHT)];

        window
            .update_with_buffer(&buffer, WIDTH.into(), HEIGHT.into())
            .unwrap();

        self.0 = Some(window);
        self.1 = buffer;
    }

    fn finalize(&mut self) { }

    fn refresh(&mut self) -> bool {
        let window = self.0.as_mut().expect(NOWIN);
        let ref framebuffer = self.1;
        window
            .update_with_buffer(&framebuffer, WIDTH.into(), HEIGHT.into())
            .unwrap();
        window.is_open()
    }

    fn get_key(&self) -> Option<KeyInfo> {
        let window = self.0.as_ref().expect(NOWIN);
        let key = Self::key_to_crossterm(window.get_keys(), window.get_keys_pressed(KeyRepeat::Yes));
        match key {
            Some(key) => Some(KeyInfo::Key(key)),
            None => None,
        }
    }

    fn get_size(&self) -> PCoord {
        PCoord::new(
            HEIGHT.checked_div(PIXELFACTOR).unwrap(),
            WIDTH.checked_div(PIXELFACTOR).unwrap()
        ).unwrap()
    }

    fn draw_camera(&mut self, dim: PCoord, buffer: Vec<OPixel>, show_cursors: bool,
                   boundary: &Rectangle) {
        let ref mut framebuffer = self.1;

        for i in 0..dim.x() {
            for j in 0..dim.y() {
                let scene_index = usize::from(i)*usize::from(dim.y()) +
                                  usize::from(j);
                for s in 0..PIXELFACTOR {
                    for t in 0..PIXELFACTOR {
                        let out_index =
                            //((boundary.start.x + i) as usize*PIXELFACTOR as usize*PIXELFACTOR as usize*dim.y() as usize) +
                            //(s as usize*PIXELFACTOR as usize*dim.y() as usize) +
                            //((boundary.start.y + j) as usize*PIXELFACTOR as usize) +
                            //(t as usize);
                            ((boundary.start.x + i) as usize*PIXELFACTOR as usize*WIDTH as usize) +
                            (s as usize*WIDTH as usize) +
                            ((boundary.start.y + j) as usize*PIXELFACTOR as usize) +
                            (t as usize);

                        match buffer.get(scene_index).unwrap() {
                            OPixel::Filled{ color, has_cursor, .. } => {
                                framebuffer[out_index] = ((color.r as u32) << 16) |
                                                         ((color.g as u32) << 8) |
                                                         ((color.b as u32));
                            },
                            OPixel::Empty{ has_cursor, .. } => {
                                framebuffer[out_index] = 0u32;
                            },
                            OPixel::OutOfScene => {
                                framebuffer[out_index] = 0u32;
                            },
                        }
                    }
                }
            }
        }
    }

    fn draw_statusline(&mut self, statusline: &Statusline, boundary: &Rectangle) {
        let size = self.get_size();
        let ref mut framebuffer = self.1;
        let mut chars_drawn = 0;
        let mut text = font5x8::new_renderer(
            (PIXELFACTOR*size.y()).into(),
            (PIXELFACTOR*size.x()).into(),
            0xFFFFFFFF
        );
        for colored_string in statusline.iter() {
            let string = colored_string.replace("｜", "");
            text.color = Color(colored_string.fgcolor()).into();
            text.draw_text(
                framebuffer,
                usize::from(PIXELFACTOR*boundary.start.y) + chars_drawn,
                (PIXELFACTOR*boundary.start.x).into(),
                &string,
            );
            chars_drawn += usize::from(FONT_WIDTH)*string.len();
        }
    }

    fn draw_paragraph(&mut self, paragraph: Vec<colored::ColoredString>, _boundary: &Rectangle) {
        //todo!()
    }

    fn clear(&mut self, boundary: &Rectangle) { 
        /*
        let size = self.get_size();
        let ref mut framebuffer = self.1;
        let mut text = font5x8::new_renderer(
            (PIXELFACTOR*size.y()).into(),
            (PIXELFACTOR*size.x()).into(),
            0xFFFFFFFF
        );
        let out: Option<String>;
        */
        let ref mut framebuffer = self.1;

        for i in 0..boundary.size.x() {
            for j in 0..boundary.size.y() {
                for s in 0..PIXELFACTOR {
                    for t in 0..PIXELFACTOR {
                        let index =
                            ((boundary.start.x + i) as usize*PIXELFACTOR as usize*WIDTH as usize) +
                            (s as usize*WIDTH as usize) +
                            ((boundary.start.y + j) as usize*PIXELFACTOR as usize) +
                            (t as usize);
                        framebuffer[index] = 0;
                    }
                }
            }
        }
    }

    fn console_in(&mut self, message: &str, discard_key: &ui::Key,
                  boundary: &Rectangle) -> Option<String> {
        let size = self.get_size();
        let ref mut framebuffer = self.1;
        let text = font5x8::new_renderer(
            (PIXELFACTOR*size.y()).into(),
            (PIXELFACTOR*size.x()).into(),
            0xFFFFFFFF
        );

        text.draw_text(
            framebuffer,
            (PIXELFACTOR*boundary.start.y).into(),
            (PIXELFACTOR*boundary.start.x).into(),
            message
        );
        let mut input = String::new();
        loop {
            self.refresh();
            let mkey = Self::key_to_crossterm(self.0.as_ref().expect(NOWIN).get_keys(),
                                              self.0.as_ref().expect(NOWIN)
                                              .get_keys_pressed(KeyRepeat::Yes));
            if let Some(key) = mkey {
                if ui::Key::from(key) == *discard_key {
                    return None;
                }
                match key.code {
                    KeyCode::Enter => {
                        return Some(input);
                    },
                    KeyCode::Backspace => {
                        if input.len() > 0 {
                            input.pop();
                        }
                    },
                    KeyCode::Char(c) => {
                        input.push(c);
                    },
                    _ => {},
                }
                self.clear(boundary);
                self.console_out(&(":".to_owned() + &input), &LogType::Info, boundary);
            }
        }
    }

    fn console_out(&mut self, message: &str, log_type: &LogType, boundary: &Rectangle) {
        let size = self.get_size();
        let ref mut framebuffer = self.1;
        let text = font5x8::new_renderer(
            (PIXELFACTOR*size.y()).into(),
            (PIXELFACTOR*size.x()).into(),
            0xFFFFFFFF
        );
        text.draw_text(
            framebuffer,
            (PIXELFACTOR*boundary.start.y).into(),
            (PIXELFACTOR*boundary.start.x).into(),
            message
        );
    }

    fn clear_all(&mut self) {
        self.1 = vec![0; self.1.len()];
    }
}


fn main() {
    let target = TargetMinifb::new();

    match Controller::new(Rc::new(RefCell::new(target))) {
        Ok(mut pixylene_ui) => {
            let cli = Cli::parse();

            pixylene_ui.start(&cli.command);
        },
        Err(error) => eprintln!("{}", error)
    }
}
