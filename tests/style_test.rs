//! Cleanroom Rust port of upstream Go test file: `style_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::ansi::Underline;
use rusty_lipgloss::border::Border;
use rusty_lipgloss::size;
use rusty_lipgloss::style::Style;

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
        rusty_lipgloss::color::Color::parse("#ffffff")
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

#[test]
fn test_underline_vectors() {
    // Ported from upstream TestUnderline (the full `ab c` matrix).
    let cases: &[(Style, &str)] = &[
        (
            Style::new().underline(true),
            "\x1b[4;4ma\x1b[m\x1b[4;4mb\x1b[m\x1b[4m \x1b[m\x1b[4;4mc\x1b[m",
        ),
        (
            Style::new().underline(true).underline_spaces(true),
            "\x1b[4;4ma\x1b[m\x1b[4;4mb\x1b[m\x1b[4m \x1b[m\x1b[4;4mc\x1b[m",
        ),
        (
            Style::new().underline(true).underline_spaces(false),
            "\x1b[4;4ma\x1b[m\x1b[4;4mb\x1b[m \x1b[4;4mc\x1b[m",
        ),
        (
            Style::new().underline_spaces(true),
            "ab\x1b[4m \x1b[mc",
        ),
        (
            Style::new().underline_style(Underline::Curly),
            "\x1b[4;4:3ma\x1b[m\x1b[4;4:3mb\x1b[m\x1b[4m \x1b[m\x1b[4;4:3mc\x1b[m",
        ),
        (
            Style::new()
                .underline_style(Underline::Curly)
                .underline_color(rusty_lipgloss::color::Color::parse("#FF0000")),
            "\x1b[4;58;2;255;0;0;4:3ma\x1b[m\x1b[4;58;2;255;0;0;4:3mb\x1b[m\x1b[58;2;255;0;0;4m \x1b[m\x1b[4;58;2;255;0;0;4:3mc\x1b[m",
        ),
    ];
    for (style, expected) in cases {
        let s = style.clone().set_string(&["ab c"]);
        assert_eq!(&s.render(""), expected);
    }
}

#[test]
fn test_strikethrough_vectors() {
    // Ported from upstream TestStrikethrough.
    let cases: &[(Style, &str)] = &[
        (
            Style::new().strikethrough(true),
            "\x1b[9ma\x1b[m\x1b[9mb\x1b[m\x1b[9m \x1b[m\x1b[9mc\x1b[m",
        ),
        (
            Style::new().strikethrough(true).strikethrough_spaces(true),
            "\x1b[9ma\x1b[m\x1b[9mb\x1b[m\x1b[9m \x1b[m\x1b[9mc\x1b[m",
        ),
        (
            Style::new().strikethrough(true).strikethrough_spaces(false),
            "\x1b[9ma\x1b[m\x1b[9mb\x1b[m \x1b[9mc\x1b[m",
        ),
        (Style::new().strikethrough_spaces(true), "ab\x1b[9m \x1b[mc"),
    ];
    for (style, expected) in cases {
        let s = style.clone().set_string(&["ab c"]);
        assert_eq!(&s.render(""), expected);
    }
}

#[test]
fn test_style_render_vectors() {
    // Ported from upstream TestStyleRender.
    let cases: &[(Style, &str)] = &[
        (
            Style::new().foreground("#5A56E0"),
            "\x1b[38;2;90;86;224mhello\x1b[m",
        ),
        (Style::new().bold(true), "\x1b[1mhello\x1b[m"),
        (Style::new().italic(true), "\x1b[3mhello\x1b[m"),
        (
            Style::new().underline(true),
            "\x1b[4;4mh\x1b[m\x1b[4;4me\x1b[m\x1b[4;4ml\x1b[m\x1b[4;4ml\x1b[m\x1b[4;4mo\x1b[m",
        ),
        (Style::new().blink(true), "\x1b[5mhello\x1b[m"),
        (Style::new().faint(true), "\x1b[2mhello\x1b[m"),
    ];
    for (style, expected) in cases {
        assert_eq!(&style.render("hello"), expected);
    }
}

#[test]
fn test_value_copy() {
    // Ported from upstream TestValueCopy: Style is Copy, so mutating a copy
    // must not affect the original.
    let s = Style::new().bold(true);
    let i = s.clone().bold(false);
    assert!(s.get_bold());
    assert!(!i.get_bold());
}

#[test]
fn test_style_value() {
    // Ported from upstream TestStyleValue.
    assert_eq!(Style::new().render("foo"), "foo");
    assert_eq!(Style::new().set_string(&["bar"]).render("foo"), "bar foo");
    assert_eq!(
        Style::new().set_string(&["bar"]).bold(true).render("foo"),
        "\x1b[1mbar foo\x1b[m"
    );
    assert_eq!(
        Style::new().set_string(&["bar", "foobar"]).render("foo"),
        "bar foobar foo"
    );
    assert_eq!(Style::new().margin_right(1).render("foo"), "foo ");
    assert_eq!(Style::new().margin_left(1).render("foo"), " foo");
    assert_eq!(Style::new().margin_right(1).render(""), " ");
    assert_eq!(Style::new().margin_left(1).render(""), " ");
}

#[test]
fn test_carriage_return_in_render() {
    // Ported from upstream TestCarriageReturnInRender.
    let out = "Super duper california oranges\r\nHello world\r\n";
    let style = Style::new().margin_left(1);
    let got = style.render(out);
    let want = style.render("Super duper california oranges\nHello world\n");
    assert_eq!(got, want);
}

#[test]
fn test_hyperlink_vectors() {
    // Ported from upstream TestHyperlink and TestUnsetHyperlink.
    assert_eq!(
        Style::new()
            .hyperlink("https://example.com", &[])
            .set_string(&["https://example.com"])
            .render(""),
        "\x1b]8;;https://example.com\x07https://example.com\x1b]8;;\x07"
    );
    assert_eq!(
        Style::new()
            .hyperlink("https://example.com", &["id=123"])
            .set_string(&["example"])
            .render(""),
        "\x1b]8;id=123;https://example.com\x07example\x1b]8;;\x07"
    );
    assert_eq!(
        Style::new()
            .hyperlink("https://example.com", &["id=123"])
            .set_string(&["example"])
            .bold(true)
            .foreground("234")
            .render(""),
        "\x1b]8;id=123;https://example.com\x07\x1b[1;38;5;234mexample\x1b[m\x1b]8;;\x07"
    );
    // Unset hyperlink.
    assert_eq!(
        Style::new()
            .hyperlink("https://example.com", &[])
            .set_string(&["https://example.com"])
            .unset_hyperlink()
            .render(""),
        "https://example.com"
    );
    assert_eq!(
        Style::new()
            .hyperlink("https://example.com", &["id=123"])
            .set_string(&["example"])
            .bold(true)
            .foreground("234")
            .unset_hyperlink()
            .render(""),
        "\x1b[1;38;5;234mexample\x1b[m"
    );
}

#[test]
fn test_width_and_height_with_borders() {
    // Ported from upstream TestWidth and TestHeight: the rendered width/height
    // must equal the content frame size after removing borders/padding.
    let content = "The Romans learned from the Greeks that quinces slowly cooked with honey would \u{201c}set\u{201d} when cool. The Apicius gives a recipe for preserving whole quinces, stems and leaves attached, in a bath of honey diluted with defrutum: Roman marmalade. Preserves of quince and lemon appear (along with rose, apple, plum and pear) in the Book of ceremonies of the Byzantine Emperor Constantine VII Porphyrogennetos.";

    let width_cases: &[Style] = &[
        Style::new()
            .padding(&[0, 2])
            .border(Border::normal(), &[true]),
        Style::new().padding(&[0, 2]),
        Style::new()
            .padding(&[0, 2])
            .border(Border::normal(), &[true])
            .border_left(false)
            .border_right(false),
        Style::new()
            .padding(&[0, 2])
            .border(Border::normal(), &[true])
            .unset_border_bottom()
            .unset_border_top()
            .unset_border_right(),
    ];
    for style in width_cases {
        let frame = style.get_horizontal_frame_size();
        let rendered = style.clone().width(80 - frame).render(content);
        assert_eq!(size::width(&rendered), 80 - frame);
    }

    let height_cases: &[Style] = &[
        Style::new()
            .width(80)
            .padding(&[0, 2])
            .border(Border::normal(), &[true]),
        Style::new().width(80).padding(&[0, 2]),
        Style::new()
            .width(80)
            .padding(&[0, 2])
            .border(Border::normal(), &[true])
            .border_bottom(false)
            .border_top(false),
        Style::new()
            .width(80)
            .padding(&[0, 2])
            .border(Border::normal(), &[true])
            .unset_border_left()
            .unset_border_bottom()
            .unset_border_right(),
    ];
    for style in height_cases {
        let frame = style.get_vertical_frame_size();
        let rendered = style.clone().height(20 - frame).render(content);
        assert_eq!(size::height(&rendered), 20 - frame);
    }
}
