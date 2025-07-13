use std::fmt;

use crate::blocks::Block;

#[derive(Debug, Clone)]
pub enum PointStatus {
    Occupied,
    Empty,
    MarkedForRemoval,
}

pub struct PlayableBlock {
    block: Block,
    row: usize,
    column: usize,
}

/// Canvas holds the state of the board.
pub struct Canvas {
    pub columns: usize,
    pub rows: usize,
    contents: Vec<PointStatus>,
}

pub const DEFAULT_CANVAS_HEIGHT: usize = 8;
pub const DEFAULT_CANVAS_WIDTH: usize = 8;

impl Canvas {
    pub fn new(rows: usize, columns: usize) -> Self {
        Canvas {
            columns,
            rows,
            contents: vec![PointStatus::Empty; usize::from(rows * columns)],
        }
    }

    pub fn contents(&self) -> Vec<PointStatus> {
        self.contents.clone()
    }

    pub fn clear_all(&mut self) -> &mut Self {
        for space in self.contents.iter_mut() {
            *space = PointStatus::Empty;
        }

        self
    }

    fn position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.columns as i32 || y >= self.rows as i32 {
            return None;
        }

        Some(self.columns * y as usize + x as usize)
    }

    pub fn can_fit_at(&self, block: &Block, row: i32, column: i32) -> bool {
        for p in block.coordinates() {
            if let Some(index) = self.position_to_index(column + p.x, row + p.y) {
                if let PointStatus::Occupied = self.contents[index] {
                    return false;
                }
            }
        }

        true
    }

    pub fn can_fit(&self, block: &Block) -> bool {
        for row in 0..self.rows {
            for col in 0..self.columns {
                if self.can_fit_at(block, col as i32, row as i32) {
                    return true;
                }
            }
        }

        false
    }

    pub fn try_make_playable(&self, block: &Block, row: i32, column: i32) -> Option<PlayableBlock> {
        if !self.can_fit_at(block, row, column) {
            return None;
        }

        Some(PlayableBlock {
            block: block.clone(),
            row: row as usize,
            column: column as usize,
        })
    }

    pub fn add(&mut self, block: &PlayableBlock) -> &mut Self {
        for p in block.block.coordinates() {
            if let Some(index) =
                self.position_to_index(block.column as i32 + p.x, block.row as i32 + p.y)
            {
                self.contents[index] = PointStatus::Occupied;
            }
        }

        self
    }

    /// Clear all completed rows and columns then returns number of rows and columns removed.
    pub fn clear_completed_lines(&mut self) -> usize {
        let mut removed = 0;

        // mark cols
        for col in 0..self.columns {
            if let Some(true) = self.is_complete_column(col) {
                for row in 0..self.rows {
                    if let Some(index) = self.position_to_index(col as i32, row as i32) {
                        self.contents[index] = PointStatus::MarkedForRemoval;
                    }
                }
                removed += 1;
            }
        }

        // mark rows
        for row in 0..self.rows {
            if let Some(true) = self.is_complete_row(row) {
                for col in 0..self.columns {
                    if let Some(index) = self.position_to_index(col as i32, row as i32) {
                        self.contents[index] = PointStatus::MarkedForRemoval;
                    }
                }
                removed += 1;
            }
        }

        // mark empty
        for p in self.contents.iter_mut() {
            if let PointStatus::MarkedForRemoval = *p {
                *p = PointStatus::Empty;
            }
        }

        removed
    }

    /// Return `Some(true)` if the row is completely occupied.
    pub fn is_complete_row(&self, row: usize) -> Option<bool> {
        // Invalid row selection.
        if self.rows <= row {
            return None;
        }

        let mut sum = 0;
        for col in 0..self.columns {
            if let Some(index) = self.position_to_index(col as i32, row as i32) {
                sum = match self.contents[index] {
                    PointStatus::Occupied => sum + 1,
                    PointStatus::MarkedForRemoval => sum + 1,
                    PointStatus::Empty => sum,
                };
            }
        }

        if sum != self.columns {
            return Some(false);
        }

        Some(true)
    }

    /// Return `Some(true)` if the column is completely occupied.
    pub fn is_complete_column(&self, column: usize) -> Option<bool> {
        // Invalid column selection.
        if self.columns <= column {
            return None;
        }

        let mut sum = 0;

        for row in 0..self.rows {
            if let Some(index) = self.position_to_index(column as i32, row as i32) {
                sum = match self.contents[index] {
                    PointStatus::Occupied => sum + 1,
                    PointStatus::MarkedForRemoval => sum + 1,
                    PointStatus::Empty => sum,
                };
            }
        }

        if sum != self.rows {
            return Some(false);
        }

        Some(true)
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas::new(DEFAULT_CANVAS_HEIGHT, DEFAULT_CANVAS_WIDTH)
    }
}

impl fmt::Debug for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut canvas_char_view = Vec::new();
        for row in (0..self.rows).rev() {
            canvas_char_view.push(char::from_digit(row as u32, 10).unwrap());
            canvas_char_view.push(' ');
            for col in 0..self.columns {
                let content_index = self.position_to_index(col as i32, row as i32).unwrap();
                let marker = match self.contents[content_index] {
                    PointStatus::Occupied => '▅',
                    PointStatus::MarkedForRemoval => '⏲',
                    PointStatus::Empty => '.',
                };
                canvas_char_view.push(marker);
                canvas_char_view.push(' ');
            }
            canvas_char_view.push('\n');
        }

        // whitespace before x labels
        for _ in 0..2 {
            canvas_char_view.push(' ');
        }

        // x labels
        for c in "01234567".chars() {
            canvas_char_view.push(c);
            canvas_char_view.push(' ');
        }
        canvas_char_view.push('\n');

        let canvas_str_view: String = canvas_char_view.into_iter().collect();
        write!(f, "{}", canvas_str_view)
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::*;

    use super::*;

    macro_rules! test_position_to_index {
        ( $name:ident, $x:expr, $y:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let board = Canvas::new(8, 8);
                let index = board.position_to_index($x, $y);
                if let Some(i) = index {
                    assert_eq!($expected, i);
                } else {
                    assert!(false, "Expected a valid index from a value position.");
                }
            }
        };
    }

    macro_rules! test_position_to_index_fail {
        ( $name:ident, $x:expr, $y:expr) => {
            #[test]
            fn $name() {
                let board = Canvas::new(8, 8);
                let index = board.position_to_index($x, $y);
                if let Some(_) = index {
                    assert!(false, "Expected a invalid position to fail.");
                }
            }
        };
    }

    test_position_to_index!(pos_to_idx_x0_y0_maps_to_0, 0, 0, 0);
    test_position_to_index!(pos_to_idx_x1_y0_maps_to_1, 1, 0, 1);
    test_position_to_index!(pos_to_idx_x0_y1_maps_to_8, 0, 1, 8);
    test_position_to_index!(pos_to_idx_x0_y2_maps_to_16, 0, 2, 16);
    test_position_to_index!(pos_to_idx_x1_y2_maps_to_17, 1, 2, 17);
    test_position_to_index!(pos_to_idx_x8_y8_maps_to_63, 7, 7, 63);

    test_position_to_index_fail!(pos_to_idx_negative_x, -1, 0);
    test_position_to_index_fail!(pos_to_idx_negative_y, 0, -1);
    test_position_to_index_fail!(pos_to_idx_negative_x_and_y, -3, -3);
    test_position_to_index_fail!(pos_to_idx_large_x, 10, 1);
    test_position_to_index_fail!(pos_to_idx_large_y, 1, 10);
    test_position_to_index_fail!(pos_to_idx_large_x_and_y, 8, 8);

    macro_rules! test_add_blocks {
        ( $name:ident, $blocks:expr, $should_add:expr, $where_to_add:expr ) => {
            #[test]
            fn $name() {
                let mut board = Canvas::new(8, 8);

                // validate the test input
                assert!(
                    $blocks.len() == $should_add.len(),
                    "All lists should be equal length."
                );
                assert!(
                    $blocks.len() == $where_to_add.len(),
                    "All lists should be equal length."
                );

                for (i, b) in $blocks.into_iter().enumerate() {
                    let maybe_playable =
                        board.try_make_playable(&b, $where_to_add[i].y, $where_to_add[i].x);

                    if let Some(playable) = maybe_playable {
                        board.add(&playable);
                    } else {
                        assert!(!$should_add[i], "Unable to add block[{i}]\n{board:?}");
                    }
                }
            }
        };
    }

    test_add_blocks!(
        can_add_one_and_only_one_1x1_in_a_position,
        [Block::rectangle(1, 1), Block::rectangle(1, 1)],
        [true, false],
        [Point { x: 0, y: 0 }, Point { x: 0, y: 0 }]
    );

    test_add_blocks!(
        can_add_many_1x1s_to_different_positions,
        [
            Block::rectangle(1, 1),
            Block::rectangle(1, 1),
            Block::rectangle(1, 1),
            Block::rectangle(1, 1),
            Block::rectangle(1, 1),
        ],
        [true, true, true, true, true],
        [
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 4, y: 4 },
            Point { x: 7, y: 7 },
        ]
    );

    test_add_blocks!(
        can_add_many_rectangles,
        [
            Block::rectangle(1, 1),
            Block::rectangle(2, 2),
            Block::rectangle(3, 3),
            Block::rectangle(5, 1),
            Block::rectangle(5, 1),
        ],
        [true, true, true, true, true],
        [
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 3 },
            Point { x: 0, y: 6 },
            Point { x: 0, y: 7 },
        ]
    );

    test_add_blocks!(
        can_fill_board,
        [
            Block::rectangle(1, 5),
            Block::rectangle(1, 5),
            Block::rectangle(1, 5),
            Block::rectangle(1, 5),
            Block::rectangle(1, 5),
            Block::rectangle(1, 5),
            Block::rectangle(1, 5),
            Block::rectangle(1, 5),
            Block::rectangle(1, 3),
            Block::rectangle(1, 3),
            Block::rectangle(1, 3),
            Block::rectangle(1, 3),
            Block::rectangle(1, 3),
            Block::rectangle(1, 3),
            Block::rectangle(1, 3),
            Block::rectangle(1, 3),
        ],
        [
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true,
        ],
        [
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 5, y: 0 },
            Point { x: 6, y: 0 },
            Point { x: 7, y: 0 },
            Point { x: 0, y: 5 },
            Point { x: 1, y: 5 },
            Point { x: 2, y: 5 },
            Point { x: 3, y: 5 },
            Point { x: 4, y: 5 },
            Point { x: 5, y: 5 },
            Point { x: 6, y: 5 },
            Point { x: 7, y: 5 },
        ]
    );
}
