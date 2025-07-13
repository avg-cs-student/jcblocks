/// The smallest component of a peice.
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// The playing board.
pub mod canvas;

/// Each of line, square, rectangle are all implemented using the rectangle module.
pub mod rectangle;

/// The T shaped peice.
pub mod tee;
