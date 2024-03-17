use crate::ui::{ UserInterface, self, Rectangle, Mode };

use libpixylene::{ types::{ UCoord, PCoord }, project::{ Project, OPixel } };
use pixylene_actions::{ memento::ActionManager, LogType };
use crossterm::event::{ KeyCode, KeyModifiers };
use minifb::{ Window, WindowOptions, Key, KeyRepeat, Scale };
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

    fn convert_key_instance(ks: Vec<Key>) -> ui::Key {
        use Key::*;
        let m = KeyModifiers::empty();

        //Shift
        if let Some(i) = ks.position(|&k| k == Key::LeftShift) {
            m.insert(KeyModifiers::SHIFT);
            ks.remove(i);
        }
        if let Some(i) = ks.contains(|&k| k == Key::RightShift) {
            m.insert(KeyModifiers::SHIFT);
            ks.remove(i);
        }

        //Ctrl
        if let Some(i) = ks.position(|&k| k == Key::LeftCtrl) {
            m.insert(KeyModifiers::CONTROL);
            ks.remove(i);
        }
        if let Some(i) = ks.contains(|&k| k == Key::RightCtrl) {
            m.insert(KeyModifiers::CONTROL);
            ks.remove(i);
        }

        //over here
        if k >= Key0 && k <= Key9 { ui::Key::new(KeyCode::Char( key + 48 ), KeyModifiers::empty() }
    }
}

impl UserInterface for TargetMinifb {
    fn initialize(&mut self) {
        let mut window = Window::new(
            "Pixylene",
            usize::from(WIDTH),
            usize::from(HEIGHT),
            WindowOptions {
                scale: Scale::FitScreen,
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
        let window = self.0.as_ref().expect(NOWIN);
        for key in window.get_keys_pressed(KeyRepeat::No) {
            match key {
                Key::Escape => {
                    return Some(ui::Key::new(KeyCode::Esc, KeyModifiers::empty()));
                },
                Key::I => {
                    return Some(ui::Key::new(KeyCode::Char('i'), KeyModifiers::empty()));
                },
                Key::O => {
                    return Some(ui::Key::new(KeyCode::Char('o'), KeyModifiers::empty()));
                },
                _ => (),
            }
        }
        None
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

    fn draw_statusline(&mut self, project: &Project, action_manager: &ActionManager, mode: &Mode,
                       session: &u8, boundary: &Rectangle) {
    }

    fn draw_paragraph(&mut self, paragraph: Vec<String>) { todo!() }

    fn console_clear(&mut self, boundary: &Rectangle) {  }

    fn console_in(&mut self, message: &str, discard_key: &ui::Key,
                  boundary: &Rectangle) -> Option<String> {
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
        let mut input: String;
        loop {
            for key in window.get_keys_pressed(KeyRepeat::No) {
                if self.is_alphabetic(key) {
                }
                if key >= Key::0 and key <= Key::Z {
                }
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
}
