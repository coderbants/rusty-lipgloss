//! Cleanroom Rust port of upstream Go source file: `lipgloss.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <user-docs>
//! A cleanroom Rust port of Charmbracelet's upstream Go `lipgloss` library
//! (pinned to release `v2.0.5`).
//!
//! Provides style declarations, ANSI 16/256/TrueColor support, layout alignment,
//! string joining, borders, tables, trees, lists, color blending, canvas
//! composition, terminal writers, and explicit profile-aware style materialization.
//! </user-docs>
//!
//! Internal maintainer note: this root module is the public facade. Keep new
//! public types re-exported here when downstream consumers should not depend on
//! defining submodules.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod align;
pub mod ansi;
pub mod blending;
pub mod border;
pub mod canvas;
pub mod color;
pub mod compat;
pub mod join;
pub mod layer;
pub mod list;
pub mod platform;
pub mod position;
pub mod query;
pub mod ranges;
pub mod runes;
pub mod size;
pub mod style;
pub mod table;
pub mod tree;
pub mod whitespace;
pub mod wrap;
pub mod writer;

pub use align::{Position, BOTTOM, CENTER, LEFT, RIGHT, TOP};
pub use border::Border;
/// The closed terminal color capability used by profile-aware rendering.
pub use color::{Color, NoColor, Profile};
pub use style::Style;

/// The `Align` type is an alias for `Position`.
pub type Align = Position;

/// <upstream-comment>NewStyle returns a new, empty Style.</upstream-comment>
pub fn new_style() -> Style {
    Style::new()
}
