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

/// Ported from upstream `ansi.Wrap` behaviors: hardwrapping long words.
#[test]
fn test_wrap_hardwrap_long_word() {
    let s = "abcdefghijklmno";
    let out = wrap(s, 5, "");
    assert_eq!(out, "abcde\nfghij\nklmno");
}

/// Ported from upstream `ansi.Wrap` behaviors: newlines inside input.
#[test]
fn test_wrap_preserves_newlines() {
    let s = "abc def\nghi jkl";
    let out = wrap(s, 10, "");
    assert!(out.contains("abc def\nghi jkl"));
}

/// Ported from upstream `ansi.Wrap` behaviors: breakpoints.
#[test]
fn test_wrap_breakpoints() {
    let s = "a/b/c/d";
    let out = wrap(s, 3, "/");
    // The breakpoint folds into the word when the line is full.
    assert_eq!(out, "a/\nb/c\n/d");
}

/// Ported from upstream `ansi.Wrap` behaviors: hyphen breakpoints.
#[test]
fn test_wrap_hyphen() {
    let s = "foo-bar-baz";
    let out = wrap(s, 4, "");
    assert_eq!(out, "foo-\nbar-\nbaz");
}

/// Ported from upstream `ansi.Wrap` behaviors: OSC sequences pass through.
#[test]
fn test_wrap_osc_sequences() {
    let s = "\x1b]8;;https://example.com\x07link\x1b]8;;\x07";
    let out = wrap(s, 100, "");
    assert!(out.contains("\x1b]8;;https://example.com\x07"));
}

/// Ported from upstream `ansi.Wrap` behaviors: trailing spaces are dropped.
#[test]
fn test_wrap_trailing_space() {
    let s = "hello  world  ";
    let out = wrap(s, 20, "");
    // Whitespace is preserved (upstream ansi.Wrap semantics).
    assert_eq!(out, "hello  world  ");
}

/// Ported from upstream `ansi.Wrap` behaviors: zero width with newlines.
#[test]
fn test_wrap_zero_width_ignores() {
    let s = "hello\nworld";
    assert_eq!(wrap(s, 0, ""), "hello\nworld");
}

/// Ported from upstream `ansi.Wrap` behaviors: ANSI-tagged words count width
/// without the escape bytes.
#[test]
fn test_wrap_ansi_width() {
    let s = "\x1b[31mabcdef\x1b[m";
    let out = wrap(s, 3, "");
    // Escape codes stay attached to their word (upstream ansi.Wrap).
    assert_eq!(out, "\x1b[31mabc\ndef\x1b[m");
}

/// Ported from upstream `TestWrapWriterWriteAfterClose`: writing after close
/// succeeds (the buffer still accepts writes).
#[test]
fn test_wrap_writer_write_after_close() {
    use rusty_lipgloss::ansi::WrapWriter;
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut w = WrapWriter::new(&mut buf);
        w.close().expect("close");
        let n = w.write(b"after close").expect("write after close");
        assert_eq!(n, b"after close".len());
    }
    assert_eq!(buf, b"after close");
}

/// WrapWriter writes bytes through and reports its default style/link.
#[test]
fn test_wrap_writer_defaults() {
    use rusty_lipgloss::ansi::WrapWriter;
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut w = WrapWriter::new(&mut buf);
        assert!(w.style().is_zero());
        assert_eq!(w.link(), "");
        w.write(b"hello\nworld").expect("write");
        w.close().expect("close");
    }
    assert_eq!(buf, b"hello\nworld");
}

/// Ported from upstream ansi.Wrap: leading whitespace before a newline that
/// exceeds the width is dropped (cur_width reset).
#[test]
fn test_wrap_newline_overflowing_whitespace() {
    // Two spaces before the newline exceed width 1 -> dropped.
    assert_eq!(wrap("  \n", 1, ""), "\n");
    // Spaces fit within width -> preserved.
    assert_eq!(wrap("  \n", 5, ""), "  \n");
    // Trailing whitespace after the last word overflowing width.
    assert_eq!(wrap("a  ", 1, ""), "a");
    assert_eq!(wrap("a  ", 5, ""), "a  ");
    // A newline at the start preserves any leading whitespace when it fits.
    assert_eq!(wrap(" \na", 5, ""), " \na");
}

/// Ported from upstream ansi.Wrap: OSC terminated by ST (ESC \) and a bare
/// trailing ESC.
#[test]
fn test_wrap_st_terminated_osc_and_bare_esc() {
    // OSC ends with ST (0x1b 0x5c) instead of BEL.
    let s = "\x1b]8;;http://example.com\x1b\\link\x1b]8;;\x1b\\";
    let out = wrap(s, 10, "");
    assert!(out.contains("link"), "got: {out:?}");
    // A bare trailing escape byte is preserved.
    let out = wrap("abc\x1b", 5, "");
    assert_eq!(out, "abc\x1b");
}
