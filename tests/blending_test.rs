//! Cleanroom Rust port of upstream Go test file: `blending_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::blending::{blend_1d, blend_2d};
use rusty_lipgloss::color::Color;

fn rgba(r: u8, g: u8, b: u8) -> Color {
    Color::TrueColor { r, g, b }
}

#[test]
fn test_blend_1d_two_colors_ten_steps() {
    let stops = vec![rgba(255, 0, 0), rgba(0, 0, 255)];
    let got = blend_1d(10, &stops);
    assert_eq!(got.len(), 10);
    assert_eq!(got[0], rgba(255, 0, 0));
    assert_eq!(got[9], rgba(0, 0, 255));
}

#[test]
fn test_blend_1d_three_colors_four_steps() {
    let stops = vec![rgba(255, 0, 0), rgba(0, 255, 0), rgba(0, 0, 255)];
    let got = blend_1d(4, &stops);
    assert_eq!(got.len(), 4);
    assert_eq!(got[0], rgba(255, 0, 0));
    assert_eq!(got[1], rgba(0, 255, 0));
    assert_eq!(got[2], rgba(0, 255, 0));
    assert_eq!(got[3], rgba(0, 0, 255));
}

#[test]
fn test_blend_1d_black_to_white() {
    let stops = vec![rgba(0, 0, 0), rgba(255, 255, 255)];
    let got = blend_1d(5, &stops);
    assert_eq!(got[0], rgba(0, 0, 0));
    assert_eq!(got[2], rgba(119, 119, 119));
    assert_eq!(got[4], rgba(255, 255, 255));
}

#[test]
fn test_blend_1d_single_stop() {
    let stops = vec![rgba(255, 0, 0)];
    let got = blend_1d(3, &stops);
    assert_eq!(got, vec![rgba(255, 0, 0); 3]);
}

#[test]
fn test_blend_1d_edge_cases() {
    assert_eq!(blend_1d(0, &[]), Vec::<Color>::new());
    let stops = vec![rgba(255, 0, 0)];
    assert_eq!(blend_1d(0, &stops), Vec::<Color>::new());
}

#[test]
fn test_blend_2d_lengths() {
    let stops = vec![rgba(255, 0, 0), rgba(0, 0, 255)];
    assert_eq!(blend_2d(2, 2, 0.0, &stops).len(), 4);
    assert_eq!(blend_2d(3, 2, 90.0, &stops).len(), 6);
    assert_eq!(blend_2d(2, 3, 180.0, &stops).len(), 6);
    assert_eq!(blend_2d(2, 2, 270.0, &stops).len(), 4);
    assert_eq!(blend_2d(2, 2, 450.0, &stops).len(), 4);
    assert_eq!(blend_2d(2, 2, -90.0, &stops).len(), 4);
}

#[test]
fn test_blend_2d_invalid_dimensions_fallback() {
    let stops = vec![rgba(255, 0, 0)];
    let got = blend_2d(0, 0, 0.0, &stops);
    assert_eq!(got.len(), 1);
    assert_eq!(got[0], rgba(255, 0, 0));
}

#[test]
fn test_blend_2d_empty_stops() {
    let got = blend_2d(2, 2, 0.0, &[]);
    assert_eq!(got.len(), 0);
}

/// Ported from upstream `blend` pair behavior: blending with non-TrueColor
/// colors falls back to the source color.
#[test]
fn test_blend_1d_pair_fallbacks() {
    use rusty_lipgloss::blending::blend_1d_pair;
    // TrueColor pair blends.
    let c = blend_1d_pair(&rgba(255, 0, 0), &rgba(0, 0, 255), 0.5);
    assert!(matches!(c, Color::TrueColor { .. }));
    // NoColor start falls back to the start color.
    let c = blend_1d_pair(&Color::NoColor, &rgba(0, 0, 255), 0.5);
    assert_eq!(c, Color::NoColor);
    // Ansi16 start falls back to the start color.
    let c = blend_1d_pair(&Color::Ansi16(1), &rgba(0, 0, 255), 0.5);
    assert_eq!(c, Color::Ansi16(1));
    // Factor clamping.
    let c = blend_1d_pair(&rgba(0, 0, 0), &rgba(255, 255, 255), 1.5);
    assert!(matches!(c, Color::TrueColor { .. }));
    let c = blend_1d_pair(&rgba(0, 0, 0), &rgba(255, 255, 255), -0.5);
    assert!(matches!(c, Color::TrueColor { .. }));
}

/// Ported from upstream `blend` NoColor filtering: NoColor stops are dropped.
#[test]
fn test_blend_1d_nocolor_stops() {
    let got = blend_1d(4, &[Color::NoColor, rgba(255, 0, 0), rgba(0, 0, 255)]);
    assert_eq!(got.len(), 4);
    let got = blend_1d(4, &[Color::NoColor]);
    assert_eq!(got.len(), 0);
}

/// Ported from upstream LUV blending: LUV-space interpolation round-trips.
#[test]
fn test_blend_luv_pair() {
    use rusty_lipgloss::blending::blend_luv_pair;
    let c = blend_luv_pair(&rgba(255, 0, 0), &rgba(0, 0, 255), 0.5);
    assert!(matches!(c, Color::TrueColor { .. }));
    let c = blend_luv_pair(&rgba(0, 0, 0), &rgba(255, 255, 255), 0.0);
    assert_eq!(c, rgba(0, 0, 0));
    let c = blend_luv_pair(&rgba(0, 0, 0), &rgba(255, 255, 255), 1.0);
    assert_eq!(c, rgba(255, 255, 255));
}

/// Ported from upstream LUV blending: raw LUV-space RGB components.
#[test]
fn test_blend_luv_rgb() {
    use rusty_lipgloss::blending::blend_luv_rgb;
    let (r, g, b) = blend_luv_rgb(&rgba(0, 0, 0), &rgba(255, 255, 255), 0.0);
    assert_eq!((r, g, b), (0.0, 0.0, 0.0));
    let (r, g, b) = blend_luv_rgb(&rgba(0, 0, 0), &rgba(255, 255, 255), 1.0);
    assert!((r - 1.0).abs() < 0.01);
    assert!((g - 1.0).abs() < 0.01);
    assert!((b - 1.0).abs() < 0.01);
    // Non-truecolor start values clamp to 0-1.
    let (r, _, _) = blend_luv_rgb(&Color::NoColor, &rgba(255, 0, 0), 1.0);
    assert!((0.0..=1.0).contains(&r));
}

/// Ported from upstream blending: uneven segment distribution (some segments
/// get one extra step).
#[test]
fn test_blend_1d_uneven_segments() {
    let stops = vec![
        rgba(255, 0, 0),
        rgba(0, 255, 0),
        rgba(0, 0, 255),
        rgba(0, 0, 0),
    ];
    // 10 steps across 3 segments -> some segments get 4.
    let got = blend_1d(10, &stops);
    assert_eq!(got.len(), 10);
}

/// Ported from upstream blending: blend_2d with a wide gradient sampling.
#[test]
fn test_blend_2d_wide() {
    let stops = vec![rgba(255, 0, 0), rgba(0, 0, 255)];
    let got = blend_2d(1, 10, 0.0, &stops);
    assert_eq!(got.len(), 10);
    let got = blend_2d(10, 1, 45.0, &stops);
    assert_eq!(got.len(), 10);
}
