#[cfg(feature = "crossterm")]
mod target_crossterm;
#[cfg(feature = "crossterm")]
pub use target_crossterm::TargetCrossterm;

#[cfg(feature = "minifb")]
mod target_minifb;
#[cfg(feature = "minifb")]
pub use target_minifb::TargetMinifb;
