//! Table rendering model and cell formatting matching upstream `lipgloss/table`.

use crate::border::Border;

/// Table component matching upstream `lipgloss/table.Table`.
#[derive(Debug, Clone, Default)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub border: Option<Border>,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn headers(mut self, headers: &[&str]) -> Self {
        self.headers = headers.iter().map(|h| h.to_string()).collect();
        self
    }

    pub fn row(mut self, row: &[&str]) -> Self {
        self.rows.push(row.iter().map(|r| r.to_string()).collect());
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        if !self.headers.is_empty() {
            out.push_str(&self.headers.join(" | "));
            out.push('\n');
        }
        for row in &self.rows {
            out.push_str(&row.join(" | "));
            out.push('\n');
        }
        out.trim_end().to_string()
    }
}
