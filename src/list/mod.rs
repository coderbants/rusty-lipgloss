//! Cleanroom Rust port of upstream Go source file: `list/list.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! The `list` submodule mirrors upstream `charmbracelet/lipgloss/list`: a list
//! component with enumerators, indenters, and item styling.
//! </public-docs>

pub mod enumerator;
#[allow(clippy::module_inception)] // deliberate mirror of upstream list/list.go layout
pub mod list;

pub use enumerator::{alphabet, arabic, asterisk, bullet, dash, roman, Enumerator, Indenter};
pub use list::List;

/// Returns a new list with the given items.
///
/// ```text
/// alphabet := list.New("A", "B", "C", "D", "E", "F", ...)
/// ```
pub fn new(items: &[&str]) -> List {
    let mut l = List::new();
    for item in items {
        l = l.item(item);
    }
    l
}
