//! Cleanroom Rust port of upstream Go source file: `tree/enumerator.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Enumerators and indenters control how tree branches are drawn.
//! </public-docs>

use std::sync::Arc;

use super::children::Children;

/// Enumerator enumerates a tree. Typically, this is used to draw the branches
/// for the tree nodes and is different for the last child.
///
/// For example, the default enumerator would be:
///
/// ```text
/// func TreeEnumerator(children Children, index int) string {
///     if children.Length()-1 == index {
///         return "└──"
///     }
///     return "├──"
/// }
/// ```
pub type Enumerator = Arc<dyn Fn(&dyn Children, usize) -> String>;

/// <upstream-comment>DefaultEnumerator enumerates a tree.
///
/// ```text
/// ├── Foo
/// ├── Bar
/// ├── Baz
/// └── Qux.
/// ```</upstream-comment>
pub fn default_enumerator(children: &dyn Children, index: usize) -> String {
    if children.length().saturating_sub(1) == index {
        "└──".to_string()
    } else {
        "├──".to_string()
    }
}

/// <upstream-comment>RoundedEnumerator enumerates a tree with rounded edges.
///
/// ```text
/// ├── Foo
/// ├── Bar
/// ├── Baz
/// ╰── Qux.
/// ```</upstream-comment>
pub fn rounded_enumerator(children: &dyn Children, index: usize) -> String {
    if children.length().saturating_sub(1) == index {
        "╰──".to_string()
    } else {
        "├──".to_string()
    }
}

/// Indenter indents the children of a tree.
///
/// Indenters allow for displaying nested tree items with connecting borders
/// to sibling nodes.
///
/// For example, the default indenter would be:
///
/// ```text
/// func TreeIndenter(children Children, index int) string {
///     if children.Length()-1 == index {
///         return "│  "
///     }
///     return "   "
/// }
/// ```
pub type Indenter = Arc<dyn Fn(&dyn Children, usize) -> String>;

/// <upstream-comment>DefaultIndenter indents a tree for nested trees and multiline content.
///
/// ```text
/// ├── Foo
/// ├── Bar
/// │   ├── Qux
/// │   ├── Quux
/// │   │   ├── Foo
/// │   │   └── Bar
/// │   └── Quuux
/// └── Baz.
/// ```</upstream-comment>
pub fn default_indenter(children: &dyn Children, index: usize) -> String {
    if children.length().saturating_sub(1) == index {
        "   ".to_string()
    } else {
        "│  ".to_string()
    }
}
