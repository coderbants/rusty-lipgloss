use charming_lipgloss::color::{AdaptiveColor, Color, CompleteColor, TerminalColor};

#[test]
fn test_color_parse_hex() {
    let c = Color::parse("#FF0000");
    assert_eq!(c, Color::TrueColor { r: 255, g: 0, b: 0 });
    assert_eq!(c.render_fg(), "\x1b[38;2;255;0;0m");

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
fn test_adaptive_color() {
    let ac = AdaptiveColor {
        light: "#000000".to_string(),
        dark: "#FFFFFF".to_string(),
    };
    assert_eq!(ac.render_fg(), "\x1b[38;2;255;255;255m");
}

#[test]
fn test_complete_color() {
    let cc = CompleteColor {
        true_color: "#00FF00".to_string(),
        ansi256: "46".to_string(),
        ansi: "2".to_string(),
    };
    assert_eq!(cc.render_fg(), "\x1b[38;2;0;255;0m");
}
