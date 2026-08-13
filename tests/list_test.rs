//! Cleanroom Rust port of upstream Go test file: `list/list_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use std::sync::Arc;

use rusty_lipgloss::list::{arabic, bullet, roman};

#[test]
fn test_list_render() {
    let l = rusty_lipgloss::list::List::new()
        .item("First")
        .item("Second");
    assert_eq!(l.render(), "• First\n• Second");
}

#[test]
fn test_list_new_with_items() {
    let l = rusty_lipgloss::list::new(&["Foo", "Bar", "Baz"]);
    assert_eq!(l.render(), "• Foo\n• Bar\n• Baz");
}

#[test]
fn test_list_arabic_enumerator() {
    let l = rusty_lipgloss::list::new(&["Foo", "Bar", "Baz"]).enumerator(Arc::new(arabic));
    assert_eq!(l.render(), "1. Foo\n2. Bar\n3. Baz");
}

#[test]
fn test_list_roman_enumerator() {
    let l = rusty_lipgloss::list::new(&["Foo", "Bar", "Baz"]).enumerator(Arc::new(roman));
    let out = l.render();
    assert!(out.contains("I."));
    assert!(out.contains("II."));
    assert!(out.contains("III."));
}

#[test]
fn test_enumerator_functions() {
    let items = rusty_lipgloss::tree::new_string_data(&["a", "b"]);
    let empty: Vec<rusty_lipgloss::tree::Node> = Vec::new();
    let _ = &items;
    assert_eq!(bullet(&items, 0), "•");
    assert_eq!(arabic(&items, 0), "1.");
    assert_eq!(arabic(&items, 1), "2.");
    assert_eq!(roman(&items, 0), "I.");
    assert_eq!(roman(&items, 1), "II.");
    assert_eq!(rusty_lipgloss::list::alphabet(&items, 0), "A.");
    assert_eq!(rusty_lipgloss::list::dash(&items, 0), "-");
    assert_eq!(rusty_lipgloss::list::asterisk(&items, 0), "*");
    let _ = &empty;
}
