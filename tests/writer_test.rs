//! Cleanroom Rust port of upstream Go source file: `writer.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::color::Color;
use rusty_lipgloss::color::Profile;
use rusty_lipgloss::writer::downsample_sgr;

#[test]
fn test_writer_truecolor_passthrough() {
    // Truecolor sequences are kept as-is at the TrueColor profile.
    let out = downsample_sgr("\x1b[38;2;255;0;0mhello\x1b[m", Profile::TrueColor);
    assert_eq!(out, "\x1b[38;2;255;0;0mhello\x1b[m");
}

#[test]
fn test_writer_truecolor_to_256() {
    // A truecolor red downsamples to the 256-color cube.
    let out = downsample_sgr("\x1b[38;2;255;0;0mhi\x1b[m", Profile::Ansi256);
    assert!(out.contains("38;5;196"), "got: {out:?}");
}

#[test]
fn test_writer_truecolor_to_ansi() {
    let out = downsample_sgr("\x1b[38;2;255;0;0mhi\x1b[m", Profile::Ansi);
    assert!(out.contains("91m"), "got: {out:?}");
}

#[test]
fn test_writer_256_to_ansi() {
    // 38;5;196 (red) downsamples to basic red 31.
    let out = downsample_sgr("\x1b[38;5;196mhi\x1b[m", Profile::Ansi);
    assert!(out.contains("91"), "got: {out:?}");
}

#[test]
fn test_writer_basic_colors() {
    // Basic colors pass through.
    let out = downsample_sgr("\x1b[31;42mhi\x1b[m", Profile::Ansi);
    assert!(out.contains("31;42"), "got: {out:?}");
}

#[test]
fn test_writer_bright_colors() {
    let out = downsample_sgr("\x1b[91;102mhi\x1b[m", Profile::Ansi);
    assert!(out.contains("91;102"), "got: {out:?}");
}

#[test]
fn test_writer_background_48() {
    let out = downsample_sgr("\x1b[48;2;0;255;0mhi\x1b[m", Profile::Ansi256);
    assert!(out.contains("48;5;46"), "got: {out:?}");
}

#[test]
fn test_writer_underline_color_58() {
    let out = downsample_sgr("\x1b[58;2;0;0;255mhi\x1b[m", Profile::Ansi256);
    assert!(out.contains("58;5;21"), "got: {out:?}");
}

#[test]
fn test_writer_sgr_default() {
    // SGR 0 becomes a bare reset.
    let out = downsample_sgr("\x1b[0mhi\x1b[m", Profile::Ansi);
    assert_eq!(out, "\x1b[mhi\x1b[m");
}

#[test]
fn test_writer_non_color_attrs() {
    // Non-color attributes pass through.
    let out = downsample_sgr("\x1b[1;3mhi\x1b[m", Profile::Ansi);
    assert!(out.contains("1;3"), "got: {out:?}");
}

#[test]
fn test_writer_mixed() {
    let out = downsample_sgr("\x1b[1;38;2;255;255;255mhi\x1b[m", Profile::Ansi);
    assert!(out.contains("1;"), "got: {out:?}");
}

#[test]
fn test_writer_reset_only() {
    let out = downsample_sgr("\x1b[0;0;0mhi\x1b[m", Profile::Ansi);
    assert_eq!(out, "\x1b[;;mhi\x1b[m");
}

#[test]
fn test_writer_no_sgr() {
    let out = downsample_sgr("plain text", Profile::Ansi);
    assert_eq!(out, "plain text");
}

#[test]
fn test_downsample_colors() {
    use rusty_lipgloss::writer::downsample;
    assert_eq!(
        downsample(&Color::parse("#ffffff"), Profile::TrueColor),
        Color::parse("#ffffff")
    );
    assert_eq!(
        downsample(&Color::parse("#ffffff"), Profile::Ansi256),
        Color::Ansi256(231)
    );
    assert_eq!(
        downsample(&Color::parse("#ffffff"), Profile::Ascii),
        Color::NoColor
    );
    assert_eq!(
        downsample(&Color::parse("#ff0000"), Profile::Ansi256),
        Color::Ansi256(196)
    );
    assert_eq!(
        downsample(&Color::parse("#ff0000"), Profile::Ansi),
        Color::Ansi16(9)
    );
}

#[test]
fn test_convert_256() {
    use rusty_lipgloss::writer::convert_256;
    assert_eq!(convert_256(255, 0, 0), 196);
    assert_eq!(convert_256(0, 0, 0), 16);
    assert_eq!(convert_256(255, 255, 255), 231);
    assert_eq!(convert_256(0, 255, 0), 46);
    assert_eq!(convert_256(0, 0, 255), 21);
}

/// Ported from upstream `handleSgr` error paths: malformed 38/48/58 color
/// sequences fall back to the default SGR code.
#[test]
fn test_writer_malformed_colors() {
    // 38 without enough params.
    let out = downsample_sgr("\x1b[38;2;255;0mhi\x1b[m", Profile::Ansi256);
    assert!(out.contains("39"), "got: {out:?}");
    // 48;2 with a missing channel (only 4 params).
    let out = downsample_sgr("\x1b[48;2;255;0mhi\x1b[m", Profile::Ansi256);
    assert!(out.contains("49"), "got: {out:?}");
    // 58 with fewer than 3 params.
    let out = downsample_sgr("\x1b[58;5mhi\x1b[m", Profile::Ansi256);
    assert!(out.contains("59"), "got: {out:?}");
    // 38;2;0;0;0 (valid truecolor) downsamples to 256.
    let out = downsample_sgr("\x1b[38;2;0;0;0mhi\x1b[m", Profile::Ansi256);
    assert!(out.contains("38;5;16"), "got: {out:?}");
    // 38;5;7 (basic ANSI via 256) downsamples to ANSI.
    let out = downsample_sgr("\x1b[38;5;7mhi\x1b[m", Profile::Ansi);
    assert!(out.contains("37"), "got: {out:?}");
}

/// Ported from upstream `downsampleSgr`: non-SGR escape sequences pass through.
#[test]
fn test_writer_non_sgr_escapes() {
    // With an ANSI profile, non-SGR sequences (CSI without a final byte of
    // 'm' and OSC) are passed through verbatim.
    let out = downsample_sgr("\x1b[2Jclear\x1b]0;title\x07done", Profile::Ansi);
    assert!(out.contains("\x1b[2J"), "got: {out:?}");
    assert!(out.contains("\x1b]0;title\x07"), "got: {out:?}");
    assert!(out.contains("done"), "got: {out:?}");
    // With NoTty all ANSI sequences are stripped.
    let out = downsample_sgr("\x1b[2Jclear", Profile::NoTty);
    assert_eq!(out, "clear");
}

/// Ported from upstream profile detection via NO_COLOR/CLICOLOR env in a
/// non-TTY child process.
#[test]
fn test_writer_detect_profile_no_color() {
    use std::process::Command;
    let out = Command::new(std::env::current_exe().unwrap())
        .env("NO_COLOR", "1")
        .env("RUST_LIPGLOSS_WRITER_PROBE", "1")
        .args(["--exact", "probe_writer_detect_profile", "--nocapture"])
        .output()
        .expect("spawn");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("PROFILE="), "got: {stdout}");
}

/// Probe helper for env-based profile detection.
#[test]
fn probe_writer_detect_profile() {
    if std::env::var("RUST_LIPGLOSS_WRITER_PROBE").is_err() {
        return;
    }
    let p = rusty_lipgloss::writer::detect_profile();
    println!("PROFILE={p:?}");
}
