use super::Point;
use std::fmt;

/// A 'T' shaped peice.
///
/// ```
///    OOO
///     O
///
///     O
///    OOO
///
///    O
///    OO
///    O
///
///     O
///    OO
///     O
/// ```
pub struct Tee {
    coords: Box<[Point]>,
}

impl Tee {
    pub fn new() -> Tee {
        let mut coords = Vec::new();

        for i in 0..3 {
            coords.push(Point { x: i, y: 0 });
        }
        coords.push(Point { x: 1, y: 1 });

        Tee {
            coords: coords.into_boxed_slice(),
        }
    }
}

impl fmt::Debug for Tee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows = vec![vec![' '; 3]; 3];

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
