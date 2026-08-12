//! Cleanroom Rust port of upstream Go source file: `list/list.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! A list component built on top of the tree renderer. Lists can contain
//! lists as items, which are rendered as nested (sub)lists.
//! </public-docs>

use std::sync::Arc;

use crate::tree::renderer::StyleFunc;
use crate::tree::Tree;

use super::enumerator::{bullet, Enumerator, Indenter};

/// List represents a list of items that can be displayed. Lists can contain
/// lists as items; they will be rendered as nested (sub)lists.
#[derive(Clone)]
pub struct List {
    tree: Tree,
}

/// <upstream-comment>List returns a new list with the given items.
///
/// ```text
/// alphabet := list.New("A", "B", "C", "D", "E", "F", ...)
/// ```
///
/// Items can be other lists, trees, tables, rendered markdown; anything you
/// want, really.</upstream-comment>
impl Default for List {
    fn default() -> Self {
        let tree = Tree::new()
            .enumerator(Arc::new(bullet))
            .indenter(Arc::new(|_, _| " ".to_string()));
        List { tree }
    }
}

impl List {
    /// Returns a new empty list.
    pub fn new() -> List {
        List::default()
    }

    /// Hidden returns whether this list is hidden.
    pub fn hidden(&self) -> bool {
        self.tree.hidden()
    }

    /// Hide hides this list. If this list is hidden, it will not be shown when
    /// rendered.
    pub fn hide(mut self, hide: bool) -> List {
        self.tree = self.tree.hide(hide);
        self
    }

    /// Offset sets the start and end offset for the list.
    pub fn offset(mut self, start: usize, end: usize) -> List {
        self.tree = self.tree.offset(start, end);
        self
    }

    /// Value returns the value of this node.
    pub fn value(&self) -> &str {
        self.tree.value()
    }

    /// Returns the rendered string representation of the list.
    pub fn string(&self) -> String {
        self.tree.render()
    }

    /// Renders the list to a string.
    pub fn render(&self) -> String {
        self.tree.render()
    }

    /// EnumeratorStyle sets the enumerator style for all enumerators.
    pub fn enumerator_style(mut self, style: crate::style::Style) -> List {
        self.tree = self.tree.enumerator_style(style);
        self
    }

    /// EnumeratorStyleFunc sets the enumerator style function for the list items.
    pub fn enumerator_style_func(mut self, f: StyleFunc) -> List {
        self.tree = self.tree.enumerator_style_func(f);
        self
    }

    /// IndenterStyle sets the enumerator style for all indenters.
    pub fn indenter_style(mut self, style: crate::style::Style) -> List {
        self.tree = self.tree.indenter_style(style);
        self
    }

    /// IndenterStyleFunc sets the indenter style function for the list items.
    pub fn indenter_style_func(mut self, f: StyleFunc) -> List {
        self.tree = self.tree.indenter_style_func(f);
        self
    }

    /// Indenter sets the indenter implementation.
    pub fn indenter(mut self, ind: Indenter) -> List {
        self.tree = self.tree.indenter(ind);
        self
    }

    /// ItemStyle sets the item style for all items.
    pub fn item_style(mut self, style: crate::style::Style) -> List {
        self.tree = self.tree.item_style(style);
        self
    }

    /// ItemStyleFunc sets the item style function for the list items.
    pub fn item_style_func(mut self, f: StyleFunc) -> List {
        self.tree = self.tree.item_style_func(f);
        self
    }

    /// Item appends an item to the list.
    pub fn item(mut self, item: &str) -> List {
        self.tree = self.tree.child(crate::tree::Child::Str(item.to_string()));
        self
    }

    /// Items appends multiple items to the list.
    pub fn items(mut self, items: &[&str]) -> List {
        let children: Vec<crate::tree::Child> = items
            .iter()
            .map(|i| crate::tree::Child::Str(i.to_string()))
            .collect();
        self.tree = self.tree.child_nodes(&children);
        self
    }

    /// Adds a nested sub-list as an item of this list.
    pub fn child_list(mut self, sub: List) -> List {
        self.tree = self
            .tree
            .child(crate::tree::Child::Tree(Box::new(sub.tree)));
        self
    }

    /// Enumerator sets the list enumerator.
    pub fn enumerator(mut self, enumerator: Enumerator) -> List {
        self.tree = self.tree.enumerator(enumerator);
        self
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
