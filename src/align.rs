//! Cleanroom Rust port of upstream Go source file: `position.go` and `align.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Alignment and positioning primitives. `Position` is a float value between
//! 0.0 and 1.0 along an axis (0 = start, 1 = end, 0.5 = center), matching
//! upstream Lip Gloss. Constants `Top`, `Bottom`, `Center`, `Left`, `Right`
//! are provided for readability.
//! </public-docs>

use std::cmp::max;
use std::fmt;

/// <upstream-comment>Position represents a position along a horizontal or vertical axis. It's in
/// situations where an axis is involved, like alignment, joining, placement and
/// so on.
///
/// A value of 0 represents the start (the left or top) and 1 represents the end
/// (the right or bottom). 0.5 represents the center.
///
/// There are constants Top, Bottom, Center, Left and Right in this package that
/// can be used to aid readability.</upstream-comment>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub f64);

impl Position {
    /// The position value clamped to `[0, 1]`.
    pub fn value(self) -> f64 {
        self.0.clamp(0.0, 1.0)
    }
}

/// <upstream-comment>Position aliases.</upstream-comment>
pub const TOP: Position = Position(0.0);
/// <upstream-comment>Position alias for the bottom (end) of an axis.</upstream-comment>
pub const BOTTOM: Position = Position(1.0);
/// <upstream-comment>Position alias for the center of an axis.</upstream-comment>
pub const CENTER: Position = Position(0.5);
/// <upstream-comment>Position alias for the left (start) of an axis.</upstream-comment>
pub const LEFT: Position = Position(0.0);
/// <upstream-comment>Position alias for the right (end) of an axis.</upstream-comment>
pub const RIGHT: Position = Position(1.0);

/// The `Align` type is an alias for `Position`.
pub type Align = Position;

impl Default for Position {
    fn default() -> Self {
        Position(0.0)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Position {
    /// <upstream-comment>Position aliases.</upstream-comment>
    pub const TOP: Position = TOP;
    /// <upstream-comment>Position alias for the bottom (end) of an axis.</upstream-comment>
    pub const BOTTOM: Position = BOTTOM;
    /// <upstream-comment>Position alias for the center of an axis.</upstream-comment>
    pub const CENTER: Position = CENTER;
    /// <upstream-comment>Position alias for the left (start) of an axis.</upstream-comment>
    pub const LEFT: Position = LEFT;
    /// <upstream-comment>Position alias for the right (end) of an axis.</upstream-comment>
    pub const RIGHT: Position = RIGHT;
}

/// Splits a string into lines, additionally returning the size of the widest
/// line. Tabs are replaced with four spaces and `\r\n` sequences are normalized.
pub(crate) fn get_lines(s: &str) -> (Vec<String>, usize) {
    let s = s.replace('\t', "    ");
    let s = s.replace("\r\n", "\n");
    let lines: Vec<String> = s.split('\n').map(|l| l.to_string()).collect();
    let widest = lines
        .iter()
        .map(|l| crate::size::width(l))
        .max()
        .unwrap_or(0);
    (lines, widest)
}

/// Perform text alignment. If the string is multi-lined, we also make all lines
/// the same width by padding them with spaces. If a style is passed, use that
/// to style the spaces added.
pub(crate) fn align_text_horizontal(
    str: &str,
    pos: Position,
    width: usize,
    style: Option<&crate::ansi::Style>,
) -> String {
    let (lines, widest_line) = get_lines(str);
    let mut out = String::new();

    for (i, l) in lines.iter().enumerate() {
        let line_width = crate::size::width(l);

        let mut short_amount = widest_line - line_width;
        short_amount += max(
            0,
            width as isize - (short_amount as isize + line_width as isize),
        ) as usize;

        let mut line = l.to_string();
        if short_amount > 0 {
            match pos {
                RIGHT => {
                    let mut s = " ".repeat(short_amount);
                    if let Some(st) = style {
                        s = st.styled(&s);
                    }
                    line = format!("{}{}", s, l);
                }
                CENTER => {
                    // Note: remainder goes on the right.
                    let left = short_amount / 2;
                    let right = left + short_amount % 2;
                    let left_spaces = " ".repeat(left);
                    let right_spaces = " ".repeat(right);
                    let left_spaces = if let Some(st) = style {
                        st.styled(&left_spaces)
                    } else {
                        left_spaces
                    };
                    let right_spaces = if let Some(st) = style {
                        st.styled(&right_spaces)
                    } else {
                        right_spaces
                    };
                    line = format!("{}{}{}", left_spaces, l, right_spaces);
                }
                _ => {
                    // Left
                    let mut s = " ".repeat(short_amount);
                    if let Some(st) = style {
                        s = st.styled(&s);
                    }
                    line = format!("{}{}", l, s);
                }
            }
        }

        out.push_str(&line);
        if i < lines.len() - 1 {
            out.push('\n');
        }
    }

    out
}

pub(crate) fn align_text_vertical(str: &str, pos: Position, height: usize) -> String {
    let str_height = str.matches('\n').count() + 1;
    if height < str_height {
        return str.to_string();
    }

    match pos {
        TOP => format!("{}{}", str, "\n".repeat(height - str_height)),
        CENTER => {
            let mut top_padding = (height - str_height) / 2;
            let mut bottom_padding = (height - str_height) / 2;
            if str_height + top_padding + bottom_padding > height {
                top_padding -= 1;
            } else if str_height + top_padding + bottom_padding < height {
                bottom_padding += 1;
            }
            format!(
                "{}{}{}",
                "\n".repeat(top_padding),
                str,
                "\n".repeat(bottom_padding)
            )
        }
        BOTTOM => format!("{}{}", "\n".repeat(height - str_height), str),
        _ => str.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_text_vertical() {
        assert_eq!(align_text_vertical("Foo", TOP, 2), "Foo\n");
        assert_eq!(align_text_vertical("Foo", CENTER, 5), "\n\nFoo\n\n");
        assert_eq!(align_text_vertical("Foo", BOTTOM, 5), "\n\n\n\nFoo");
        assert_eq!(
            align_text_vertical("Foo\nBar\nBaz", CENTER, 5),
            "\nFoo\nBar\nBaz\n"
        );
    }

    #[test]
    fn test_align_text_horizontal() {
        assert_eq!(align_text_horizontal("Foo", RIGHT, 5, None), "  Foo");
        assert_eq!(align_text_horizontal("Foo", CENTER, 5, None), " Foo ");
        assert_eq!(align_text_horizontal("Foo", LEFT, 5, None), "Foo  ");
    }
}
