//! Tree component matching upstream `lipgloss/tree.Tree`.

#[derive(Debug, Clone, Default)]
pub struct Tree {
    pub root: String,
    pub children: Vec<Tree>,
}

impl Tree {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: Tree) -> Self {
        self.children.push(child);
        self
    }

    pub fn render(&self) -> String {
        let mut out = self.root.clone();
        for child in &self.children {
            out.push_str("\n├── ");
            out.push_str(&child.render());
        }
        out
    }
}
