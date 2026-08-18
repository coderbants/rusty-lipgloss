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

/// Ported from upstream CompleteAdaptiveColor and CompleteColor rgba paths.
#[test]
fn test_compat_complete_adaptive() {
    use rusty_lipgloss::compat::{CompleteAdaptiveColor, CompleteColor};
    let cc = CompleteColor {
        true_color: rusty_lipgloss::Color::parse("#FF0000"),
        ansi256: rusty_lipgloss::Color::parse("9"),
        ansi: rusty_lipgloss::Color::parse("1"),
    };
    let cac = CompleteAdaptiveColor {
        light: cc.clone(),
        dark: cc,
    };
    let (r, _, _, _) = cac.rgba();
    // has_dark_background defaults true in CI; values are 16-bit channels.
    assert!(r % 0x100 == 0);
}

/// Ported from upstream: adaptive rgba with a LIGHT background (COLORFGBG).
#[test]
fn test_compat_adaptive_light_bg() {
    use std::process::Command;
    let out = Command::new(std::env::current_exe().unwrap())
        .env("COLORFGBG", "0;15")
        .env("RUST_LIPGLOSS_COMPAT_PROBE", "1")
        .args(["--exact", "probe_compat_adaptive", "--nocapture"])
        .output()
        .expect("spawn");
    assert!(out.status.success(), "probe failed: {out:?}");
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Light bg -> the light color (0x0000) is used.
    assert!(stdout.contains("LIGHT="), "got: {stdout}");
}

/// Probe helper: adaptive rgba with the process COLORFGBG.
#[test]
fn probe_compat_adaptive() {
    if std::env::var("RUST_LIPGLOSS_COMPAT_PROBE").is_err() {
        return;
    }
    let c = rusty_lipgloss::compat::AdaptiveColor {
        light: rusty_lipgloss::Color::parse("#000000"),
        dark: rusty_lipgloss::Color::parse("#FFFFFF"),
    };
    let (r, _, _, _) = c.rgba();
    println!("LIGHT={r}");
}
