//! Fluent `Style` builder and rendering pipeline matching upstream `lipgloss.Style`.

use crate::color::{Color, TerminalColor};

/// <upstream-comment>Style contains styling options for text rendering.</upstream-comment>
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub reverse: bool,
    pub blink: bool,
    pub faint: bool,
}

impl Style {
    /// <upstream-comment>New returns a new, empty Style.</upstream-comment>
    pub fn new() -> Self {
        Self::default()
    }

    /// <upstream-comment>Foreground sets the foreground color.</upstream-comment>
    pub fn foreground(mut self, color: &str) -> Self {
        self.foreground = Some(Color::parse(color));
        self
    }

    /// <upstream-comment>Background sets the background color.</upstream-comment>
    pub fn background(mut self, color: &str) -> Self {
        self.background = Some(Color::parse(color));
        self
    }

    /// <upstream-comment>Bold sets the bold text attribute.</upstream-comment>
    pub fn bold(mut self, value: bool) -> Self {
        self.bold = value;
        self
    }

    /// <upstream-comment>Italic sets the italic text attribute.</upstream-comment>
    pub fn italic(mut self, value: bool) -> Self {
        self.italic = value;
        self
    }

    /// <upstream-comment>Underline sets the underline text attribute.</upstream-comment>
    pub fn underline(mut self, value: bool) -> Self {
        self.underline = value;
        self
    }

    /// <upstream-comment>Strikethrough sets the strikethrough text attribute.</upstream-comment>
    pub fn strikethrough(mut self, value: bool) -> Self {
        self.strikethrough = value;
        self
    }

    /// <upstream-comment>Render applies the style to the string and returns the rendered output.</upstream-comment>
    pub fn render(&self, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }

        let mut ansi_starts = String::new();
        let mut ansi_ends = String::new();

        if self.bold {
            ansi_starts.push_str("\x1b[1m");
        }
        if self.faint {
            ansi_starts.push_str("\x1b[2m");
        }
        if self.italic {
            ansi_starts.push_str("\x1b[3m");
        }
        if self.underline {
            ansi_starts.push_str("\x1b[4m");
        }
        if self.blink {
            ansi_starts.push_str("\x1b[5m");
        }
        if self.reverse {
            ansi_starts.push_str("\x1b[7m");
        }
        if self.strikethrough {
            ansi_starts.push_str("\x1b[9m");
        }

        if let Some(ref fg) = self.foreground {
            ansi_starts.push_str(&fg.render_fg());
        }
        if let Some(ref bg) = self.background {
            ansi_starts.push_str(&bg.render_bg());
        }

        if !ansi_starts.is_empty() {
            ansi_ends.push_str("\x1b[0m");
        }

        format!("{}{}{}", ansi_starts, text, ansi_ends)
    }
}

