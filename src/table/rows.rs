//! Cleanroom Rust port of upstream Go source file: `table/rows.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Table data models: the `Data` interface, `StringData`, and `Filter`.
//! </public-docs>

use std::cmp::max;

/// Data is the interface that wraps the basic methods of a table model.
pub trait Data {
    /// At returns the contents of the cell at the given index.
    fn at(&self, row: usize, cell: usize) -> String;
    /// Rows returns the number of rows in the table.
    fn rows(&self) -> usize;
    /// Columns returns the number of columns in the table.
    fn columns(&self) -> usize;
    /// Provides access to the underlying concrete type for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// StringData is a string-based implementation of the Data interface.
#[derive(Debug, Clone, Default)]
pub struct StringData {
    rows: Vec<Vec<String>>,
    columns: usize,
}

impl StringData {
    /// Creates a new StringData with the given rows.
    pub fn new(rows: &[&[&str]]) -> StringData {
        let mut m = StringData {
            rows: Vec::new(),
            columns: 0,
        };
        for row in rows {
            m.append(row);
        }
        m
    }

    /// Appends the given row to the table.
    pub fn append(&mut self, row: &[&str]) {
        self.columns = max(self.columns, row.len());
        self.rows
            .push(row.iter().map(|s| s.to_string()).collect());
    }

    /// Item appends the given row to the table.
    pub fn item(&mut self, row: &[&str]) {
        self.append(row);
    }
}

impl Data for StringData {
    /// At returns the contents of the cell at the given index.
    fn at(&self, row: usize, cell: usize) -> String {
        self.rows
            .get(row)
            .and_then(|r| r.get(cell))
            .cloned()
            .unwrap_or_default()
    }

    /// Columns returns the number of columns in the table.
    fn columns(&self) -> usize {
        self.columns
    }

    /// Rows returns the number of rows in the table.
    fn rows(&self) -> usize {
        self.rows.len()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Filter applies a filter on some data.
pub struct Filter {
    data: Box<dyn Data>,
    filter: Option<Box<dyn Fn(usize) -> bool>>,
}

/// NewFilter initializes a new Filter.
pub fn new_filter(data: Box<dyn Data>) -> Filter {
    Filter {
        data,
        filter: None,
    }
}

impl Filter {
    /// Filter applies the given filter function to the data.
    pub fn filter(mut self, f: Box<dyn Fn(usize) -> bool>) -> Filter {
        self.filter = Some(f);
        self
    }

    fn passes(&self, i: usize) -> bool {
        match &self.filter {
            Some(f) => f(i),
            None => true,
        }
    }
}

impl Data for Filter {
    /// At returns the row at the given index.
    fn at(&self, row: usize, cell: usize) -> String {
        let mut j = 0usize;
        for i in 0..self.data.rows() {
            if self.passes(i) {
                if j == row {
                    return self.data.at(i, cell);
                }
                j += 1;
            }
        }
        String::new()
    }

    /// Columns returns the number of columns in the table.
    fn columns(&self) -> usize {
        self.data.columns()
    }

    /// Rows returns the number of rows in the table.
    fn rows(&self) -> usize {
        let mut j = 0usize;
        for i in 0..self.data.rows() {
            if self.passes(i) {
                j += 1;
            }
        }
        j
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// DataToMatrix is a helper function that converts an object that implements
/// the Data interface into a matrix.
pub fn data_to_matrix(data: &dyn Data) -> Vec<Vec<String>> {
    let num_rows = data.rows();
    let num_cols = data.columns();
    let mut rows = Vec::with_capacity(num_rows);
    for i in 0..num_rows {
        let mut row = Vec::with_capacity(num_cols);
        for j in 0..num_cols {
            row.push(data.at(i, j));
        }
        rows.push(row);
    }
    rows
}
