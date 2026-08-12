//! Cleanroom Rust port of upstream Go test file: `style_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use charming_lipgloss::ansi::Underline;
use charming_lipgloss::border::Border;
use charming_lipgloss::size;
use charming_lipgloss::style::Style;

#[test]
fn test_style_render_bold() {
    let s = Style::new().bold(true);
    assert_eq!(s.render("hello"), "\x1b[1mhello\x1b[m");
}

#[test]
fn test_style_render_fg_bg() {
    let s = Style::new().foreground("201").background("#000000");
    let rendered = s.render("Test");
    assert!(rendered.contains("38;5;201"));
    assert!(rendered.contains("48;2;0;0;0"));
    assert!(rendered.starts_with("\x1b["));
    assert!(rendered.ends_with("\x1b[m"));
}

#[test]
fn test_style_italic() {
    assert_eq!(
        Style::new().italic(true).render("hello"),
        "\x1b[3mhello\x1b[m"
    );
}

#[test]
fn test_style_underline() {
    assert_eq!(
        Style::new().underline(true).render("hello"),
        "\x1b[4;4mh\x1b[m\x1b[4;4me\x1b[m\x1b[4;4ml\x1b[m\x1b[4;4ml\x1b[m\x1b[4;4mo\x1b[m"
    );
}

#[test]
fn test_style_blink_faint_reverse() {
    assert_eq!(
        Style::new().blink(true).render("hello"),
        "\x1b[5mhello\x1b[m"
    );
    assert_eq!(
        Style::new().faint(true).render("hello"),
        "\x1b[2mhello\x1b[m"
    );
    assert_eq!(
        Style::new().reverse(true).render("hello"),
        "\x1b[7mhello\x1b[m"
    );
}

#[test]
fn test_style_set_string() {
    let s = Style::new().set_string(&["bar"]);
    assert_eq!(s.render("foo"), "bar foo");
    let s = Style::new().set_string(&["bar"]).bold(true);
    assert_eq!(s.render("foo"), "\x1b[1mbar foo\x1b[m");
}

#[test]
fn test_tab_conversion() {
    assert_eq!(Style::new().render("[\t]"), "[    ]");
    assert_eq!(Style::new().tab_width(2).render("[\t]"), "[  ]");
    assert_eq!(Style::new().tab_width(0).render("[\t]"), "[]");
    assert_eq!(Style::new().tab_width(-1).render("[\t]"), "[\t]");
}

#[test]
fn test_transform() {
    let s = Style::new().bold(true).transform(|x| x.to_uppercase());
    assert_eq!(s.render("raow"), "\x1b[1mRAOW\x1b[m");
}

#[test]
fn test_custom_padding_char() {
    let s = Style::new().padding(&[0, 3]).padding_char('x');
    assert_eq!(s.render("TEST"), "xxxTESTxxx");
}

#[test]
fn test_margins() {
    let s = Style::new().margin(&[0, 1]);
    assert_eq!(s.render("foo"), " foo ");
    let s = Style::new().margin_left(1);
    assert_eq!(s.render("foo"), " foo");
    let s = Style::new().margin_right(1);
    assert_eq!(s.render("foo"), "foo ");
}

#[test]
fn test_width_and_alignment() {
    let s = Style::new().width(10);
    let out = s.render("hi");
    assert_eq!(size::width(&out), 10);
    assert_eq!(out, "hi        ");
}

#[test]
fn test_width_with_border() {
    let s = Style::new().width(10).border(Border::normal(), &[]);
    let out = s.render("hi");
    assert_eq!(size::width(&out), 10);
}

#[test]
fn test_border_render() {
    let s = Style::new().border(Border::normal(), &[]);
    let out = s.render("hello");
    assert!(out.starts_with("┌─────┐"));
    assert!(out.contains("│hello│"));
    assert!(out.ends_with("└─────┘"));
}

#[test]
fn test_hyperlink() {
    let s = Style::new()
        .hyperlink("https://example.com", &[])
        .set_string(&["https://example.com"]);
    assert_eq!(
        s.render(""),
        "\x1b]8;;https://example.com\x07https://example.com\x1b]8;;\x07"
    );
}

#[test]
fn test_underline_spaces() {
    let s = Style::new().underline_spaces(true).set_string(&["ab c"]);
    assert_eq!(s.render(""), "ab\x1b[4m \x1b[mc");
}

#[test]
fn test_underline_styles() {
    let s = Style::new()
        .underline_style(Underline::Curly)
        .set_string(&["ab c"]);
    assert_eq!(
        s.render(""),
        "\x1b[4;4:3ma\x1b[m\x1b[4;4:3mb\x1b[m\x1b[4m \x1b[m\x1b[4;4:3mc\x1b[m"
    );
}

#[test]
fn test_inherit() {
    let base = Style::new().bold(true).italic(true).foreground("#ffffff");
    let inherited = Style::new().inherit(&base);
    assert!(inherited.get_bold());
    assert!(inherited.get_italic());
    assert_eq!(
        inherited.get_foreground(),
        charming_lipgloss::color::Color::parse("#ffffff")
    );
}

#[test]
fn test_unset() {
    let s = Style::new().bold(true);
    assert!(s.get_bold());
    let s = s.unset_bold();
    assert!(!s.get_bold());
}

#[test]
fn test_max_width() {
    let s = Style::new().max_width(5);
    assert_eq!(s.render("hello world"), "hello");
}

#[test]
fn test_height() {
    let s = Style::new().height(3);
    let out = s.render("foo");
    assert_eq!(size::height(&out), 3);
}
