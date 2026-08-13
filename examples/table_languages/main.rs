//! Cleanroom Rust port of upstream Go example: `examples/table/languages/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! A table of greetings in various languages with styled rows and borders.

use rusty_lipgloss::align::Position;
use rusty_lipgloss::border::Border;
use rusty_lipgloss::style::Style;
use rusty_lipgloss::table::{self, HEADER_ROW};
use rusty_lipgloss::writer::println;

const PURPLE: &str = "99";
const GRAY: &str = "245";
const LIGHT_GRAY: &str = "241";

fn main() {
    let header_style = Style::new()
        .foreground(PURPLE)
        .bold(true)
        .align(&[Position::CENTER]);
    let cell_style = Style::new().padding(&[0, 1]).width(14);
    let odd_row_style = cell_style.clone().foreground(GRAY);
    let even_row_style = cell_style.foreground(LIGHT_GRAY);
    let border_style = Style::new().foreground(PURPLE);

    let rows: Vec<Vec<&str>> = vec![
        vec!["Chinese", "您好", "你好"],
        vec!["Japanese", "こんにちは", "やあ"],
        vec!["Arabic", "أهلين", "أهلا"],
        vec!["Russian", "Здравствуйте", "Привет"],
        vec!["Spanish", "Hola", "¿Qué tal?"],
    ];

    let rows_clone = rows.clone();

    let mut t = table::Table::new()
        .border(Border::thick())
        .border_style(border_style)
        .headers(&["LANGUAGE", "FORMAL", "INFORMAL"])
        .style_func(Box::new(move |row, col| {
            let mut style = if row == HEADER_ROW {
                header_style.clone()
            } else if row % 2 == 0 {
                even_row_style.clone()
            } else {
                odd_row_style.clone()
            };

            // Make the second column a little wider.
            if col == 1 {
                style = style.width(22);
            }

            // Arabic is a right-to-left language, so right align the text.
            if row >= 0
                && (row as usize) < rows_clone.len()
                && rows_clone[row as usize][0] == "Arabic"
                && col != 0
            {
                style = style.align(&[Position::RIGHT]);
            }

            style
        }));

    for row in &rows {
        let cells: Vec<&str> = row.to_vec();
        t = t.row(&cells);
    }
    t = t.row(&[
        "English",
        "You look absolutely fabulous.",
        "How's it going?",
    ]);

    println(&t.string()).unwrap();
}
