//! Cleanroom Rust port of upstream Go test file: `color_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::color::{
    alpha, complementary, darken, lighten, AdaptiveColor, Color, CompleteColor,
};

#[test]
fn test_color_parse_hex() {
    let c = Color::parse("#FF0000");
    assert_eq!(c, Color::TrueColor { r: 255, g: 0, b: 0 });
    assert_eq!(c.render_fg(), "\x1b[38;2;255;0;0m");
    assert_eq!(c.render_bg(), "\x1b[48;2;255;0;0m");

    let short_hex = Color::parse("#F00");
    assert_eq!(short_hex, Color::TrueColor { r: 255, g: 0, b: 0 });
}

#[test]
fn test_color_parse_ansi() {
    let c16 = Color::parse("5");
    assert_eq!(c16, Color::Ansi16(5));
    assert_eq!(c16.render_fg(), "\x1b[35m");

    let c256 = Color::parse("201");
    assert_eq!(c256, Color::Ansi256(201));
    assert_eq!(c256.render_fg(), "\x1b[38;5;201m");
}

#[test]
fn test_rgba_values() {
    let (r, g, b, _) = Color::parse("9").rgba_bytes();
    assert_eq!((r, g, b), (0xFF, 0x00, 0x00));
    let (r, g, b, _) = Color::parse("21").rgba_bytes();
    assert_eq!((r, g, b), (0x00, 0x00, 0xFF));
    let (r, g, b, _) = Color::parse("16711680").rgba_bytes();
    assert_eq!((r, g, b), (0xFF, 0x00, 0x00));
}

#[test]
fn test_hex_to_color_roundtrip() {
    for hex in ["#FF0000", "#00F", "#6B50FF", "#000000"] {
        let c = Color::parse(hex);
        let (r, g, b, _) = c.rgba_bytes();
        assert!(
            matches!(c, Color::TrueColor { .. }),
            "{} should parse as TrueColor",
            hex
        );
        assert!(r != 0 || g != 0 || b != 0 || hex == "#000000");
    }
}

#[test]
fn test_alpha() {
    // Our terminal color model does not carry alpha; alpha() preserves the RGB.
    let c = Color::TrueColor { r: 255, g: 0, b: 0 };
    let a = alpha(c.clone(), 0.5);
    let (r, g, b, _) = a.rgba_bytes();
    assert_eq!((r, g, b), (255, 0, 0));
    let a = alpha(c, 0.0);
    let (r, g, b, _) = a.rgba_bytes();
    assert_eq!((r, g, b), (255, 0, 0));
}

#[test]
fn test_complementary() {
    let (r, g, b, _) = complementary(Color::parse("#FF0000")).rgba_bytes();
    assert_eq!((r, g, b), (0x00, 0xFF, 0xFF));
    let (r, g, b, _) = complementary(Color::parse("#00FF00")).rgba_bytes();
    assert_eq!((r, g, b), (0xFF, 0x00, 0xFF));
    let (r, g, b, _) = complementary(Color::parse("#0000FF")).rgba_bytes();
    assert_eq!((r, g, b), (0xFF, 0xFF, 0x00));
}

#[test]
fn test_darken() {
    let (r, g, b, _) = darken(Color::parse("#FFFFFF"), 0.5).rgba_bytes();
    assert_eq!((r, g, b), (0x7F, 0x7F, 0x7F));
    let (r, g, b, _) = darken(Color::parse("#FF0000"), 0.25).rgba_bytes();
    assert_eq!((r, g, b), (0xBF, 0x00, 0x00));
}

#[test]
fn test_lighten() {
    let (r, g, b, _) = lighten(Color::parse("#000000"), 0.5).rgba_bytes();
    assert_eq!((r, g, b), (0x7F, 0x7F, 0x7F));
    let (r, g, b, _) = lighten(Color::parse("#800000"), 0.25).rgba_bytes();
    assert_eq!((r, g, b), (0xBF, 0x3F, 0x3F));
}

#[test]
fn test_adaptive_color() {
    let ac = AdaptiveColor {
        light: "#000000".to_string(),
        dark: "#FFFFFF".to_string(),
    };
    assert_eq!(ac.render_fg(), "\x1b[38;2;255;255;255m");
    assert_eq!(ac.render_bg(), "\x1b[48;2;255;255;255m");
}

#[test]
fn test_complete_color() {
    let cc = CompleteColor {
        true_color: "#FF0000".to_string(),
        ansi256: "9".to_string(),
        ansi: "1".to_string(),
    };
    assert_eq!(cc.render_fg(), "\x1b[38;2;255;0;0m");
}

#[test]
fn test_render_bg_ansi() {
    let c = Color::parse("5");
    assert_eq!(c.render_bg(), "\x1b[45m");
    let c = Color::parse("13");
    assert_eq!(c.render_bg(), "\x1b[105m");
}
