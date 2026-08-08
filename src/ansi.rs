//! Cleanroom Rust port of upstream Go dependency: `github.com/charmbracelet/x/ansi` (SGR `Style` subset)
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! An ANSI SGR style builder mirroring the subset of `charmbracelet/x/ansi` used by
//! upstream Lip Gloss to render styled text. Output sequences match upstream byte-for-byte.
//! </public-docs>

use std::io::Write;

use crate::color::Color;
use crate::size;

/// ANSI reset sequence (matches `ansi.ResetStyle`).
pub const RESET_STYLE: &str = "\x1b[m";

/// Style of the underline.
///
/// Caveats:
/// - Not all terminals support all underline styles.
/// - Some terminals may render unsupported styles as standard underlines.
/// - Terminal themes may affect the visibility of different underline styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Underline {
    /// No underline.
    None,
    /// A single underline. This is the default when underline is enabled.
    Single,
    /// A double underline.
    Double,
    /// A curly underline.
    Curly,
    /// A dotted underline.
    Dotted,
    /// A dashed underline.
    Dashed,
}

impl Default for Underline {
    fn default() -> Self {
        Underline::None
    }
}

/// <upstream-comment>UnderlineNone is no underline.</upstream-comment>
pub const UNDERLINE_NONE: Underline = Underline::None;
/// <upstream-comment>UnderlineSingle is a single underline. This is the default when underline is enabled.</upstream-comment>
pub const UNDERLINE_SINGLE: Underline = Underline::Single;
/// <upstream-comment>UnderlineDouble is a double underline.</upstream-comment>
pub const UNDERLINE_DOUBLE: Underline = Underline::Double;
/// <upstream-comment>UnderlineCurly is a curly underline.</upstream-comment>
pub const UNDERLINE_CURLY: Underline = Underline::Curly;
/// <upstream-comment>UnderlineDotted is a dotted underline.</upstream-comment>
pub const UNDERLINE_DOTTED: Underline = Underline::Dotted;
/// <upstream-comment>UnderlineDashed is a dashed underline.</upstream-comment>
pub const UNDERLINE_DASHED: Underline = Underline::Dashed;

/// An ANSI SGR style. A zero value renders nothing.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    /// Bold text attribute (SGR 1).
    pub bold: bool,
    /// Italic text attribute (SGR 3).
    pub italic: bool,
    /// Underline text attribute (SGR 4).
    pub underline: bool,
    /// The style of the underline, if underlined.
    pub underline_style: Underline,
    /// Reverse video attribute (SGR 7).
    pub reverse: bool,
    /// Blink attribute (SGR 5).
    pub blink: bool,
    /// Faint/dim attribute (SGR 2).
    pub faint: bool,
    /// Strikethrough attribute (SGR 9).
    pub strikethrough: bool,
    /// Foreground color.
    pub fg_color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Underline color.
    pub ul_color: Option<Color>,
}

impl Style {
    /// Returns whether this style has no attributes or colors set.
    pub fn is_zero(&self) -> bool {
        self == &Style::default()
    }

    /// Returns the SGR sequence for this style, e.g. `\x1b[1;38;2;90;86;224m`.
    pub fn string(&self) -> String {
        let mut params: Vec<String> = Vec::new();
        if self.bold {
            params.push("1".to_string());
        }
        if self.faint {
            params.push("2".to_string());
        }
        if self.italic {
            params.push("3".to_string());
        }
        if self.underline {
            params.push("4".to_string());
        }
        if self.blink {
            params.push("5".to_string());
        }
        if self.reverse {
            params.push("7".to_string());
        }
        if self.strikethrough {
            params.push("9".to_string());
        }
        if let Some(ref c) = self.fg_color {
            params.push(color_seq(c, 3));
        }
        if let Some(ref c) = self.bg_color {
            params.push(color_seq(c, 4));
        }
        if let Some(ref c) = self.ul_color {
            params.push(color_seq(c, 5));
        }
        if self.underline {
            match self.underline_style {
                Underline::None => {}
                Underline::Single => params.push("4".to_string()),
                Underline::Double => params.push("21".to_string()),
                Underline::Curly => params.push("4:3".to_string()),
                Underline::Dotted => params.push("4:4".to_string()),
                Underline::Dashed => params.push("4:5".to_string()),
            }
        }
        if params.is_empty() {
            return String::new();
        }
        format!("\x1b[{}m", params.join(";"))
    }

    /// Applies the style to the given string, wrapping it in the SGR sequence
    /// and an ANSI reset.
    pub fn styled(&self, s: &str) -> String {
        if self.is_zero() {
            return s.to_string();
        }
        format!("{}{}{}", self.string(), s, RESET_STYLE)
    }
}

// ---------------------------------------------------------------------------
// ANSI string helpers
// ---------------------------------------------------------------------------

/// Strips ANSI escape sequences from the given string.
pub fn strip(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c == '\x1b' {
            // Consume the escape sequence.
            if it.peek() == Some(&'[') || it.peek() == Some(&']') {
                let _ = it.next();
                if it.peek() == Some(&']') {
                    // OSC sequence: read until BEL (0x07) or ST (ESC \).
                    while let Some(c) = it.next() {
                        if c == '\x07' {
                            break;
                        }
                        if c == '\x1b' && it.peek() == Some(&'\\') {
                            let _ = it.next();
                            break;
                        }
                    }
                } else {
                    // CSI sequence: read until final byte (0x40..=0x7E).
                    for c in it.by_ref() {
                        if ('\x40'..='\x7e').contains(&c) {
                            break;
                        }
                    }
                }
            }
            continue;
        }
        out.push(c);
    }
    out
}

/// Cuts the given string from `start` to `end` cell positions, preserving ANSI
/// sequences. Positions beyond the string's length are clamped.
pub fn cut(s: &str, start: usize, end: usize) -> String {
    let mut out = String::new();
    let mut width = 0usize;
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c == '\x1b' {
            // Copy the escape sequence verbatim.
            out.push(c);
            if it.peek() == Some(&'[') || it.peek() == Some(&']') {
                let o = it.next().unwrap();
                out.push(o);
                if o == ']' {
                    while let Some(c) = it.next() {
                        out.push(c);
                        if c == '\x07' {
                            break;
                        }
                        if c == '\x1b' && it.peek() == Some(&'\\') {
                            out.push(it.next().unwrap());
                            break;
                        }
                    }
                } else {
                    for c in it.by_ref() {
                        out.push(c);
                        if ('\x40'..='\x7e').contains(&c) {
                            break;
                        }
                    }
                }
            }
            continue;
        }
        let w = size::width(&c.to_string());
        if width >= end {
            break;
        }
        if width >= start {
            out.push(c);
        }
        width += w;
    }
    out
}

/// Truncates the given string to `max_width` cells, appending `tail` (usually
/// `"…"`) if truncation occurred. ANSI sequences are preserved.
pub fn truncate(s: &str, max_width: usize, tail: &str) -> String {
    let width = size::width(s);
    if width <= max_width {
        return s.to_string();
    }
    if max_width == 0 {
        return tail.to_string();
    }
    let tw = size::width(tail);
    if tw >= max_width {
        // The tail is wider than the available space; truncate the tail itself.
        return truncate(tail, max_width, "");
    }
    let cut_at = max_width - tw;
    format!("{}{}", cut(s, 0, cut_at), tail)
}

/// Truncates the given string keeping the leftmost `max_width` cells.
pub fn truncate_left(s: &str, max_width: usize, head: &str) -> String {
    let width = size::width(s);
    if width <= max_width {
        return s.to_string();
    }
    if max_width == 0 {
        return head.to_string();
    }
    let hw = size::width(head);
    if hw >= max_width {
        return truncate_left(head, max_width, "");
    }
    let keep = max_width - hw;
    format!("{}{}", head, cut(s, width - keep, width))
}

/// Hyperlink OSC sequence prefix.
pub fn set_hyperlink(link: &str, params: &str) -> String {
    if params.is_empty() {
        format!("\x1b]8;;{}\x07", link)
    } else {
        format!("\x1b]8;{};{}\x07", params, link)
    }
}

/// Hyperlink reset sequence.
pub fn reset_hyperlink() -> &'static str {
    "\x1b]8;;\x07"
}

/// Wraps the given string to the given width, preserving ANSI styles and links.
pub fn wrap(s: &str, width: usize, breakpoints: &str) -> String {
    if width == 0 {
        return s.to_string();
    }
    let mut out = Vec::new();
    {
        let mut w = WrapWriter::new(&mut out);
        let wrapped = word_wrap_ansi(s, width, breakpoints);
        let _ = w.write(wrapped.as_bytes());
        let _ = w.close();
    }
    String::from_utf8(out).unwrap_or_else(|_| s.to_string())
}

/// Consumes a full ANSI escape sequence starting at index `i` (which must point
/// at `\x1b`), returning the sequence string and the index of the next char.
fn take_escape(chars: &[char], i: usize) -> (String, usize) {
    let mut j = i + 1;
    let mut seq = String::new();
    seq.push('\x1b');
    if j >= chars.len() {
        return (seq, j);
    }
    seq.push(chars[j]);
    if chars[j] == '[' {
        // CSI: consume until the final byte (0x40..=0x7e).
        j += 1;
        while j < chars.len() {
            let c = chars[j];
            seq.push(c);
            j += 1;
            if ('\x40'..='\x7e').contains(&c) {
                break;
            }
        }
    } else if chars[j] == ']' {
        // OSC: consume until BEL (0x07) or ST (ESC \).
        j += 1;
        while j < chars.len() {
            let c = chars[j];
            seq.push(c);
            j += 1;
            if c == '\x07' {
                break;
            }
            if c == '\x1b' && chars.get(j) == Some(&'\\') {
                seq.push('\\');
                j += 1;
                break;
            }
        }
    }
    (seq, j)
}

/// Word-wraps a string that may contain ANSI escape sequences, mirroring
/// `ansi.Wrap` (grapheme variant) byte-for-byte: whitespace is preserved,
/// words are not broken, and escape sequences do not count towards width.
#[allow(unused_assignments)]
fn word_wrap_ansi(s: &str, width: usize, breakpoints: &str) -> String {
    if width < 1 {
        return s.to_string();
    }
    let breakpoints: Vec<char> = breakpoints.chars().collect();

    let mut buf = String::new();
    let mut word = String::new();
    let mut space = String::new();
    let mut space_width = 0usize;
    let mut cur_width = 0usize;
    let mut word_len = 0usize;

    macro_rules! add_space {
        () => {
            if space_width == 0 && space.is_empty() {
                // no-op
            } else {
                cur_width += space_width;
                buf.push_str(&space);
                space.clear();
                space_width = 0;
            }
        };
    }
    macro_rules! add_word {
        () => {
            if !word.is_empty() {
                add_space!();
                cur_width += word_len;
                buf.push_str(&word);
                word.clear();
                word_len = 0;
            }
        };
    }
    macro_rules! add_newline {
        () => {
            buf.push('\n');
            cur_width = 0;
            space.clear();
            space_width = 0;
        };
    }

    // Tokenize the input into graphemes and ANSI escape sequences.
    let mut tokens: Vec<String> = Vec::new();
    let mut rest = s;
    while !rest.is_empty() {
        if rest.starts_with('\x1b') {
            let (seq, next) = take_escape_str(rest);
            tokens.push(seq);
            rest = &rest[next..];
        } else {
            let g = unicode_segmentation::UnicodeSegmentation::graphemes(rest, true)
                .next()
                .unwrap();
            tokens.push(g.to_string());
            rest = &rest[g.len()..];
        }
    }

    for token in &tokens {
        if token.starts_with('\x1b') {
            // Escape sequences are accumulated in the word without width.
            word.push_str(token);
            continue;
        }
        let r = token.chars().next().unwrap();
        let w = size::width(token);
        if r == '\n' {
            if word_len == 0 {
                if cur_width + space_width > width {
                    cur_width = 0;
                } else {
                    // preserve whitespaces
                    buf.push_str(&space);
                }
                space.clear();
                space_width = 0;
            }
            add_word!();
            add_newline!();
        } else if r.is_whitespace() && r != '\u{00A0}' {
            add_word!();
            space.push_str(token);
            space_width += w;
        } else if breakpoints.contains(&r) || r == '-' {
            add_space!();
            if cur_width + word_len + w > width {
                // We can't fit the breakpoint in the current line, treat it
                // as part of the word.
                word.push_str(token);
                word_len += w;
            } else {
                add_word!();
                buf.push_str(token);
                cur_width += w;
            }
        } else {
            if word_len + w > width {
                // Hardwrap the word if it's too long.
                add_word!();
            }
            word.push_str(token);
            word_len += w;
            if cur_width + word_len + space_width > width {
                add_newline!();
            }
            if word_len == width {
                // Hardwrap the word if it's too long.
                add_word!();
            }
        }
    }

    if word_len == 0 {
        if cur_width + space_width > width {
            cur_width = 0;
        } else {
            // preserve whitespaces
            buf.push_str(&space);
        }
        space.clear();
        space_width = 0;
    }
    add_word!();

    buf
}

/// Consumes a full ANSI escape sequence starting at the start of the string,
/// returning the sequence and the number of bytes consumed.
fn take_escape_str(s: &str) -> (String, usize) {
    let chars: Vec<char> = s.chars().collect();
    let (seq, next) = take_escape(&chars, 0);
    (seq, next)
}

/// WrapWriter writes to a buffer and keeps track of the current pen style and
/// link state for the purpose of wrapping with newlines.
///
/// When it encounters a newline, it resets the style and link, writes the
/// newline, and then reapplies the style and link to the next line.
pub struct WrapWriter<'a> {
    w: &'a mut dyn Write,
    style: Style,
    link: String,
    link_params: String,
    link_on: bool,
}

impl<'a> WrapWriter<'a> {
    /// Returns a new `WrapWriter`.
    pub fn new(w: &'a mut dyn Write) -> WrapWriter<'a> {
        WrapWriter {
            w,
            style: Style::default(),
            link: String::new(),
            link_params: String::new(),
            link_on: false,
        }
    }

    /// Returns the current pen style.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Returns the current pen link.
    pub fn link(&self) -> &str {
        &self.link
    }

    /// Writes bytes to the buffer.
    pub fn write(&mut self, p: &[u8]) -> std::io::Result<usize> {
        for &b in p {
            if b == b'\n' {
                if !self.style.is_zero() {
                    self.w.write_all(RESET_STYLE.as_bytes())?;
                }
                if self.link_on {
                    self.w.write_all(reset_hyperlink().as_bytes())?;
                }
            }
            self.w.write_all(&[b])?;
            if b == b'\n' {
                if self.link_on {
                    self.w
                        .write_all(set_hyperlink(&self.link, &self.link_params).as_bytes())?;
                }
                if !self.style.is_zero() {
                    self.w.write_all(self.style.string().as_bytes())?;
                }
            }
        }
        Ok(p.len())
    }

    /// Closes the writer, resetting the style and link if necessary.
    pub fn close(&mut self) -> std::io::Result<()> {
        if !self.style.is_zero() {
            self.w.write_all(RESET_STYLE.as_bytes())?;
        }
        if self.link_on {
            self.w.write_all(reset_hyperlink().as_bytes())?;
        }
        Ok(())
    }
}

fn color_seq(c: &Color, base: u8) -> String {
    match c {
        Color::Ansi16(v) => {
            if *v < 8 {
                format!("{}", base * 10 + v)
            } else {
                format!("{}", base * 10 + 60 + (v - 8))
            }
        }
        Color::Ansi256(v) => format!("{};5;{}", base * 10 + 8, v),
        Color::TrueColor { r, g, b } => format!("{};2;{};{};{}", base * 10 + 8, r, g, b),
        Color::Adaptive { dark, .. } => color_seq(dark, base),
        Color::Complete { true_color, .. } => color_seq(true_color, base),
        Color::NoColor => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sgr_sequences() {
        let mut s = Style::default();
        s.bold = true;
        assert_eq!(s.string(), "\x1b[1m");
        assert_eq!(s.styled("hello"), "\x1b[1mhello\x1b[m");

        let mut s = Style::default();
        s.fg_color = Some(Color::parse("#5A56E0"));
        assert_eq!(s.string(), "\x1b[38;2;90;86;224m");
        assert_eq!(s.styled("hello"), "\x1b[38;2;90;86;224mhello\x1b[m");
    }

    #[test]
    fn test_underline_styles() {
        let mut s = Style::default();
        s.underline = true;
        s.underline_style = Underline::Single;
        assert_eq!(s.string(), "\x1b[4;4m");

        let mut s = Style::default();
        s.underline = true;
        s.underline_style = Underline::Curly;
        s.ul_color = Some(Color::parse("#FF0000"));
        assert_eq!(s.string(), "\x1b[4;58;2;255;0;0;4:3m");

        let mut s = Style::default();
        s.underline = true;
        s.underline_style = Underline::Curly;
        assert_eq!(s.string(), "\x1b[4;4:3m");
    }
}
