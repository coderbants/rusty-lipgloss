//! Cleanroom Rust port of upstream Go test file: `wrap_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::wrap::wrap;

#[test]
fn test_wrap_basic() {
    let s = "The quick brown fox jumps over the lazy dog";
    let out = wrap(s, 10, "");
    for line in out.lines() {
        assert!(rusty_lipgloss::size::width(line) <= 10);
    }
}

#[test]
fn test_wrap_preserves_ansi() {
    let s = "\x1b[1mThe quick brown fox\x1b[m";
    let out = wrap(s, 10, "");
    assert!(out.contains("\x1b[1m"));
    assert!(out.contains("\x1b[m"));
}

#[test]
fn test_wrap_zero_width() {
    let s = "hello";
    assert_eq!(wrap(s, 0, ""), "hello");
}

#[test]
fn test_wrap_short_width() {
    let s = "aaa bbb ccc";
    let out = wrap(s, 3, "");
    assert_eq!(out, "aaa\nbbb\nccc");
}
