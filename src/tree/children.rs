//! Cleanroom Rust port of upstream Go source file: `tree/children.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Children containers for tree nodes, including filtered views.
//! </public-docs>

use super::Node;

/// Children is the interface that wraps the basic methods of a tree model.
pub trait Children {
    /// At returns the content item of the given index.
    fn at(&self, index: usize) -> Option<&Node>;
    /// Length returns the number of children in the tree.
    fn length(&self) -> usize;
}

impl Children for Vec<Node> {
    fn at(&self, index: usize) -> Option<&Node> {
        self.get(index)
    }
    fn length(&self) -> usize {
        self.len()
    }
}

impl Children for &[Node] {
    fn at(&self, index: usize) -> Option<&Node> {
        self.get(index)
    }
    fn length(&self) -> usize {
        self.len()
    }
}

/// NodeChildren is the implementation of the Children interface with tree Nodes.
pub type NodeChildren = Vec<Node>;

/// NewStringData returns a Children of string leaves.
pub fn new_string_data(data: &[&str]) -> Vec<Node> {
    data.iter()
        .map(|d| Node::leaf(d.to_string()))
        .collect()
}

/// Filter applies a filter on some data. You could use this to create a new
/// tree whose values all satisfy the condition provided in the `filter`
/// function.
pub struct Filter<'a> {
    data: &'a [Node],
    filter: Box<dyn Fn(usize) -> bool>,
}

/// NewFilter initializes a new Filter.
pub fn new_filter<'a>(data: &'a [Node], filter: Box<dyn Fn(usize) -> bool>) -> Filter<'a> {
    Filter { data, filter }
}

impl Children for Filter<'_> {
    /// At returns the item at the given index. The index is relative to the
    /// filtered results.
    fn at(&self, index: usize) -> Option<&Node> {
        let mut j = 0usize;
        for i in 0..self.data.length() {
            if (self.filter)(i) {
                if j == index {
                    return self.data.at(i);
                }
                j += 1;
            }
        }
        None
    }

    fn length(&self) -> usize {
        let mut j = 0usize;
        for i in 0..self.data.length() {
            if (self.filter)(i) {
                j += 1;
            }
        }
        j
    }
}
