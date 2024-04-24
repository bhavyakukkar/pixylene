use pixylene_ui::{
    Cli, controller::Controller,
    ui::{ UserInterface, Key, Rectangle, Statusline, UiFn, KeyInfo },
};

use libpixylene::{ types::{ PCoord }, project::{ OPixel } };
use pixylene_actions::{ LogType };
use std::rc::Rc;
use std::cell::RefCell;
use clap::Parser;


struct TargetCLI;

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

    fn draw_camera(&mut self, _dim: PCoord, _buffer: Vec<OPixel>, _show_cursors: bool,
                   _boundary: &Rectangle) {
        //println!("canvas stuff");
    }
    fn draw_paragraph(&mut self, paragraph: Vec<colored::ColoredString>, _boundary: &Rectangle) {
        println!("{}", paragraph.into_iter().map(|s| s.to_string()).collect::<String>());
    }

    fn draw_statusline(&mut self, _statusline: &Statusline, _boundary: &Rectangle) {}

    fn console_in(&mut self, _message: &str, _discard_key: &Key, _boundary: &Rectangle) -> Option<String> {
        //println!("{}", message);
        let mut line = String::new();
        _ = std::io::stdin().read_line(&mut line).unwrap();
        Some(line[0..(line.len() - 1)].to_string())
    }

    fn console_out(&mut self, message: &str, _log_type: &LogType, _boundary: &Rectangle) {
        println!("{}", message);
    }

    fn clear(&mut self, _boundary: &Rectangle) {}
    fn clear_all(&mut self) {}
}


fn main() {
    let target = TargetCLI;

    match Controller::new(Rc::new(RefCell::new(target))) {
        Ok(mut pixylene_ui) => {
            let cli = Cli::parse();

            pixylene_ui.start(&cli.command);
        },
        Err(error) => eprintln!("{}", error)
    }
}
