//! Cleanroom Rust port of upstream Go source file: `ranges.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Functions for applying styling to ranges within a string.
//! </public-docs>

use crate::ansi;
use crate::style::Style;

/// <upstream-comment>Range is a range of text and associated styling to be used with
/// [StyleRanges].</upstream-comment>
#[derive(Debug, Clone)]
pub struct Range {
    /// The start position of the range (in cells).
    pub start: usize,
    /// The end position of the range (in cells).
    pub end: usize,
    /// The style to apply to the range.
    pub style: Style,
}

/// <upstream-comment>NewRange returns a range and style that can be used with [StyleRanges].</upstream-comment>
pub fn new_range(start: usize, end: usize, style: Style) -> Range {
    Range { start, end, style }
}

/// <upstream-comment>StyleRanges applying styling to ranges in a string. Existing styles will be
/// taken into account. Ranges should not overlap.</upstream-comment>
pub fn style_ranges(s: &str, ranges: &[Range]) -> String {
    if ranges.is_empty() {
        return s.to_string();
    }

    let mut out = String::new();
    let mut last_idx = 0usize;
    let stripped = ansi::strip(s);

    // Use Truncate and TruncateLeft to style matched ranges without losing the
    // original option style.
    for rng in ranges {
        // Add the text before this match.
        if rng.start > last_idx {
            out.push_str(&ansi::cut(s, last_idx, rng.start));
        }
        // Add the matched range with its highlight.
        let matched = ansi::cut(&stripped, rng.start, rng.end);
        out.push_str(&rng.style.render(&matched));
        last_idx = rng.end;
    }

    // Add any remaining text after the last match.
    let total = crate::size::width(&stripped);
    out.push_str(&ansi::truncate_left(s, total.saturating_sub(last_idx), ""));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_ranges() {
        let s = "hello world";
        let style = Style::new().bold(true);
        let ranges = vec![new_range(6, 11, style)];
        let out = style_ranges(s, &ranges);
        assert_eq!(out, "hello \x1b[1mworld\x1b[m");
    }
}
