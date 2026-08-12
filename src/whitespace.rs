//! Cleanroom Rust port of upstream Go source file: `whitespace.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Whitespace rendering options used by the placement functions.
//! </public-docs>

use crate::size;
use crate::style::Style;

/// A whitespace renderer.
#[derive(Debug, Clone, Default)]
pub struct Whitespace {
    /// The characters to render in the whitespace.
    pub chars: Option<String>,
    /// The style applied to the whitespace.
    pub style: Option<Style>,
}

impl Whitespace {
    /// Creates a new whitespace renderer from the given options.
    pub fn new(opts: &[Whitespace]) -> Whitespace {
        let mut w = Whitespace::default();
        for opt in opts {
            if opt.chars.is_some() {
                w.chars = opt.chars.clone();
            }
            if opt.style.is_some() {
                w.style = opt.style.clone();
            }
        }
        w
    }

    /// Renders `width` cells of whitespace.
    pub fn render(&self, width: usize) -> String {
        let chars = self.chars.clone().unwrap_or_else(|| " ".to_string());
        let runes: Vec<char> = chars.chars().collect();
        let mut j = 0usize;
        let mut b = String::new();

        // Cycle through runes and print them into the whitespace.
        let mut i = 0usize;
        while i < width {
            let r = runes[j];
            b.push(r);
            // Measure the width of the rune we just wrote, ensuring we always
            // make progress to avoid infinite loops with zero-width characters
            // like tabs.
            let mut rune_width = size::width(&r.to_string());
            if rune_width < 1 {
                rune_width = 1;
            }
            i += rune_width;
            j += 1;
            if j >= runes.len() {
                j = 0;
            }
        }

        // Fill any extra gaps with white spaces. This might be necessary if any
        // runes are more than one cell wide, which could leave a one-rune gap.
        let short = width as isize - size::width(&b) as isize;
        if short > 0 {
            b.push_str(&" ".repeat(short as usize));
        }

        match &self.style {
            Some(s) => s.render(&b),
            None => b,
        }
    }
}

/// <upstream-comment>WithWhitespaceChars sets the characters to be rendered in the whitespace.</upstream-comment>
pub fn with_whitespace_chars(s: &str) -> Whitespace {
    Whitespace {
        chars: Some(s.to_string()),
        style: None,
    }
}

/// <upstream-comment>WithWhitespaceStyle sets the style for the whitespace.</upstream-comment>
pub fn with_whitespace_style(s: Style) -> Whitespace {
    Whitespace {
        chars: None,
        style: Some(s),
    }
}

/// Generates a whitespace string of specified length with the given space character.
pub fn space(len: usize) -> String {
    " ".repeat(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_render() {
        let ws = Whitespace::new(&[]);
        assert_eq!(ws.render(5), "     ");
        let ws = with_whitespace_chars("*");
        assert_eq!(ws.render(5), "*****");
        let ws = with_whitespace_chars("\t");
        assert!(!ws.render(10).is_empty());
    }

    #[test]
    fn test_whitespace_options() {
        let ws = Whitespace::new(&[with_whitespace_chars("猫咪")]);
        // 猫咪 is 2 cells per character; width 6 is filled by three characters.
        assert_eq!(ws.render(6), "猫咪猫");
        let ws = Whitespace::new(&[with_whitespace_style(Style::new().bold(true))]);
        assert_eq!(ws.render(3), "\x1b[1m   \x1b[m");
    }
}
