use super::Point;
use std::fmt;

pub struct Rectangle {
    coords: Box<[Point]>,
}

/// All squares/rectangles must be no longer than `LONGEST_RECTANGLE_EDGE`.
pub const LONGEST_RECTANGLE_EDGE: usize = 5;

impl Rectangle {
    pub fn new(height: usize, width: usize) -> Self {
        let mut coords = Vec::new();

        // There is no reason for bounding the size other than to simplify the problem.
        if height > LONGEST_RECTANGLE_EDGE || width > LONGEST_RECTANGLE_EDGE {
            panic!("Edge size must be {LONGEST_RECTANGLE_EDGE} or less.")
        }

        // Assumption: Rectangles are always 'edge_length' by 2.
        for i in 0..height {
            for j in 0..width {
                coords.push(Point { x: i, y: j });
            }
        }

        Rectangle {
            coords: coords.into_boxed_slice(),
        }
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows = vec![vec![' '; LONGEST_RECTANGLE_EDGE]; LONGEST_RECTANGLE_EDGE];

        // Write out the populated points.
        for p in &self.coords {
            rows[p.y][p.x] = '@';
        }

        // Concatenate the vectors.
        for row in rows.iter_mut() {
            row.push('\n');
        }

        let out_str: String = rows.into_iter().rev().flatten().collect::<String>();
        write!(f, "\n{}", out_str)
    }
}
