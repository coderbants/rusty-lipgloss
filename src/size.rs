//! Cleanroom Rust port of upstream Go source file: `size.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! String metrics calculations based on terminal cell width. ANSI sequences are
//! ignored and characters wider than one cell (such as Chinese characters and
//! emojis) are appropriately measured.
//! </public-docs>

use unicode_width::UnicodeWidthChar;

/// <upstream-comment>Width returns the cell width of characters in the string. ANSI sequences are
/// ignored and characters wider than one cell (such as Chinese characters and
/// emojis) are appropriately measured.
///
/// You should use this instead of `len(string)` or `len([]rune(string))` as
/// neither will give you accurate results.</upstream-comment>
pub fn width(s: &str) -> usize {
    let s = crate::ansi::strip(s);
    s.lines()
        .map(|line| {
            // Sum per-character widths. This matches go-runewidth semantics:
            // every character is measured individually (e.g. Arabic lam-alef
            // sequences count as two cells, unlike UAX #11 ligature rules).
            line.chars()
                .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap_or(0)
}

/// <upstream-comment>Height returns height of a string in cells. This is done simply by
/// counting `\n` characters. If your output has `\r\n`, that sequence will be
/// replaced with a `\n` in `[Style.Render]`.</upstream-comment>
pub fn height(s: &str) -> usize {
    s.matches('\n').count() + 1
}

/// <upstream-comment>Size returns the width and height of the string in cells. ANSI sequences are
/// ignored and characters wider than one cell (such as Chinese characters and
/// emojis) are appropriately measured.</upstream-comment>
pub fn size(s: &str) -> (usize, usize) {
    (width(s), height(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width() {
        assert_eq!(width("hello"), 5);
        assert_eq!(width("你好"), 4);
        assert_eq!(width("\x1b[31mhello\x1b[m"), 5);
        assert_eq!(width("hello\nworld"), 5);
    }

    #[test]
    fn test_height() {
        assert_eq!(height("hello"), 1);
        assert_eq!(height("a\nb\nc"), 3);
        assert_eq!(height(""), 1);
    }
}
