//! Cleanroom Rust port of upstream Go example: `examples/color/standalone/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! Detects the terminal background color and chooses light or dark colors
//! accordingly, in a standalone (non-Bubble Tea) fashion.

use charming_lipgloss::align::CENTER;
use charming_lipgloss::border::Border;
use charming_lipgloss::join::join_vertical;
use charming_lipgloss::query;
use charming_lipgloss::style::Style;
use charming_lipgloss::writer::println;

fn pick<'a>(dark: bool, light: &'a str, dark_c: &'a str) -> &'a str {
    if dark {
        dark_c
    } else {
        light
    }
}

fn main() {
    // Query for the background color. We only need to do this once, and only
    // when using Lip Gloss standalone.
    let has_dark_bg = query::has_dark_background();

    // Define some styles, choosing appropriate light or dark colors.
    let frame_style = Style::new()
        .border(Border::rounded(), &[])
        .border_foreground(&[pick(has_dark_bg, "#C5ADF9", "#864EFF")])
        .padding(&[1, 3])
        .margin(&[1, 3]);
    let paragraph_style = Style::new().width(40).margin_bottom(1).align(&[CENTER]);
    let text_style = Style::new().foreground(pick(has_dark_bg, "#696969", "#bdbdbd"));
    let keyword_style = Style::new()
        .foreground(pick(has_dark_bg, "#37CD96", "#22C78A"))
        .bold(true);

    let active_button = Style::new()
        .padding(&[0, 3])
        .background("#FF6AD2")
        .foreground("#FFFCC2");
    let inactive_button = active_button
        .clone()
        .background(pick(has_dark_bg, "#988F95", "#978692"))
        .foreground(pick(has_dark_bg, "#FDFCE3", "#FBFAE7"));

    // Build layout.
    let text = paragraph_style.render(&format!(
        "{}Are you sure you want to eat that {} banana?",
        text_style.render(""),
        keyword_style.render("moderatly ripe")
    ));
    let buttons = format!(
        "{}  {}",
        active_button.render("Yes"),
        inactive_button.render("No")
    );
    let block = frame_style.render(&join_vertical(CENTER, &[&text, &buttons]));

    // Print the block to stdout.
    println(&block).unwrap();
}
