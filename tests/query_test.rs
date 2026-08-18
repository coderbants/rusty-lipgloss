//! Cleanroom Rust port of upstream Go test file: `query_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use std::process::Command;

/// Ported from upstream `HasDarkBackground` env fallback: COLORFGBG with a dark
/// background id (<8) reports dark.
#[test]
fn test_has_dark_background_dark_env() {
    let out = Command::new(std::env::current_exe().unwrap())
        .env("COLORFGBG", "15;0")
        .env("RUST_LIPGLOSS_QUERY_DARK_PROBE", "1")
        .args(["--exact", "probe_has_dark_background", "--nocapture"])
        .output()
        .expect("spawn probe");
    assert!(out.status.success(), "probe failed: {out:?}");
    assert!(String::from_utf8_lossy(&out.stdout).contains("DARK=true"));
}

/// Ported from upstream `HasDarkBackground` env fallback: COLORFGBG with a
/// light background id (>=8) reports light.
#[test]
fn test_has_dark_background_light_env() {
    let out = Command::new(std::env::current_exe().unwrap())
        .env("COLORFGBG", "0;15")
        .env("RUST_LIPGLOSS_QUERY_DARK_PROBE", "1")
        .args(["--exact", "probe_has_dark_background", "--nocapture"])
        .output()
        .expect("spawn probe");
    assert!(out.status.success(), "probe failed: {out:?}");
    assert!(String::from_utf8_lossy(&out.stdout).contains("DARK=false"));
}

/// Probe helper used by the env tests above. Runs in a child process so the
/// COLORFGBG mutation does not affect other tests.
#[test]
fn probe_has_dark_background() {
    if std::env::var("RUST_LIPGLOSS_QUERY_DARK_PROBE").is_err() {
        return;
    }
    let dark = rusty_lipgloss::query::has_dark_background();
    println!("DARK={dark}");
}
