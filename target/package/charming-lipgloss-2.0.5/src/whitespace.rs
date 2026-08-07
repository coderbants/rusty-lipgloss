//! Whitespace options and helper rendering functions matching upstream `lipgloss.Whitespace`.

/// Generates a whitespace string of specified length with given space character.
pub fn space(len: usize) -> String {
    " ".repeat(len)
}
