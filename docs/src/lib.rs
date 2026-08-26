//! Cleanroom user documentation source for profile-aware terminal styling.
//!
//! <user-docs>
//! `Style::render_with_profile` renders a style with an explicitly selected
//! terminal color capability. Use `Profile::TrueColor` for full color,
//! `Profile::Ansi256` or `Profile::Ansi` for reduced palettes,
//! `Profile::Ascii` for decoration without color, and `Profile::NoTty` for
//! plain text without ANSI control sequences.
//!
//! The operation is deterministic and does not inspect process environment:
//!
//! ```
//! use rusty_lipgloss::{Profile, Style};
//!
//! let rendered = Style::new()
//!     .bold(true)
//!     .foreground("#ff0000")
//!     .render_with_profile("hello", Profile::Ansi256);
//! assert_eq!(rendered, "\x1b[1;38;5;196mhello\x1b[m");
//! ```
//! </user-docs>
//!
//! Internal maintainer note: this source is the documentation-owned projection
//! for the generated ticket target. Keep examples synchronized with the public
//! facade and `Style::render_with_profile` contract.
