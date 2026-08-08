//! Cleanroom Rust port of upstream Go example: `examples/list/simple/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! A simple nested list with a custom roman-numeral sub-list enumerator.

use std::sync::Arc;

use charming_lipgloss::list::{self, roman};
use charming_lipgloss::writer::println;

fn main() {
    let sub = list::new(&["D", "E", "F"]).enumerator(Arc::new(roman));
    let l = list::new(&["A", "B", "C"]).child_list(sub).item("G");
    println(&l.render()).unwrap();
}
