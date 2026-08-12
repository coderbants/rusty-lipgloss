//! Cleanroom Rust port of upstream Go source file: `table/table.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! A styled table renderer for terminals, mirroring upstream
//! `charmbracelet/lipgloss/table`.
//! </public-docs>

use std::cmp::min;

use super::resizing;
use super::rows::{Data, StringData};
use crate::align::TOP;
use crate::ansi;
use crate::border::Border;
use crate::join::join_horizontal;
use crate::style::Style;

/// HeaderRow denotes the header's row index used when rendering headers. Use
/// this value when looking to customize header styles in StyleFunc.
pub const HEADER_ROW: isize = -1;

/// StyleFunc is the style function that determines the style of a Cell.
///
/// It takes the row and column of the cell as an input and determines the
/// lipgloss Style to use for that cell position.
pub type StyleFunc = Box<dyn Fn(isize, usize) -> Style>;

/// DefaultStyles is a StyleFunc that returns a new Style with no attributes.
pub fn default_styles(_: isize, _: usize) -> Style {
    Style::new()
}

/// Table is a type for rendering tables.
pub struct Table {
    base_style: Style,
    style_func: Option<StyleFunc>,
    border: Border,

    border_top: bool,
    border_bottom: bool,
    border_left: bool,
    border_right: bool,
    border_header: bool,
    border_column: bool,
    border_row: bool,

    border_style: Style,
    headers: Vec<String>,
    data: Box<dyn Data>,

    width: usize,
    height: usize,
    use_manual_height: bool,
    y_offset: usize,
    wrap: bool,

    first_visible_row_index: usize,
    last_visible_row_index: isize,
    overflow_height: usize,
}

/// <upstream-comment>New returns a new Table that can be modified through different
/// attributes.
///
/// By default, a table has normal border, no styling, and no rows.</upstream-comment>
impl Default for Table {
    fn default() -> Self {
        Table {
            style_func: Some(Box::new(default_styles)),
            border: Border::normal(),
            border_bottom: true,
            border_column: true,
            border_header: true,
            border_left: true,
            border_right: true,
            border_top: true,
            wrap: true,
            data: Box::new(StringData::new(&[])),
            base_style: Style::new(),
            border_style: Style::new(),
            border_row: false,
            headers: Vec::new(),
            width: 0,
            height: 0,
            use_manual_height: false,
            y_offset: 0,
            first_visible_row_index: 0,
            last_visible_row_index: 0,
            overflow_height: 0,
        }
    }
}

impl Table {
    /// Returns a new empty Table.
    pub fn new() -> Table {
        Table::default()
    }

    /// ClearRows clears the table rows.
    pub fn clear_rows(mut self) -> Table {
        self.data = Box::new(StringData::new(&[]));
        self
    }

    /// BaseStyle sets the base style for the whole table. If you need to set a
    /// background color for the whole table, use this.
    pub fn base_style(mut self, base_style: Style) -> Table {
        self.base_style = base_style.clone();
        self.border_style = self.border_style.clone().inherit(&base_style);
        self
    }

    /// StyleFunc sets the style for a cell based on its position (row, column).
    pub fn style_func(mut self, style: StyleFunc) -> Table {
        self.style_func = Some(style);
        self
    }

    /// style returns the style for a cell based on its position (row, column).
    fn style(&self, row: isize, col: usize) -> Style {
        match &self.style_func {
            Some(f) => f(row, col).inherit(&self.base_style),
            None => self.base_style.clone(),
        }
    }

    /// Data sets the table data.
    pub fn data(mut self, data: Box<dyn Data>) -> Table {
        self.data = data;
        self
    }

    /// Rows appends rows to the table data.
    pub fn rows(mut self, rows: &[&[&str]]) -> Table {
        if let Some(sd) = self.data.as_any_mut().downcast_mut::<StringData>() {
            for row in rows {
                sd.append(row);
            }
        }
        self
    }

    /// Row appends a row to the table data.
    pub fn row(mut self, row: &[&str]) -> Table {
        if let Some(sd) = self.data.as_any_mut().downcast_mut::<StringData>() {
            sd.append(row);
        }
        self
    }

    /// Headers sets the table headers.
    pub fn headers(mut self, headers: &[&str]) -> Table {
        self.headers = headers.iter().map(|h| h.to_string()).collect();
        self
    }

    /// GetHeaders returns the table headers.
    pub fn get_headers(&self) -> &[String] {
        &self.headers
    }

    /// Border sets the table border.
    pub fn border(mut self, border: Border) -> Table {
        self.border = border;
        self
    }

    /// BorderTop sets the top border.
    pub fn border_top(mut self, v: bool) -> Table {
        self.border_top = v;
        self
    }

    /// BorderBottom sets the bottom border.
    pub fn border_bottom(mut self, v: bool) -> Table {
        self.border_bottom = v;
        self
    }

    /// BorderLeft sets the left border.
    pub fn border_left(mut self, v: bool) -> Table {
        self.border_left = v;
        self
    }

    /// BorderRight sets the right border.
    pub fn border_right(mut self, v: bool) -> Table {
        self.border_right = v;
        self
    }

    /// BorderHeader sets the header separator border.
    pub fn border_header(mut self, v: bool) -> Table {
        self.border_header = v;
        self
    }

    /// BorderColumn sets the column border separator.
    pub fn border_column(mut self, v: bool) -> Table {
        self.border_column = v;
        self
    }

    /// BorderRow sets the row border separator.
    pub fn border_row(mut self, v: bool) -> Table {
        self.border_row = v;
        self
    }

    /// BorderStyle sets the style for the table border.
    pub fn border_style(mut self, style: Style) -> Table {
        self.border_style = style.clone().inherit(&self.base_style);
        self
    }

    /// GetBorderTop gets the top border.
    pub fn get_border_top(&self) -> bool {
        self.border_top
    }

    /// GetBorderBottom gets the bottom border.
    pub fn get_border_bottom(&self) -> bool {
        self.border_bottom
    }

    /// GetBorderLeft gets the left border.
    pub fn get_border_left(&self) -> bool {
        self.border_left
    }

    /// GetBorderRight gets the right border.
    pub fn get_border_right(&self) -> bool {
        self.border_right
    }

    /// GetBorderHeader gets the header separator border.
    pub fn get_border_header(&self) -> bool {
        self.border_header
    }

    /// GetBorderColumn gets the column border separator.
    pub fn get_border_column(&self) -> bool {
        self.border_column
    }

    /// GetBorderRow gets the row border separator.
    pub fn get_border_row(&self) -> bool {
        self.border_row
    }

    /// Width sets the table width. This auto-sizes the columns to fit the width
    /// by either expanding or contracting the widths of each column as a best
    /// effort approach.
    pub fn width(mut self, w: usize) -> Table {
        self.width = w;
        self
    }

    /// Height sets the table height.
    pub fn height(mut self, h: usize) -> Table {
        self.height = h;
        self.use_manual_height = true;
        self
    }

    /// GetHeight returns the height of the table.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// YOffset sets the table rendering offset.
    pub fn y_offset(mut self, o: usize) -> Table {
        self.y_offset = o;
        self
    }

    /// GetYOffset returns the table rendering offset.
    pub fn get_y_offset(&self) -> usize {
        self.y_offset
    }

    /// FirstVisibleRowIndex returns the index of the first visible row.
    pub fn first_visible_row_index(&self) -> usize {
        self.first_visible_row_index
    }

    /// LastVisibleRowIndex returns the index of the last visible row.
    pub fn last_visible_row_index(&self) -> isize {
        self.last_visible_row_index
    }

    /// VisibleRows returns the number of visible rows in the table.
    pub fn visible_rows(&self) -> usize {
        if self.last_visible_row_index == -2 {
            return self.data.rows() - self.first_visible_row_index;
        }
        (self.last_visible_row_index - self.first_visible_row_index as isize + 1) as usize
    }

    /// Wrap dictates whether or not the table content should wrap.
    pub fn wrap(mut self, w: bool) -> Table {
        self.wrap = w;
        self
    }

    /// String returns the table as a string.
    pub fn string(&mut self) -> String {
        let has_headers = !self.headers.is_empty();
        let has_rows = self.data.rows() > 0;

        if !has_headers && !has_rows {
            return String::new();
        }

        // Add empty cells to the headers, until it's the same length as the
        // longest row (only if there are headers in the first place).
        let mut headers = self.headers.clone();
        if has_headers {
            for _ in headers.len()..self.data.columns() {
                headers.push(String::new());
            }
        }

        // Do all the sizing calculations for width and height.
        let (widths, heights, first_visible, last_visible, overflow) =
            self.compute_layout(&headers);
        let _ = overflow;

        let mut sb = String::new();

        if self.border_top {
            sb.push_str(&self.construct_top_border(&widths));
            sb.push('\n');
        }

        if has_headers {
            sb.push_str(&self.construct_headers(&headers, &widths, &heights));
        }

        let bottom = if self.border_bottom {
            self.construct_bottom_border(&widths)
        } else {
            String::new()
        };

        // If there are no data rows render nothing.
        if self.data.rows() > 0 {
            let mut r = first_visible;
            while r < self.data.rows() {
                if last_visible != -2 && r as isize > last_visible {
                    break;
                }
                sb.push_str(&self.construct_row(r, false, &headers, &widths, &heights));
                r += 1;
            }

            // Add an overflow row to show that there are more rows not being
            // rendered.
            if last_visible != -2 {
                sb.push_str(&self.construct_row(
                    (last_visible + 1) as usize,
                    true,
                    &headers,
                    &widths,
                    &heights,
                ));
            }
        }

        sb.push_str(&bottom);

        let height = self.compute_height(&headers, &heights, &data_row_count(&*self.data));
        Style::new()
            .max_height(min(self.height, height))
            .max_width(self.width)
            .render(sb.trim_end_matches('\n'))
    }

    /// Render returns the table as a string.
    pub fn render(&mut self) -> String {
        self.string()
    }

    /// computeHeight computes the height of the table in its current configuration.
    fn compute_height(&self, _headers: &[String], heights: &[usize], row_count: &usize) -> usize {
        let has_headers = !self.headers.is_empty();
        super::util::sum(heights).saturating_sub(1)
            + super::util::btoi(has_headers)
            + super::util::btoi(self.border_top)
            + super::util::btoi(self.border_bottom)
            + super::util::btoi(self.border_header)
            + *row_count * super::util::btoi(self.border_row)
    }

    fn construct_top_border(&self, widths: &[usize]) -> String {
        let mut s = String::new();
        if self.border_left {
            s.push_str(&self.border_style.render(&self.border.top_left));
        }
        for i in 0..widths.len() {
            s.push_str(&self.border_style.render(&self.border.top.repeat(widths[i])));
            if i < widths.len() - 1 && self.border_column {
                s.push_str(&self.border_style.render(&self.border.middle_top));
            }
        }
        if self.border_right {
            s.push_str(&self.border_style.render(&self.border.top_right));
        }
        s
    }

    fn construct_bottom_border(&self, widths: &[usize]) -> String {
        let mut s = String::new();
        if self.border_left {
            s.push_str(&self.border_style.render(&self.border.bottom_left));
        }
        for i in 0..widths.len() {
            s.push_str(
                &self
                    .border_style
                    .render(&self.border.bottom.repeat(widths[i])),
            );
            if i < widths.len() - 1 && self.border_column {
                s.push_str(&self.border_style.render(&self.border.middle_bottom));
            }
        }
        if self.border_right {
            s.push_str(&self.border_style.render(&self.border.bottom_right));
        }
        s
    }

    fn construct_headers(&self, headers: &[String], widths: &[usize], heights: &[usize]) -> String {
        let mut s = String::new();
        let mut cells: Vec<String> = Vec::new();
        let height = heights[0];

        let left = format!("{}\n", self.border_style.render(&self.border.left)).repeat(height);
        if self.border_left {
            cells.push(left.clone());
        }

        for (j, header) in headers.iter().enumerate() {
            let cell_style = self.style(HEADER_ROW, j);

            // We always truncate headers.
            let header = self.truncate_cell(header, HEADER_ROW, j, widths, heights);

            cells.push(
                cell_style
                    .clone()
                    .height(height.saturating_sub(cell_style.get_vertical_margins()))
                    .width(widths[j].saturating_sub(cell_style.get_horizontal_margins()))
                    .render(&header),
            );

            if j < headers.len() - 1 && self.border_column {
                cells.push(left.clone());
            }
        }

        if self.border_right {
            let right =
                format!("{}\n", self.border_style.render(&self.border.right)).repeat(height);
            cells.push(right);
        }

        for cell in &mut cells {
            *cell = cell.trim_end_matches('\n').to_string();
        }

        let joined: Vec<&str> = cells.iter().map(|c| c.as_str()).collect();
        s.push_str(&join_horizontal(TOP, &joined));
        s.push('\n');

        if self.border_header {
            if self.border_left {
                s.push_str(&self.border_style.render(&self.border.middle_left));
            }
            for (i, width) in widths.iter().enumerate().take(headers.len()) {
                s.push_str(&self.border_style.render(&self.border.top.repeat(*width)));
                if i < headers.len() - 1 && self.border_column {
                    s.push_str(&self.border_style.render(&self.border.middle));
                }
            }
            if self.border_right {
                s.push_str(&self.border_style.render(&self.border.middle_right));
            }
            s.push('\n');
        }

        s
    }

    fn construct_row(
        &self,
        index: usize,
        is_overflow: bool,
        headers: &[String],
        widths: &[usize],
        heights: &[usize],
    ) -> String {
        let mut s = String::new();
        let mut cells: Vec<String> = Vec::new();

        let has_headers = !headers.is_empty();

        let height = if !is_overflow {
            heights[index + super::util::btoi(has_headers)]
        } else {
            self.overflow_height
        };

        let left = format!("{}\n", self.border_style.render(&self.border.left)).repeat(height);
        if self.border_left {
            cells.push(left.clone());
        }

        for c in 0..self.data.columns() {
            let mut cell = "…".to_string();
            if !is_overflow {
                cell = self.data.at(index, c);
            }

            let cell_style = self.style(index as isize, c);
            if !self.wrap {
                cell = self.truncate_cell(&cell, index as isize, c, widths, heights);
            }
            cells.push(
                cell_style
                    .clone()
                    // Account for the margins in the cell sizing.
                    .height(height.saturating_sub(cell_style.get_vertical_margins()))
                    .max_height(height)
                    .width(widths[c].saturating_sub(cell_style.get_horizontal_margins()))
                    .max_width(widths[c])
                    .render(&cell),
            );

            if c < self.data.columns() - 1 && self.border_column {
                cells.push(left.clone());
            }
        }

        if self.border_right {
            let right =
                format!("{}\n", self.border_style.render(&self.border.right)).repeat(height);
            cells.push(right);
        }

        for cell in &mut cells {
            *cell = cell.trim_end_matches('\n').to_string();
        }

        let joined: Vec<&str> = cells.iter().map(|c| c.as_str()).collect();
        s.push_str(&join_horizontal(TOP, &joined));
        s.push('\n');

        if self.border_row && !is_overflow && index < self.data.rows() - 1 {
            if self.border_left {
                s.push_str(&self.border_style.render(&self.border.middle_left));
            }
            for i in 0..widths.len() {
                s.push_str(
                    &self
                        .border_style
                        .render(&self.border.bottom.repeat(widths[i])),
                );
                if i < widths.len() - 1 && self.border_column {
                    s.push_str(&self.border_style.render(&self.border.middle));
                }
            }
            if self.border_right {
                s.push_str(&self.border_style.render(&self.border.middle_right));
            }
            s.push('\n');
        }

        s
    }

    fn truncate_cell(
        &self,
        cell: &str,
        row_index: isize,
        col_index: usize,
        widths: &[usize],
        heights: &[usize],
    ) -> String {
        let has_headers = !self.headers.is_empty();
        let mut height = heights[(row_index + super::util::btoi(has_headers) as isize) as usize];
        let cell_width = widths[col_index];
        let cell_style = self.style(row_index, col_index);

        // We always truncate headers to 1 line.
        if row_index == HEADER_ROW {
            height = 1;
        }

        let length = (cell_width * height)
            .saturating_sub(cell_style.get_horizontal_padding())
            .saturating_sub(cell_style.get_horizontal_margins());
        ansi::truncate(cell, length, "…")
    }

    /// Computes the table layout (column widths, row heights, visible rows).
    fn compute_layout(
        &mut self,
        headers: &[String],
    ) -> (Vec<usize>, Vec<usize>, usize, isize, usize) {
        let style_func: &dyn Fn(isize, usize) -> Style = match &self.style_func {
            Some(f) => &|row, col| f(row, col),
            None => &default_styles,
        };

        let data = &*self.data;

        let (widths, heights, first_visible, last_visible, overflow) = resizing::resize(
            data,
            headers,
            self.width,
            self.height,
            self.y_offset,
            self.use_manual_height,
            self.wrap,
            self.border_top,
            self.border_bottom,
            self.border_left,
            self.border_right,
            self.border_header,
            self.border_row,
            self.border_column,
            style_func,
        );

        self.first_visible_row_index = first_visible;
        self.last_visible_row_index = last_visible;
        self.overflow_height = overflow;

        (widths, heights, first_visible, last_visible, overflow)
    }
}

fn data_row_count(d: &dyn Data) -> usize {
    d.rows()
}
