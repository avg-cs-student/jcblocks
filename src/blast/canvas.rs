use std::fmt;

#[derive(Debug, Clone)]
pub enum PointStatus {
    Occupied(u8),
    Empty,
}

/// Canvas holds the state of the board.
pub struct Canvas {
    columns: usize,
    rows: usize,
    contents: Box<[PointStatus]>,
}

impl Canvas {
    /// Construct a new canvas object.
    pub fn new(rows: usize, columns: usize) -> Self {
        Canvas {
            columns,
            rows,
            contents: vec![PointStatus::Empty; usize::from(rows * columns)].into_boxed_slice(),
        }
    }

    /// Return the indexes of all completed rows.
    pub fn check_rows(&self) -> Vec<usize> {
        let mut completed_rows_by_index = vec![];
        for row in 0..self.rows {
            let is_complete = self.is_complete_row(row);
            match is_complete {
                Some(true) => completed_rows_by_index.push(row),
                Some(false) => continue,
                None => continue,
            }
        }

        completed_rows_by_index
    }

    /// Return the indexes of all completed columns.
    pub fn check_columns(&self) -> Vec<usize> {
        let mut completed_columns_by_index = vec![];
        for column in 0..self.columns {
            let is_complete = self.is_complete_column(column);
            match is_complete {
                Some(true) => completed_columns_by_index.push(column),
                Some(false) => continue,
                None => continue,
            }
        }

        completed_columns_by_index
    }

    /// Return `Some(true)` if the row is completely occupied.
    pub fn is_complete_row(&self, row: usize) -> Option<bool> {
        // Invalid row selection.
        if self.rows <= row {
            return None;
        }

        let mut sum = 0;
        for i in 0..self.columns {
            let pos = i + row * self.columns;
            sum = match self.contents[pos] {
                PointStatus::Occupied(_) => sum + 1,
                PointStatus::Empty => sum,
            };
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

        for i in 0..self.rows {
            let pos = column + self.columns * i;
            sum = match self.contents[pos] {
                PointStatus::Occupied(_) => sum + 1,
                PointStatus::Empty => sum,
            };
        }

        if sum != self.rows {
            return Some(false);
        }

        Some(true)
    }
}

impl fmt::Debug for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // To display a grid, we need space for a newline on each line.
        let nrows = self.contents.len() / usize::from(self.columns);
        let needed_capacity = self.contents.len() + nrows;

        let mut canvas_char_view = vec!['_'; needed_capacity];
        for row in 0..nrows {
            for pos in 0..self.columns {
                let content_index = row * self.columns + pos;
                let view_index = content_index + row;
                canvas_char_view[view_index] = match self.contents[content_index] {
                    PointStatus::Occupied(n) => char::from(n),
                    PointStatus::Empty => '.',
                }
            }
            canvas_char_view[(row + 1) * self.columns + row] = '\n';
        }

        let canvas_str_view: String = canvas_char_view.into_iter().collect();
        write!(f, "Canvas:\n{}", canvas_str_view)
    }
}
