//! Cleanroom Rust port of upstream Go source file: `table/table.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! The `table` submodule mirrors upstream `charmbracelet/lipgloss/table`: a
//! styled table renderer with automatic column width and row height sizing.
//! </public-docs>

mod resizing;
pub mod rows;
#[allow(clippy::module_inception)] // deliberate mirror of upstream table/table.go layout
pub mod table;
mod util;

pub use rows::{data_to_matrix, new_filter, Data, Filter, StringData};
pub use table::{default_styles, StyleFunc, Table, HEADER_ROW};

/// Returns a new Table.
pub fn new() -> Table {
    Table::new()
}
