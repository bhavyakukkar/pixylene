use crate::ui::{ UserInterface, Key, Rectangle, Statusline, Color, UiFn, KeyInfo };

use libpixylene::{ types::{ PCoord }, project::{ OPixel } };
use pixylene_actions::{ LogType };


pub struct TargetCLI;
impl TargetCLI {
    pub fn new() -> TargetCLI { TargetCLI }
}

impl UserInterface for TargetCLI {
    fn initialize(&mut self) {}
    fn finalize(&mut self) {}

    /// Makes the target refresh between frames, returning whether target is still alive
    fn refresh(&mut self) -> bool { true }

    /// Get the inputted key from the target
    ///
    /// Targets that block until key is received may always return Some(key), however targets that
    /// poll user-input may return None's until some key is received
    fn get_key(&self) -> Option<KeyInfo> { Some(KeyInfo::UiFn(UiFn::RunCommandSpecify)) }
    fn get_size(&self) -> PCoord { PCoord::new(6, 6).unwrap() }

    fn draw_camera(&mut self, dim: PCoord, buffer: Vec<OPixel>, show_cursors: bool,
                   boundary: &Rectangle) {
        println!("canvas stuff");
    }
    fn draw_paragraph(&mut self, paragraph: Vec<String>) {
        println!("{}", paragraph.into_iter().collect::<String>());
    }

    fn draw_statusline(&mut self, statusline: &Statusline, boundary: &Rectangle) {}

    fn console_in(&mut self, message: &str, discard_key: &Key, boundary: &Rectangle) -> Option<String> {
        println!("{}", message);
        let mut line = String::new();
        _ = std::io::stdin().read_line(&mut line).unwrap();
        Some(line[0..(line.len() - 1)].to_string())
    }

    fn console_out(&mut self, message: &str, log_type: &LogType, boundary: &Rectangle) {
        println!("{}", message);
    }

    fn clear(&mut self, boundary: &Rectangle) {}
    fn clear_all(&mut self) {}
}
