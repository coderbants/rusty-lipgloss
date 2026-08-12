//! Cleanroom Rust port of upstream Go source file: `position.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Placement functions that place a string inside an unstyled box of a given
//! width or height.
//! </public-docs>

use crate::align::{get_lines, Position, LEFT, RIGHT, TOP};
use crate::size;
use crate::whitespace::Whitespace;

/// <upstream-comment>Place places a string or text block vertically in an unstyled box of a given
/// width or height.</upstream-comment>
pub fn place(
    width: usize,
    height: usize,
    h_pos: Position,
    v_pos: Position,
    str: &str,
    opts: &[Whitespace],
) -> String {
    let h = place_horizontal(width, h_pos, str, opts);
    place_vertical(height, v_pos, &h, opts)
}

/// <upstream-comment>PlaceHorizontal places a string or text block horizontally in an unstyled
/// block of a given width. If the given width is shorter than the max width of
/// the string (measured by its longest line) this will be a noöp.</upstream-comment>
pub fn place_horizontal(width: usize, pos: Position, str: &str, opts: &[Whitespace]) -> String {
    let (lines, content_width) = get_lines(str);
    let gap = width as isize - content_width as isize;

    if gap <= 0 {
        return str.to_string();
    }
    let gap = gap as usize;

    let ws = Whitespace::new(opts);

    let mut out = String::new();
    for (i, l) in lines.iter().enumerate() {
        // Is this line shorter than the longest line?
        let short = max0(content_width as isize - size::width(l) as isize);

        match pos {
            LEFT => {
                out.push_str(l);
                out.push_str(&ws.render(gap + short));
            }
            RIGHT => {
                out.push_str(&ws.render(gap + short));
                out.push_str(l);
            }
            _ => {
                // somewhere in the middle
                let total_gap = gap + short;
                let split = (total_gap as f64 * pos.value()).round() as usize;
                let left = total_gap - split;
                let right = total_gap - left;
                out.push_str(&ws.render(left));
                out.push_str(l);
                out.push_str(&ws.render(right));
            }
        }

        if i < lines.len() - 1 {
            out.push('\n');
        }
    }

    out
}

/// <upstream-comment>PlaceVertical places a string or text block vertically in an unstyled block
/// of a given height. If the given height is shorter than the height of the
/// string (measured by its newlines) then this will be a noöp.</upstream-comment>
pub fn place_vertical(height: usize, pos: Position, str: &str, opts: &[Whitespace]) -> String {
    let content_height = str.matches('\n').count() + 1;
    let gap = height as isize - content_height as isize;

    if gap <= 0 {
        return str.to_string();
    }
    let gap = gap as usize;

    let ws = Whitespace::new(opts);

    let (_, width) = get_lines(str);
    let empty_line = ws.render(width);
    let mut out = String::new();

    match pos {
        TOP => {
            out.push_str(str);
            out.push('\n');
            for i in 0..gap {
                out.push_str(&empty_line);
                if i < gap - 1 {
                    out.push('\n');
                }
            }
        }
        crate::align::BOTTOM => {
            for _ in 0..gap {
                out.push_str(&empty_line);
                out.push('\n');
            }
            out.push_str(str);
        }
        _ => {
            // Somewhere in the middle
            let split = (gap as f64 * pos.value()).round() as usize;
            let top = gap - split;
            let bottom = gap - top;
            for _ in 0..top {
                out.push_str(&empty_line);
                out.push('\n');
            }
            out.push_str(str);
            for _ in 0..bottom {
                out.push('\n');
                out.push_str(&empty_line);
            }
        }
    }

    out
}

fn max0(v: isize) -> usize {
    if v < 0 {
        0
    } else {
        v as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_horizontal() {
        assert_eq!(
            place_horizontal(10, Position::CENTER, "Hi", &[]),
            "    Hi    "
        );
        assert_eq!(
            place_horizontal(10, Position::LEFT, "Hi", &[]),
            "Hi        "
        );
        assert_eq!(
            place_horizontal(10, Position::RIGHT, "Hi", &[]),
            "        Hi"
        );
    }

    #[test]
    fn test_place_vertical() {
        assert_eq!(place_vertical(3, Position::CENTER, "Hi", &[]), "  \nHi\n  ");
        assert_eq!(place_vertical(3, Position::TOP, "Hi", &[]), "Hi\n  \n  ");
    }

    #[test]
    fn test_place() {
        assert_eq!(
            place(6, 3, Position::CENTER, Position::CENTER, "Hi", &[]),
            "      \n  Hi  \n      "
        );
    }
}
