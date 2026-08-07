//! Alignment and positioning primitives matching upstream `lipgloss.Position`.

/// Position / Alignment enum matching upstream `lipgloss.Position`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    #[default]
    Top,
    Bottom,
    Center,
    Left,
    Right,
}

pub type Align = Position;

pub const TOP: Position = Position::Top;
pub const BOTTOM: Position = Position::Bottom;
pub const CENTER: Position = Position::Center;
pub const LEFT: Position = Position::Left;
pub const RIGHT: Position = Position::Right;
