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

/// Ported from upstream `TestBullet`: alphabet and roman enumerators at
/// various indices.
#[test]
fn test_bullet_alphabet_and_roman() {
    use rusty_lipgloss::list::{alphabet, roman};
    use rusty_lipgloss::tree::new_string_data;
    let items = new_string_data(&["a", "b", "c"]);
    // Alphabet: column arithmetic beyond Z.
    let cases: &[(usize, &str)] = &[
        (0, "A"),
        (25, "Z"),
        (26, "AA"),
        (51, "AZ"),
        (52, "BA"),
        (79, "CB"),
        (701, "ZZ"),
        (702, "AAA"),
        (801, "ADV"),
        (1000, "ALM"),
    ];
    for (i, exp) in cases {
        let bullet = alphabet(&items, *i);
        let bullet = bullet.trim_end_matches('.');
        assert_eq!(bullet, *exp, "alphabet index {i}");
    }
    // Roman numerals.
    let roman_cases: &[(usize, &str)] = &[
        (0, "I"),
        (25, "XXVI"),
        (26, "XXVII"),
        (50, "LI"),
        (100, "CI"),
        (701, "DCCII"),
        (1000, "MI"),
    ];
    for (i, exp) in roman_cases {
        let bullet = roman(&items, *i);
        let bullet = bullet.trim_end_matches('.');
        assert_eq!(bullet, *exp, "roman index {i}");
    }
}

/// Ported from upstream `TestList` / `TestListItems`: item and multi-item
/// construction render identically.
#[test]
fn test_list_items_variants() {
    use rusty_lipgloss::list::List;
    let a = List::new().item("Foo").item("Bar").item("Baz");
    let b = List::new().items(&["Foo", "Bar", "Baz"]);
    assert_eq!(a.render(), b.render());
}

/// Ported from upstream `TestListIntegers` and `TestMultiline`.
#[test]
fn test_list_multiline_items() {
    use rusty_lipgloss::list::List;
    let l = List::new()
        .item("Item1\nline 2\nline 3")
        .item("Item2\nline 2\nline 3")
        .item("3");
    let out = l.render();
    assert!(out.contains("Item1"));
    assert!(out.contains("line 3"));
    assert!(out.contains("3"));
}

/// Ported from upstream `TestEnumerators`: each enumerator renders its symbol.
#[test]
fn test_list_all_enumerators() {
    use rusty_lipgloss::list::{alphabet, arabic, asterisk, bullet, dash, roman, List};
    use std::sync::Arc;
    let build = |e: fn(&dyn rusty_lipgloss::tree::Children, usize) -> String| {
        List::new()
            .enumerator(Arc::new(e))
            .item("Foo")
            .item("Bar")
            .item("Baz")
            .render()
    };
    assert_eq!(build(alphabet), "A. Foo\nB. Bar\nC. Baz");
    assert_eq!(build(arabic), "1. Foo\n2. Bar\n3. Baz");
    assert_eq!(build(bullet), "• Foo\n• Bar\n• Baz");
    assert_eq!(build(asterisk), "* Foo\n* Bar\n* Baz");
    assert_eq!(build(dash), "- Foo\n- Bar\n- Baz");
    let roman_out = build(roman);
    assert!(roman_out.contains("  I. Foo"));
}
