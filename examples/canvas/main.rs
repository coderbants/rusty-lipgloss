//! Cleanroom Rust port of upstream Go example: `examples/canvas/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! Composes layers with z-ordering onto a canvas via a compositor.

use charming_lipgloss::border::Border;
use charming_lipgloss::color::Color;
use charming_lipgloss::layer::{new_compositor, new_layer, Layer};
use charming_lipgloss::query;
use charming_lipgloss::style::Style;
use charming_lipgloss::writer::println;

/// Fills a rectangular area with a given character in a given color.
fn new_field(rows: usize, cols: usize, color: &str) -> String {
    let field_style = Style::new().foreground(color);
    let mut field = String::new();
    for i in 0..rows {
        for _ in 0..cols {
            field.push('/');
        }
        if i < rows - 1 {
            field.push('\n');
        }
    }
    field_style.render(&field)
}

/// Creates a little card with rounded borders and a text label.
fn new_card(dark_mode: bool, text: &str) -> String {
    let border_colors = ["#FF7582", "#6A5AFF", "#76FFA0", "#6A5AFF", "#FF5F00"];
    let fg = if dark_mode { "#FFE8C8" } else { "#4A4A4A" };
    Style::new()
        .border(Border::rounded(), &[])
        .border_foreground_blend(&border_colors)
        .foreground(fg)
        .height(9)
        .width(16)
        .padding_top(3)
        .align(&[charming_lipgloss::align::CENTER])
        .render(text)
}

fn main() {
    let dark_mode = query::has_dark_background();

    // A few text blocks.
    let lighter_field = new_field(17, 43, if dark_mode { "#3A3A3A" } else { "#E8E8E8" });
    let darker_field = new_field(17, 43, if dark_mode { "#1E1E1E" } else { "#2E2E2E" });

    // A few layers. Layers are created from strings (or blocks of text).
    let pickles = new_layer(&new_card(dark_mode, "Pickles"), &[]);
    let melon = new_layer(&new_card(dark_mode, "Bitter Melon"), &[]);
    let sriracha = new_layer(&new_card(dark_mode, "Sriracha"), &[]);

    // Layers can have X, Y, and Z offsets.
    let lighter = new_layer(&lighter_field, &[]).x(5).y(2);
    let darker = {
        let mut l = new_layer(&darker_field, &[]);
        let mut pickles = pickles.x(4).y(2).z(1); // the Z index places this layer above the others
        let mut melon = melon.x(22).y(1);
        let mut sriracha = sriracha.x(11).y(7);
        l.add_layers(&[pickles.clone(), melon.clone(), sriracha.clone()]);
        let _ = (&mut pickles, &mut melon, &mut sriracha);
        l.clone()
    };

    // A compositor takes multiple layers and composites them together into
    // a single output.
    let layers = vec![lighter, darker];
    let comp = new_compositor(&layers);

    println(&comp.render()).unwrap();
    let _ = &Layer::new;
    let _ = Color::NoColor;
}
