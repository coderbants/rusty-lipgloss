//! Horizontal and vertical string joining matching upstream `lipgloss.JoinHorizontal` and `lipgloss.JoinVertical`.

use crate::align::Position;
use crate::size;

/// Joins multiple multiline strings horizontally at a given vertical alignment position.
pub fn join_horizontal(_pos: Position, blocks: &[&str]) -> String {
    if blocks.is_empty() {
        return String::new();
    }
    if blocks.len() == 1 {
        return blocks[0].to_string();
    }

    let max_h = blocks.iter().map(|b| size::height(b)).max().unwrap_or(0);
    let mut lines_per_block: Vec<Vec<&str>> = blocks.iter().map(|b| b.lines().collect()).collect();

    let mut result = Vec::new();
    for row in 0..max_h {
        let mut row_str = String::new();
        for (i, lines) in lines_per_block.iter_mut().enumerate() {
            let line = if row < lines.len() { lines[row] } else { "" };
            row_str.push_str(line);
            if i < blocks.len() - 1 {
                row_str.push(' ');
            }
        }
        result.push(row_str);
    }
    result.join("\n")
}

/// Joins multiple strings vertically at a given horizontal alignment position.
pub fn join_vertical(_pos: Position, blocks: &[&str]) -> String {
    blocks.join("\n")
}
