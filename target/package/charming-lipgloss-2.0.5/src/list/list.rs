//! List component matching upstream `lipgloss/list.List`.

#[derive(Debug, Clone, Default)]
pub struct List {
    pub items: Vec<String>,
    pub enumerator: String,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            enumerator: "•".to_string(),
        }
    }

    pub fn item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }

    pub fn render(&self) -> String {
        self.items
            .iter()
            .map(|i| format!("{} {}", self.enumerator, i))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
