//! Cleanroom Rust port of upstream Go example: `examples/list/grocery/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! A grocery list with purchased-item check marks, styled enumerators, and
//! strikethrough items.

use std::sync::Arc;

use rusty_lipgloss::list::{self, List};
use rusty_lipgloss::style::Style;
use rusty_lipgloss::tree::Children;
use rusty_lipgloss::writer::println;

const PURCHASED: [&str; 9] = [
    "Bananas",
    "Barley",
    "Cashews",
    "Coconut Milk",
    "Dill",
    "Eggs",
    "Fish Cake",
    "Leeks",
    "Papaya",
];

fn is_purchased(value: &str) -> bool {
    PURCHASED.contains(&value)
}

fn grocery_enumerator(children: &dyn Children, i: usize) -> String {
    match children.at(i) {
        Some(node) if is_purchased(node.value()) => "✓".to_string(),
        _ => "•".to_string(),
    }
}

fn enum_style_func(children: &dyn Children, i: usize) -> Style {
    let dim = Style::new().foreground("240").margin_right(1);
    let highlighted = Style::new().foreground("10").margin_right(1);
    match children.at(i) {
        Some(node) if is_purchased(node.value()) => highlighted,
        _ => dim,
    }
}

fn item_style_func(children: &dyn Children, i: usize) -> Style {
    let item_style = Style::new().foreground("255");
    match children.at(i) {
        Some(node) if is_purchased(node.value()) => item_style.strikethrough(true),
        _ => item_style,
    }
}

fn main() {
    let l: List = list::new(&[
        "Artichoke",
        "Baking Flour",
        "Bananas",
        "Barley",
        "Bean Sprouts",
        "Cashew Apple",
        "Cashews",
        "Coconut Milk",
        "Curry Paste",
        "Currywurst",
        "Dill",
        "Dragonfruit",
        "Dried Shrimp",
        "Eggs",
        "Fish Cake",
        "Furikake",
        "Jicama",
        "Kohlrabi",
        "Leeks",
        "Lentils",
        "Licorice Root",
    ])
    .enumerator(Arc::new(grocery_enumerator))
    .enumerator_style_func(Arc::new(enum_style_func))
    .item_style_func(Arc::new(item_style_func));

    println(&l.render()).unwrap();
}
