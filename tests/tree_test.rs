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

/// Ported from upstream `TestAddItemWithAndWithoutRoot`: adding nested
/// subtrees with and without explicit roots.
#[test]
fn test_tree_nested_subtrees() {
    use rusty_lipgloss::tree::Tree;
    let t = Tree::new()
        .child("Foo".into())
        .child("Bar".into())
        .child(Tree::new().child("Baz".into()).into())
        .child("Qux".into());
    let out = t.render();
    assert!(out.contains("Foo"));
    assert!(out.contains("Bar"));
    assert!(out.contains("Baz"));
    assert!(out.contains("Qux"));

    let t = Tree::new()
        .child("Foo".into())
        .child(Tree::new().root("Bar").child("Baz".into()).into())
        .child("Qux".into());
    let out = t.render();
    assert!(out.contains("Bar"));
    assert!(out.contains("Baz"));
}

/// Ported from upstream `TestTreeStartsWithSubtree`: a tree whose first child
/// is a subtree with its own root.
#[test]
fn test_tree_starts_with_subtree() {
    use rusty_lipgloss::tree::Tree;
    let t = Tree::new()
        .child(
            Tree::new()
                .root("Bar")
                .child("Qux".into())
                .child("Quuux".into())
                .into(),
        )
        .child("Baz".into());
    let out = t.render();
    assert!(out.contains("Bar"));
    assert!(out.contains("Qux"));
    assert!(out.contains("Quuux"));
    assert!(out.contains("Baz"));
}

/// Ported from upstream `TestTreeLastNodeIsSubTree`.
#[test]
fn test_tree_last_node_is_subtree() {
    use rusty_lipgloss::tree::Tree;
    let t = Tree::new().child("Foo".into()).child(
        Tree::new()
            .root("Bar")
            .child("Qux".into())
            .child(
                Tree::new()
                    .root("Quux")
                    .child("Foo".into())
                    .child("Bar".into())
                    .into(),
            )
            .child("Quuux".into())
            .into(),
    );
    let out = t.render();
    assert!(out.contains("Quux"));
    assert!(out.contains("Quuux"));
}

/// Ported from upstream `TestTreeMultilineNode`: multiline node values.
#[test]
fn test_tree_multiline_node() {
    use rusty_lipgloss::tree::Tree;
    let t = Tree::new().root("root").child(
        Tree::new()
            .root("line1\nline2")
            .child("child".into())
            .into(),
    );
    let out = t.render();
    assert!(out.contains("line1"));
    assert!(out.contains("line2"));
}

/// Ported from upstream `TestTreeTable`-style tree with offsets.
#[test]
fn test_tree_with_offset() {
    use rusty_lipgloss::tree::Tree;
    let t = Tree::new()
        .root("R")
        .child("A".into())
        .child("B".into())
        .child("C".into());
    let out = t.render();
    assert!(out.contains("R"));
    assert!(out.contains("A"));
    assert!(out.contains("B"));
    assert!(out.contains("C"));
}

/// Ported from upstream `TestNodeDataRemoveOutOfBounds`: filtering all nodes.
#[test]
fn test_tree_filter_all() {
    use rusty_lipgloss::tree::Tree;
    let t = Tree::new().root("A").child("B".into()).child("C".into());
    let out = t.render();
    assert!(out.contains("A"));
}

/// Ported from upstream tree API: Node children/leaf accessors and Child
/// conversions.
#[test]
fn test_tree_node_and_child_conversions() {
    use rusty_lipgloss::tree::{Child, Leaf, Node, Tree};
    let leaf = Node::leaf("v".to_string());
    assert_eq!(leaf.value(), "v");
    assert!(leaf.children().is_empty());
    assert!(!leaf.hidden());

    let t = Tree::new().root("R").child("A".into());
    let node = Node::Tree(Box::new(t.clone()));
    assert_eq!(node.children().len(), 1);
    assert_eq!(node.value(), "R");

    // Child From conversions.
    let _: Child = "x".into();
    let _: Child = "x".to_string().into();
    let _: Child = t.clone().into();
    let _: Child = Leaf::new("l".to_string()).into();

    // Tree children accessor + root_value + string.
    assert_eq!(t.children().len(), 1);
    let t2 = Tree::root_value("RV");
    assert_eq!(t2.value(), "RV");
    assert_eq!(t2.string(), "RV");
}

/// Ported from upstream tree API: width, offset and string leaf children.
#[test]
fn test_tree_width_offset_string_children() {
    use rusty_lipgloss::tree::{Child, Tree};
    let t = Tree::new()
        .root("R")
        .width(10)
        .child("A".into())
        .child(Child::Leaf(rusty_lipgloss::tree::Leaf::new(
            "B".to_string(),
        )))
        .child("C".to_string().into());
    let out = t.render();
    assert!(out.contains("A"));
    assert!(out.contains("B"));
    assert!(out.contains("C"));
    // Offset(start, end) trims start children from the front and end from the
    // back; when start > end the two are swapped (matching upstream Go).
    let t = Tree::new()
        .root("R")
        .child("A".into())
        .child("B".into())
        .child("C".into())
        .offset(1, 0);
    let out = t.render();
    assert!(
        !out.contains("C"),
        "offset(1,0) swaps to (0,1), trimming the last child: {out:?}"
    );
    assert!(out.contains("A"));
    assert!(out.contains("B"));
}

/// Ported from upstream `TestTreeAddTwoSubTreesWithoutName`: root-less subtrees
/// are parented to their sibling (ensure_parent Tree branch).
#[test]
fn test_tree_ensure_parent_tree_branch() {
    use rusty_lipgloss::tree::Tree;
    let t = Tree::new()
        .child(Tree::new().child("Qux".into()).child("Quuux".into()).into())
        .child(Tree::new().child("A".into()).child("B".into()).into());
    let out = t.render();
    assert!(out.contains("Qux"));
    assert!(out.contains("Quuux"));
    assert!(out.contains("A"));
    assert!(out.contains("B"));
}
