use charming_lipgloss::Style;

#[test]
fn test_style_render_bold() {
    let s = Style::new().bold(true);
    assert_eq!(s.render("Hello"), "\x1b[1mHello\x1b[0m");
}

#[test]
fn test_style_render_fg_bg() {
    let s = Style::new().foreground("201").background("#000000");
    let rendered = s.render("Test");
    assert!(rendered.contains("\x1b[38;5;201m"));
    assert!(rendered.contains("\x1b[48;2;0;0;0m"));
    assert!(rendered.ends_with("\x1b[0m"));
}

#[test]
fn test_style_empty_text() {
    let s = Style::new().bold(true);
    assert_eq!(s.render(""), "");
}
