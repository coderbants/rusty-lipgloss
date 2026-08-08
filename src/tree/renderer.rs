//! Cleanroom Rust port of upstream Go source file: `tree/renderer.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! The tree renderer walks a node hierarchy and renders it to a string.
//! </public-docs>

use std::sync::Arc;

use super::children::Children;
use super::enumerator::{default_enumerator, default_indenter, Enumerator, Indenter};
use super::Node;
use crate::align::{LEFT, TOP};
use crate::join::{join_horizontal, join_vertical};
use crate::size;
use crate::style::Style;

/// StyleFunc allows the tree to be styled per item.
pub type StyleFunc = Arc<dyn Fn(&dyn Children, usize) -> Style>;

/// Style is the styling applied to the tree.
#[derive(Clone)]
pub struct TreeStyle {
    /// Function returning the enumerator style for each child.
    pub enumerator_func: StyleFunc,
    /// Function returning the indenter style for each child.
    pub indenter_func: StyleFunc,
    /// Function returning the item style for each child.
    pub item_func: StyleFunc,
    /// The style applied to the root element.
    pub root: Style,
}

/// Returns a new default tree style.
pub fn default_tree_style() -> TreeStyle {
    TreeStyle {
        enumerator_func: Arc::new(|_, _| Style::new().padding_right(1)),
        indenter_func: Arc::new(|_, _| Style::new().padding_right(1)),
        item_func: Arc::new(|_, _| Style::new()),
        root: Style::new(),
    }
}

/// The renderer used to render a tree.
#[derive(Clone)]
pub struct Renderer {
    /// The styling applied to the tree.
    pub style: TreeStyle,
    /// The enumerator used to draw branches.
    pub enumerator: Enumerator,
    /// The indenter used to indent children.
    pub indenter: Indenter,
    /// The desired total width of the tree.
    pub width: usize,
}

/// Returns a new renderer.
pub fn new_renderer() -> Renderer {
    Renderer {
        style: default_tree_style(),
        enumerator: Arc::new(default_enumerator),
        indenter: Arc::new(default_indenter),
        width: 0,
    }
}

/// An effective (offset-and-hidden-filtered) view over a node's children.
pub(crate) struct EffectiveChildren<'a> {
    pub nodes: &'a [Node],
    pub indices: Vec<usize>,
}

impl Children for EffectiveChildren<'_> {
    fn at(&self, index: usize) -> Option<&Node> {
        self.indices.get(index).and_then(|&idx| self.nodes.get(idx))
    }
    fn length(&self) -> usize {
        self.indices.len()
    }
}

const EMPTY_NODES: [Node; 0] = [];

/// Returns the effective children of a tree node, honoring offsets.
pub(crate) fn node_children(node: &Node) -> &[Node] {
    match node {
        Node::Leaf(_) => &EMPTY_NODES,
        Node::Tree(t) => t.effective_children(),
    }
}

/// render is responsible for actually rendering the tree.
pub(crate) fn render(
    r: &Renderer,
    value: &str,
    children: &[Node],
    hidden: bool,
    root: bool,
    prefix: &str,
) -> String {
    if hidden {
        return String::new();
    }

    let mut max_len = 0usize;
    let all_children = children;
    let mut indices: Vec<usize> = (0..all_children.len()).collect();

    // Remove trailing hidden children so the last visible element gets the
    // correct (last-child) prefix.
    let mut i = 0usize;
    while i + 1 < indices.len() {
        let next = indices[i + 1];
        if all_children[next].hidden() {
            indices.remove(i + 1);
        } else {
            i += 1;
        }
    }

    let eff_children = EffectiveChildren {
        nodes: all_children,
        indices,
    };

    let mut strs: Vec<String> = Vec::new();

    // Print the root node name if it's not empty.
    if root && !value.is_empty() {
        let mut line = r.style.root.render(value);
        // If the line is shorter than the desired width, pad it with spaces.
        if let Some(pad) = r.width.checked_sub(size::width(&line)) {
            if pad > 0 {
                line = format!("{}{}", value, r.style.root.render(&" ".repeat(pad)));
            }
        }
        strs.push(r.style.root.render(&line));
    }

    let prefix_s = prefix.to_string();

    // First pass: compute the max enumerator prefix width.
    for i in 0..eff_children.length() {
        let p = (r.enumerator)(&eff_children, i);
        let styled = (r.style.enumerator_func)(&eff_children, i).render(&p);
        max_len = max_len.max(size::width(&styled));
    }

    // Second pass: render each child.
    for i in 0..eff_children.length() {
        let child = match eff_children.at(i) {
            Some(c) => c,
            None => continue,
        };
        if child.hidden() {
            continue;
        }

        let indent_style = (r.style.indenter_func)(&eff_children, i);
        let enum_style = (r.style.enumerator_func)(&eff_children, i);
        let item_style = (r.style.item_func)(&eff_children, i);

        let indent = indent_style.render(&(r.indenter)(&eff_children, i));
        let mut node_prefix = enum_style.render(&(r.enumerator)(&eff_children, i));

        // Preserve the background color of the enumerator when adding the padding.
        let enum_bg_style = Style::new().background_color(enum_style.get_background());

        // Add padding to the left of the node to align it with the longest
        // prefix of its siblings.
        if let Some(l) = max_len.checked_sub(size::width(&node_prefix)) {
            if l > 0 {
                node_prefix = format!("{}{}", enum_bg_style.render(&" ".repeat(l)), node_prefix);
            }
        }

        let item = item_style.render(child.value());
        let mut multiline_prefix = enum_bg_style.render(&prefix_s);

        // This dance below is to account for multiline prefixes, e.g. "|\n|".
        while size::height(&item) > size::height(&node_prefix) {
            node_prefix = join_vertical(LEFT, &[&node_prefix, &indent]);
        }
        while size::height(&node_prefix) > size::height(&multiline_prefix) {
            multiline_prefix = join_vertical(LEFT, &[&multiline_prefix, &prefix_s]);
        }

        let mut line = join_horizontal(TOP, &[&multiline_prefix, &node_prefix, &item]);

        // If the line is shorter than the desired width, pad it with spaces.
        if let Some(pad) = r.width.checked_sub(size::width(&line)) {
            if pad > 0 {
                line = format!("{}{}", line, item_style.render(&" ".repeat(pad)));
            }
        }
        strs.push(line);

        if eff_children.length() > 0 {
            let (child_renderer, child_children) = match child {
                Node::Tree(t) => (t.renderer().unwrap_or(r), t.effective_children()),
                Node::Leaf(_) => (r, node_children(child)),
            };
            let s = render(
                child_renderer,
                child.value(),
                child_children,
                child.hidden(),
                false,
                &format!("{}{}", prefix_s, indent),
            );
            if !s.is_empty() {
                strs.push(s);
            }
        }
    }

    strs.join("\n")
}
