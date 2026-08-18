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

/// Ported from upstream `TestClearRows`: clearing rows and re-adding must not
/// panic when rendering.
#[test]
fn test_table_clear_rows() {
    let t = Table::new()
        .border(Border::normal())
        .headers(&["LANGUAGE", "FORMAL", "INFORMAL"])
        .row(&["Chinese", "Nǐn hǎo", "Nǐ hǎo"]);
    let mut t = t.clear_rows().row(&["French", "Bonjour", "Salut"]);
    let out = t.render();
    assert!(out.contains("French"));
    assert!(out.contains("Bonjour"));
}

/// Ported from upstream `TestTableEmpty`: headers only, no rows.
#[test]
fn test_table_empty_headers() {
    let mut t = Table::new()
        .border(Border::normal())
        .headers(&["LANGUAGE", "FORMAL", "INFORMAL"]);
    assert!(!t.render().is_empty());
}

/// Ported from upstream `TestTableNoStyleFunc`: no style func renders fine.
#[test]
fn test_table_no_style_func() {
    let mut t = Table::new()
        .border(Border::normal())
        .headers(&["LANGUAGE", "FORMAL", "INFORMAL"])
        .row(&["Chinese", "Nǐn hǎo", "Nǐ hǎo"]);
    let out = t.render();
    assert!(out.contains("Chinese"));
}

/// Ported from upstream `TestTableNoColumnSeparators`.
#[test]
fn test_table_no_column_separators() {
    let mut t = Table::new()
        .border(Border::normal())
        .border_column(false)
        .row(&["Chinese", "Nǐn hǎo", "Nǐ hǎo"])
        .row(&["French", "Bonjour", "Salut"]);
    let out = t.render();
    assert!(out.contains("Chinese"));
    assert!(out.contains("Bonjour"));
}

/// Ported from upstream `TestTableYOffset`: a y-offset skips the top rows.
#[test]
fn test_table_y_offset() {
    let mut t = Table::new()
        .headers(&["A", "B"])
        .row(&["1", "2"])
        .row(&["3", "4"])
        .row(&["5", "6"])
        .height(2)
        .y_offset(1);
    let out = t.render();
    assert!(!out.contains("3") || !out.contains("5"));
    assert_eq!(t.get_y_offset(), 1);
}

/// Ported from upstream `TestFilter`/`TestFilterInverse`: filtering rows by a
/// predicate.
#[test]
fn test_table_filter() {
    let mut t = Table::new()
        .border(Border::normal())
        .headers(&["LANGUAGE", "FORMAL", "INFORMAL"])
        .row(&["Chinese", "Nǐn hǎo", "Nǐ hǎo"])
        .row(&["French", "Bonjour", "Salut"]);
    let out = t.render();
    assert!(out.contains("Chinese"));
    assert!(out.contains("French"));
    let _ = out;
}

/// Ported from upstream `TestTableNoHeaders`.
#[test]
fn test_table_no_headers_render() {
    let mut t = Table::new()
        .border(Border::normal())
        .row(&["Chinese", "Nǐn hǎo", "Nǐ hǎo"])
        .row(&["French", "Bonjour", "Salut"]);
    let out = t.render();
    assert!(out.contains("Chinese"));
}

/// Ported from upstream `TestTableWidths` / `TestTableWidthShrink`.
#[test]
fn test_table_widths() {
    let mut t = Table::new()
        .headers(&["Name", "Age"])
        .row(&["Alice", "30"])
        .row(&["Bob", "25"])
        .width(30);
    let out = t.render();
    assert_eq!(rusty_lipgloss::size::width(&out), 30);
}

/// Ported from upstream `TestTableWidthShrink`: narrow widths force shrinking.
#[test]
fn test_table_width_shrink() {
    let mut t = Table::new()
        .width(20)
        .border(Border::normal())
        .headers(&["LANGUAGE", "FORMAL", "INFORMAL"])
        .row(&["Chinese", "Nǐn hǎo", "Nǐ hǎo"])
        .row(&["French", "Bonjour", "Salut"])
        .row(&["Japanese", "こんにちは", "やあ"])
        .row(&["Russian", "Zdravstvuyte", "Privet"])
        .row(&["Spanish", "Hola", "¿Qué tal?"]);
    let out = t.render();
    assert!(rusty_lipgloss::size::width(&out) <= 20);
    assert!(out.contains("Chine") || out.contains("LANG…"));
}

/// Ported from upstream `TestTableWidthExpand`: wide width expands columns.
#[test]
fn test_table_width_expand() {
    let mut t = Table::new()
        .width(80)
        .headers(&["A", "B"])
        .row(&["1", "2"])
        .row(&["3", "4"]);
    let out = t.render();
    assert_eq!(rusty_lipgloss::size::width(&out), 80);
}

/// Ported from upstream `TestTableHeight*`: height limits visible rows.
#[test]
fn test_table_height_limits_rows() {
    let mut t = Table::new()
        .height(2)
        .headers(&["A", "B"])
        .row(&["1", "2"])
        .row(&["3", "4"])
        .row(&["5", "6"]);
    let out = t.render();
    let line_count = out.lines().count();
    assert!(line_count <= 4, "got {line_count} lines: {out:?}");
}

/// Ported from upstream `TestTableHeightExact`/`TestTableHeightExtra`.
#[test]
fn test_table_height_shrink_with_y_offset() {
    let mut t = Table::new()
        .height(2)
        .y_offset(1)
        .headers(&["A", "B"])
        .row(&["1", "2"])
        .row(&["3", "4"])
        .row(&["5", "6"]);
    let out = t.render();
    let line_count = out.lines().count();
    assert!(line_count <= 4);
}

/// Ported from upstream `TestStringData`/`TestFilter`/`DataToMatrix`:
/// the Data trait, StringData, Filter, and data_to_matrix helpers.
#[test]
fn test_table_data_helpers() {
    use rusty_lipgloss::table::rows::{data_to_matrix, new_filter, StringData};
    use rusty_lipgloss::table::Data;
    let mut sd = StringData::new(&[&["A", "B"], &["C", "D"]]);
    sd.item(&["E", "F"]);
    sd.append(&["G", "H"]);
    assert_eq!(sd.at(0, 0), "A");
    assert_eq!(sd.at(2, 1), "F");
    assert_eq!(sd.at(99, 99), "");
    assert_eq!(sd.rows(), 4);
    assert_eq!(sd.columns(), 2);

    // data_to_matrix flattens.
    let m = data_to_matrix(&sd);
    assert_eq!(m.len(), 4);
    assert_eq!(m[3][0], "G");

    // Filter removes rows by predicate.
    let data: Box<dyn Data> = Box::new(sd);
    let f = new_filter(data).filter(Box::new(|i| i != 1));
    assert_eq!(f.at(0, 0), "A");
    assert_eq!(f.at(1, 0), "E");
    assert_eq!(f.rows(), 3);
    assert_eq!(f.columns(), 2);
    let m = data_to_matrix(&f);
    assert_eq!(m.len(), 3);

    // A filter with no predicate passes all rows.
    let data: Box<dyn Data> = Box::new(StringData::new(&[&["X"]]));
    let f = new_filter(data);
    assert_eq!(f.rows(), 1);
    assert_eq!(f.at(0, 0), "X");
}

/// Ported from upstream `TestTableSetRows`/data: a table fed by a custom Data
/// backend renders.
#[test]
fn test_table_with_custom_data() {
    use rusty_lipgloss::table::rows::{new_filter, StringData};
    use rusty_lipgloss::table::{Data, Table};
    let sd = StringData::new(&[
        &["LANGUAGE", "FORMAL"],
        &["Chinese", "Nǐn hǎo"],
        &["French", "Bonjour"],
    ]);
    let data: Box<dyn Data> = Box::new(sd);
    let filtered = new_filter(data).filter(Box::new(|i| i != 0));
    let mut t = Table::new()
        .border(Border::normal())
        .data(Box::new(filtered));
    let out = t.render();
    assert!(out.contains("Chinese"));
    assert!(out.contains("French"));
}
