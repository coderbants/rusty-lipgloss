//! Cleanroom Rust port of upstream Go test file: `table/table_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::border::Border;
use rusty_lipgloss::table::Table;

#[test]
fn test_table_render() {
    let mut t = Table::new()
        .headers(&["Name", "Age"])
        .row(&["Alice", "30"])
        .row(&["Bob", "25"]);
    let out = t.render();
    assert!(out.contains("Name"));
    assert!(out.contains("Age"));
    assert!(out.contains("Alice"));
    assert!(out.contains("Bob"));
}

#[test]
fn test_table_empty() {
    let mut t = Table::new();
    assert_eq!(t.render(), "");
}

#[test]
fn test_table_border_render() {
    let mut t = Table::new()
        .border(Border::normal())
        .headers(&["A", "B"])
        .row(&["1", "2"]);
    let out = t.render();
    assert!(out.starts_with("┌─"));
    assert!(out.contains("│"));
}

#[test]
fn test_table_width() {
    let mut t = Table::new()
        .width(30)
        .headers(&["Name", "Age"])
        .row(&["Alice", "30"]);
    let out = t.render();
    assert_eq!(rusty_lipgloss::size::width(&out), 30);
}

#[test]
fn test_table_no_headers() {
    let mut t = Table::new().row(&["1", "2"]).row(&["3", "4"]);
    let out = t.render();
    assert!(out.contains("1"));
    assert!(out.contains("4"));
}

#[test]
fn test_table_wrap_disabled() {
    let mut t = Table::new()
        .wrap(false)
        .width(20)
        .headers(&["Name"])
        .row(&["a very long name that must be truncated"]);
    let out = t.render();
    assert!(rusty_lipgloss::size::width(&out) <= 20);
}
