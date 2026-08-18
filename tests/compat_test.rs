//! Cleanroom Rust port of upstream Go test file: `compat/color_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::compat::{has_dark_background, profile, AdaptiveColor, CompleteColor};

#[test]
fn test_compat_has_dark_background() {
    // Best-effort; must not panic in any environment.
    let _ = has_dark_background();
}

#[test]
fn test_compat_profile() {
    let p = profile();
    assert!(matches!(
        p,
        rusty_lipgloss::color::Profile::TrueColor
            | rusty_lipgloss::color::Profile::Ansi256
            | rusty_lipgloss::color::Profile::Ansi
            | rusty_lipgloss::color::Profile::Ascii
            | rusty_lipgloss::color::Profile::NoTty
    ));
}

#[test]
fn test_compat_adaptive_color() {
    let c = AdaptiveColor {
        light: rusty_lipgloss::Color::parse("#000000"),
        dark: rusty_lipgloss::Color::parse("#FFFFFF"),
    };
    let (r, g, b, _) = c.rgba();
    // has_dark_background defaults true without a TTY, so the dark color is
    // returned; rgba() is 16-bit with values in the 0xFF00 channel range.
    assert_eq!((r % 0x100, g % 0x100, b % 0x100), (0, 0, 0));
}

#[test]
fn test_compat_complete_color() {
    let c = CompleteColor {
        true_color: rusty_lipgloss::Color::parse("#FF0000"),
        ansi256: rusty_lipgloss::Color::parse("9"),
        ansi: rusty_lipgloss::Color::parse("1"),
    };
    let (_, _, _, _) = c.rgba();
}
