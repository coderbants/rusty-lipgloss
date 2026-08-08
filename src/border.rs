//! Cleanroom Rust port of upstream Go source file: `borders.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Border definitions, preset border styles, and border measurement utilities.
//! </public-docs>

use crate::size;

/// A precomputed set of blended border colors for each edge.
#[derive(Debug, Clone)]
pub struct BorderBlend {
    /// Gradient for the top edge.
    pub top_gradient: Vec<crate::color::Color>,
    /// Gradient for the right edge.
    pub right_gradient: Vec<crate::color::Color>,
    /// Gradient for the bottom edge.
    pub bottom_gradient: Vec<crate::color::Color>,
    /// Gradient for the left edge.
    pub left_gradient: Vec<crate::color::Color>,
}

impl BorderBlend {
    /// Computes the border blend gradients for a box of the given dimensions.
    pub fn new(
        width: usize,
        height: usize,
        colors: &[crate::color::Color],
        offset: isize,
    ) -> BorderBlend {
        let total = (height + width + 2) * 2;
        let mut gradient = crate::blending::blend_1d(total, colors);

        // Rotate the array forward or reverse based on the offset if provided.
        let r = -offset;
        if r != 0 {
            let n = gradient.len();
            let r = r.rem_euclid(n as isize) as usize;
            if n > 0 {
                gradient[..r].reverse();
                gradient[r..].reverse();
                gradient.reverse();
            }
        }

        let mut cursor = 0usize;
        let mut take = |size: usize, g: &mut Vec<crate::color::Color>| {
            let s = gradient[cursor..cursor + size].to_vec();
            cursor += size;
            g.extend(s);
        };

        let mut blend = BorderBlend {
            top_gradient: Vec::new(),
            right_gradient: Vec::new(),
            bottom_gradient: Vec::new(),
            left_gradient: Vec::new(),
        };
        take(width + 2, &mut blend.top_gradient);
        take(height, &mut blend.right_gradient);
        take(width + 2, &mut blend.bottom_gradient);
        take(height, &mut blend.left_gradient);

        // Bottom and left gradients are reversed because they are drawn in
        // reverse order.
        blend.bottom_gradient.reverse();
        blend.left_gradient.reverse();

        blend
    }
}

/// <upstream-comment>Border contains a series of values which comprise the various parts of a
/// border.</upstream-comment>
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Border {
    /// The character drawn along the top edge.
    pub top: String,
    /// The character drawn along the bottom edge.
    pub bottom: String,
    /// The character drawn along the left edge.
    pub left: String,
    /// The character drawn along the right edge.
    pub right: String,
    /// The top-left corner character.
    pub top_left: String,
    /// The top-right corner character.
    pub top_right: String,
    /// The bottom-left corner character.
    pub bottom_left: String,
    /// The bottom-right corner character.
    pub bottom_right: String,
    /// The middle-left edge character (used for row separators).
    pub middle_left: String,
    /// The middle-right edge character (used for row separators).
    pub middle_right: String,
    /// The middle crossing character (used for internal intersections).
    pub middle: String,
    /// The middle-top character (used for column separators on the top edge).
    pub middle_top: String,
    /// The middle-bottom character (used for column separators on the bottom edge).
    pub middle_bottom: String,
}

impl Border {
    /// <upstream-comment>GetTopSize returns the width of the top border. If borders contain runes of
    /// varying widths, the widest rune is returned. If no border exists on the top
    /// edge, 0 is returned.</upstream-comment>
    pub fn get_top_size(&self) -> usize {
        get_border_edge_width(&[self.top_left.as_str(), self.top.as_str(), self.top_right.as_str()])
    }

    /// <upstream-comment>GetRightSize returns the width of the right border. If borders contain
    /// runes of varying widths, the widest rune is returned. If no border exists on
    /// the right edge, 0 is returned.</upstream-comment>
    pub fn get_right_size(&self) -> usize {
        get_border_edge_width(&[
            self.top_right.as_str(),
            self.right.as_str(),
            self.bottom_right.as_str(),
        ])
    }

    /// <upstream-comment>GetBottomSize returns the width of the bottom border. If borders contain
    /// runes of varying widths, the widest rune is returned. If no border exists on
    /// the bottom edge, 0 is returned.</upstream-comment>
    pub fn get_bottom_size(&self) -> usize {
        get_border_edge_width(&[
            self.bottom_left.as_str(),
            self.bottom.as_str(),
            self.bottom_right.as_str(),
        ])
    }

    /// <upstream-comment>GetLeftSize returns the width of the left border. If borders contain runes
    /// of varying widths, the widest rune is returned. If no border exists on the
    /// left edge, 0 is returned.</upstream-comment>
    pub fn get_left_size(&self) -> usize {
        get_border_edge_width(&[
            self.top_left.as_str(),
            self.left.as_str(),
            self.bottom_left.as_str(),
        ])
    }

    /// <upstream-comment>NormalBorder returns a standard-type border with a normal weight and 90
    /// degree corners.</upstream-comment>
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
            middle_left: "├".into(),
            middle_right: "┤".into(),
            middle: "┼".into(),
            middle_top: "┬".into(),
            middle_bottom: "┴".into(),
        }
    }

    /// <upstream-comment>RoundedBorder returns a border with rounded corners.</upstream-comment>
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
            middle_left: "├".into(),
            middle_right: "┤".into(),
            middle: "┼".into(),
            middle_top: "┬".into(),
            middle_bottom: "┴".into(),
        }
    }

    /// <upstream-comment>BlockBorder returns a border that takes the whole block.</upstream-comment>
    pub fn block() -> Self {
        Self {
            top: "█".into(),
            bottom: "█".into(),
            left: "█".into(),
            right: "█".into(),
            top_left: "█".into(),
            top_right: "█".into(),
            bottom_left: "█".into(),
            bottom_right: "█".into(),
            middle_left: "█".into(),
            middle_right: "█".into(),
            middle: "█".into(),
            middle_top: "█".into(),
            middle_bottom: "█".into(),
        }
    }

    /// <upstream-comment>OuterHalfBlockBorder returns a half-block border that sits outside the frame.</upstream-comment>
    pub fn outer_half_block() -> Self {
        Self {
            top: "▀".into(),
            bottom: "▄".into(),
            left: "▌".into(),
            right: "▐".into(),
            top_left: "▛".into(),
            top_right: "▜".into(),
            bottom_left: "▙".into(),
            bottom_right: "▟".into(),
            ..Self::default()
        }
    }

    /// <upstream-comment>InnerHalfBlockBorder returns a half-block border that sits inside the frame.</upstream-comment>
    pub fn inner_half_block() -> Self {
        Self {
            top: "▄".into(),
            bottom: "▀".into(),
            left: "▐".into(),
            right: "▌".into(),
            top_left: "▗".into(),
            top_right: "▖".into(),
            bottom_left: "▝".into(),
            bottom_right: "▘".into(),
            ..Self::default()
        }
    }

    /// <upstream-comment>ThickBorder returns a border that's thicker than the one returned by
    /// NormalBorder.</upstream-comment>
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
            middle_left: "┣".into(),
            middle_right: "┫".into(),
            middle: "╋".into(),
            middle_top: "┳".into(),
            middle_bottom: "┻".into(),
        }
    }

    /// <upstream-comment>DoubleBorder returns a border comprised of two thin strokes.</upstream-comment>
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
            middle_left: "╠".into(),
            middle_right: "╣".into(),
            middle: "╬".into(),
            middle_top: "╦".into(),
            middle_bottom: "╩".into(),
        }
    }

    /// <upstream-comment>HiddenBorder returns a border that renders as a series of single-cell
    /// spaces. It's useful for cases when you want to remove a standard border but
    /// maintain layout positioning. This said, you can still apply a background
    /// color to a hidden border.</upstream-comment>
    pub fn hidden() -> Self {
        Self {
            top: " ".into(),
            bottom: " ".into(),
            left: " ".into(),
            right: " ".into(),
            top_left: " ".into(),
            top_right: " ".into(),
            bottom_left: " ".into(),
            bottom_right: " ".into(),
            middle_left: " ".into(),
            middle_right: " ".into(),
            middle: " ".into(),
            middle_top: " ".into(),
            middle_bottom: " ".into(),
        }
    }

    /// <upstream-comment>MarkdownBorder return a table border in markdown style.
    ///
    /// Make sure to disable top and bottom border for the best result. This will
    /// ensure that the output is valid markdown.
    ///
    /// ```text
    /// table.New().Border(lipgloss.MarkdownBorder()).BorderTop(false).BorderBottom(false)
    /// ```</upstream-comment>
    pub fn markdown() -> Self {
        Self {
            top: "-".into(),
            bottom: "-".into(),
            left: "|".into(),
            right: "|".into(),
            top_left: "|".into(),
            top_right: "|".into(),
            bottom_left: "|".into(),
            bottom_right: "|".into(),
            middle_left: "|".into(),
            middle_right: "|".into(),
            middle: "|".into(),
            middle_top: "|".into(),
            middle_bottom: "|".into(),
        }
    }

    /// <upstream-comment>ASCIIBorder returns a table border with ASCII characters.</upstream-comment>
    pub fn ascii() -> Self {
        Self {
            top: "-".into(),
            bottom: "-".into(),
            left: "|".into(),
            right: "|".into(),
            top_left: "+".into(),
            top_right: "+".into(),
            bottom_left: "+".into(),
            bottom_right: "+".into(),
            middle_left: "+".into(),
            middle_right: "+".into(),
            middle: "+".into(),
            middle_top: "+".into(),
            middle_bottom: "+".into(),
        }
    }
}

/// Alias matching upstream `NormalBorder()`.
pub fn normal_border() -> Border {
    Border::normal()
}

/// Alias matching upstream `RoundedBorder()`.
pub fn rounded_border() -> Border {
    Border::rounded()
}

/// Alias matching upstream `BlockBorder()`.
pub fn block_border() -> Border {
    Border::block()
}

/// Alias matching upstream `OuterHalfBlockBorder()`.
pub fn outer_half_block_border() -> Border {
    Border::outer_half_block()
}

/// Alias matching upstream `InnerHalfBlockBorder()`.
pub fn inner_half_block_border() -> Border {
    Border::inner_half_block()
}

/// Alias matching upstream `ThickBorder()`.
pub fn thick_border() -> Border {
    Border::thick()
}

/// Alias matching upstream `DoubleBorder()`.
pub fn double_border() -> Border {
    Border::double()
}

/// Alias matching upstream `HiddenBorder()`.
pub fn hidden_border() -> Border {
    Border::hidden()
}

/// Alias matching upstream `MarkdownBorder()`.
pub fn markdown_border() -> Border {
    Border::markdown()
}

/// Alias matching upstream `ASCIIBorder()`.
pub fn ascii_border() -> Border {
    Border::ascii()
}

/// The empty border (`Border{}` in upstream).
pub fn no_border() -> Border {
    Border::default()
}

fn get_border_edge_width(border_parts: &[&str]) -> usize {
    border_parts
        .iter()
        .map(|piece| max_rune_width(piece))
        .max()
        .unwrap_or(0)
}

/// Returns the width of the widest rune in the string.
pub fn max_rune_width(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }
    s.chars().map(|c| size::width(&c.to_string())).max().unwrap_or(0)
}

/// Returns the first rune of the string as a String.
pub fn get_first_rune_as_string(s: &str) -> &str {
    s.chars().next().map_or("", |c| &s[..c.len_utf8()])
}

/// Render the horizontal (top or bottom) portion of a border.
pub(crate) fn render_horizontal_edge(left: &str, middle: &str, right: &str, width: usize) -> String {
    let middle = if middle.is_empty() { " " } else { middle };
    let left_width = size::width(left);
    let right_width = size::width(right);

    let mut out = String::new();
    out.push_str(left);

    let runes: Vec<char> = middle.chars().collect();
    let mut j = 0;
    let mut i = 0usize;
    while i < width.saturating_sub(left_width + right_width) {
        let r = runes[j];
        out.push(r);
        i += size::width(&r.to_string());
        j += 1;
        if j >= runes.len() {
            j = 0;
        }
    }

    out.push_str(right);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presets() {
        assert_eq!(Border::normal().top_left, "┌");
        assert_eq!(Border::rounded().top_left, "╭");
        assert_eq!(Border::double().top_left, "╔");
        assert_eq!(Border::thick().top_left, "┏");
        assert_eq!(Border::block().top, "█");
        assert_eq!(Border::ascii().top_left, "+");
        assert_eq!(Border::markdown().top, "-");
        assert_eq!(Border::hidden().top, " ");
    }

    #[test]
    fn test_get_first_rune_as_string() {
        assert_eq!(get_first_rune_as_string("Hello"), "H");
        assert_eq!(get_first_rune_as_string("世"), "世");
        assert_eq!(get_first_rune_as_string("😀Happy"), "😀");
        assert_eq!(get_first_rune_as_string(""), "");
    }

    #[test]
    fn test_max_rune_width() {
        assert_eq!(max_rune_width(" "), 1);
        assert_eq!(max_rune_width("╭"), 1);
    }

    #[test]
    fn test_render_horizontal_edge() {
        let b = Border::normal();
        let edge = render_horizontal_edge(&b.top_left, &b.top, &b.top_right, 5);
        assert_eq!(edge, "┌───┐");
    }
}
