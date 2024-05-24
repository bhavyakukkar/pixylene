mod true_pixel;
pub use true_pixel::{TruePixel, TruePixelError};

mod indexed_pixel;
pub use indexed_pixel::IndexedPixel;


pub trait Pixel: Clone + Copy {
    fn empty() -> Self;
}
