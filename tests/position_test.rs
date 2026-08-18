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

/// Position Display formatting and align_text_vertical branches.
#[test]
fn test_position_display_and_vertical_align_branches() {
    use rusty_lipgloss::align::{BOTTOM, CENTER};
    // Display for Position.
    assert_eq!(format!("{}", rusty_lipgloss::TOP), "0");
    assert_eq!(format!("{}", rusty_lipgloss::CENTER), "0.5");
    assert_eq!(format!("{}", BOTTOM), "1");
    // Height smaller than content -> unchanged.
    assert_eq!(place_vertical(1, CENTER, "hi\nho", &[]), "hi\nho");
    // CENTER with odd remaining space distributes to the bottom.
    let out = place_vertical(4, CENTER, "hi", &[]);
    assert_eq!(out, "  \nhi\n  \n  ");
    // Unknown/other position -> unchanged (matches the default arm).
    let out = place_vertical(2, Position::default(), "hi", &[]);
    assert_eq!(out, "hi\n  ");
}

/// Covers the trivial accessors and helpers.
#[test]
fn test_misc_accessors() {
    // size function.
    assert_eq!(rusty_lipgloss::size::size("hi\nho"), (2, 2));
    assert_eq!(rusty_lipgloss::size::size("hello"), (5, 1));
    // new_style alias.
    let s = rusty_lipgloss::new_style();
    assert_eq!(s.render("x"), "x");
    // table::new alias.
    let t = rusty_lipgloss::table::new();
    assert!(t.get_border_top());
    // tree::new / tree::root aliases.
    let t = rusty_lipgloss::tree::new();
    assert_eq!(t.render(), "");
    let t = rusty_lipgloss::tree::root("root");
    assert_eq!(t.render(), "root");
    // space helper.
    assert_eq!(rusty_lipgloss::whitespace::space(3), "   ");
    // Place horizontal with a multiline string hits the newline branch.
    let out = rusty_lipgloss::position::place_horizontal(5, rusty_lipgloss::LEFT, "a\nb", &[]);
    assert!(out.contains("\n"));
}
