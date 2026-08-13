//! Cleanroom Rust port of upstream Go test file: `runes_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::runes::style_runes;
use rusty_lipgloss::style::Style;

#[test]
fn test_style_runes_matched_unmatched() {
    let matched = Style::new().bold(true);
    let unmatched = Style::new().faint(true);
    let out = style_runes("abcd", &[0, 2], &matched, &unmatched);
    assert_eq!(
        out,
        "\x1b[1ma\x1b[m\x1b[2mb\x1b[m\x1b[1mc\x1b[m\x1b[2md\x1b[m"
    );
}

#[test]
fn test_style_runes_grouped() {
    let matched = Style::new().reverse(true);
    let unmatched = Style::new();
    let out = style_runes("abcde", &[0, 1], &matched, &unmatched);
    assert_eq!(out, "\x1b[7mab\x1b[mcde");
}

#[test]
fn test_style_runes_out_of_bounds_ignored() {
    let matched = Style::new().bold(true);
    let unmatched = Style::new();
    let out = style_runes("abc", &[10], &matched, &unmatched);
    assert_eq!(out, "abc");
}
