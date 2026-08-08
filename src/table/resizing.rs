//! Cleanroom Rust port of upstream Go source file: `table/resizing.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! The table resizer computes optimized column widths and row heights so a
//! table exactly fits its configured width/height.
//! </public-docs>

use std::cmp::{max, min};

use super::rows::{data_to_matrix, Data};
use super::util::{btoi, bton, median, sum};
use crate::size;

/// resizerColumn is a column in the resizer.
#[derive(Debug, Clone)]
pub(crate) struct ResizerColumn {
    pub index: usize,
    pub min: usize,
    pub max: usize,
    pub median: usize,
    pub rows: Vec<Vec<String>>,
    pub x_padding: usize,
    pub fixed_width: usize,
}

/// resizer is a table resizer.
#[derive(Debug, Clone)]
pub(crate) struct Resizer {
    pub table_width: usize,
    pub table_height: usize,
    pub headers: Vec<String>,
    pub all_rows: Vec<Vec<String>>,
    pub row_heights: Vec<usize>,
    pub columns: Vec<ResizerColumn>,

    pub wrap: bool,
    pub border_column: bool,
    pub y_paddings: Vec<Vec<usize>>,

    pub y_offset: usize,
    pub use_manual_height: bool,
    pub border_top: bool,
    pub border_bottom: bool,
    pub border_left: bool,
    pub border_right: bool,
    pub border_header: bool,
    pub border_row: bool,
}

/// newResizer creates a new resizer.
pub(crate) fn new_resizer(
    table_width: usize,
    table_height: usize,
    headers: &[String],
    rows: &[Vec<String>],
) -> Resizer {
    let mut r = Resizer {
        table_width,
        table_height,
        headers: headers.to_vec(),
        all_rows: Vec::new(),
        row_heights: Vec::new(),
        columns: Vec::new(),
        wrap: true,
        border_column: true,
        y_paddings: Vec::new(),
        y_offset: 0,
        use_manual_height: false,
        border_top: true,
        border_bottom: true,
        border_left: true,
        border_right: true,
        border_header: true,
        border_row: false,
    };

    if !headers.is_empty() {
        r.all_rows.push(headers.to_vec());
    }
    r.all_rows.extend(rows.iter().cloned());

    for row in &r.all_rows {
        for (i, cell) in row.iter().enumerate() {
            let cell_len = size::width(cell);
            if r.columns.len() <= i {
                r.columns.push(ResizerColumn {
                    index: i,
                    min: cell_len,
                    max: cell_len,
                    median: cell_len,
                    rows: Vec::new(),
                    x_padding: 0,
                    fixed_width: 0,
                });
                continue;
            }
            r.columns[i].rows.push(row.clone());
            r.columns[i].min = min(r.columns[i].min, cell_len);
            r.columns[i].max = max(r.columns[i].max, cell_len);
        }
    }
    for j in 0..r.columns.len() {
        let mut widths: Vec<usize> = Vec::with_capacity(r.columns[j].rows.len());
        for row in &r.columns[j].rows {
            widths.push(size::width(&row[j]));
        }
        r.columns[j].median = median(&mut widths);
    }

    r
}

impl Resizer {
    /// optimizedWidths returns the optimized column widths and row heights.
    pub fn optimized_widths(&mut self) -> (Vec<usize>, Vec<usize>) {
        if self.max_total() <= self.table_width {
            let col_widths = self.expand_table_width();
            return (col_widths, self.row_heights.clone());
        }
        let col_widths = self.shrink_table_width();
        (col_widths, self.row_heights.clone())
    }

    /// detectTableWidth detects the table width.
    pub fn detect_table_width(&self) -> usize {
        self.max_char_count() + self.total_horizontal_padding() + self.total_horizontal_border()
    }

    /// expandTableWidth expands the table width.
    fn expand_table_width(&mut self) -> Vec<usize> {
        let mut col_widths = self.max_column_widths();

        loop {
            let total_width = sum(&col_widths) + self.total_horizontal_border();
            if total_width >= self.table_width {
                break;
            }

            let mut shorter_column_index = 0usize;
            let mut shorter_column_width = usize::MAX;

            for (j, width) in col_widths.iter().enumerate() {
                if *width == self.columns[j].fixed_width {
                    continue;
                }
                if *width < shorter_column_width {
                    shorter_column_width = *width;
                    shorter_column_index = j;
                }
            }

            col_widths[shorter_column_index] += 1;
        }

        self.expand_row_heights(&col_widths);

        col_widths
    }

    /// shrinkTableWidth shrinks the table width.
    fn shrink_table_width(&mut self) -> Vec<usize> {
        let mut col_widths = self.max_column_widths();

        // Cut width of columns that are way too big.
        let shrink_biggest_columns = |col_widths: &mut Vec<usize>, very_big_only: bool| {
            loop {
                let total_width = sum(col_widths) + self.total_horizontal_border();
                if total_width <= self.table_width {
                    break;
                }

                let mut big_column_index: isize = -1;
                let mut big_column_width: isize = -1;

                for (j, width) in col_widths.iter().enumerate() {
                    if *width == self.columns[j].fixed_width {
                        continue;
                    }
                    if very_big_only {
                        if *width >= (self.table_width / 2) && *width as isize > big_column_width {
                            big_column_width = *width as isize;
                            big_column_index = j as isize;
                        }
                    } else if *width as isize > big_column_width {
                        big_column_width = *width as isize;
                        big_column_index = j as isize;
                    }
                }

                if big_column_index < 0 || col_widths[big_column_index as usize] == 0 {
                    break;
                }
                col_widths[big_column_index as usize] -= 1;
            }
        };

        // Cut width of columns that differ the most from the median.
        let shrink_to_median = |col_widths: &mut Vec<usize>| {
            loop {
                let total_width = sum(col_widths) + self.total_horizontal_border();
                if total_width <= self.table_width {
                    break;
                }

                let mut biggest_diff_to_median: isize = -1;
                let mut biggest_diff_to_median_index: isize = -1;

                for (j, width) in col_widths.iter().enumerate() {
                    if *width == self.columns[j].fixed_width {
                        continue;
                    }
                    let diff_to_median = *width as isize - self.columns[j].median as isize;
                    if diff_to_median > 0 && diff_to_median > biggest_diff_to_median {
                        biggest_diff_to_median = diff_to_median;
                        biggest_diff_to_median_index = j as isize;
                    }
                }

                if biggest_diff_to_median_index <= 0
                    || col_widths[biggest_diff_to_median_index as usize] == 0
                {
                    break;
                }
                col_widths[biggest_diff_to_median_index as usize] -= 1;
            }
        };

        shrink_biggest_columns(&mut col_widths, true);
        shrink_to_median(&mut col_widths);
        shrink_biggest_columns(&mut col_widths, false);

        self.expand_row_heights(&col_widths);

        col_widths
    }

    /// expandRowHeights expands the row heights.
    fn expand_row_heights(&mut self, col_widths: &[usize]) {
        self.row_heights = self.default_row_heights();
        if !self.wrap {
            return;
        }
        let has_headers = !self.headers.is_empty();

        for (i, row) in self.all_rows.clone().iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                // Headers always have a height of 1 (+ padding), even when wrap
                // is enabled.
                if has_headers && i == 0 {
                    self.row_heights[i] = 1 + self.y_padding_for_cell(i, j);
                    continue;
                }
                let width = col_widths[j].saturating_sub(self.x_padding_for_col(j));
                let height = self.detect_content_height(cell, width) + self.y_padding_for_cell(i, j);
                self.row_heights[i] = max(self.row_heights[i], height);
            }
        }
    }

    /// defaultRowHeights returns the default row heights.
    fn default_row_heights(&self) -> Vec<usize> {
        let mut row_heights = vec![1usize; self.all_rows.len()];
        for (i, h) in row_heights.iter_mut().enumerate() {
            if i < self.row_heights.len() {
                *h = max(*h, self.row_heights[i]);
            }
            *h = max(*h, 1);
        }
        row_heights
    }

    /// maxColumnWidths returns the maximum column widths.
    fn max_column_widths(&self) -> Vec<usize> {
        let mut maxes = Vec::with_capacity(self.columns.len());
        for col in &self.columns {
            if col.fixed_width > 0 {
                maxes.push(col.fixed_width);
            } else {
                maxes.push(col.max + self.x_padding_for_col(col.index));
            }
        }
        maxes
    }

    /// columnCount returns the column count.
    fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// maxCharCount returns the maximum character count.
    fn max_char_count(&self) -> usize {
        let mut count = 0usize;
        for col in &self.columns {
            if col.fixed_width > 0 {
                count += col.fixed_width - self.x_padding_for_col(col.index);
            } else {
                count += col.max;
            }
        }
        count
    }

    /// maxTotal returns the maximum total width.
    fn max_total(&self) -> usize {
        let mut max_total = 0usize;
        for (j, column) in self.columns.iter().enumerate() {
            if column.fixed_width > 0 {
                max_total += column.fixed_width;
            } else {
                max_total += column.max + self.x_padding_for_col(j);
            }
        }
        max_total
    }

    /// totalHorizontalPadding returns the total padding.
    fn total_horizontal_padding(&self) -> usize {
        self.columns.iter().map(|c| c.x_padding).sum()
    }

    /// xPaddingForCol returns the horizontal padding for a column.
    fn x_padding_for_col(&self, j: usize) -> usize {
        self.columns.get(j).map_or(0, |c| c.x_padding)
    }

    /// yPaddingForCell returns the vertical padding for a cell.
    fn y_padding_for_cell(&self, i: usize, j: usize) -> usize {
        self.y_paddings
            .get(i)
            .and_then(|row| row.get(j))
            .copied()
            .unwrap_or(0)
    }

    /// totalHorizontalBorder returns the total border.
    fn total_horizontal_border(&self) -> usize {
        btoi(self.border_left)
            + btoi(self.border_right)
            + self.column_count().saturating_sub(1) * btoi(self.border_column)
    }

    /// detectContentHeight detects the content height.
    fn detect_content_height(&self, content: &str, width: usize) -> usize {
        if width == 0 {
            return 1;
        }
        let content = content.replace("\r\n", "\n");
        let mut height = 0usize;
        for line in content.split('\n') {
            height += crate::ansi::wrap(line, width, "").matches('\n').count() + 1;
        }
        height
    }

    /// visibleRowIndexes calculates the indexes of the first and last visible
    /// rows according to the current yOffset and tableHeight.
    pub fn visible_row_indexes(&self) -> (usize, isize, usize) {
        if !self.use_manual_height {
            return (0, -2, 0);
        }

        let has_headers = !self.headers.is_empty();
        let last_index = (self.all_rows.len() - 1) as isize - btoi(has_headers) as isize;

        // Account for fixed elements (top/bottom borders, headers with their border).
        let mut available = self.table_height as isize
            - btoi(self.border_top) as isize
            - btoi(self.border_bottom) as isize
            - bton(has_headers, self.row_heights[0]) as isize
            - btoi(has_headers && self.border_header) as isize;

        // The first row we add does not need a row border.
        available += btoi(self.border_row) as isize;

        // Start from the offset with no visible rows.
        let mut first_visible_row_index = self.y_offset as isize;
        let mut last_visible_row_index = first_visible_row_index - 1;

        // First add rows at the bottom until we reach the available height, or
        // the last row.
        while available > 0 && last_visible_row_index < last_index {
            let row = self.row_heights[(last_visible_row_index + 1 + btoi(has_headers) as isize) as usize]
                + btoi(self.border_row);
            let overflow = if last_visible_row_index + 1 < last_index {
                1 + btoi(self.border_row)
                    + self.y_padding_for_cell((last_visible_row_index + 2 + btoi(has_headers) as isize) as usize, 0)
            } else {
                0
            };

            if (available - row as isize - overflow as isize) < 0 {
                break;
            }

            last_visible_row_index += 1;
            available -= row as isize;
        }

        if last_visible_row_index == last_index {
            // Then add rows at the top until we reach the available height, or
            // the first row.
            while available > 0 && first_visible_row_index > 0 {
                let row =
                    self.row_heights[(first_visible_row_index - 1 + btoi(has_headers) as isize) as usize]
                        + btoi(self.border_row);

                if (available - row as isize) < 0 {
                    break;
                }

                first_visible_row_index -= 1;
                available -= row as isize;
            }
        }

        if last_visible_row_index >= last_index {
            return (first_visible_row_index as usize, -2, 0);
        }

        let overflow = 1 + self.y_padding_for_cell(
            (last_visible_row_index + 1 + btoi(has_headers) as isize) as usize,
            0,
        );

        (
            first_visible_row_index as usize,
            last_visible_row_index,
            overflow,
        )
    }
}

/// The entry point for table resizing used by `Table::string`.
pub(crate) fn resize(
    data: &dyn Data,
    headers: &[String],
    table_width: usize,
    table_height: usize,
    y_offset: usize,
    use_manual_height: bool,
    wrap: bool,
    border_top: bool,
    border_bottom: bool,
    border_left: bool,
    border_right: bool,
    border_header: bool,
    border_row: bool,
    border_column: bool,
    style_func: &dyn Fn(isize, usize) -> crate::style::Style,
) -> (Vec<usize>, Vec<usize>, usize, isize, usize) {
    let has_headers = !headers.is_empty();
    let rows = data_to_matrix(data);
    let mut r = new_resizer(table_width, table_height, headers, &rows);
    r.wrap = wrap;
    r.border_column = border_column;
    r.y_paddings = vec![Vec::new(); r.all_rows.len()];

    r.y_offset = y_offset;
    r.use_manual_height = use_manual_height;
    r.border_top = border_top;
    r.border_bottom = border_bottom;
    r.border_left = border_left;
    r.border_right = border_right;
    r.border_header = border_header;
    r.border_row = border_row;

    let mut all_rows: Vec<Vec<String>> = Vec::new();
    if has_headers {
        all_rows.push(headers.to_vec());
    }
    all_rows.extend(rows.iter().cloned());

    r.row_heights = r.default_row_heights();

    for (i, row) in all_rows.iter().enumerate() {
        r.y_paddings[i] = vec![0usize; row.len()];

        for (j, _) in row.iter().enumerate() {
            let row_index = if has_headers { i as isize - 1 } else { i as isize };
            let style = style_func(row_index, j);

            r.columns[j].x_padding = max(r.columns[j].x_padding, style.get_horizontal_frame_size());
            r.columns[j].fixed_width = max(r.columns[j].fixed_width, style.get_width());

            r.row_heights[i] = max(r.row_heights[i], style.get_height());
            r.y_paddings[i][j] = style.get_vertical_frame_size();
        }
    }

    // A table width wasn't specified. In this case, detect according to content
    // width.
    if r.table_width <= 0 {
        r.table_width = r.detect_table_width();
    }

    let (widths, heights) = r.optimized_widths();
    let (first_visible, last_visible, overflow) = r.visible_row_indexes();

    (widths, heights, first_visible, last_visible, overflow)
}
