//! Cleanroom Rust port of upstream Go source file: `join.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Utility functions for joining multiple potentially multi-lined strings
//! horizontally or vertically.
//! </public-docs>

use crate::align::{get_lines, Position, LEFT, RIGHT, TOP};
use crate::size;

/// <upstream-comment>JoinHorizontal is a utility function for horizontally joining two
/// potentially multi-lined strings along a vertical axis. The first argument is
/// the position, with 0 being all the way at the top and 1 being all the way
/// at the bottom.
///
/// If you just want to align to the top, center or bottom you may as well just
/// use the helper constants Top, Center, and Bottom.
///
/// ```text
/// blockB := "...\n...\n..."
/// blockA := "...\n...\n...\n...\n..."
///
/// // Join 20% from the top
/// str := lipgloss.JoinHorizontal(0.2, blockA, blockB)
///
/// // Join on the top edge
/// str := lipgloss.JoinHorizontal(lipgloss.Top, blockA, blockB)
/// ```</upstream-comment>
pub fn join_horizontal(pos: Position, strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    if strs.len() == 1 {
        return strs[0].to_string();
    }

    // Groups of strings broken into multiple lines.
    let mut blocks: Vec<Vec<String>> = Vec::with_capacity(strs.len());
    let mut max_widths: Vec<usize> = Vec::with_capacity(strs.len());
    let mut max_height = 0usize;

    // Break text blocks into lines and get max widths for each text block.
    for s in strs {
        let (lines, w) = get_lines(s);
        let lines: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
        if lines.len() > max_height {
            max_height = lines.len();
        }
        max_widths.push(w);
        blocks.push(lines);
    }

    // Add extra lines to make each side the same height.
    for i in 0..blocks.len() {
        if blocks[i].len() >= max_height {
            continue;
        }
        let extra_lines = max_height - blocks[i].len();
        match pos {
            TOP => {
                for _ in 0..extra_lines {
                    blocks[i].push(String::new());
                }
            }
            crate::align::BOTTOM => {
                let mut pad = vec![String::new(); extra_lines];
                pad.extend(blocks[i].drain(..));
                blocks[i] = pad;
            }
            _ => {
                // Somewhere in the middle.
                let split = (extra_lines as f64 * pos.value()).round() as usize;
                let top = extra_lines - split;
                let mut pad_top = vec![String::new(); split];
                pad_top.extend(blocks[i].drain(..));
                let mut full = pad_top;
                full.extend(vec![String::new(); top]);
                blocks[i] = full;
            }
        }
    }

    // Merge lines.
    let mut out = String::new();
    for i in 0..blocks[0].len() {
        for (j, block) in blocks.iter().enumerate() {
            out.push_str(&block[i]);
            // Also make lines the same length.
            out.push_str(&" ".repeat(max_widths[j] - size::width(&block[i])));
        }
        if i < blocks[0].len() - 1 {
            out.push('\n');
        }
    }

    out
}

/// <upstream-comment>JoinVertical is a utility function for vertically joining two potentially
/// multi-lined strings along a horizontal axis. The first argument is the
/// position, with 0 being all the way to the left and 1 being all the way to
/// the right.
///
/// If you just want to align to the left, right or center you may as well just
/// use the helper constants Left, Center, and Right.</upstream-comment>
pub fn join_vertical(pos: Position, strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    if strs.len() == 1 {
        return strs[0].to_string();
    }

    let mut blocks: Vec<Vec<String>> = Vec::with_capacity(strs.len());
    let mut max_width = 0usize;

    for s in strs {
        let (lines, w) = get_lines(s);
        let lines: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
        if w > max_width {
            max_width = w;
        }
        blocks.push(lines);
    }

    let mut out = String::new();
    for (i, block) in blocks.iter().enumerate() {
        for (j, line) in block.iter().enumerate() {
            let w = max_width - size::width(line);

            match pos {
                LEFT => {
                    out.push_str(line);
                    out.push_str(&" ".repeat(w));
                }
                RIGHT => {
                    out.push_str(&" ".repeat(w));
                    out.push_str(line);
                }
                _ => {
                    // Somewhere in the middle.
                    if w < 1 {
                        out.push_str(line);
                    } else {
                        let split = (w as f64 * pos.value()).round() as usize;
                        let right = w - split;
                        let left = w - right;
                        out.push_str(&" ".repeat(left));
                        out.push_str(line);
                        out.push_str(&" ".repeat(right));
                    }
                }
            }

            // Write a newline as long as we're not on the last line of the
            // last block.
            if !(i == blocks.len() - 1 && j == block.len() - 1) {
                out.push('\n');
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_vertical() {
        assert_eq!(join_vertical(LEFT, &["Hello", "World"]), "Hello\nWorld");
        assert_eq!(join_vertical(LEFT, &["A", "BBBB"]), "A   \nBBBB");
        assert_eq!(join_vertical(RIGHT, &["A", "BBBB"]), "   A\nBBBB");
    }

    #[test]
    fn test_join_horizontal() {
        assert_eq!(join_horizontal(TOP, &["A", "B"]), "AB");
        assert_eq!(
            join_horizontal(TOP, &["A", "B\nB\nB\nB"]),
            "AB\n B\n B\n B"
        );
        assert_eq!(
            join_horizontal(crate::align::BOTTOM, &["A", "B\nB\nB\nB"]),
            " B\n B\n B\nAB"
        );
    }
}
