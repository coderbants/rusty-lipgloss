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

#[test]
fn test_truncate() {
    use rusty_lipgloss::ansi::truncate;
    assert_eq!(truncate("hello", 10, "…"), "hello");
    assert_eq!(truncate("hello", 3, "…"), "he…");
    assert_eq!(truncate("hello", 0, "…"), "…");
    assert_eq!(truncate("hello", 2, "hello"), "he");
}

#[test]
fn test_truncate_left() {
    use rusty_lipgloss::ansi::truncate_left;
    assert_eq!(truncate_left("hello", 10, "…"), "hello");
    assert_eq!(truncate_left("hello", 3, "…"), "…lo");
    assert_eq!(truncate_left("hello", 0, "…"), "…");
}

#[test]
fn test_styled_whitespace() {
    use rusty_lipgloss::ansi::Style;
    use rusty_lipgloss::color::Color;
    let s = Style {
        bold: true,
        faint: true,
        italic: true,
        blink: true,
        reverse: true,
        strikethrough: true,
        bg_color: Some(Color::TrueColor { r: 0, g: 255, b: 0 }),
        fg_color: Some(Color::Ansi16(9)),
        ..Style::default()
    };
    let ws = s.string_whitespace();
    assert!(ws.contains("1;2;3;5;7;9"));
    assert!(ws.contains("91"));
    assert!(ws.contains("48;2;0;255;0"));
    // styled/styled_whitespace wrap the content.
    assert!(s.styled("x").contains("x"));
    assert!(s.styled_whitespace(" ").contains(" "));
}

#[test]
fn test_color_seq_variants() {
    use rusty_lipgloss::ansi::Style;
    use rusty_lipgloss::color::Color;
    // Ansi16 basic (<8) and bright (>=8) via style string.
    let s = Style {
        fg_color: Some(Color::Ansi16(3)),
        ..Style::default()
    };
    assert_eq!(s.string(), "\x1b[33m");
    let s = Style {
        fg_color: Some(Color::Ansi16(9)),
        ..Style::default()
    };
    assert_eq!(s.string(), "\x1b[91m");
    // Adaptive/Complete resolve through color_seq.
    let s = Style {
        fg_color: Some(Color::Adaptive {
            light: Box::new(Color::Ansi16(1)),
            dark: Box::new(Color::Ansi16(2)),
        }),
        ..Style::default()
    };
    assert_eq!(s.string(), "\x1b[32m");
    let s = Style {
        fg_color: Some(Color::Complete {
            true_color: Box::new(Color::Ansi16(4)),
            ansi256: Box::new(Color::Ansi16(4)),
            ansi: Box::new(Color::Ansi16(4)),
        }),
        ..Style::default()
    };
    assert_eq!(s.string(), "\x1b[34m");
    // NoColor yields an empty color param (reset-style sequence).
    let s = Style {
        fg_color: Some(Color::NoColor),
        ..Style::default()
    };
    assert_eq!(s.string(), "\x1b[m");
}

#[test]
fn test_cut_st_osc() {
    use rusty_lipgloss::ansi::cut;
    // ST-terminated OSC (ESC \) is preserved.
    let out = cut("\x1b]8;;https://x\x1b\\link\x1b]8;;\x1b\\", 0, 100);
    assert!(out.contains("\x1b]8;;https://x\x1b\\"));
    assert!(out.contains("link"));
}
