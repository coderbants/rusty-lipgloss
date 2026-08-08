//! Cleanroom Rust port of upstream Go example: `examples/table/ansi/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! A minimal table with ANSI-styled cells.

use charming_lipgloss::style::Style;
use charming_lipgloss::table::Table;
use charming_lipgloss::writer::println;

fn main() {
    let s = |text: &str| Style::new().foreground("240").render(text);

    let mut t = Table::new();
    t = t.row(&[&s("Bubble Tea"), &s("Milky")]);
    t = t.row(&[&s("Milk Tea"), &s("Also milky")]);
    t = t.row(&[&s("Actual milk"), &s("Milky as well")]);
    println(&t.string()).unwrap();
}
