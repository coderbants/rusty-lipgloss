use charming_lipgloss::list::List;

#[test]
fn test_list_render() {
    let l = List::new().item("First").item("Second");
    assert_eq!(l.render(), "• First\n• Second");
}
