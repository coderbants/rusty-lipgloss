//! Cleanroom Rust port of upstream Go example: `examples/brightness/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! Demonstrates the Lighten and Darken functions to create progressive
//! brightness variations.

use charming_lipgloss::color::{darken, lighten, Color};
use charming_lipgloss::query;
use charming_lipgloss::style::Style;
use charming_lipgloss::writer::println;

fn main() {
    let has_dark_bg = query::has_dark_background();
    let name_color = if has_dark_bg { "#E2E8F0" } else { "#2D3748" };
    let color_name_style = Style::new().bold(true).foreground(name_color);

    // Base colors to demonstrate lightening and darkening.
    let base_colors: Vec<(&str, Color)> = vec![
        ("Red", Color::parse("#FF0000")),
        ("Blue", Color::parse("#0066FF")),
        ("Green", Color::parse("#00FF00")),
        ("Gray", Color::parse("#808080")),
    ];

    // Percentage to lighten/darken by.
    let percentage = 0.05; // 5%

    // Number of steps to generate.
    let steps = 20;

    let mut content = String::new();
    for (name, base_color) in &base_colors {
        content.push_str(&color_name_style.render(name));
        content.push('\n');

        // Create lightened variations.
        content.push_str("Lightened: ");
        for i in 0..steps {
            content.push_str(
                &Style::new()
                    .foreground_color(lighten(base_color.clone(), percentage * (i as f64 + 1.0)))
                    .render("██"),
            );
        }
        content.push_str("\n");

        // Create darkened variations.
        content.push_str("Darkened:  ");
        for i in 0..steps {
            content.push_str(
                &Style::new()
                    .foreground_color(darken(base_color.clone(), percentage * (i as f64 + 1.0)))
                    .render("██"),
            );
        }
        content.push_str("\n\n");
    }

    println(&content).unwrap();
}
