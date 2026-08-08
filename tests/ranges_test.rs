//! Cleanroom Rust port of upstream Go test file: `ranges_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use charming_lipgloss::ranges::{new_range, style_ranges};
use charming_lipgloss::style::Style;

#[test]
fn test_style_ranges_basic() {
    let s = "The quick brown fox";
    let bold = Style::new().bold(true);
    let ranges = vec![new_range(4, 9, bold)];
    let out = style_ranges(s, &ranges);
    assert_eq!(out, "The \x1b[1mquick\x1b[m brown fox");
}

#[test]
fn test_style_ranges_no_ranges() {
    let s = "hello";
    assert_eq!(style_ranges(s, &[]), "hello");
}

#[test]
fn test_style_ranges_multiple() {
    let s = "abcdef";
    let fg = Style::new().foreground("#FF0000");
    let ranges = vec![new_range(0, 2, fg.clone()), new_range(4, 6, fg)];
    let out = style_ranges(s, &ranges);
    assert_eq!(
        out,
        "\x1b[38;2;255;0;0mab\x1b[mcd\x1b[38;2;255;0;0mef\x1b[m"
    );
}
