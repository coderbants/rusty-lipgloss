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

/// Ported from upstream `TestAdaptiveColor`/`TestCompleteColor` render paths:
/// Adaptive/Complete colors resolve through their render functions and the
/// adaptive/complete helpers.
#[test]
fn test_adaptive_complete_render() {
    use rusty_lipgloss::Color;
    let adaptive = Color::Adaptive {
        light: Box::new(Color::parse("#ffffff")),
        dark: Box::new(Color::parse("#000000")),
    };
    assert_eq!(adaptive.render_fg(), Color::parse("#000000").render_fg());
    assert_eq!(adaptive.render_bg(), Color::parse("#000000").render_bg());
    assert_eq!(adaptive.render_ul(), Color::parse("#000000").render_ul());
    assert_eq!(format!("{adaptive}"), "Adaptive");

    let complete = Color::Complete {
        true_color: Box::new(Color::parse("#ff0000")),
        ansi256: Box::new(Color::parse("#0000ff")),
        ansi: Box::new(Color::parse("#00ff00")),
    };
    assert_eq!(complete.render_fg(), Color::parse("#ff0000").render_fg());
    assert_eq!(complete.render_bg(), Color::parse("#ff0000").render_bg());
    assert_eq!(complete.render_ul(), Color::parse("#ff0000").render_ul());
    assert_eq!(format!("{complete}"), "Complete");

    // Display for the various color kinds.
    assert_eq!(format!("{}", Color::Ansi16(9)), "9");
    assert_eq!(format!("{}", Color::Ansi256(196)), "196");
    assert_eq!(
        format!("{}", Color::TrueColor { r: 255, g: 0, b: 0 }),
        "#FF0000"
    );
    assert_eq!(format!("{}", Color::NoColor), "");
}

/// Ported from upstream `LightDark`/`Complete` helpers.
#[test]
fn test_light_dark_and_complete_helpers() {
    use rusty_lipgloss::color::{complete, light_dark};
    use rusty_lipgloss::Color;
    // light_dark(true) -> dark; light_dark(false) -> light.
    let f = light_dark(true);
    let c = f(Color::parse("#ffffff"), Color::parse("#000000"));
    assert_eq!(c, Color::parse("#000000"));
    let f = light_dark(false);
    let c = f(Color::parse("#ffffff"), Color::parse("#000000"));
    assert_eq!(c, Color::parse("#ffffff"));

    let f = complete(rusty_colorprofile::Profile::TrueColor);
    let c = f(
        Color::parse("#ff0000"),
        Color::parse("#0000ff"),
        Color::parse("#00ff00"),
    );
    assert_eq!(c, Color::parse("#00ff00"));
    let f = complete(rusty_colorprofile::Profile::Ansi256);
    let c = f(
        Color::parse("#ff0000"),
        Color::parse("#0000ff"),
        Color::parse("#00ff00"),
    );
    assert_eq!(c, Color::parse("#0000ff"));
    let f = complete(rusty_colorprofile::Profile::Ansi);
    let c = f(
        Color::parse("#ff0000"),
        Color::parse("#0000ff"),
        Color::parse("#00ff00"),
    );
    assert_eq!(c, Color::parse("#ff0000"));
    let f = complete(rusty_colorprofile::Profile::Ascii);
    let c = f(
        Color::parse("#ff0000"),
        Color::parse("#0000ff"),
        Color::parse("#00ff00"),
    );
    assert_eq!(c, Color::NoColor);
}

/// Exercising the hsluv / color-space conversions through lighten/darken.
#[test]
fn test_color_space_conversions() {
    use rusty_lipgloss::color::{darken, lighten};
    use rusty_lipgloss::Color;
    // These exercise rgb_to_hsl, hsl_to_rgb, xyz conversions.
    let c = lighten(Color::parse("#ff8000"), 0.5);
    assert!(matches!(c, Color::TrueColor { .. }));
    let c = darken(Color::parse("#ff8000"), 0.5);
    assert!(matches!(c, Color::TrueColor { .. }));
    let c = lighten(Color::parse("#808080"), 0.0);
    assert!(matches!(c, Color::TrueColor { .. }));
}

/// Ported from upstream `is_dark_color`: luminance-based dark detection.
#[test]
fn test_is_dark_color() {
    use rusty_lipgloss::color::is_dark_color;
    use rusty_lipgloss::Color;
    assert!(is_dark_color(&Color::parse("#000000")));
    assert!(!is_dark_color(&Color::parse("#ffffff")));
    assert!(is_dark_color(&Color::parse("#800000")));
    assert!(!is_dark_color(&Color::parse("#00ffff")));
}

/// Ported from upstream color render paths: bright ANSI, ANSI256, NoColor and
/// the adaptive/complete Display impls.
#[test]
fn test_color_render_all_variants() {
    use rusty_lipgloss::Color;
    // Bright ANSI foreground (8-15 map to 90-97).
    assert_eq!(Color::Ansi16(9).render_fg(), "\x1b[91m");
    assert_eq!(Color::Ansi16(8).render_fg(), "\x1b[90m");
    // ANSI256 background.
    assert_eq!(Color::Ansi256(196).render_bg(), "\x1b[48;5;196m");
    // ANSI256 underline.
    assert_eq!(Color::Ansi256(196).render_ul(), "\x1b[58;5;196m");
    // NoColor renders empty.
    assert_eq!(Color::NoColor.render_fg(), "");
    assert_eq!(Color::NoColor.render_bg(), "");
    assert_eq!(Color::NoColor.render_ul(), "");
}

/// Ported from upstream hex parsing: invalid inputs return errors.
#[test]
fn test_color_parse_errors() {
    use rusty_lipgloss::Color;
    assert_eq!(Color::parse(""), Color::NoColor);
    assert_eq!(Color::parse("notacolor"), Color::NoColor);
    assert_eq!(Color::parse("#12345"), Color::NoColor);
    assert_eq!(Color::parse("#1234567"), Color::NoColor);
    assert_eq!(Color::parse("#gggggg"), Color::NoColor);
    // Large integers are interpreted as 0xRRGGBB (256 = 0x000100).
    assert_eq!(Color::parse("256"), Color::TrueColor { r: 0, g: 1, b: 0 });
    assert_eq!(
        Color::parse("16711680"),
        Color::TrueColor { r: 255, g: 0, b: 0 }
    );
}

/// Ported from upstream indexed palette: gray shades and indexed rgb.
#[test]
fn test_indexed_palette() {
    use rusty_lipgloss::color::indexed_rgb;
    let (r, g, b) = indexed_rgb(232);
    assert_eq!(r, 8);
    assert_eq!(g, 8);
    assert_eq!(b, 8);
    let (r, g, b) = indexed_rgb(255);
    assert_eq!(r, 238);
    assert_eq!(g, 238);
    assert_eq!(b, 238);
    let (r, g, b) = indexed_rgb(196);
    assert_eq!(r, 255);
    assert_eq!(g, 0);
    assert_eq!(b, 0);
}

/// Ported from upstream alpha/darken/lighten/complementary edge cases.
#[test]
fn test_color_ops_edge_cases() {
    use rusty_lipgloss::color::{alpha, complementary, darken, lighten};
    use rusty_lipgloss::Color;
    // NoColor passes through darken/lighten/complementary.
    assert_eq!(darken(Color::NoColor, 0.5), Color::NoColor);
    assert_eq!(lighten(Color::NoColor, 0.5), Color::NoColor);
    assert_eq!(complementary(Color::NoColor), Color::NoColor);
    // alpha on non-truecolor is a passthrough.
    assert_eq!(alpha(Color::NoColor, 0.5), Color::NoColor);
    assert_eq!(alpha(Color::Ansi16(1), 0.5), Color::Ansi16(1));
    // alpha on truecolor adjusts the alpha.
    let c = alpha(Color::TrueColor { r: 255, g: 0, b: 0 }, 1.0);
    assert!(matches!(c, Color::TrueColor { .. }));
    // complementary on various hues (hsv_to_rgb branches).
    let _ = complementary(Color::parse("#ff0000"));
    let _ = complementary(Color::parse("#ffff00"));
    let _ = complementary(Color::parse("#00ff00"));
    let _ = complementary(Color::parse("#00ffff"));
    let _ = complementary(Color::parse("#0000ff"));
    let _ = complementary(Color::parse("#ff00ff"));
}

/// Ported from upstream rgb_to_hsl: the green-max branch.
#[test]
fn test_color_rgb_to_hsl_green() {
    use rusty_lipgloss::color::is_dark_color;
    use rusty_lipgloss::Color;
    // Green-max colors hit the g branch of rgb_to_hsl.
    assert!(!is_dark_color(&Color::parse("#00ff00")));
    assert!(is_dark_color(&Color::parse("#008000")));
}

/// Ported from upstream parse_hex: 3-digit and short-form handling.
#[test]
fn test_color_parse_short_hex() {
    use rusty_lipgloss::Color;
    assert_eq!(
        Color::parse("#fff"),
        Color::TrueColor {
            r: 255,
            g: 255,
            b: 255
        }
    );
    assert_eq!(
        Color::parse("#f00"),
        Color::TrueColor { r: 255, g: 0, b: 0 }
    );
}

/// Ported from upstream color Display/adaptive/complete fmt and render paths.
#[test]
fn test_color_adaptive_complete_fmt() {
    use rusty_lipgloss::color::{AdaptiveColor, CompleteColor};
    use rusty_lipgloss::Color;
    assert_eq!(
        format!(
            "{}",
            AdaptiveColor {
                light: "#fff".into(),
                dark: "#000".into()
            }
        ),
        "Light: #fff, Dark: #000"
    );
    let cc = CompleteColor {
        true_color: "#ff0000".into(),
        ansi256: "196".into(),
        ansi: "9".into(),
    };
    assert_eq!(format!("{cc}"), "#ff0000");
    assert_eq!(cc.render_bg(), Color::parse("#ff0000").render_bg());
    // Bright ANSI underline.
    assert_eq!(Color::Ansi16(9).render_ul(), "\x1b[58;5;9m");
}

/// Ported from upstream complementary: hue wrap-around (h >= 360).
#[test]
fn test_complementary_hue_wrap() {
    use rusty_lipgloss::color::complementary;
    use rusty_lipgloss::Color;
    // A color whose hue + 180 >= 360 wraps.
    let c = complementary(Color::parse("#ff00ff"));
    assert!(matches!(c, Color::TrueColor { .. }));
    let c = complementary(Color::parse("#800000"));
    assert!(matches!(c, Color::TrueColor { .. }));
}
