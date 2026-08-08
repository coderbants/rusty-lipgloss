//! Cleanroom Rust port of upstream Go test file: `borders_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use charming_lipgloss::border::{get_first_rune_as_string, max_rune_width, Border};

#[test]
fn test_preset_borders() {
    let normal = Border::normal();
    assert_eq!(normal.top_left, "┌");
    assert_eq!(normal.top, "─");
    assert_eq!(normal.left, "│");
    assert_eq!(normal.middle_left, "├");
    assert_eq!(normal.middle, "┼");

    let rounded = Border::rounded();
    assert_eq!(rounded.top_left, "╭");
    assert_eq!(rounded.top_right, "╮");
    assert_eq!(rounded.bottom_left, "╰");
    assert_eq!(rounded.bottom_right, "╯");

    let double = Border::double();
    assert_eq!(double.top_left, "╔");
    assert_eq!(double.middle, "╬");

    let thick = Border::thick();
    assert_eq!(thick.top_left, "┏");
    assert_eq!(thick.middle, "╋");

    assert_eq!(Border::block().top, "█");
    assert_eq!(Border::outer_half_block().top, "▀");
    assert_eq!(Border::inner_half_block().top, "▄");
    assert_eq!(Border::hidden().top, " ");
    assert_eq!(Border::markdown().top, "-");
    assert_eq!(Border::ascii().top, "-");
    assert_eq!(Border::ascii().top_left, "+");
}

#[test]
fn test_border_sizes() {
    let normal = Border::normal();
    assert_eq!(normal.get_top_size(), 1);
    assert_eq!(normal.get_left_size(), 1);
    assert_eq!(normal.get_right_size(), 1);
    assert_eq!(normal.get_bottom_size(), 1);
}

#[test]
fn test_get_first_rune_as_string() {
    assert_eq!(get_first_rune_as_string(""), "");
    assert_eq!(get_first_rune_as_string("A"), "A");
    assert_eq!(get_first_rune_as_string("世"), "世");
    assert_eq!(get_first_rune_as_string("Hello"), "H");
    assert_eq!(get_first_rune_as_string("你好世界"), "你");
    assert_eq!(get_first_rune_as_string("😀Happy"), "😀");
    assert_eq!(get_first_rune_as_string("ñoño"), "ñ");
}

#[test]
fn test_max_rune_width() {
    assert_eq!(max_rune_width(" "), 1);
    assert_eq!(max_rune_width("+"), 1);
    assert_eq!(max_rune_width("╭"), 1);
}
