//! Cleanroom Rust port of upstream Go test file: `whitespace_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::whitespace::{with_whitespace_chars, Whitespace};

/// Ported from upstream `TestWhitespaceRenderNormal` and the tab/zero-width
/// render cases (the Go tests guard against an infinite loop; the Rust port
/// must simply complete and produce output of the requested width).
#[test]
fn test_whitespace_render() {
    // Normal behaviour: render exactly `width` chars.
    let ws = Whitespace::new(&[with_whitespace_chars("*")]);
    assert_eq!(ws.render(5), "*****");
    assert_eq!(ws.render(0), "");
    assert_eq!(ws.render(3), "***");
}

/// Ported from upstream `TestWhitespaceRenderWithTab` (issue #108): a tab
/// whitespace char must not loop indefinitely.
#[test]
fn test_whitespace_render_with_tab() {
    let ws = Whitespace::new(&[with_whitespace_chars("\t")]);
    let out = ws.render(10);
    assert!(!out.is_empty());
}

/// Ported from upstream `TestWhitespaceRenderWithZeroWidthChar`.
#[test]
fn test_whitespace_render_with_zero_width_char() {
    let ws = Whitespace::new(&[with_whitespace_chars("\u{200d}")]); // zero-width joiner
    let out = ws.render(5);
    assert!(!out.is_empty());
}
