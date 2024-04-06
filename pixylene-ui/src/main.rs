//todo: make prelude of all types required to impl UserInterface and Console for a target

mod ui;

mod config;

mod targets;

pub mod actions;

mod controller;


use std::rc::Rc;
use std::cell::RefCell;
use clap::Parser;


#[derive(Parser)]
#[command(arg_required_else_help = true, author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<controller::StartType>,
}


fn main() {
    #[cfg(feature = "crossterm")]
    let target = targets::TargetCrossterm;

    #[cfg(feature = "minifb")]
    let target = targets::TargetMinifb::new();

    #[cfg(feature = "cli")]
    let target = targets::TargetCLI::new();

    match controller::Controller::new(Rc::new(RefCell::new(target))) {
        Ok(mut pixylene_ui) => {
            let cli = Cli::parse();

            pixylene_ui.new_session(&cli.command);
        },
        Err(error) => eprintln!("{}", error)
    }
}
