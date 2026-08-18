//! Cleanroom Rust port of upstream Go test file: `ansi_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::ansi::{cut, strip, Style, Underline};
use rusty_lipgloss::color::Color;

#[test]
fn test_strip_basic() {
    assert_eq!(strip("hello"), "hello");
    assert_eq!(strip("\x1b[31mhello\x1b[m"), "hello");
}

#[test]
fn test_strip_osc() {
    assert_eq!(
        strip("\x1b]8;;https://example.com\x07link\x1b]8;;\x07"),
        "link"
    );
    assert_eq!(
        strip("\x1b]8;;https://example.com\x1b\\link\x1b]8;;\x1b\\"),
        "link"
    );
}

#[test]
fn test_cut_basic() {
    assert_eq!(cut("hello", 1, 3), "el");
    assert_eq!(cut("hello", 0, 100), "hello");
}

#[test]
fn test_cut_ansi_preserved() {
    // Escape sequences are copied verbatim regardless of cut position.
    let out = cut("\x1b[31mhello\x1b[m", 1, 3);
    assert!(out.contains("el"));
    assert!(out.contains("\x1b[31m"));
}

#[test]
fn test_cut_osc_preserved() {
    let out = cut("\x1b]8;;https://x\x07link\x1b]8;;\x07", 0, 100);
    assert!(out.contains("\x1b]8;;https://x\x07"));
}

#[test]
fn test_style_string_basic() {
    let s = Style::default();
    assert_eq!(s.string(), "");
    let s = Style {
        bold: true,
        italic: true,
        underline: true,
        underline_style: Underline::Double,
        ..Style::default()
    };
    let out = s.string();
    assert!(out.starts_with("\x1b["));
    assert!(out.contains("1;3;4;21"));
}

#[test]
fn test_style_string_underline_styles() {
    for (ul, expect) in [
        (Underline::Single, "4"),
        (Underline::Double, "21"),
        (Underline::Curly, "4:3"),
        (Underline::Dotted, "4:4"),
        (Underline::Dashed, "4:5"),
        (Underline::None, ""),
    ] {
        let s = Style {
            underline: true,
            underline_style: ul,
            ..Style::default()
        };
        let out = s.string();
        if expect.is_empty() {
            assert!(!out.contains("4:"));
        } else {
            assert!(out.contains(expect), "underline {ul:?} -> {out:?}");
        }
    }
}

#[test]
fn test_style_string_colors() {
    let s = Style {
        fg_color: Some(Color::TrueColor {
            r: 90,
            g: 86,
            b: 224,
        }),
        bg_color: Some(Color::TrueColor { r: 0, g: 0, b: 0 }),
        ul_color: Some(Color::TrueColor { r: 255, g: 0, b: 0 }),
        ..Style::default()
    };
    let out = s.string();
    assert!(out.contains("38;2;90;86;224"));
    assert!(out.contains("48;2;0;0;0"));
    assert!(out.contains("58;2;255;0;0"));
}

#[test]
fn test_style_string_whitespace() {
    // The whitespace styler omits the underline-flag suffix ordering quirk:
    // params read color;...;4 (underline color before underline flag).
    let s = Style {
        underline: true,
        ul_color: Some(Color::TrueColor { r: 255, g: 0, b: 0 }),
        ..Style::default()
    };
    let ws = s.string_whitespace();
    assert!(ws.contains("58;2;255;0;0;4"), "got: {ws:?}");
    // Empty style -> empty whitespace string.
    assert_eq!(Style::default().string_whitespace(), "");
}

#[test]
fn test_style_reverse() {
    let s = Style {
        reverse: true,
        ..Style::default()
    };
    assert!(s.string().contains("7"));
    assert!(s.string_whitespace().contains("7"));
}

#[test]
fn test_style_blink_faint() {
    let s = Style {
        blink: true,
        faint: true,
        ..Style::default()
    };
    let out = s.string();
    assert!(out.contains("5"));
    assert!(out.contains("2"));
}
