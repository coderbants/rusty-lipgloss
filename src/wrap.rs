//! Cleanroom Rust port of upstream Go source file: `wrap.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Word wrapping that preserves ANSI styles and hyperlinks.
//! </public-docs>

/// <upstream-comment>Wrap wraps the given string to the given width, preserving ANSI styles and links.</upstream-comment>
pub fn wrap(s: &str, width: usize, breakpoints: &str) -> String {
    crate::ansi::wrap(s, width, breakpoints)
}

/// <upstream-comment>WrapWriter is a writer that writes to a buffer and keeps track of the
/// current pen style and link state for the purpose of wrapping with newlines.
///
/// When it encounters a newline, it resets the style and link, writes the
/// newline, and then reapplies the style and link to the next line.</upstream-comment>
pub type WrapWriter<'a> = crate::ansi::WrapWriter<'a>;
