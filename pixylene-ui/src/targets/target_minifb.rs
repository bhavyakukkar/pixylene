use pixylene_ui::{
    Cli, controller::Controller,
    ui::{ UserInterface, self, Rectangle, Statusline, Color, KeyInfo },
};

use libpixylene::{ types::{ PCoord }, project::OPixel };
use pixylene_actions::{ LogType };
use crossterm::event::{ KeyCode, KeyModifiers };
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
    fn key_to_crossterm(mut ksd: Vec<minifb::Key>, ksp: Vec<minifb::Key>) -> Option<ui::Key> {
        use minifb::Key::*;
        use ui::Key;
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
                return Some(Key::new(if !shift {
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
                    }, Some(m)));
            }

            if k >= A && k <= Z {
                return Some(Key::new(if !shift {
                        //small letters
                        KeyCode::Char( ((k as u8 - 10) + 97) as char)
                    } else {
                        //capital letters
                        KeyCode::Char( ((k as u8 - 10) + 65) as char )
                    }, Some(m)));
            }

            //Fn keys
            if k >= F1 && k <= F15 {
                return Some(if !shift {
                        Key::new(KeyCode::F(k as u8 - 35), Some(m))
                    } else {
                        Key::new(KeyCode::F(k as u8 - 35), Some(m.union(KeyModifiers::SHIFT)))
                    });
            }

            //Numpad
            if k >= NumPad0/*86*/ && k <= NumPad9/*95*/ {
                return Some(if !shift {
                        Key::new(KeyCode::Char(((k as u8 - 86) + 48) as char), Some(m))
                    } else {
                        Key::new(KeyCode::Char(((k as u8 - 86) + 48) as char),
                                 Some(m.union(KeyModifiers::SHIFT)))
                    });
            }

            return Some(match (k, shift) {
                (Down, false) => Key::new(KeyCode::Down, Some(m)),
                (Down, true) => Key::new(KeyCode::PageDown, Some(m)),
                (Left, false) => Key::new(KeyCode::Left, Some(m)),
                (Left, true) => Key::new(KeyCode::Home, Some(m)),
                (Right, false) => Key::new(KeyCode::Right, Some(m)),
                (Right, true) => Key::new(KeyCode::End, Some(m)),
                (Up, false) => Key::new(KeyCode::Up, Some(m)),
                (Up, true) => Key::new(KeyCode::PageUp, Some(m)),

                (Apostrophe, false) => Key::new(KeyCode::Char('\''), Some(m)),
                (Apostrophe, true) => Key::new(KeyCode::Char('"'), Some(m)),
                (Backquote, false) => Key::new(KeyCode::Char('`'), Some(m)),
                (Backquote, true) => Key::new(KeyCode::Char('~'), Some(m)),
                (Backslash, false) => Key::new(KeyCode::Char('\\'), Some(m)),
                (Backslash, true) => Key::new(KeyCode::Char('|'), Some(m)),
                (Comma, false) => Key::new(KeyCode::Char(','), Some(m)),
                (Comma, true) => Key::new(KeyCode::Char('<'), Some(m)),
                (Equal, false) => Key::new(KeyCode::Char('='), Some(m)),
                (Equal, true) => Key::new(KeyCode::Char('+'), Some(m)),
                (LeftBracket, false) => Key::new(KeyCode::Char('['), Some(m)),
                (LeftBracket, true) => Key::new(KeyCode::Char('{'), Some(m)),
                (Minus, false) => Key::new(KeyCode::Char('-'), Some(m)),
                (Minus, true) => Key::new(KeyCode::Char('_'), Some(m)),
                (Period, false) => Key::new(KeyCode::Char('.'), Some(m)),
                (Period, true) => Key::new(KeyCode::Char('>'), Some(m)),
                (RightBracket, false) => Key::new(KeyCode::Char(']'), Some(m)),
                (RightBracket, true) => Key::new(KeyCode::Char('}'), Some(m)),
                (Semicolon, false) => Key::new(KeyCode::Char(';'), Some(m)),
                (Semicolon, true) => Key::new(KeyCode::Char(':'), Some(m)),
                (Slash, false) => Key::new(KeyCode::Char('/'), Some(m)),
                (Slash, true) => Key::new(KeyCode::Char('?'), Some(m)),
                (Backspace, false) => Key::new(KeyCode::Backspace, Some(m)),
                (Backspace, true) => Key::new(KeyCode::Backspace, Some(m.union(KeyModifiers::SHIFT))),
                (Delete, false) => Key::new(KeyCode::Delete, Some(m)),
                (Delete, true) => Key::new(KeyCode::Delete, Some(m.union(KeyModifiers::SHIFT))),
                (End, false) => Key::new(KeyCode::End, Some(m)),
                (End, true) => Key::new(KeyCode::End, Some(m.union(KeyModifiers::SHIFT))),
                (Enter, false) => Key::new(KeyCode::Enter, Some(m)),
                (Enter, true) => Key::new(KeyCode::Enter, Some(m.union(KeyModifiers::SHIFT))),
                (Escape, false) => Key::new(KeyCode::Esc, Some(m)),
                (Escape, true) => Key::new(KeyCode::Esc, Some(m.union(KeyModifiers::SHIFT))),
                (Home, false) => Key::new(KeyCode::Home, Some(m)),
                (Home, true) => Key::new(KeyCode::Home, Some(m.union(KeyModifiers::SHIFT))),
                (Insert, false) => Key::new(KeyCode::Insert, Some(m)),
                (Insert, true) => Key::new(KeyCode::Insert, Some(m.union(KeyModifiers::SHIFT))),
                (Menu, false) => Key::new(KeyCode::Menu, Some(m)),
                (Menu, true) => Key::new(KeyCode::Menu, Some(m.union(KeyModifiers::SHIFT))),
                (PageDown, false) => Key::new(KeyCode::PageDown, Some(m)),
                (PageDown, true) => Key::new(KeyCode::PageDown, Some(m.union(KeyModifiers::SHIFT))),
                (PageUp, false) => Key::new(KeyCode::PageUp, Some(m)),
                (PageUp, true) => Key::new(KeyCode::PageUp, Some(m.union(KeyModifiers::SHIFT))),
                (Pause, false) => Key::new(KeyCode::Pause, Some(m)),
                (Pause, true) => Key::new(KeyCode::Pause, Some(m.union(KeyModifiers::SHIFT))),
                (Space, false) => Key::new(KeyCode::Char(' '), Some(m)),
                (Space, true) => Key::new(KeyCode::Char(' '), Some(m.union(KeyModifiers::SHIFT))),
                (Tab, false) => Key::new(KeyCode::Tab, Some(m)),
                (Tab, true) => Key::new(KeyCode::Tab, Some(m.union(KeyModifiers::SHIFT))),
                (NumLock, false) => Key::new(KeyCode::NumLock, Some(m)),
                (NumLock, true) => Key::new(KeyCode::NumLock, Some(m.union(KeyModifiers::SHIFT))),

                //todo: manage capitalization
                (CapsLock, false) => Key::new(KeyCode::CapsLock, Some(m)),
                (CapsLock, true) => Key::new(KeyCode::CapsLock, Some(m.union(KeyModifiers::SHIFT))),

                (ScrollLock, false) => Key::new(KeyCode::ScrollLock, Some(m)),
                (ScrollLock, true) => Key::new(KeyCode::ScrollLock, Some(m.union(KeyModifiers::SHIFT))),
                (LeftShift|RightShift, _) => Key::new(KeyCode::Null, Some(m.union(KeyModifiers::SHIFT))),
                (LeftCtrl|RightCtrl, false) => Key::new(KeyCode::Null, Some(m)),
                (LeftCtrl|RightCtrl, true) => Key::new(KeyCode::Null, Some(m.union(KeyModifiers::SHIFT))),
                (LeftAlt|RightAlt, false) => Key::new(KeyCode::Null, Some(m)),
                (LeftAlt|RightAlt, true) => Key::new(KeyCode::Null, Some(m.union(KeyModifiers::SHIFT))),
                (LeftSuper|RightSuper, false) => Key::new(KeyCode::Null,
                                                          Some(m.union(KeyModifiers::SUPER))),
                (LeftSuper|RightSuper, true) => Key::new(KeyCode::Null,
                                                         Some(m.union(KeyModifiers::SUPER)
                                                          .union(KeyModifiers::SHIFT))),
                (NumPadDot, false) => Key::new(KeyCode::Char('.'), Some(m)),
                (NumPadDot, true) => Key::new(KeyCode::Char('.'), Some(m.union(KeyModifiers::SHIFT))),
                (NumPadSlash, false) => Key::new(KeyCode::Char('/'), Some(m)),
                (NumPadSlash, true) => Key::new(KeyCode::Char('/'), Some(m.union(KeyModifiers::SHIFT))),
                (NumPadAsterisk, false) => Key::new(KeyCode::Char('*'), Some(m)),
                (NumPadAsterisk, true) => Key::new(KeyCode::Char('*'),
                                                   Some(m.union(KeyModifiers::SHIFT))),
                (NumPadMinus, false) => Key::new(KeyCode::Char('-'), Some(m)),
                (NumPadMinus, true) => Key::new(KeyCode::Char('-'), Some(m.union(KeyModifiers::SHIFT))),
                (NumPadPlus, false) => Key::new(KeyCode::Char('+'), Some(m)),
                (NumPadPlus, true) => Key::new(KeyCode::Char('+'), Some(m.union(KeyModifiers::SHIFT))),
                (NumPadEnter, false) => Key::new(KeyCode::Enter, Some(m)),
                (NumPadEnter, true) => Key::new(KeyCode::Enter, Some(m.union(KeyModifiers::SHIFT))),
                (Unknown, false) => Key::new(KeyCode::Null, Some(m)),
                (Unknown, true) => Key::new(KeyCode::Null, Some(m.union(KeyModifiers::SHIFT))),
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
            text.color = Color(colored_string.fgcolor()).into();
            text.draw_text(
                framebuffer,
                usize::from(PIXELFACTOR*boundary.start.y) + chars_drawn,
                (PIXELFACTOR*boundary.start.x).into(),
                &colored_string,
            );
            chars_drawn += usize::from(FONT_WIDTH)*colored_string.len();
        }
    }

    fn draw_paragraph(&mut self, paragraph: Vec<colored::ColoredString>, _boundary: &Rectangle) {
        todo!()
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
                if key == *discard_key {
                    return None;
                }
                match key.code() {
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
