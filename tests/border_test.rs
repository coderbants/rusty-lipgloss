//! Cleanroom Rust port of upstream Go test file: `borders_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::border::{get_first_rune_as_string, max_rune_width, Border};

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

/// Ported from upstream `NormalBorder()` free functions: the top-level border
/// constructors match the Border::preset forms.
#[test]
fn test_top_level_border_functions() {
    use rusty_lipgloss::border::{
        block_border, double_border, inner_half_block_border, normal_border,
        outer_half_block_border, rounded_border, thick_border,
    };
    assert_eq!(normal_border().top_left, Border::normal().top_left);
    assert_eq!(rounded_border().top_left, Border::rounded().top_left);
    assert_eq!(block_border().top, Border::block().top);
    assert_eq!(
        outer_half_block_border().top,
        Border::outer_half_block().top
    );
    assert_eq!(
        inner_half_block_border().top,
        Border::inner_half_block().top
    );
    assert_eq!(thick_border().middle, Border::thick().middle);
    assert_eq!(double_border().middle, Border::double().middle);
}

/// Ported from upstream `BorderBlend`: blending border colors with offsets.
#[test]
fn test_border_blend_new() {
    use rusty_lipgloss::border::BorderBlend;
    use rusty_lipgloss::color::Color;
    let colors = vec![Color::parse("#ff0000"), Color::parse("#00ff00")];
    let b = BorderBlend::new(2, 1, &colors, 0);
    assert!(!b.top_gradient.is_empty());
    assert_eq!(b.top_gradient.len(), 4);
    // Non-zero offset rotates the gradient.
    let b2 = BorderBlend::new(2, 1, &colors, 1);
    assert_eq!(b2.top_gradient.len(), 4);
}
