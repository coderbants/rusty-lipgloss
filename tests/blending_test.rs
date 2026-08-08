//! Cleanroom Rust port of upstream Go test file: `blending_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use charming_lipgloss::blending::{blend_1d, blend_2d};
use charming_lipgloss::color::Color;

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
