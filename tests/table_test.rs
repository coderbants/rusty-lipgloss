use charming_lipgloss::table::Table;

#[test]
fn test_table_render() {
    let t = Table::new()
        .headers(&["Name", "Age"])
        .row(&["Alice", "30"])
        .row(&["Bob", "25"]);
    assert_eq!(t.render(), "Name | Age\nAlice | 30\nBob | 25");
}
