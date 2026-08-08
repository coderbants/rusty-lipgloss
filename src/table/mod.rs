//! Cleanroom Rust port of upstream Go source file: `table/table.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! The `table` submodule mirrors upstream `charmbracelet/lipgloss/table`: a
//! styled table renderer with automatic column width and row height sizing.
//! </public-docs>

mod resizing;
pub mod rows;
pub mod table;
mod util;

pub use rows::{new_filter, data_to_matrix, Data, Filter, StringData};
pub use table::{default_styles, Table, StyleFunc, HEADER_ROW};

/// Returns a new Table.
pub fn new() -> Table {
    Table::new()
}
