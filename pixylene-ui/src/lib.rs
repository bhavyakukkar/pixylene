use clap::Parser;

pub mod ui;

pub mod config;

pub mod actions;

pub mod controller;

#[derive(Parser)]
#[command(arg_required_else_help = false, author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<controller::StartType>,
}
