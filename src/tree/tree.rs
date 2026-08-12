//! Cleanroom Rust port of upstream Go source file: `tree/tree.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! The `Tree` component renders a hierarchy of nodes with configurable
//! enumerators, indenters, and styles.
//! </public-docs>

use super::renderer::{new_renderer, render, Renderer, StyleFunc};
use std::sync::Arc;

use crate::style::Style;

/// Node is a node in a tree.
#[derive(Clone)]
pub enum Node {
    /// A leaf node without children.
    Leaf(Leaf),
    /// A tree node with children.
    Tree(Box<Tree>),
}

impl Node {
    /// Returns a leaf node wrapping the given string.
    pub fn leaf(value: String) -> Node {
        Node::Leaf(Leaf::new(value))
    }

    /// Returns the value of this node.
    pub fn value(&self) -> &str {
        match self {
            Node::Leaf(l) => l.value(),
            Node::Tree(t) => t.value(),
        }
    }

    /// Returns the children of this node (empty for leaves).
    pub fn children(&self) -> &[Node] {
        match self {
            Node::Leaf(_) => &[],
            Node::Tree(t) => t.children(),
        }
    }

    /// Returns whether this node is hidden.
    pub fn hidden(&self) -> bool {
        match self {
            Node::Leaf(l) => l.hidden(),
            Node::Tree(t) => t.hidden(),
        }
    }
}

/// Leaf is a node without children.
#[derive(Clone)]
pub struct Leaf {
    value: String,
    hidden: bool,
}

impl Leaf {
    /// Returns a new Leaf.
    pub fn new(value: String) -> Leaf {
        Leaf {
            value,
            hidden: false,
        }
    }

    /// Returns the value of a Leaf node.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns whether a Leaf node is hidden.
    pub fn hidden(&self) -> bool {
        self.hidden
    }
}

/// An argument to `Tree::child` / `Tree::children`.
#[derive(Clone)]
pub enum Child {
    /// A tree node.
    Tree(Box<Tree>),
    /// A leaf node.
    Leaf(Leaf),
    /// A string leaf.
    Str(String),
}

impl From<&str> for Child {
    fn from(s: &str) -> Self {
        Child::Str(s.to_string())
    }
}

impl From<String> for Child {
    fn from(s: String) -> Self {
        Child::Str(s)
    }
}

impl From<Tree> for Child {
    fn from(t: Tree) -> Self {
        Child::Tree(Box::new(t))
    }
}

impl From<Leaf> for Child {
    fn from(l: Leaf) -> Self {
        Child::Leaf(l)
    }
}

/// Tree implements a Node.
#[derive(Clone, Default)]
pub struct Tree {
    value: String,
    hidden: bool,
    offset: (usize, usize),
    children: Vec<Node>,
    renderer: Option<Renderer>,
}

impl Tree {
    /// Returns a new tree.
    pub fn new() -> Tree {
        Tree::default()
    }

    /// Returns a new tree with the root set. It is shorthand for
    /// `Tree::new().root(root)`.
    pub fn root_value(root: &str) -> Tree {
        Tree::new().root(root)
    }

    /// Sets the root value of this tree.
    pub fn root(mut self, root: &str) -> Tree {
        self.value = root.to_string();
        self
    }

    /// Returns whether a Tree node is hidden.
    pub fn hidden(&self) -> bool {
        self.hidden
    }

    /// Hide sets whether to hide the Tree node. Use this when creating a new
    /// hidden Tree.
    pub fn hide(mut self, hide: bool) -> Tree {
        self.hidden = hide;
        self
    }

    /// Offset sets the Tree children offsets.
    pub fn offset(mut self, start: usize, end: usize) -> Tree {
        let mut start = start;
        let mut end = end;
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        if end > self.children.len() {
            end = self.children.len();
        }
        self.offset = (start, end);
        self
    }

    /// Returns the root value of this node.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the string representation of the Tree node.
    pub fn string(&self) -> String {
        self.render()
    }

    /// Renders the tree to a string.
    pub fn render(&self) -> String {
        let r = match &self.renderer {
            Some(r) => r,
            None => {
                return render(
                    &new_renderer(),
                    &self.value,
                    self.effective_children(),
                    self.hidden,
                    true,
                    "",
                )
            }
        };
        render(
            r,
            &self.value,
            self.effective_children(),
            self.hidden,
            true,
            "",
        )
    }

    /// Returns the children of a node, honoring offsets.
    pub fn children(&self) -> &[Node] {
        &self.children
    }

    /// Returns the effective children slice honoring offsets.
    pub(crate) fn effective_children(&self) -> &[Node] {
        let start = self.offset.0.min(self.children.len());
        let end = self.children.len().saturating_sub(self.offset.1).max(start);
        &self.children[start..end]
    }

    /// Adds a child to this Tree.
    ///
    /// If a Child Tree is passed without a root, it will be parented to its
    /// sibling child (auto-nesting).
    pub fn child(mut self, child: Child) -> Tree {
        match child {
            Child::Tree(mut item) => {
                let (new_item, rm) = ensure_parent(&self.children, &mut item);
                if let Some(rm) = rm {
                    self.children.remove(rm);
                }
                self.children.push(Node::Tree(Box::new(new_item)));
            }
            Child::Leaf(l) => {
                self.children.push(Node::Leaf(l));
            }
            Child::Str(s) => {
                self.children.push(Node::Leaf(Leaf::new(s)));
            }
        }
        self
    }

    /// Adds multiple children to this Tree.
    pub fn child_nodes(mut self, children: &[Child]) -> Tree {
        for child in children {
            self = self.child(child.clone());
        }
        self
    }

    /// EnumeratorStyle sets a static style for all enumerators.
    pub fn enumerator_style(mut self, style: Style) -> Tree {
        self.ensure_renderer_mut().style.enumerator_func = Arc::new(move |_, _| style.clone());
        self
    }

    /// EnumeratorStyleFunc sets the enumeration style function.
    pub fn enumerator_style_func(mut self, f: StyleFunc) -> Tree {
        self.ensure_renderer_mut().style.enumerator_func = f;
        self
    }

    /// IndenterStyle sets a static style for all indenters.
    pub fn indenter_style(mut self, style: Style) -> Tree {
        self.ensure_renderer_mut().style.indenter_func = Arc::new(move |_, _| style.clone());
        self
    }

    /// IndenterStyleFunc sets the indentation style function.
    pub fn indenter_style_func(mut self, f: StyleFunc) -> Tree {
        self.ensure_renderer_mut().style.indenter_func = f;
        self
    }

    /// RootStyle sets a style for the root element.
    pub fn root_style(mut self, style: Style) -> Tree {
        self.ensure_renderer_mut().style.root = style;
        self
    }

    /// ItemStyle sets a static style for all items.
    pub fn item_style(mut self, style: Style) -> Tree {
        self.ensure_renderer_mut().style.item_func = Arc::new(move |_, _| style.clone());
        self
    }

    /// ItemStyleFunc sets the item style function.
    pub fn item_style_func(mut self, f: StyleFunc) -> Tree {
        self.ensure_renderer_mut().style.item_func = f;
        self
    }

    /// Enumerator sets the enumerator implementation.
    pub fn enumerator(mut self, enumerator: super::enumerator::Enumerator) -> Tree {
        self.ensure_renderer_mut().enumerator = enumerator;
        self
    }

    /// Indenter sets the indenter implementation.
    pub fn indenter(mut self, ind: super::enumerator::Indenter) -> Tree {
        self.ensure_renderer_mut().indenter = ind;
        self
    }

    /// Width sets the tree width. Items will be padded to account for the
    /// entire width of the tree.
    pub fn width(mut self, width: usize) -> Tree {
        self.ensure_renderer_mut().width = width;
        self
    }

    pub(crate) fn renderer(&self) -> Option<&Renderer> {
        self.renderer.as_ref()
    }

    fn ensure_renderer_mut(&mut self) -> &mut Renderer {
        if self.renderer.is_none() {
            self.renderer = Some(new_renderer());
        }
        self.renderer.as_mut().unwrap()
    }
}

/// ensure_parent parents a root-less tree onto its sibling.
fn ensure_parent(nodes: &[Node], item: &mut Tree) -> (Tree, Option<usize>) {
    if !item.value.is_empty() || nodes.is_empty() {
        return (item.clone(), None);
    }
    let j = nodes.len() - 1;
    match &nodes[j] {
        Node::Tree(parent) => {
            let mut p = parent.clone();
            for i in 0..item.children.len() {
                let c = item.children[i].clone();
                p.children.push(c);
            }
            (*p, Some(j))
        }
        Node::Leaf(l) => {
            item.value = l.value().to_string();
            (item.clone(), Some(j))
        }
    }
}
