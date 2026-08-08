//! Cleanroom Rust port of upstream Go example: `examples/layout/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! Demonstrates various Lip Gloss style and layout features: tabs, a title,
//! a dialog box, lists, a color grid, marmalade history, and a status bar,
//! composed together with a floating modal.

use charming_lipgloss::align::{BOTTOM, CENTER, LEFT, RIGHT, TOP};
use charming_lipgloss::blending::blend_1d;
use charming_lipgloss::border::Border;
use charming_lipgloss::color::Color;
use charming_lipgloss::join::{join_horizontal, join_vertical};
use charming_lipgloss::layer::{new_compositor, new_layer};
use charming_lipgloss::position;
use charming_lipgloss::query;
use charming_lipgloss::size;
use charming_lipgloss::style::Style;
use charming_lipgloss::whitespace::{with_whitespace_chars, with_whitespace_style, Whitespace};
use charming_lipgloss::writer::println;

/// The document width used for the layout.
const WIDTH: usize = 96;

/// How wide to render various columns in the layout.
const COLUMN_WIDTH: usize = 30;

fn pick<'a>(dark: bool, light: &'a str, dark_c: &'a str) -> &'a str {
    if dark {
        dark_c
    } else {
        light
    }
}

/// Generates a grid of colors from four corner quadrants.
fn color_grid(x_steps: usize, y_steps: usize) -> Vec<Vec<Color>> {
    let left_colors = blend_1d(y_steps, &[Color::parse("#F25D94"), Color::parse("#643AFF")]);
    let right_colors = blend_1d(y_steps, &[Color::parse("#EDFF82"), Color::parse("#14F9D5")]);

    let mut grid = Vec::with_capacity(y_steps);
    for y in 0..y_steps {
        let row_colors = blend_1d(x_steps, &[left_colors[y].clone(), right_colors[y].clone()]);
        grid.push(row_colors);
    }
    grid
}

/// Applies a gradient to the given string.
fn apply_gradient(base: Style, input: &str, from: Color, to: Color) -> String {
    use unicode_segmentation::UnicodeSegmentation;
    let chars: Vec<&str> = input.graphemes(true).collect();
    let gradient = blend_1d(chars.len(), &[from, to]);
    let mut output = String::new();
    for (i, char) in chars.iter().enumerate() {
        output.push_str(&base.clone().foreground_color(gradient[i].clone()).render(char));
    }
    output
}

fn main() {
    // Detect the background color.
    let has_dark_bg = query::has_dark_background();

    // General.
    let subtle = pick(has_dark_bg, "#D9DCCF", "#383838");
    let highlight = pick(has_dark_bg, "#874BFD", "#7D56F4");
    let special = pick(has_dark_bg, "#43BF6D", "#73F59F");

    let divider = Style::new()
        .set_string(&["•"])
        .padding(&[0, 1])
        .foreground(subtle)
        .string();
    let url = |s: &str| Style::new().foreground(special).render(s);

    // Tabs.
    let active_tab_border = Border {
        top: "─".into(),
        bottom: " ".into(),
        left: "│".into(),
        right: "│".into(),
        top_left: "╭".into(),
        top_right: "╮".into(),
        bottom_left: "┘".into(),
        bottom_right: "└".into(),
        ..Border::default()
    };
    let tab_border = Border {
        top: "─".into(),
        bottom: "─".into(),
        left: "│".into(),
        right: "│".into(),
        top_left: "╭".into(),
        top_right: "╮".into(),
        bottom_left: "┴".into(),
        bottom_right: "┴".into(),
        ..Border::default()
    };

    let tab = Style::new()
        .border(tab_border, &[true])
        .border_foreground(&[highlight])
        .padding(&[0, 1]);
    let active_tab = tab.clone().border(active_tab_border, &[true]);
    let tab_gap = tab
        .clone()
        .border_top(false)
        .border_left(false)
        .border_right(false);

    // Title.
    let title_style = Style::new()
        .margin_left(1)
        .margin_right(5)
        .padding(&[0, 1])
        .italic(true)
        .foreground("#FFF7DB")
        .set_string(&["Lip Gloss"]);
    let desc_style = Style::new().margin_top(1);
    let info_style = Style::new()
        .border_style(Border::normal())
        .border_top(true)
        .border_foreground(&[subtle]);

    // Dialog.
    let dialog_box_style = Style::new()
        .border(Border::rounded(), &[])
        .border_foreground(&["#874BFD"])
        .padding(&[1, 0]);
    let button_style = Style::new()
        .foreground("#FFF7DB")
        .background("#888B7E")
        .padding(&[0, 3])
        .margin_top(1);
    let active_button_style = button_style
        .clone()
        .foreground("#FFF7DB")
        .background("#F25D94")
        .margin_right(2)
        .underline(true);

    // List.
    let list_style = Style::new()
        .border(Border::normal(), &[false, true, false, false])
        .border_foreground(&[subtle])
        .margin_right(1)
        .height(8)
        .width(WIDTH / 3);
    let list_header = Style::new()
        .border_style(Border::normal())
        .border_bottom(true)
        .border_foreground(&[subtle])
        .margin_right(2);
    let list_item = |s: &str| Style::new().padding_left(2).render(s);
    let check_mark = Style::new()
        .set_string(&["✓"])
        .foreground(special)
        .padding_right(1)
        .string();
    let list_done = |s: &str| {
        format!(
            "{}{}",
            check_mark,
            Style::new()
                .strikethrough(true)
                .foreground(pick(has_dark_bg, "#969B86", "#696969"))
                .render(s)
        )
    };

    // Paragraphs/history.
    let history_style = Style::new()
        .align(&[LEFT])
        .foreground("#FAFAFA")
        .background(highlight)
        .margin(&[1, 3, 0, 0])
        .padding(&[1, 2])
        .height(19)
        .width(COLUMN_WIDTH);

    // Status bar.
    let status_nugget = Style::new().foreground("#FFFDF5").padding(&[0, 1]);
    let status_bar_style = Style::new()
        .foreground(pick(has_dark_bg, "#343433", "#C1C6B2"))
        .background(pick(has_dark_bg, "#D9DCCF", "#353533"));
    let status_style = status_bar_style
        .clone()
        .foreground("#FFFDF5")
        .background("#FF5F87")
        .padding(&[0, 1])
        .margin_right(1);
    let encoding_style = status_nugget.clone().background("#A550DF").align(&[RIGHT]);
    let status_text = status_bar_style.clone();
    let fish_cake_style = status_nugget.background("#6124DF");

    // Floating thing.
    let floating_style = Style::new()
        .italic(true)
        .foreground("#FFF7DB")
        .background("#F25D94")
        .padding(&[1, 6])
        .align(&[CENTER]);

    // Page.
    let doc_style = Style::new().padding(&[1, 2, 1, 2]);

    let mut doc = String::new();

    // Tabs.
    {
        let row = join_horizontal(
            TOP,
            &[
                &active_tab.render("Lip Gloss"),
                &tab.render("Blush"),
                &tab.render("Eye Shadow"),
                &tab.render("Mascara"),
                &tab.render("Foundation"),
            ],
        );
        let gap = tab_gap.render(&" ".repeat(WIDTH.saturating_sub(size::width(&row)).saturating_sub(2)));
        let row = join_horizontal(BOTTOM, &[&row, &gap]);
        doc.push_str(&row);
        doc.push_str("\n\n");
    }

    // Title.
    {
        let colors = color_grid(1, 5);
        let mut title = String::new();
        for (i, v) in colors.iter().enumerate() {
            const OFFSET: usize = 2;
            title.push_str(
                &title_style
                    .clone()
                    .margin_left(i * OFFSET)
                    .background_color(v[0].clone())
                    .string(),
            );
            if i < colors.len() - 1 {
                title.push('\n');
            }
        }

        let desc = join_vertical(
            LEFT,
            &[
                &desc_style.render("Style Definitions for Nice Terminal Layouts"),
                &info_style.render(&format!(
                    "From Charm{}{}",
                    divider,
                    url("https://github.com/charmbracelet/lipgloss")
                )),
            ],
        );

        let row = join_horizontal(TOP, &[&title, &desc]);
        doc.push_str(&row);
        doc.push_str("\n\n");
    }

    // Dialog.
    {
        let ok_button = active_button_style.render("Yes");
        let cancel_button = button_style.render("Maybe");

        let grad = apply_gradient(
            Style::new(),
            "Are you sure you want to eat marmalade?",
            Color::parse("#EDFF82"),
            Color::parse("#F25D94"),
        );

        let question = Style::new().width(50).align(&[CENTER]).render(&grad);

        let buttons = join_horizontal(TOP, &[&ok_button, &cancel_button]);
        let ui = join_vertical(CENTER, &[&question, &buttons]);

        let opts: Vec<Whitespace> = vec![
            with_whitespace_chars("猫咪"),
            with_whitespace_style(Style::new().foreground(subtle)),
        ];
        let dialog = position::place(
            WIDTH,
            9,
            CENTER,
            CENTER,
            &dialog_box_style.render(&ui),
            &opts,
        );

        doc.push_str(&dialog);
        doc.push_str("\n\n");
    }

    // Color grid.
    let colors: String = {
        let colors = color_grid(14, 8);
        let mut b = String::new();
        for x in &colors {
            for y in x {
                b.push_str(&Style::new().set_string(&["  "]).background_color(y.clone()).string());
            }
            b.push('\n');
        }
        b
    };

    let lists = join_horizontal(
        TOP,
        &[
            &list_style.render(&join_vertical(
                LEFT,
                &[
                    &list_header.render("Citrus Fruits to Try"),
                    &list_done("Grapefruit"),
                    &list_done("Yuzu"),
                    &list_item("Citron"),
                    &list_item("Kumquat"),
                    &list_item("Pomelo"),
                ],
            )),
            &list_style.render(&join_vertical(
                LEFT,
                &[
                    &list_header.render("Actual Lip Gloss Vendors"),
                    &list_item("Glossier"),
                    &list_item("Claire‘s Boutique"),
                    &list_done("Nyx"),
                    &list_item("Mac"),
                    &list_done("Milk"),
                ],
            )),
        ],
    );

    doc.push_str(&join_horizontal(
        TOP,
        &[&lists, &Style::new().margin_left(1).render(&colors)],
    ));

    // Marmalade history.
    {
        const HISTORY_A: &str = "The Romans learned from the Greeks that quinces slowly cooked with honey would “set” when cool. The Apicius gives a recipe for preserving whole quinces, stems and leaves attached, in a bath of honey diluted with defrutum: Roman marmalade. Preserves of quince and lemon appear (along with rose, apple, plum and pear) in the Book of ceremonies of the Byzantine Emperor Constantine VII Porphyrogennetos.";
        const HISTORY_B: &str = "Medieval quince preserves, which went by the French name cotignac, produced in a clear version and a fruit pulp version, began to lose their medieval seasoning of spices in the 16th century. In the 17th century, La Varenne provided recipes for both thick and clear cotignac.";
        const HISTORY_C: &str = "In 1524, Henry VIII, King of England, received a “box of marmalade” from Mr. Hull of Exeter. This was probably marmelada, a solid quince paste from Portugal, still made and sold in southern Europe today. It became a favourite treat of Anne Boleyn and her ladies in waiting.";

        doc.push_str(&join_horizontal(
            TOP,
            &[
                &history_style.clone().align(&[RIGHT]).render(HISTORY_A),
                &history_style.clone().align(&[CENTER]).render(HISTORY_B),
                &history_style.clone().margin_right(0).render(HISTORY_C),
            ],
        ));

        doc.push_str("\n\n");
    }

    // Status bar.
    {
        let light_dark_state = if has_dark_bg { "Dark" } else { "Light" };

        let status_key = status_style.render("STATUS");
        let encoding = encoding_style.render("UTF-8");
        let fish_cake = fish_cake_style.render("🍥 Fish Cake");
        let status_val = status_text
            .clone()
            .width(WIDTH - size::width(&status_key) - size::width(&encoding) - size::width(&fish_cake))
            .render(&format!("Ravishingly {}!", light_dark_state));

        let bar = join_horizontal(TOP, &[&status_key, &status_val, &encoding, &fish_cake]);

        doc.push_str(&status_bar_style.clone().width(WIDTH).render(&bar));
    }

    // physicalWidth is 0 when output is not a TTY, in which case no truncation
    // is applied (matching upstream `term.GetSize` behavior).

    // Render the document.
    let document = doc_style.render(&doc);

    // Surprise! Composite some bonus content on top of the document.
    let modal = floating_style.render("Now with Compositing!");
    let layers = vec![
        new_layer(&document, &[]),
        new_layer(&modal, &[]).x(58).y(44),
    ];
    let comp = new_compositor(&layers);

    // Print with a special writer that downsamples colors to the terminal's
    // palette (or strips them entirely when output is not a TTY).
    println(&comp.render()).unwrap();
}

#[allow(dead_code)]
fn debug_doc(doc: &str) {
    for (i, line) in doc.lines().enumerate() {
        println!("doc line {}: {} | width={}", i, line, charming_lipgloss::size::width(line));
    }
}
