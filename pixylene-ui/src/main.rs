//todo: make prelude of all types required to impl UserInterface and Console for a target

mod keybinds;

mod ui;

mod targets;

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
    //let target = targets::TargetCrossterm;
    let target = targets::TargetMinifb::new();
    let mut pixylene_ui = controller::Controller::new(Rc::new(RefCell::new(target)));
    let cli = Cli::parse();

    pixylene_ui.new_session(&cli.command);
}
