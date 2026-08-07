use charming_lipgloss::tree::Tree;

#[test]
fn test_tree_render() {
    let t = Tree::new("Root").child(Tree::new("Child"));
    assert_eq!(t.render(), "Root\n├── Child");
}
