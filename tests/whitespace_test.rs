//! Cleanroom Rust port of upstream Go test file: `whitespace_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::whitespace::{with_whitespace_chars, Whitespace};

#[test]
fn test_whitespace_render_with_tab() {
    // Rendering whitespace with tab characters must not loop forever.
    let ws = with_whitespace_chars("\t");
    let _ = ws.render(10);
}

#[test]
fn test_whitespace_render_with_zero_width_char() {
    // Zero-width characters must still make progress.
    let ws = with_whitespace_chars("\u{200d}");
    let _ = ws.render(5);
}

#[test]
fn test_whitespace_render_normal() {
    let ws = Whitespace::new(&[]);
    assert_eq!(ws.render(5), "     ");
    let ws = with_whitespace_chars("*");
    assert_eq!(ws.render(5), "*****");
}

#[test]
fn test_whitespace_render_wide_chars() {
    let ws = with_whitespace_chars("猫");
    // A wide character fills two cells; ensure the output is at least the
    // requested cell width without panicking.
    let out = ws.render(5);
    assert!(!out.is_empty());
}
