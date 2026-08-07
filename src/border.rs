//! Border definitions and preset border styles matching upstream `lipgloss.Border`.

/// Defines border characters for box rendering.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Border {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub top_left: String,
    pub top_right: String,
    pub bottom_left: String,
    pub bottom_right: String,
}

impl Border {
    /// Normal thin line border.
    pub fn normal() -> Self {
        Self {
            top: "─".into(),
            bottom: "─".into(),
            left: "│".into(),
            right: "│".into(),
            top_left: "┌".into(),
            top_right: "┐".into(),
            bottom_left: "└".into(),
            bottom_right: "┘".into(),
        }
    }

    /// Rounded corner border.
    pub fn rounded() -> Self {
        Self {
            top: "─".into(),
            bottom: "─".into(),
            left: "│".into(),
            right: "│".into(),
            top_left: "╭".into(),
            top_right: "╮".into(),
            bottom_left: "╰".into(),
            bottom_right: "╯".into(),
        }
    }

    /// Double line border.
    pub fn double() -> Self {
        Self {
            top: "═".into(),
            bottom: "═".into(),
            left: "║".into(),
            right: "║".into(),
            top_left: "╔".into(),
            top_right: "╗".into(),
            bottom_left: "╚".into(),
            bottom_right: "╝".into(),
        }
    }

    /// Thick line border.
    pub fn thick() -> Self {
        Self {
            top: "━".into(),
            bottom: "━".into(),
            left: "┃".into(),
            right: "┃".into(),
            top_left: "┏".into(),
            top_right: "┓".into(),
            bottom_left: "┗".into(),
            bottom_right: "┛".into(),
        }
    }
}
