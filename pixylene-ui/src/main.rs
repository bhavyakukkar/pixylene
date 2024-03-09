mod keybinds;

mod target;

mod targets;

mod pixylene_ui;


use std::rc::Rc;
use std::cell::RefCell;
use clap::Parser;


#[derive(Parser)]
#[command(arg_required_else_help = true, author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<pixylene_ui::StartType>,
}


fn main() {
    let target = targets::TargetCrossterm;
    let mut pixylene_ui = pixylene_ui::PixyleneUI::new(Rc::new(RefCell::new(target)));
    let cli = Cli::parse();

    pixylene_ui.new_session(&cli.command);
}
