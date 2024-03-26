use libpixylene::types::Coord;


pub enum AbsOrRel<A, B> {
    Abs(A),
    Rel(B),
}

/// Helper enum describing Direction, to be used in actions
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns a unit-vector in the specified direction
    pub fn unit(&self) -> Coord {
        use Direction::*;

        match self {
            Up => Coord{ x: -1, y: 0 },
            Down => Coord{ x: 1, y: 0 },
            Left => Coord{ x: 0, y: -1 },
            Right => Coord{ x: 0, y: 1 },
        }
    }
}
