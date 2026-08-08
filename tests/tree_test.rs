//! Cleanroom Rust port of upstream Go test file: `tree/tree_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use std::sync::Arc;

use charming_lipgloss::tree::{rounded_enumerator, Child, Tree};

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
    let tree = t("Root").child(Child::from(
        t("Parent").child(Child::from(t("Child"))),
    ));
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
    let tree = t("Root").child(Child::Str("a".to_string())).child(Child::Str("b".to_string()));
    assert_eq!(tree.render(), "Root\n├── a\n└── b");
}
