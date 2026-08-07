use charming_lipgloss::align::Position;
use charming_lipgloss::blending::{blend_1d, blend_2d};
use charming_lipgloss::color::Color;
use charming_lipgloss::position::{place, place_horizontal, place_vertical};

#[test]
fn test_blend_1d() {
    let red = Color::parse("#FF0000");
    let blue = Color::parse("#0000FF");
    let mid = blend_1d(&red, &blue, 0.5);
    assert_eq!(mid, Color::TrueColor { r: 128, g: 0, b: 128 });
}

#[test]
fn test_blend_2d() {
    let c = Color::parse("#FF0000");
    let res = blend_2d(&c, &c, &c, &c, 0.5, 0.5);
    assert_eq!(res, Color::TrueColor { r: 255, g: 0, b: 0 });
}

#[test]
fn test_place_horizontal() {
    let res = place_horizontal(10, Position::Center, "Hi");
    assert_eq!(res, "    Hi    ");
}

#[test]
fn test_place_vertical() {
    let res = place_vertical(3, Position::Center, "Hi");
    assert_eq!(res, "  \nHi\n  ");
}

#[test]
fn test_place() {
    let res = place(10, 3, Position::Center, Position::Center, "Hi");
    assert_eq!(res, "          \n    Hi    \n          ");
}
