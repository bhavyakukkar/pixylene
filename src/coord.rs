#[derive(Copy, Clone)]
pub struct Coord { pub x: isize, pub y: isize }

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
