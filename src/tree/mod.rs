//! Cleanroom Rust port of upstream Go source file: `tree/tree.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! The `tree` submodule mirrors upstream `charmbracelet/lipgloss/tree`: a
//! tree rendering component with nodes, enumerators, indenters, and styling.
//! </public-docs>

pub mod children;
pub mod enumerator;
pub mod renderer;
pub mod tree;

pub use children::{new_filter, new_string_data, Children, Filter, NodeChildren};
pub use enumerator::{
    default_enumerator, default_indenter, rounded_enumerator, Enumerator, Indenter,
};
pub use renderer::StyleFunc;
pub use tree::{Child, Leaf, Node, Tree};

/// Returns a new tree.
pub fn new() -> Tree {
    Tree::new()
}

/// Returns a new tree with the root set. It is a shorthand for
/// `Tree::new().root(root)`.
pub fn root(root: &str) -> Tree {
    Tree::root_value(root)
}
