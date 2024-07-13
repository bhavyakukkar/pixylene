#[cfg(feature = "crossterm")]
mod target_crossterm;
#[cfg(feature = "crossterm")]
pub use target_crossterm::TargetCrossterm;

#[cfg(feature = "minifb")]
mod target_minifb;
#[cfg(feature = "minifb")]
pub use target_minifb::TargetMinifb;

#[cfg(feature = "cli")]
mod target_cli;
#[cfg(feature = "cli")]
pub use target_cli::TargetCLI;

#[cfg(feature = "web")]
mod target_web;
#[cfg(feature = "web")]
pub use target_web::TargetWeb;
