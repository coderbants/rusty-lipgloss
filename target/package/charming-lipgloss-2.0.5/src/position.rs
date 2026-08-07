//! Positioning and canvas placement functions matching upstream `lipgloss/position.go`.

use crate::align::Position;
use crate::size;

/// <upstream-comment>Place places a string inside a box of width and height with horizontal and vertical alignment.</upstream-comment>
pub fn place(width: usize, height: usize, h_pos: Position, v_pos: Position, str: &str) -> String {
    let place_h = place_horizontal(width, h_pos, str);
    place_vertical(height, v_pos, &place_h)
}

/// <upstream-comment>PlaceHorizontal places a string inside a box of target width with horizontal alignment.</upstream-comment>
pub fn place_horizontal(width: usize, pos: Position, str: &str) -> String {
    let str_w = size::width(str);
    if str_w >= width {
        return str.to_string();
    }
    let gap = width - str_w;
    let (left_pad, right_pad) = match pos {
        Position::Left => (0, gap),
        Position::Right => (gap, 0),
        Position::Center | Position::Top | Position::Bottom => (gap / 2, gap - (gap / 2)),
    };

    str.lines()
        .map(|line| format!("{}{}{}", " ".repeat(left_pad), line, " ".repeat(right_pad)))
        .collect::<Vec<_>>()
        .join("\n")
}

/// <upstream-comment>PlaceVertical places a string inside a box of target height with vertical alignment.</upstream-comment>
pub fn place_vertical(height: usize, pos: Position, str: &str) -> String {
    let str_h = size::height(str);
    if str_h >= height {
        return str.to_string();
    }
    let gap = height - str_h;
    let (top_pad, bottom_pad) = match pos {
        Position::Top => (0, gap),
        Position::Bottom => (gap, 0),
        Position::Center | Position::Left | Position::Right => (gap / 2, gap - (gap / 2)),
    };

    let line_w = size::width(str);
    let empty_line = " ".repeat(line_w);

    let mut lines = Vec::new();
    for _ in 0..top_pad {
        lines.push(empty_line.clone());
    }
    for line in str.lines() {
        lines.push(line.to_string());
    }
    for _ in 0..bottom_pad {
        lines.push(empty_line.clone());
    }

    lines.join("\n")
}
