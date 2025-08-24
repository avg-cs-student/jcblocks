use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use rand::distr::{Distribution, StandardUniform};
use rand::{Rng, random};

/// The smallest component of a peice.
/// ```
/// ┌─┐
/// └─┘
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Rotate right 90 degrees about the origin.
    pub fn rotate_right(&mut self) -> &mut Self {
        let tmp = self.x;
        self.x = self.y;
        self.y = 0 - tmp;
        self
    }

    /// Rotate left 90 degrees about the origin.
    pub fn rotate_left(&mut self) -> &mut Self {
        let tmp = self.x;
        self.x = 0 - self.y;
        self.y = tmp;
        self
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub const MAX_RECTANGLE_EDGE: usize = 3;
pub const MAX_LINE_LENGTH: usize = 5;
pub const MIN_ELLE_EDGE: usize = 2;
pub const MAX_ELLE_EDGE: usize = 3;

#[derive(Debug, Clone)]
pub enum Variant {
    /// The following shapes can be created as a Rectangle:
    /// ```
    /// ┌─┐ ┌─┬─┐ ┌─┬─┐ ┌─┬─┬─┐ ┌─┬─┬─┐
    /// └─┘ ├─┼─┤ ├─┼─┤ ├─┼─┼─┤ ├─┼─┼─┤
    ///     └─┴─┘ ├─┼─┤ └─┴─┴─┘ ├─┼─┼─┤
    ///           └─┴─┘         └─┴─┴─┘
    /// ```
    /// Where the origin (0,0) is the lower leftmost block.
    ///
    /// NOTE: Though you are free to create `Line`s this way, prefer using `Line` explicitly.
    Rectangle,

    /// ```
    ///   ┌─┐  
    /// ┌─┼─┼─┐
    /// └─┴─┴─┘
    /// ```
    /// Where the origin (0,0) is the lower leftmost block.
    Tee,
    Diagonal,
    Elle,

    /// ┌─┬─┐    ┌─┬─┬─┐   ┌─┬─┬─┬─┐  ┌─┬─┬─┬─┬─┐
    /// └─┴─┘    └─┴─┴─┘   └─┴─┴─┴─┘  └─┴─┴─┴─┴─┘
    /// Where the origin (0,0) is the lower leftmost block.
    Line,
}

impl Distribution<Variant> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Variant {
        match rng.random_range(0..=4) {
            0 => Variant::Rectangle,
            1 => Variant::Tee,
            2 => Variant::Diagonal,
            3 => Variant::Elle,
            _ => Variant::Line,
        }
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Variant::Diagonal => "Diagonal",
            Variant::Elle => "Elle",
            Variant::Tee => "Tee",
            Variant::Rectangle => "Rectangle",
            Variant::Line => "Line",
        };
        write!(f, "{name}")
    }
}

pub struct Dimension {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone)]
pub struct Block {
    coords: Vec<Point>,
    variant: Variant,
}

impl Block {
    /// Tee constructor. Tees are always the same size.
    pub fn tee() -> Self {
        let mut coords = Vec::new();

        for i in 0..3 {
            coords.push(Point { x: i, y: 0 });
        }
        coords.push(Point { x: 1, y: 1 });

        Self {
            coords,
            variant: Variant::Tee,
        }
    }

    /// Width/Height are restricted to the range [1, `MAX_RECTANGLE_EDGE`].
    pub fn rectangle(width: usize, height: usize) -> Self {
        let mut coords = Vec::new();

        for i in 0..width.clamp(1, MAX_RECTANGLE_EDGE) {
            for j in 0..height.clamp(1, MAX_RECTANGLE_EDGE) {
                coords.push(Point {
                    x: i as i32,
                    y: j as i32,
                });
            }
        }

        Self {
            coords,
            variant: Variant::Rectangle,
        }
    }

    /// Length is restricted to the range [2, `MAX_LINE_LENGTH`].
    pub fn line(length: usize) -> Self {
        let mut coords = Vec::new();

        for i in 0..length.clamp(2, MAX_LINE_LENGTH) {
            coords.push(Point {
                x: i as i32,
                y: 0 as i32,
            });
        }

        Self {
            coords,
            variant: Variant::Line,
        }
    }

    pub fn diagonal(width: usize) -> Self {
        let mut coords = Vec::new();

        for i in 0..width {
            coords.push(Point {
                x: i as i32,
                y: i as i32,
            });
        }

        Self {
            coords,
            variant: Variant::Diagonal,
        }
    }

    /// Width/Height are restricted to the range [`MIN_ELLE_EDGE`, `MAX_ELLE_EDGE`].
    pub fn elle(width: usize, height: usize) -> Self {
        let mut coords = Vec::new();

        coords.push(Point { x: 0, y: 0 });
        for i in 1..height.clamp(MIN_ELLE_EDGE, MAX_ELLE_EDGE) {
            coords.push(Point { x: i as i32, y: 0 });
        }

        for i in 1..width.clamp(MIN_ELLE_EDGE, MAX_ELLE_EDGE) {
            coords.push(Point { x: 0, y: i as i32 });
        }

        Self {
            coords,
            variant: Variant::Elle,
        }
    }

    pub fn coordinates(&self) -> &Vec<Point> {
        &self.coords
    }

    pub fn coordinates_mut(&mut self) -> &mut Vec<Point> {
        &mut self.coords
    }

    pub fn dimensions(&self) -> Dimension {
        // diagonals can be computed trivially
        if let Variant::Diagonal = self.variant {
            let points = self.coords.len();
            return Dimension {
                width: points,
                height: points,
            };
        }

        // Create a histogram of x/y values then just grab the max.
        let height: i32 = self
            .coordinates()
            .iter()
            .fold(&mut HashMap::new(), |acc, coord| {
                if let Some(val) = acc.get(&coord.x) {
                    acc.insert(coord.x, val + 1);
                } else {
                    acc.insert(coord.x, 1);
                };

                acc
            })
            .drain()
            .map(|(_k, v)| v)
            .max()
            .unwrap_or(0);

        let width: i32 = self
            .coordinates()
            .iter()
            .fold(&mut HashMap::new(), |acc, coord| {
                if let Some(val) = acc.get(&coord.y) {
                    acc.insert(coord.y, val + 1);
                } else {
                    acc.insert(coord.y, 1);
                };

                acc
            })
            .drain()
            .map(|(_k, v)| v)
            .max()
            .unwrap_or(0);

        Dimension {
            width: width as usize,
            height: height as usize,
        }
    }

    /// Rotate 90 degrees to the right about the origin.
    pub fn rotate_right(&mut self) -> &mut Self {
        self.coordinates_mut().iter_mut().for_each(|p| {
            p.rotate_right();
        });
        self
    }

    /// Rotate 90 degrees to the left about the origin.
    pub fn rotate_left(&mut self) -> &mut Self {
        self.coordinates_mut().iter_mut().for_each(|p| {
            p.rotate_left();
        });
        self
    }
}

impl Distribution<Block> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Block {
        let variant: Variant = random();
        let width = rng.random::<u8>() as usize % MAX_RECTANGLE_EDGE + 1;
        let height = rng.random::<u8>() as usize % MAX_RECTANGLE_EDGE + 1;

        match variant {
            Variant::Rectangle => Block::rectangle(width, height),
            Variant::Tee => Block::tee(),
            Variant::Elle => Block::elle(width, height),
            Variant::Diagonal => Block::diagonal(width),
            Variant::Line => Block::line(width),
        }
    }
}

impl Display for Block {
    /// Textual (unicode) representation of a block.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Printing a block requires the allocation of a large enough rectangle to fit it plus some
        // whitespace in between points and new lines at the end of each row.
        let dimensions = self.dimensions();
        let display_repr_height = dimensions.height;
        let display_repr_width = dimensions.width * 2 + 1;

        // Blocks are encoded assuming a standard coordinate system, where x grows right and y
        // grows up. Printing to stdout naturally occurs top to bottom, so a bit of translation is
        // required.
        let min_y = self.coordinates().iter().map(|p| p.y).min().unwrap_or(0);
        let min_x = self.coordinates().iter().map(|p| p.x).min().unwrap_or(0);
        let coord_to_index = |p: &Point| -> usize {
            // normalize all shapes to be in the first quadrant
            let norm_x = (p.x - min_x) as usize;
            let norm_y = (p.y - min_y) as usize;

            display_repr_width * (display_repr_height - 1 - norm_y) + norm_x * 2
        };

        let mut buf = vec![' '; display_repr_width * display_repr_height];
        for row in 1..=display_repr_height {
            // 2x2 Rectangle, view vs buffer index
            //  ▅ _ ▅ _ \n
            //  0 1 2 3 4
            //  ▅ _ ▅ _ \n
            //  5 6 7 8 9
            let end_of_row_position = display_repr_width * row - 1;
            buf[end_of_row_position] = '\n';
        }

        for c in self.coordinates().iter() {
            let index = coord_to_index(c);
            buf[index] = '▅';
        }

        let block_str_view: String = buf.into_iter().collect();
        write!(f, "{}", block_str_view)
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dimensions = self.dimensions();
        write!(
            f,
            "{}: {}x{}\n{}",
            self.variant, dimensions.width, dimensions.height, self
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_can_create {
        ( $name:ident, $width:expr, $height:expr, $expected_coords:expr ) => {
            #[test]
            fn $name() {
                let block = Block::rectangle($width, $height);
                assert_eq!($height * $width, block.coordinates().len());

                for c in $expected_coords {
                    assert!(block.coordinates().contains(&c));
                }
            }
        };
    }

    // Exhaustively test all blocks, as there are a small set of them and they are core to the
    // puzzle.

    // ┌─┐
    // └─┘
    test_can_create!(can_create_1x1, 1, 1, vec![Point { x: 0, y: 0 }]);

    // ┌─┬─┐
    // ├─┼─┤
    // └─┴─┘
    test_can_create!(can_create_2x2, 2, 2, vec![Point { x: 0, y: 0 }]);

    // ┌─┬─┐
    // ├─┼─┤
    // ├─┼─┤
    // └─┴─┘
    test_can_create!(
        can_create_2x3_rect,
        2,
        3,
        vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ]
    );

    // ┌─┬─┬─┐
    // ├─┼─┼─┤
    // └─┴─┴─┘
    test_can_create!(
        can_create_3x2_rect,
        3,
        2,
        vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
        ]
    );

    // ┌─┬─┬─┐
    // ├─┼─┼─┤
    // ├─┼─┼─┤
    // └─┴─┴─┘
    test_can_create!(
        can_create_3x3_rect,
        3,
        3,
        vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ]
    );

    macro_rules! test_dimensions {
        ( $name:ident, $block:expr, $expected_width:expr, $expected_height:expr ) => {
            #[test]
            fn $name() {
                let dimensions = $block.dimensions();
                assert_eq!($expected_width, dimensions.width);
                assert_eq!($expected_height, dimensions.height);
            }
        };
    }

    test_dimensions!(test_dimensions_1x1_rect, Block::rectangle(1, 1), 1, 1);
    test_dimensions!(
        test_dimensions_1x1_rect_rot,
        Block::rectangle(1, 1).rotate_right(),
        1,
        1
    );
    test_dimensions!(test_dimensions_2x2_rect, Block::rectangle(2, 2), 2, 2);
    test_dimensions!(
        test_dimensions_2x2_rect_rot,
        Block::rectangle(2, 2).rotate_left(),
        2,
        2
    );
    test_dimensions!(test_dimensions_len2_line, Block::line(2), 2, 1);
    test_dimensions!(
        test_dimensions_len2_line_rot,
        Block::line(2).rotate_left(),
        1,
        2
    );
    test_dimensions!(test_dimensions_len3_line, Block::line(3), 3, 1);
    test_dimensions!(test_dimensions_len5_line, Block::line(5), 5, 1);
    test_dimensions!(test_dimensions_0deg_tee, Block::tee(), 3, 2);
    test_dimensions!(test_dimensions_90deg_tee, Block::tee().rotate_left(), 2, 3);
    test_dimensions!(test_dimensions_3x2_0deg_elle, Block::elle(2, 3), 3, 2);
    test_dimensions!(
        test_dimensions_3x2_90deg_elle,
        Block::elle(2, 3).rotate_right(),
        2,
        3
    );
    test_dimensions!(test_dimensions_2x2_0deg_elle, Block::elle(2, 2), 2, 2);
    test_dimensions!(
        test_dimensions_2x2_90deg_elle,
        Block::elle(2, 2).rotate_right(),
        2,
        2
    );
    test_dimensions!(test_dimensions_2_0deg_diag, Block::diagonal(2), 2, 2);
    test_dimensions!(
        test_dimensions_2_90deg_diag,
        Block::diagonal(2).rotate_left(),
        2,
        2
    );

    test_dimensions!(test_dimensions_5_0deg_diag, Block::diagonal(5), 5, 5);
    test_dimensions!(
        test_dimensions_5_90deg_diag,
        Block::diagonal(5).rotate_left(),
        5,
        5
    );

    macro_rules! test_rotate_right {
        ( $name:ident, $block:expr, $num_rotations:expr, $expected_coords:expr ) => {
            #[test]
            fn $name() {
                let mut shape = $block;
                for _ in 0..$num_rotations {
                    shape.rotate_right();
                }

                assert_eq!($expected_coords.len(), shape.coordinates().len());
                for p in $expected_coords {
                    assert!(
                        shape.coordinates().contains(&p),
                        "Expected {:?} in coordinates after {} right rotation(s). Got {:?}",
                        p,
                        $num_rotations,
                        shape.coordinates(),
                    );
                }
            }
        };
    }

    test_rotate_right!(
        can_rotate_right_once,
        Block::tee(),
        1,
        vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: -2 },
            Point { x: 1, y: -1 },
        ]
    );

    test_rotate_right!(
        can_rotate_right_twice,
        Block::tee(),
        2,
        vec![
            Point { x: 0, y: 0 },
            Point { x: -1, y: 0 },
            Point { x: -2, y: 0 },
            Point { x: -1, y: -1 },
        ]
    );

    test_rotate_right!(
        can_rotate_right_thrice,
        Block::tee(),
        3,
        vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: -1, y: 1 },
        ]
    );

    test_rotate_right!(
        can_rotate_right_completely,
        Block::tee(),
        4,
        Block::tee().coordinates()
    );

    macro_rules! test_rotate_left {
        ( $name:ident, $block:expr, $num_rotations:expr, $expected_coords:expr ) => {
            #[test]
            fn $name() {
                let mut shape = $block;
                for _ in 0..$num_rotations {
                    shape.rotate_left();
                }

                assert_eq!($expected_coords.len(), shape.coordinates().len());
                for p in $expected_coords {
                    assert!(
                        shape.coordinates().contains(&p),
                        "Expected {:?} in coordinates after {} left rotation(s). Got {:?}",
                        p,
                        $num_rotations,
                        shape.coordinates(),
                    );
                }
            }
        };
    }

    test_rotate_left!(
        can_rotate_left_once,
        Block::tee(),
        1,
        vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: -1, y: 1 },
        ]
    );

    test_rotate_left!(
        can_rotate_left_twice,
        Block::tee(),
        2,
        vec![
            Point { x: 0, y: 0 },
            Point { x: -1, y: 0 },
            Point { x: -2, y: 0 },
            Point { x: -1, y: -1 },
        ]
    );

    test_rotate_left!(
        can_rotate_left_thrice,
        Block::tee(),
        3,
        vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: -2 },
            Point { x: 1, y: -1 },
        ]
    );

    test_rotate_left!(
        can_rotate_left_completely,
        Block::tee(),
        4,
        Block::tee().coordinates()
    );
}
