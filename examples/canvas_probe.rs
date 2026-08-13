use rusty_lipgloss::canvas::{new_canvas, Canvas};
use rusty_lipgloss::position::place;
use rusty_lipgloss::whitespace::{with_whitespace_chars, with_whitespace_style};
use rusty_lipgloss::{new_style, Color, LEFT, TOP};

fn main() {
    let bg = place(
        60,
        5,
        TOP,
        LEFT,
        &new_style()
            .foreground_color(Color::parse("239"))
            .padding(&[1, 2])
            .render("Click to spawn."),
        &[
            with_whitespace_chars("/"),
            with_whitespace_style(new_style().foreground_color(Color::parse("238"))),
        ],
    );
    println!("BG: {:?}", bg);
    let mut c: Canvas = new_canvas(60, 5);

    let bounds = c.bounds();
    let comp = rusty_ultraviolet::new_styled_string(&bg);
    comp.draw(&mut c, bounds);
    let out = c.render();
    println!("CANVAS: {:?}", out);
}
