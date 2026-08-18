//! Cleanroom Rust port of upstream Go test file: `tree/tree_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use std::sync::Arc;

use rusty_lipgloss::tree::{rounded_enumerator, Child, Node, Tree};

fn t(value: &str) -> Tree {
    Tree::new().root(value)
}

#[test]
fn test_tree_render() {
    let tree = t("Root").child(Child::from(t("Child")));
    assert_eq!(tree.render(), "Root\n└── Child");
}

#[test]
fn test_tree_multiple_children() {
    let tree = t("Root")
        .child(Child::from(t("A")))
        .child(Child::from(t("B")))
        .child(Child::from(t("C")));
    assert_eq!(tree.render(), "Root\n├── A\n├── B\n└── C");
}

#[test]
fn test_tree_nested() {
    let tree = t("Root").child(Child::from(t("Parent").child(Child::from(t("Child")))));
    assert_eq!(tree.render(), "Root\n└── Parent\n    └── Child");
}

#[test]
fn test_tree_rounded_enumerator() {
    let tree = t("Root")
        .enumerator(Arc::new(rounded_enumerator))
        .child(Child::from(t("A")))
        .child(Child::from(t("B")));
    assert_eq!(tree.render(), "Root\n├── A\n╰── B");
}

#[test]
fn test_tree_hidden() {
    let tree = t("Root")
        .child(Child::from(t("A").hide(true)))
        .child(Child::from(t("B")));
    assert_eq!(tree.render(), "Root\n└── B");
}

#[test]
fn test_tree_string_leaves() {
    let tree = t("Root")
        .child(Child::Str("a".to_string()))
        .child(Child::Str("b".to_string()));
    assert_eq!(tree.render(), "Root\n├── a\n└── b");
}

/// Ported from upstream `TestAt`: out-of-range indices return `None`.
#[test]
fn test_tree_children_at() {
    use rusty_lipgloss::tree::Children;
    let data: Vec<rusty_lipgloss::tree::Node> = vec![
        rusty_lipgloss::tree::Node::leaf("Foo".to_string()),
        rusty_lipgloss::tree::Node::leaf("Bar".to_string()),
    ];
    assert_eq!(
        <Vec<Node> as Children>::at(&data, 0).map(|n| n.value()),
        Some("Foo")
    );
    assert!(<Vec<Node> as Children>::at(&data, 10).is_none());
}

/// Ported from upstream `TestFilter`.
#[test]
fn test_tree_filter() {
    use rusty_lipgloss::tree::{new_filter, new_string_data, Children};
    let data = new_string_data(&["Foo", "Bar", "Baz", "Nope"]);
    let filtered = new_filter(&data, Box::new(|index| index != 3));
    assert_eq!(filtered.length(), 3);
    assert_eq!(Children::at(&filtered, 1).map(|n| n.value()), Some("Bar"));
    assert!(Children::at(&filtered, 10).is_none());
}

/// Ported from upstream `TestTreeStyleAt`: a custom enumerator that inspects
/// the item value.
#[test]
fn test_tree_custom_enumerator_inspects_value() {
    use rusty_lipgloss::tree::Children;
    use std::sync::Arc;

    let data: Vec<Child> = vec!["Foo".into(), "Bar".into()];
    let tree = Tree::new()
        .root("Root")
        .child_nodes(&data)
        .enumerator(Arc::new(|children: &dyn Children, i: usize| {
            if children.at(i).map(|n| n.value()) == Some("Foo") {
                ">".to_string()
            } else {
                "-".to_string()
            }
        }));
    let out = tree.render();
    assert!(out.contains("> Foo"));
    assert!(out.contains("- Bar"));
}

/// Ported from upstream `TestRootStyle`.
#[test]
fn test_tree_root_and_item_style() {
    use rusty_lipgloss::style::Style;
    let tree = Tree::new()
        .root("Root")
        .child("Foo".into())
        .child("Baz".into())
        .root_style(Style::new().background("#5A56E0"))
        .item_style(Style::new().background("#04B575"));
    assert!(tree.render().contains("Root"));
}

/// Ported from upstream `TestTreeAllHidden`.
#[test]
fn test_tree_all_hidden() {
    let tree = Tree::new().root("Foo").child("Bar".into()).hide(true);
    assert_eq!(tree.render(), "");
}
