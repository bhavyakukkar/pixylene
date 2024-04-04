use crate::ui::{ UserInterface, self, Rectangle, Statusline };

use libpixylene::{ types::{ UCoord, PCoord }, project::{ Project, OPixel } };
use pixylene_actions::{ memento::ActionManager, LogType };
use crossterm::event::{ KeyCode, KeyModifiers };
use minifb::{ Window, WindowOptions, KeyRepeat, Scale };
use minifb_fonts::{ font5x8 };


const NOWIN: &str = "No Minifb Window found in Target, something is wrong.";
const WIDTH: u16 = 720;
const HEIGHT: u16 = 360;
const PIXELFACTOR: u16 = 8;

pub struct TargetMinifb(/*window*/Option<Window>, /*buffer*/Vec<u32>);

impl TargetMinifb {
    pub fn new() -> Self {
        TargetMinifb(None, Vec::new())
    }

    /// Converts vector returned by [`get_keys_pressed`](minifb::Window::get_keys_pressed) into a
    /// single [`KeyEvent`](crossterm::event::KeyEvent)
    ///
    /// This method only resolves a single key along with Ctrl/Shift/Alt modifiers
    fn key_to_crossterm(mut ksd: Vec<minifb::Key>, mut ksp: Vec<minifb::Key>) -> Option<ui::Key> {
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
                    }, m));
            }

            if k >= A && k <= Z {
                return Some(Key::new(if !shift {
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
                        Key::new(KeyCode::F(k as u8 - 35), m)
                    } else {
                        Key::new(KeyCode::F(k as u8 - 35), m.union(KeyModifiers::SHIFT))
                    });
            }

            //Numpad
            if k >= NumPad0/*86*/ && k <= NumPad9/*95*/ {
                return Some(if !shift {
                        Key::new(KeyCode::Char(((k as u8 - 86) + 48) as char), m)
                    } else {
                        Key::new(KeyCode::Char(((k as u8 - 86) + 48) as char),
                                 m.union(KeyModifiers::SHIFT))
                    });
            }

            return Some(match (k, shift) {
                (Down, false) => Key::new(KeyCode::Down, m),
                (Down, true) => Key::new(KeyCode::PageDown, m),
                (Left, false) => Key::new(KeyCode::Left, m),
                (Left, true) => Key::new(KeyCode::Home, m),
                (Right, false) => Key::new(KeyCode::Right, m),
                (Right, true) => Key::new(KeyCode::End, m),
                (Up, false) => Key::new(KeyCode::Up, m),
                (Up, true) => Key::new(KeyCode::PageUp, m),

                (Apostrophe, false) => Key::new(KeyCode::Char('\''), m),
                (Apostrophe, true) => Key::new(KeyCode::Char('"'), m),
                (Backquote, false) => Key::new(KeyCode::Char('`'), m),
                (Backquote, true) => Key::new(KeyCode::Char('~'), m),
                (Backslash, false) => Key::new(KeyCode::Char('\\'), m),
                (Backslash, true) => Key::new(KeyCode::Char('|'), m),
                (Comma, false) => Key::new(KeyCode::Char(','), m),
                (Comma, true) => Key::new(KeyCode::Char('<'), m),
                (Equal, false) => Key::new(KeyCode::Char('='), m),
                (Equal, true) => Key::new(KeyCode::Char('+'), m),
                (LeftBracket, false) => Key::new(KeyCode::Char('['), m),
                (LeftBracket, true) => Key::new(KeyCode::Char('{'), m),
                (Minus, false) => Key::new(KeyCode::Char('-'), m),
                (Minus, true) => Key::new(KeyCode::Char('_'), m),
                (Period, false) => Key::new(KeyCode::Char('.'), m),
                (Period, true) => Key::new(KeyCode::Char('>'), m),
                (RightBracket, false) => Key::new(KeyCode::Char(']'), m),
                (RightBracket, true) => Key::new(KeyCode::Char('}'), m),
                (Semicolon, false) => Key::new(KeyCode::Char(';'), m),
                (Semicolon, true) => Key::new(KeyCode::Char(':'), m),
                (Slash, false) => Key::new(KeyCode::Char('/'), m),
                (Slash, true) => Key::new(KeyCode::Char('?'), m),
                (Backspace, false) => Key::new(KeyCode::Backspace, m),
                (Backspace, true) => Key::new(KeyCode::Backspace, m.union(KeyModifiers::SHIFT)),
                (Delete, false) => Key::new(KeyCode::Delete, m),
                (Delete, true) => Key::new(KeyCode::Delete, m.union(KeyModifiers::SHIFT)),
                (End, false) => Key::new(KeyCode::End, m),
                (End, true) => Key::new(KeyCode::End, m.union(KeyModifiers::SHIFT)),
                (Enter, false) => Key::new(KeyCode::Enter, m),
                (Enter, true) => Key::new(KeyCode::Enter, m.union(KeyModifiers::SHIFT)),
                (Escape, false) => Key::new(KeyCode::Esc, m),
                (Escape, true) => Key::new(KeyCode::Esc, m.union(KeyModifiers::SHIFT)),
                (Home, false) => Key::new(KeyCode::Home, m),
                (Home, true) => Key::new(KeyCode::Home, m.union(KeyModifiers::SHIFT)),
                (Insert, false) => Key::new(KeyCode::Insert, m),
                (Insert, true) => Key::new(KeyCode::Insert, m.union(KeyModifiers::SHIFT)),
                (Menu, false) => Key::new(KeyCode::Menu, m),
                (Menu, true) => Key::new(KeyCode::Menu, m.union(KeyModifiers::SHIFT)),
                (Insert, false) => Key::new(KeyCode::Insert, m),
                (Insert, true) => Key::new(KeyCode::Insert, m.union(KeyModifiers::SHIFT)),
                (PageDown, false) => Key::new(KeyCode::PageDown, m),
                (PageDown, true) => Key::new(KeyCode::PageDown, m.union(KeyModifiers::SHIFT)),
                (PageUp, false) => Key::new(KeyCode::PageUp, m),
                (PageUp, true) => Key::new(KeyCode::PageUp, m.union(KeyModifiers::SHIFT)),
                (Pause, false) => Key::new(KeyCode::Pause, m),
                (Pause, true) => Key::new(KeyCode::Pause, m.union(KeyModifiers::SHIFT)),
                (Space, false) => Key::new(KeyCode::Char(' '), m),
                (Space, true) => Key::new(KeyCode::Char(' '), m.union(KeyModifiers::SHIFT)),
                (Tab, false) => Key::new(KeyCode::Tab, m),
                (Tab, true) => Key::new(KeyCode::Tab, m.union(KeyModifiers::SHIFT)),
                (NumLock, false) => Key::new(KeyCode::NumLock, m),
                (NumLock, true) => Key::new(KeyCode::NumLock, m.union(KeyModifiers::SHIFT)),

                //todo: manage capitalization
                (CapsLock, false) => Key::new(KeyCode::CapsLock, m),
                (CapsLock, true) => Key::new(KeyCode::CapsLock, m.union(KeyModifiers::SHIFT)),

                (ScrollLock, false) => Key::new(KeyCode::ScrollLock, m),
                (ScrollLock, true) => Key::new(KeyCode::ScrollLock, m.union(KeyModifiers::SHIFT)),
                (LeftShift|RightShift, _) => Key::new(KeyCode::Null, m.union(KeyModifiers::SHIFT)),
                (LeftCtrl|RightCtrl, false) => Key::new(KeyCode::Null, m),
                (LeftCtrl|RightCtrl, true) => Key::new(KeyCode::Null, m.union(KeyModifiers::SHIFT)),
                (LeftAlt|RightAlt, false) => Key::new(KeyCode::Null, m),
                (LeftAlt|RightAlt, true) => Key::new(KeyCode::Null, m.union(KeyModifiers::SHIFT)),
                (LeftSuper|RightSuper, false) => Key::new(KeyCode::Null,
                                                          m.union(KeyModifiers::SUPER)),
                (LeftSuper|RightSuper, true) => Key::new(KeyCode::Null,
                                                         m.union(KeyModifiers::SUPER)
                                                          .union(KeyModifiers::SHIFT)),
                (RightSuper, false) => Key::new(KeyCode::Null, m.union(KeyModifiers::SUPER)),
                (RightSuper, true) => Key::new(KeyCode::Null, m.union(KeyModifiers::SUPER)
                                                               .union(KeyModifiers::SHIFT)),
                (NumPadDot, false) => Key::new(KeyCode::Char('.'), m),
                (NumPadDot, true) => Key::new(KeyCode::Char('.'), m.union(KeyModifiers::SHIFT)),
                (NumPadSlash, false) => Key::new(KeyCode::Char('/'), m),
                (NumPadSlash, true) => Key::new(KeyCode::Char('/'), m.union(KeyModifiers::SHIFT)),
                (NumPadAsterisk, false) => Key::new(KeyCode::Char('*'), m),
                (NumPadAsterisk, true) => Key::new(KeyCode::Char('*'),
                                                   m.union(KeyModifiers::SHIFT)),
                (NumPadMinus, false) => Key::new(KeyCode::Char('-'), m),
                (NumPadMinus, true) => Key::new(KeyCode::Char('-'), m.union(KeyModifiers::SHIFT)),
                (NumPadPlus, false) => Key::new(KeyCode::Char('+'), m),
                (NumPadPlus, true) => Key::new(KeyCode::Char('+'), m.union(KeyModifiers::SHIFT)),
                (NumPadEnter, false) => Key::new(KeyCode::Enter, m),
                (NumPadEnter, true) => Key::new(KeyCode::Enter, m.union(KeyModifiers::SHIFT)),
                (Unknown, false) => Key::new(KeyCode::Null, m),
                (Unknown, true) => Key::new(KeyCode::Null, m.union(KeyModifiers::SHIFT)),
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
        let mut buffer = vec![0; usize::from(WIDTH)*usize::from(HEIGHT)];

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

    fn get_key(&self) -> Option<ui::Key> {
        use minifb::Key::*;
        let window = self.0.as_ref().expect(NOWIN);
        let key = Self::key_to_crossterm(window.get_keys(), window.get_keys_pressed(KeyRepeat::Yes));
        key
        //if key.is_some() {
        //    println!("{:?}", key);
        //}
        //if window.get_keys().len() > 0 {
        //    println!("{:?}", window.get_keys());
        //}
        //None
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

    fn draw_statusline(&mut self, statusline: &Statusline, boundary: &Rectangle) { }

    fn draw_paragraph(&mut self, paragraph: Vec<String>) { todo!() }

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
        let mut text = font5x8::new_renderer(
            (PIXELFACTOR*size.y()).into(),
            (PIXELFACTOR*size.x()).into(),
            0xFFFFFFFF
        );
        let out: Option<String>;

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
                let ui::Key { code, .. } = key;
                match code {
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
        let mut text = font5x8::new_renderer(
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
