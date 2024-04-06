cfg_if::cfg_if! {
    if #[cfg(all(feature = "minifb", not(any(feature = "crossterm", feature = "cli"))))] {
        fn main() {}
    } else if #[cfg(all(feature = "crossterm", not(any(feature = "minifb", feature = "cli"))))] {
        fn main() {}
    } else if #[cfg(all(feature = "cli", not(any(feature = "minifb", feature = "crossterm"))))] {
        fn main() {}
    } else {
        fn main() {
            compile_error!("You need to pick exactly one of the targets: minifb, crossterm, or cli");
        }
    }
}
