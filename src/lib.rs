//! # Charming Lip Gloss
//!
//! A cleanroom Rust port of Charmbracelet's upstream Go `lipgloss` library (pinned to release `v2.0.5`).
//!
//! Provides style declarations, ANSI 16/256/TrueColor support, layout alignment, string joining, borders, and tables.

#![deny(unsafe_code)]

pub mod align;
pub mod border;
pub mod color;
pub mod join;
pub mod list;
pub mod size;
pub mod style;
pub mod table;
pub mod tree;
pub mod whitespace;

pub use align::Position;
pub use border::Border;
pub use color::{AdaptiveColor, Color, CompleteColor, TerminalColor};
pub use list::List;
pub use style::Style;
pub use table::Table;
pub use tree::Tree;


