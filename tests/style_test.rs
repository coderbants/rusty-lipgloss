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

/// Ported from upstream `TestStyleInherit`: inheriting a fully-populated style
/// copies all non-margin/padding attributes but NOT margins or padding.
#[test]
fn test_inherit_full() {
    let s = Style::new()
        .bold(true)
        .italic(true)
        .underline(true)
        .strikethrough(true)
        .blink(true)
        .faint(true)
        .foreground("#ffffff")
        .background("#111111")
        .margin(&[1, 1, 1, 1])
        .padding(&[1, 1, 1, 1]);
    let i = Style::new().inherit(&s);
    assert_eq!(i.get_bold(), s.get_bold());
    assert_eq!(i.get_italic(), s.get_italic());
    assert_eq!(i.get_underline(), s.get_underline());
    assert_eq!(i.get_strikethrough(), s.get_strikethrough());
    assert_eq!(i.get_blink(), s.get_blink());
    assert_eq!(i.get_faint(), s.get_faint());
    assert_eq!(i.get_foreground(), s.get_foreground());
    assert_eq!(i.get_background(), s.get_background());
    // Margins and padding are NOT inherited.
    assert_ne!(i.get_margin(), s.get_margin());
    assert_ne!(i.get_padding(), s.get_padding());
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

/// Ported from upstream `TestStyleUnset`: every setter's paired unset returns
/// the default.
#[test]
fn test_style_unset() {
    // bold
    let mut s = Style::new().bold(true);
    assert!(s.get_bold());
    s = s.unset_bold();
    assert!(!s.get_bold());

    // italic
    s = Style::new().italic(true);
    assert!(s.get_italic());
    s = s.unset_italic();
    assert!(!s.get_italic());

    // underline
    s = Style::new().underline(true);
    assert!(s.get_underline());
    s = s.unset_underline();
    assert!(!s.get_underline());

    // underline spaces
    s = Style::new().underline_spaces(true);
    assert!(s.get_underline_spaces());
    s = s.unset_underline_spaces();
    assert!(!s.get_underline_spaces());

    // strikethrough
    s = Style::new().strikethrough(true);
    assert!(s.get_strikethrough());
    s = s.unset_strikethrough();
    assert!(!s.get_strikethrough());

    // strikethrough spaces
    s = Style::new().strikethrough_spaces(true);
    assert!(s.get_strikethrough_spaces());
    s = s.unset_strikethrough_spaces();
    assert!(!s.get_strikethrough_spaces());

    // reverse
    s = Style::new().reverse(true);
    assert!(s.get_reverse());
    s = s.unset_reverse();
    assert!(!s.get_reverse());

    // blink
    s = Style::new().blink(true);
    assert!(s.get_blink());
    s = s.unset_blink();
    assert!(!s.get_blink());

    // faint
    s = Style::new().faint(true);
    assert!(s.get_faint());
    s = s.unset_faint();
    assert!(!s.get_faint());

    // inline
    s = Style::new().inline(true);
    assert!(s.get_inline());
    s = s.unset_inline();
    assert!(!s.get_inline());

    // colors
    let col = rusty_lipgloss::color::Color::parse("#ffffff");
    s = Style::new().foreground("#ffffff");
    assert_eq!(s.get_foreground(), col);
    s = s.unset_foreground();
    assert_ne!(s.get_foreground(), col);

    s = Style::new().background("#ffffff");
    assert_eq!(s.get_background(), col);
    s = s.unset_background();
    assert_ne!(s.get_background(), col);

    // margins
    s = Style::new().margin(&[1, 2, 3, 4]);
    assert_eq!(s.get_margin_top(), 1);
    s = s.unset_margin_top();
    assert_eq!(s.get_margin_top(), 0);
    assert_eq!(s.get_margin_right(), 2);
    s = s.unset_margin_right();
    assert_eq!(s.get_margin_right(), 0);
    assert_eq!(s.get_margin_bottom(), 3);
    s = s.unset_margin_bottom();
    assert_eq!(s.get_margin_bottom(), 0);
    assert_eq!(s.get_margin_left(), 4);
    s = s.unset_margin_left();
    assert_eq!(s.get_margin_left(), 0);

    // padding
    s = Style::new().padding(&[1, 2, 3, 4]).padding_char('x');
    assert_eq!(s.get_padding_top(), 1);
    s = s.unset_padding_top();
    assert_eq!(s.get_padding_top(), 0);
    assert_eq!(s.get_padding_right(), 2);
    s = s.unset_padding_right();
    assert_eq!(s.get_padding_right(), 0);
    assert_eq!(s.get_padding_bottom(), 3);
    s = s.unset_padding_bottom();
    assert_eq!(s.get_padding_bottom(), 0);
    assert_eq!(s.get_padding_left(), 4);
    s = s.unset_padding_left();
    assert_eq!(s.get_padding_left(), 0);

    // Padding char is set and unset independently.
    let pc = Style::new().padding_char('x');
    assert_eq!(pc.get_padding_char(), 'x');
    let pc = pc.unset_padding_char();
    assert_eq!(pc.get_padding_char(), ' ');

    // borders
    s = Style::new().border(Border::normal(), &[true, true, true, true]);
    assert!(s.get_border_top());
    s = s.unset_border_top();
    assert!(!s.get_border_top());
    assert!(s.get_border_right());
    s = s.unset_border_right();
    assert!(!s.get_border_right());
    assert!(s.get_border_bottom());
    s = s.unset_border_bottom();
    assert!(!s.get_border_bottom());
    assert!(s.get_border_left());
    s = s.unset_border_left();
    assert!(!s.get_border_left());

    // tab width
    s = Style::new().tab_width(2);
    assert_eq!(s.get_tab_width(), 2);
    s = s.unset_tab_width();
    assert_ne!(s.get_tab_width(), 4);
}

/// Ported from upstream `TestStyleCopy` / `TestValueCopy`: cloning preserves
/// every field and the two styles are independent.
#[test]
fn test_style_copy() {
    let s = Style::new()
        .bold(true)
        .italic(true)
        .underline(true)
        .strikethrough(true)
        .blink(true)
        .faint(true)
        .foreground("#ffffff")
        .background("#111111")
        .margin(&[1, 1, 1, 1])
        .padding(&[1, 1, 1, 1])
        .tab_width(2);

    let i = s.clone();
    assert_eq!(s.get_bold(), i.get_bold());
    assert_eq!(s.get_italic(), i.get_italic());
    assert_eq!(s.get_underline(), i.get_underline());
    assert_eq!(s.get_underline_spaces(), i.get_underline_spaces());
    assert_eq!(s.get_strikethrough(), i.get_strikethrough());
    assert_eq!(s.get_strikethrough_spaces(), i.get_strikethrough_spaces());
    assert_eq!(s.get_blink(), i.get_blink());
    assert_eq!(s.get_faint(), i.get_faint());
    assert_eq!(s.get_foreground(), i.get_foreground());
    assert_eq!(s.get_background(), i.get_background());
    assert_eq!(s.get_margin_left(), i.get_margin_left());
    assert_eq!(s.get_margin_right(), i.get_margin_right());
    assert_eq!(s.get_margin_top(), i.get_margin_top());
    assert_eq!(s.get_margin_bottom(), i.get_margin_bottom());
    assert_eq!(s.get_padding_left(), i.get_padding_left());
    assert_eq!(s.get_padding_right(), i.get_padding_right());
    assert_eq!(s.get_padding_top(), i.get_padding_top());
    assert_eq!(s.get_padding_bottom(), i.get_padding_bottom());
    assert_eq!(s.get_tab_width(), i.get_tab_width());
}

/// Ported from upstream `TestGetUnderlineColor` and underline-style accessors.
#[test]
fn test_get_underline_color_and_style() {
    use rusty_lipgloss::ansi::Underline;
    let red = rusty_lipgloss::color::Color::parse("#FF0000");
    let s = Style::new()
        .underline(true)
        .underline_color(rusty_lipgloss::color::Color::parse("#FF0000"));
    assert_eq!(s.get_underline_color(), red);
    let s = Style::new().underline_style(Underline::Curly);
    assert_eq!(s.get_underline_style(), Underline::Curly);
}

/// Ported from upstream `TestStyleCopy` extended: copying a style that sets
/// every attribute must copy all of them, exercising every `set_from` branch.
#[test]
fn test_inherit_every_attribute() {
    let s = Style::new()
        .bold(true)
        .italic(true)
        .underline(true)
        .underline_color(rusty_lipgloss::color::Color::parse("#f00"))
        .strikethrough(true)
        .blink(true)
        .faint(true)
        .foreground("#ffffff")
        .background("#111111")
        .width(30)
        .height(2)
        .align(&[rusty_lipgloss::CENTER, rusty_lipgloss::TOP])
        .padding_char('.')
        .margin_char('*')
        .border(
            rusty_lipgloss::border::Border::normal(),
            &[true, true, true, true],
        )
        .border_foreground(&["#ff0000"])
        .border_background(&["#00ff00"])
        .border_foreground_blend(&["#111111", "#ffffff"])
        .border_foreground_blend_offset(1)
        .max_width(40)
        .max_height(5)
        .tab_width(8);
    let i = Style::new().inherit(&s);
    assert_eq!(i.get_width(), 30);
    assert_eq!(i.get_height(), 2);
    assert_eq!(i.get_max_width(), 40);
    assert_eq!(i.get_max_height(), 5);
    assert_eq!(i.get_tab_width(), 8);
    assert_eq!(
        i.get_underline_color(),
        rusty_lipgloss::color::Color::parse("#f00")
    );
    assert_eq!(i.get_align_horizontal(), rusty_lipgloss::CENTER);
    assert_eq!(i.get_align_vertical(), rusty_lipgloss::TOP);
    assert_eq!(i.get_padding_char(), '.');
    assert_eq!(i.get_margin_char(), '*');
}

/// Ported from upstream `TestStyleUnset` extended: every non-boolean property
/// can be unset back to its default.
#[test]
fn test_unset_all_properties() {
    use rusty_lipgloss::border::Border;
    use rusty_lipgloss::color::Color;
    let s = Style::new()
        .width(30)
        .height(2)
        .align(&[rusty_lipgloss::CENTER, rusty_lipgloss::TOP])
        .padding(&[1, 1, 1, 1])
        .padding_char('.')
        .margin(&[2, 2, 2, 2])
        .margin_char('*')
        .margin_background("#000000")
        .border(Border::normal(), &[true, true, true, true])
        .border_style(Border::rounded())
        .border_foreground(&["#ffffff"])
        .border_background(&["#000000"])
        .border_foreground_blend(&["#000000", "#ffffff"])
        .border_foreground_blend_offset(2)
        .max_width(40)
        .max_height(5)
        .transform(|s| s.to_string());
    // Unset everything numeric/color/border.
    let u = s
        .unset_width()
        .unset_height()
        .unset_align()
        .unset_align_horizontal()
        .unset_align_vertical()
        .unset_padding()
        .unset_padding_char()
        .unset_padding_left()
        .unset_padding_right()
        .unset_padding_top()
        .unset_padding_bottom()
        .unset_color_whitespace()
        .unset_margins()
        .unset_margin_background()
        .unset_border_style()
        .unset_border_top_foreground()
        .unset_border_right_foreground()
        .unset_border_bottom_foreground()
        .unset_border_left_foreground()
        .unset_border_foreground_blend()
        .unset_border_foreground_blend_offset()
        .unset_border_top_background()
        .unset_border_right_background()
        .unset_border_bottom_background()
        .unset_border_left_background()
        .unset_max_width()
        .unset_max_height()
        .unset_transform()
        .unset_string();
    assert_eq!(u.get_width(), 0);
    assert_eq!(u.get_height(), 0);
    assert_eq!(u.get_padding(), (0, 0, 0, 0));
    assert_eq!(u.get_margin(), (0, 0, 0, 0));
    assert_eq!(u.get_max_width(), 0);
    assert_eq!(u.get_max_height(), 0);
    assert!(u.get_transform().is_none());
    assert_eq!(u.value(), "");
    let _ = Color::default();
}

/// Ported from upstream `TestBorderStyle`/border color setters: setting border
/// colors per-side and via shorthand arrays.
#[test]
fn test_border_side_colors() {
    use rusty_lipgloss::border::Border;
    // One color -> all sides.
    let s = Style::new()
        .border(Border::normal(), &[true, true, true, true])
        .border_foreground(&["#ff0000"]);
    assert_eq!(
        s.get_border_top_foreground(),
        rusty_lipgloss::color::Color::parse("#ff0000")
    );
    assert_eq!(
        s.get_border_left_foreground(),
        rusty_lipgloss::color::Color::parse("#ff0000")
    );
    // Two colors -> top/bottom, right/left.
    let s = Style::new().border_foreground(&["#ff0000", "#00ff00"]);
    assert_eq!(
        s.get_border_top_foreground(),
        rusty_lipgloss::color::Color::parse("#ff0000")
    );
    assert_eq!(
        s.get_border_right_foreground(),
        rusty_lipgloss::color::Color::parse("#00ff00")
    );
    assert_eq!(
        s.get_border_bottom_foreground(),
        rusty_lipgloss::color::Color::parse("#ff0000")
    );
    assert_eq!(
        s.get_border_left_foreground(),
        rusty_lipgloss::color::Color::parse("#00ff00")
    );
    // Three colors -> top, right/bottom, left.
    let s = Style::new().border_foreground(&["#ff0000", "#00ff00", "#0000ff"]);
    assert_eq!(
        s.get_border_top_foreground(),
        rusty_lipgloss::color::Color::parse("#ff0000")
    );
    assert_eq!(
        s.get_border_right_foreground(),
        rusty_lipgloss::color::Color::parse("#00ff00")
    );
    assert_eq!(
        s.get_border_bottom_foreground(),
        rusty_lipgloss::color::Color::parse("#0000ff")
    );
    assert_eq!(
        s.get_border_left_foreground(),
        rusty_lipgloss::color::Color::parse("#00ff00")
    );
    // Four colors.
    let s = Style::new().border_foreground(&["#ff0000", "#00ff00", "#0000ff", "#ffff00"]);
    assert_eq!(
        s.get_border_top_foreground(),
        rusty_lipgloss::color::Color::parse("#ff0000")
    );
    assert_eq!(
        s.get_border_right_foreground(),
        rusty_lipgloss::color::Color::parse("#00ff00")
    );
    assert_eq!(
        s.get_border_bottom_foreground(),
        rusty_lipgloss::color::Color::parse("#0000ff")
    );
    assert_eq!(
        s.get_border_left_foreground(),
        rusty_lipgloss::color::Color::parse("#ffff00")
    );
    // Per-side setters.
    let s = Style::new()
        .border_top_foreground("#111111")
        .border_right_foreground("#222222")
        .border_bottom_foreground("#333333")
        .border_left_foreground("#444444");
    assert_eq!(
        s.get_border_top_foreground(),
        rusty_lipgloss::color::Color::parse("#111111")
    );
    assert_eq!(
        s.get_border_right_foreground(),
        rusty_lipgloss::color::Color::parse("#222222")
    );
    assert_eq!(
        s.get_border_bottom_foreground(),
        rusty_lipgloss::color::Color::parse("#333333")
    );
    assert_eq!(
        s.get_border_left_foreground(),
        rusty_lipgloss::color::Color::parse("#444444")
    );
    // Backgrounds, same variants.
    let s = Style::new().border_background(&["#ff0000"]);
    assert_eq!(
        s.get_border_top_background(),
        rusty_lipgloss::color::Color::parse("#ff0000")
    );
    let s = Style::new().border_background(&["#ff0000", "#00ff00"]);
    assert_eq!(
        s.get_border_top_background(),
        rusty_lipgloss::color::Color::parse("#ff0000")
    );
    assert_eq!(
        s.get_border_right_background(),
        rusty_lipgloss::color::Color::parse("#00ff00")
    );
    let s = Style::new()
        .border_top_background("#111111")
        .border_right_background("#222222")
        .border_bottom_background("#333333")
        .border_left_background("#444444");
    assert_eq!(
        s.get_border_top_background(),
        rusty_lipgloss::color::Color::parse("#111111")
    );
    assert_eq!(
        s.get_border_right_background(),
        rusty_lipgloss::color::Color::parse("#222222")
    );
    assert_eq!(
        s.get_border_bottom_background(),
        rusty_lipgloss::color::Color::parse("#333333")
    );
    assert_eq!(
        s.get_border_left_background(),
        rusty_lipgloss::color::Color::parse("#444444")
    );
    // Border blend.
    let s = Style::new()
        .border_foreground_blend(&["#111111", "#ffffff"])
        .border_foreground_blend_offset(2);
    assert_eq!(s.get_border_foreground_blend_offset(), 2);
}

/// Ported from upstream `TestBorderStyle`/`TestStyleRender` render paths:
/// rendering a bordered style with per-side foreground/background colors.
#[test]
fn test_border_render_with_colors() {
    use rusty_lipgloss::border::Border;
    // Plain border, no colors.
    let out = Style::new()
        .border(Border::normal(), &[true, true, true, true])
        .render("hello");
    assert!(out.starts_with('┌'));
    assert!(out.contains('│'));
    // Border with foreground color -> exercises style_border.
    let out = Style::new()
        .border(Border::normal(), &[true, true, true, true])
        .border_foreground(&["#ff0000"])
        .render("hello");
    assert!(out.contains("\x1b[38;2;255;0;0m"));
    // Border with background color.
    let out = Style::new()
        .border(Border::normal(), &[true, true, true, true])
        .border_background(&["#ff0000"])
        .render("hello");
    assert!(out.contains("\x1b[48;2;255;0;0m"));
    // Border with blend colors.
    let out = Style::new()
        .border(Border::normal(), &[true, true, true, true])
        .border_foreground_blend(&["#ff0000", "#00ff00"])
        .border_foreground_blend_offset(0)
        .render("hello");
    assert!(out.contains("\x1b[38;2;"));
    // Border with both fg and bg.
    let out = Style::new()
        .border(Border::normal(), &[true, true, true, true])
        .border_foreground(&["#ff0000"])
        .border_background(&["#00ff00"])
        .render("hello");
    assert!(out.contains("\x1b[38;2;255;0;0;48;2;0;255;0m"));
}

/// Ported from upstream render internals: reverse mode styles whitespace
/// (padding) separately, exercising the style_whitespace branches.
#[test]
fn test_render_reverse_styles_whitespace() {
    let out = Style::new()
        .reverse(true)
        .foreground("#ffffff")
        .background("#000000")
        .padding(&[0, 2, 0, 2])
        .render("hi");
    assert!(out.contains("\x1b[7;38;2;255;255;255;48;2;0;0;0m"));
}

/// Ported from upstream render internals: underline spaces uses a separate
/// space styler, exercising the use_space_styler branches.
#[test]
fn test_render_underline_spaces_styler() {
    let out = Style::new()
        .underline(true)
        .underline_spaces(true)
        .foreground("#ff0000")
        .render("a b");
    assert!(out.contains("4m"));
}

/// Ported from upstream render internals: strikethrough spaces styler.
#[test]
fn test_render_strikethrough_spaces_styler() {
    let out = Style::new()
        .strikethrough(true)
        .strikethrough_spaces(true)
        .foreground("#ff0000")
        .render("a b");
    assert_eq!(
        out,
        "\x1b[9;38;2;255;0;0ma\x1b[m\x1b[9;38;2;255;0;0m \x1b[m\x1b[9;38;2;255;0;0mb\x1b[m"
    );
}

/// Ported from upstream render internals: inline mode.
#[test]
fn test_render_inline() {
    let out = Style::new()
        .inline(true)
        .foreground("#ff0000")
        .render("hi\nho");
    // Inline mode removes newlines.
    assert_eq!(out, "\x1b[38;2;255;0;0mhiho\x1b[m");
}

/// Ported from upstream render internals: max-width truncation.
#[test]
fn test_render_max_width() {
    let out = Style::new().max_width(3).render("abcdef");
    assert_eq!(out, "abc");
}

/// Ported from upstream render internals: borders with specific sides enabled
/// exercise the partial-border corner logic.
#[test]
fn test_render_partial_borders() {
    use rusty_lipgloss::border::Border;
    // Only top and bottom (no left/right).
    let out = Style::new()
        .border(Border::normal(), &[true, false, true, false])
        .render("hi");
    assert!(out.starts_with("─"));
    assert!(out.contains("─"));
    // Only left and right.
    let out = Style::new()
        .border(Border::normal(), &[false, true, false, true])
        .render("hi");
    assert!(out.contains('│'));
    // Only top.
    let out = Style::new()
        .border(Border::normal(), &[true, false, false, false])
        .render("hi");
    assert!(out.starts_with("─"));
    // Empty-side border (all sides off) aborts to plain text.
    let out = Style::new()
        .border(Border::normal(), &[false, false, false, false])
        .render("hi");
    assert_eq!(out, "hi");
    // Custom border with empty chars.
    let empty = Border {
        top: "".into(),
        right: "".into(),
        bottom: "".into(),
        left: "".into(),
        top_left: "".into(),
        top_right: "".into(),
        bottom_left: "".into(),
        bottom_right: "".into(),
        ..Border::default()
    };
    let out = Style::new()
        .border(empty, &[true, true, true, true])
        .render("hi");
    assert_eq!(out, "hi");
}

/// Ported from upstream render internals: margins render blank lines/columns
/// around the styled output.
#[test]
fn test_render_margins() {
    let out = Style::new()
        .margin(&[1, 2, 1, 2])
        .background("#ff0000")
        .render("hi");
    // The margin creates a surrounding blank area.
    let lines: Vec<&str> = out.lines().collect();
    assert!(lines.len() >= 3);
    assert!(out.contains("\x1b[48;2;255;0;0m"));
}

/// Ported from upstream render internals: get_frame_size sums margins/padding/
/// borders.
#[test]
fn test_get_frame_size() {
    let s = Style::new()
        .margin(&[1, 2, 3, 4])
        .padding(&[1, 2, 3, 4])
        .border(Border::normal(), &[true, true, true, true]);
    let (x, y) = s.get_frame_size();
    assert_eq!(x, 6 + 6 + 2); // margin(4+2) + padding(4+2) + border(1+1)
    assert_eq!(y, 4 + 4 + 2); // margin(1+3) + padding(1+3) + border(1+1)
}

/// Ported from upstream style getters: get_border returns the style + side flags.
#[test]
fn test_get_border_and_align() {
    use rusty_lipgloss::border::Border;
    let s = Style::new()
        .border(Border::rounded(), &[true, false, true, true])
        .align_horizontal(rusty_lipgloss::CENTER)
        .align_vertical(rusty_lipgloss::BOTTOM);
    let (b, top, right, bottom, left) = s.get_border();
    assert_eq!(b.top_left, "╭");
    assert!(top);
    assert!(!right);
    assert!(bottom);
    assert!(left);
    assert_eq!(s.get_align_horizontal(), rusty_lipgloss::CENTER);
    assert_eq!(s.get_align_vertical(), rusty_lipgloss::BOTTOM);
    // Unset border getters default to false / empty border.
    let s = Style::new();
    assert!(!s.get_border_top());
    assert!(!s.get_border_right());
    assert!(!s.get_border_bottom());
    assert!(!s.get_border_left());
    assert_eq!(s.get_border_style(), Border::default());
}

/// Ported from upstream style setters: individual color/align/padding setters.
#[test]
fn test_individual_setters() {
    use rusty_lipgloss::color::Color;
    let s = Style::new()
        .foreground_color(Color::parse("#ff0000"))
        .align_horizontal(rusty_lipgloss::LEFT)
        .align_vertical(rusty_lipgloss::TOP)
        .padding_left(1)
        .padding_top(2)
        .padding_bottom(3)
        .padding_right(4)
        .color_whitespace(true)
        .margin_top(1)
        .margin_bottom(1)
        .margin_left(2)
        .margin_right(2)
        .transform(|x| x.to_string())
        .set_string(&["set"]);
    assert_eq!(s.get_foreground(), Color::parse("#ff0000"));
    assert_eq!(s.get_padding(), (2, 4, 3, 1));
    assert_eq!(s.get_margin(), (1, 2, 1, 2));
    assert!(s.get_color_whitespace());
    assert_eq!(s.value(), "set");
    // set_string + render merges value and arg.
    let s = Style::new().set_string(&["hi"]);
    let out = s.render("there");
    assert!(out.contains("hi"));
    assert!(out.contains("there"));
}

/// Ported from upstream style API: aggregate border unsetters.
#[test]
fn test_unset_border_aggregates() {
    use rusty_lipgloss::border::Border;
    let s = Style::new()
        .border(Border::normal(), &[true, true, true, true])
        .border_foreground(&["#ff0000"])
        .border_background(&["#00ff00"]);
    assert_ne!(
        s.get_border_top_foreground(),
        rusty_lipgloss::color::Color::NoColor
    );
    let u = s.clone().unset_border_foreground();
    assert_eq!(
        u.get_border_top_foreground(),
        rusty_lipgloss::color::Color::NoColor
    );
    assert_eq!(
        u.get_border_right_foreground(),
        rusty_lipgloss::color::Color::NoColor
    );
    let u = s.clone().unset_border_background();
    assert_eq!(
        u.get_border_top_background(),
        rusty_lipgloss::color::Color::NoColor
    );
    assert_eq!(
        u.get_border_left_background(),
        rusty_lipgloss::color::Color::NoColor
    );
}

/// Ported from upstream style API: which_sides_int/bool variants through
/// padding/margin shorthand arrays of length 2/3.
#[test]
fn test_padding_margin_shorthand() {
    let s = Style::new().padding(&[1, 2]);
    assert_eq!(s.get_padding(), (1, 2, 1, 2));
    let s = Style::new().padding(&[1, 2, 3]);
    assert_eq!(s.get_padding(), (1, 2, 3, 2));
    let s = Style::new().margin(&[1, 2]);
    assert_eq!(s.get_margin(), (1, 2, 1, 2));
    let s = Style::new().margin(&[1, 2, 3]);
    assert_eq!(s.get_margin(), (1, 2, 3, 2));
}

/// Ported from upstream render internals: apply_border corner combinations.
#[test]
fn test_render_border_corner_combos() {
    use rusty_lipgloss::border::Border;
    // Top without left or right: corners cleared.
    let out = Style::new()
        .border(Border::normal(), &[true, false, false, false])
        .render("hi");
    assert!(out.starts_with("─"));
    // Top + right only: top-left cleared.
    let out = Style::new()
        .border(Border::normal(), &[true, true, false, false])
        .render("hi");
    assert!(out.starts_with("─"));
    // Top + left only: top-right cleared.
    let out = Style::new()
        .border(Border::normal(), &[true, false, false, true])
        .render("hi");
    assert!(out.starts_with("┌"));
    // Bottom without left or right: corners cleared.
    let out = Style::new()
        .border(Border::normal(), &[false, false, true, false])
        .render("hi");
    assert!(out.ends_with("─"));
    // Bottom + left only: bottom-right cleared.
    let out = Style::new()
        .border(Border::normal(), &[false, false, true, true])
        .render("hi");
    assert!(out.lines().last().unwrap().starts_with("└"));
    // Bottom + right only: bottom-left cleared.
    let out = Style::new()
        .border(Border::normal(), &[false, true, true, false])
        .render("hi");
    assert!(out.lines().last().unwrap().ends_with("┘"));
}

/// Ported from upstream render internals: border_style without explicit sides
/// renders borders on all sides (is_border_style_set_without_sides).
#[test]
fn test_render_border_style_all_sides() {
    use rusty_lipgloss::border::Border;
    let out = Style::new().border_style(Border::normal()).render("hi");
    assert!(out.starts_with("┌"));
    assert!(out.contains("│"));
    assert!(out.ends_with("┘"));
    // A custom border with empty edges but set corners is non-default and
    // renders; empty edges are filled with spaces.
    let sparse = Border {
        top: "".into(),
        right: "".into(),
        bottom: "".into(),
        left: "".into(),
        top_left: "┌".into(),
        top_right: "┐".into(),
        bottom_left: "└".into(),
        bottom_right: "┘".into(),
        ..Border::default()
    };
    let out = Style::new().border_style(sparse).render("hi");
    assert!(out.starts_with("┌"));
    assert!(out.contains(' '));
}
