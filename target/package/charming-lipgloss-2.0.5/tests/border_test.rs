use charming_lipgloss::border::Border;

#[test]
fn test_preset_borders() {
    let normal = Border::normal();
    assert_eq!(normal.top_left, "┌");

    let rounded = Border::rounded();
    assert_eq!(rounded.top_left, "╭");

    let double = Border::double();
    assert_eq!(double.top_left, "╔");

    let thick = Border::thick();
    assert_eq!(thick.top_left, "┏");
}
