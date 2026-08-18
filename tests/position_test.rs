//! Cleanroom Rust port of upstream Go test files: `position_test.go` (via `align_test.go`)
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::align::{Position, BOTTOM, CENTER, TOP};
use rusty_lipgloss::blending::{blend_1d, blend_2d};
use rusty_lipgloss::color::Color;
use rusty_lipgloss::position::{place, place_horizontal, place_vertical};

#[test]
fn test_blend_1d() {
    let red = Color::parse("#FF0000");
    let blue = Color::parse("#0000FF");
    let stops = vec![red.clone(), blue.clone()];
    let got = blend_1d(2, &stops);
    assert_eq!(got.len(), 2);
    assert_eq!(got[0], red);
    assert_eq!(got[1], blue);
}

#[test]
fn test_blend_2d() {
    let c = Color::parse("#FF0000");
    let stops = vec![c.clone()];
    let res = blend_2d(2, 2, 0.0, &stops);
    assert_eq!(res.len(), 4);
    for color in &res {
        assert_eq!(*color, c);
    }
}

#[test]
fn test_place_horizontal() {
    assert_eq!(place_horizontal(10, CENTER, "Hi", &[]), "    Hi    ");
    assert_eq!(
        place_horizontal(10, Position::LEFT, "Hi", &[]),
        "Hi        "
    );
    assert_eq!(
        place_horizontal(10, Position::RIGHT, "Hi", &[]),
        "        Hi"
    );
    assert_eq!(place_horizontal(2, CENTER, "Hi", &[]), "Hi");
}

#[test]
fn test_place_vertical() {
    assert_eq!(place_vertical(3, CENTER, "Hi", &[]), "  \nHi\n  ");
    assert_eq!(place_vertical(3, TOP, "Hi", &[]), "Hi\n  \n  ");
    assert_eq!(place_vertical(3, BOTTOM, "Hi", &[]), "  \n  \nHi");
}

#[test]
fn test_place() {
    let res = place(6, 3, CENTER, CENTER, "Hi", &[]);
    assert_eq!(res, "      \n  Hi  \n      ");
}

/// Ported from upstream align: placement functions.
#[test]
fn test_position_fmt_and_default() {
    use rusty_lipgloss::position::{place, place_horizontal, place_vertical};
    let out = place(
        10,
        1,
        rusty_lipgloss::CENTER,
        rusty_lipgloss::TOP,
        "hi",
        &[],
    );
    assert!(out.contains("hi"));
    assert!(rusty_lipgloss::size::width(&out) >= 10);
    let out = place_horizontal(10, rusty_lipgloss::LEFT, "hi", &[]);
    assert!(out.starts_with("hi"));
    let out = place_vertical(3, rusty_lipgloss::BOTTOM, "hi", &[]);
    assert!(out.ends_with("hi"));
}
