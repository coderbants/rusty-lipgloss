//! Cleanroom Rust port of upstream Go example: `examples/blending/linear-2d/standalone/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! Demonstrates 2D color gradients created with `blend_2d`.

use charming_lipgloss::align::CENTER;
use charming_lipgloss::blending::blend_2d;
use charming_lipgloss::border::Border;
use charming_lipgloss::color::Color;
use charming_lipgloss::query;
use charming_lipgloss::style::Style;
use charming_lipgloss::writer::println;

struct Gradient {
    name: &'static str,
    stops: Vec<Color>,
    angle: f64,
}

fn main() {
    let has_dark_bg = query::has_dark_background();

    let gradients = vec![
        Gradient {
            name: "Sunset Diagonal",
            stops: vec![
                Color::parse("#FF6B6B"),
                Color::parse("#FFB74D"),
                Color::parse("#FFDFBA"),
            ],
            angle: 45.0,
        },
        Gradient {
            name: "Ocean Wave",
            stops: vec![
                Color::parse("#0077B6"),
                Color::parse("#48CAE4"),
                Color::parse("#ADE8F4"),
            ],
            angle: 90.0,
        },
        Gradient {
            name: "Forest Mist",
            stops: vec![
                Color::parse("#228B22"),
                Color::parse("#90EE90"),
                Color::parse("#FFFFE0"),
            ],
            angle: 135.0,
        },
        Gradient {
            name: "Purple Dream",
            stops: vec![
                Color::parse("#9370DB"),
                Color::parse("#DDA0DD"),
                Color::parse("#FFB6C1"),
            ],
            angle: 180.0,
        },
        Gradient {
            name: "Fire Gradient",
            stops: vec![
                Color::parse("#FF0000"),
                Color::parse("#FFA500"),
                Color::parse("#FFFF00"),
            ],
            angle: 225.0,
        },
    ];

    let title_style = Style::new()
        .bold(true)
        .foreground(if has_dark_bg { "#E2E8F0" } else { "#2D3748" })
        .margin_bottom(1)
        .align(&[CENTER]);
    let gradient_style = Style::new()
        .border(Border::rounded(), &[])
        .border_foreground(&[if has_dark_bg { "#A0AEC0" } else { "#718096" }])
        .margin_bottom(1);
    let name_style = Style::new()
        .bold(true)
        .foreground(if has_dark_bg { "#CBD5E0" } else { "#4A5568" })
        .margin_bottom(1);

    let mut content = String::new();
    content.push_str(&title_style.render("2D Color Gradient Examples with Blend2D"));
    content.push_str("\n\n");

    for g in &gradients {
        let (width, height) = (30usize, 12usize);
        let blended = blend_2d(width, height, g.angle, &g.stops);

        let mut box_str = String::new();
        for y in 0..height {
            for x in 0..width {
                box_str.push_str(
                    &Style::new()
                        .foreground_color(blended[y * width + x].clone())
                        .render("█"),
                );
            }
            if y < height - 1 {
                box_str.push('\n');
            }
        }

        content.push_str(&name_style.render(&format!("{} (Angle: {:.0}°)", g.name, g.angle)));
        content.push_str("\n");
        content.push_str(&gradient_style.render(&box_str));
        content.push_str("\n");
    }

    println(&content).unwrap();
}
