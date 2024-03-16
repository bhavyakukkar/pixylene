use crate::ui::{ UserInterface, self, Rectangle, Mode };

use libpixylene::{ types::{ UCoord, PCoord }, project::{ Project, OPixel } };
use pixylene_actions::{ memento::ActionManager, LogType };
use crossterm::event::{ KeyCode, KeyModifiers };
use minifb::{ Window, WindowOptions, Key, KeyRepeat };


const NOWIN: &str = "No Minifb Window found in Target, something is wrong.";
const WIDTH: u16 = 640;
const HEIGHT: u16 = 360;

pub struct TargetMinifb(/*window*/Option<Window>, /*buffer*/Vec<u32>);

impl TargetMinifb {
    pub fn new() -> Self {
        TargetMinifb(None, Vec::new())
    }
}

impl UserInterface for TargetMinifb {
    fn initialize(&mut self) {
        let mut window = Window::new(
            "Pixylene",
            usize::from(WIDTH),
            usize::from(HEIGHT),
            WindowOptions::default()
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

    fn get_size(&self) -> Rectangle {
        Rectangle { start: UCoord::zero(), size: PCoord::new(HEIGHT, WIDTH).unwrap() }
    }

    fn draw_camera(&mut self, dim: PCoord, buffer: Vec<OPixel>, show_cursors: bool,
                   boundary: &Rectangle) {
        let ref mut framebuffer = self.1;

        for i in 0..dim.x() {
            for j in 0..dim.y() {
                let index = usize::from(i)*usize::from(dim.y()) + usize::from(j);
                framebuffer[index] = 0;
                match buffer.get(index).unwrap() {
                    OPixel::Filled{ color, has_cursor, .. } => {
                        framebuffer[index] = ((color.r as u32) << 16) | ((color.g as u32) << 8) |
                                             ((color.b as u32));
                    },
                    OPixel::Empty{ has_cursor, .. } => {
                        framebuffer[index] = 0u32;
                    },
                    OPixel::OutOfScene => {
                        framebuffer[index] = 0u32;
                    },
                }
            }
        }
    }

    fn draw_statusline(&self, project: &Project, action_manager: &ActionManager, mode: &Mode,
                       session: &u8, boundary: &Rectangle) { todo!() }
    fn draw_paragraph(&self, paragraph: Vec<String>) { todo!() }

    fn console_clear(&self, boundary: &Rectangle) {  }
    fn console_in(&self, message: &str, discard_key: &ui::Key,
                  boundary: &Rectangle) -> Option<String> {
        None
    }
    fn console_out(&self, message: &str, log_type: &LogType, boundary: &Rectangle) {  }
}
