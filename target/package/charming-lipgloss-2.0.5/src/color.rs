//! Terminal color abstractions for ANSI 16, 256, TrueColor (RGB), AdaptiveColor, and CompleteColor.

use std::fmt;

/// <upstream-comment>TerminalColor is the interface for colors used in Lip Gloss.</upstream-comment>
pub trait TerminalColor: fmt::Display {
    /// Renders the ANSI escape code sequence prefix for foreground color.
    fn render_fg(&self) -> String;
    /// Renders the ANSI escape code sequence prefix for background color.
    fn render_bg(&self) -> String;
}

/// <upstream-comment>Color represents a color by hexadecimal, ANSI 256, or ANSI 16 value.</upstream-comment>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    /// Standard 16-color ANSI code (0-15)
    Ansi16(u8),
    /// Extended 256-color ANSI code (0-255)
    Ansi256(u8),
    /// TrueColor RGB represented as Hex string or RGB triplet
    TrueColor { r: u8, g: u8, b: u8 },
    /// No color specified / transparent
    NoColor,
}

impl Color {
    /// <upstream-comment>Color returns a new Color from a hex or ANSI color string.</upstream-comment>
    pub fn parse(val: &str) -> Self {
        if val.is_empty() {
            return Color::NoColor;
        }
        if let Some(hex) = val.strip_prefix('#') {
            if hex.len() == 6 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex[0..2], 16),
                    u8::from_str_radix(&hex[2..4], 16),
                    u8::from_str_radix(&hex[4..6], 16),
                ) {
                    return Color::TrueColor { r, g, b };
                }
            } else if hex.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex[0..1], 16),
                    u8::from_str_radix(&hex[1..2], 16),
                    u8::from_str_radix(&hex[2..3], 16),
                ) {
                    return Color::TrueColor {
                        r: r * 17,
                        g: g * 17,
                        b: b * 17,
                    };
                }
            }
        }
        if let Ok(code) = val.parse::<u8>() {
            if code < 16 {
                return Color::Ansi16(code);
            }
            return Color::Ansi256(code);
        }
        Color::NoColor
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Ansi16(c) => write!(f, "{}", c),
            Color::Ansi256(c) => write!(f, "{}", c),
            Color::TrueColor { r, g, b } => write!(f, "#{:02X}{:02X}{:02X}", r, g, b),
            Color::NoColor => write!(f, ""),
        }
    }
}

impl TerminalColor for Color {
    fn render_fg(&self) -> String {
        match self {
            Color::Ansi16(c) => {
                if *c < 8 {
                    format!("\x1b[{}m", 30 + c)
                } else {
                    format!("\x1b[{}m", 90 + (c - 8))
                }
            }
            Color::Ansi256(c) => format!("\x1b[38;5;{}m", c),
            Color::TrueColor { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
            Color::NoColor => String::new(),
        }
    }

    fn render_bg(&self) -> String {
        match self {
            Color::Ansi16(c) => {
                if *c < 8 {
                    format!("\x1b[{}m", 40 + c)
                } else {
                    format!("\x1b[{}m", 100 + (c - 8))
                }
            }
            Color::Ansi256(c) => format!("\x1b[48;5;{}m", c),
            Color::TrueColor { r, g, b } => format!("\x1b[48;2;{};{};{}m", r, g, b),
            Color::NoColor => String::new(),
        }
    }
}

/// <upstream-comment>AdaptiveColor provides light and dark color options depending on the terminal background.</upstream-comment>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptiveColor {
    pub light: String,
    pub dark: String,
}

impl fmt::Display for AdaptiveColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Light: {}, Dark: {}", self.light, self.dark)
    }
}

impl TerminalColor for AdaptiveColor {
    fn render_fg(&self) -> String {
        Color::parse(&self.dark).render_fg()
    }

    fn render_bg(&self) -> String {
        Color::parse(&self.dark).render_bg()
    }
}

/// <upstream-comment>CompleteColor provides explicit true color, 256 color, and 16 color values.</upstream-comment>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteColor {
    pub true_color: String,
    pub ansi256: String,
    pub ansi: String,
}

impl fmt::Display for CompleteColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.true_color)
    }
}

impl TerminalColor for CompleteColor {
    fn render_fg(&self) -> String {
        Color::parse(&self.true_color).render_fg()
    }

    fn render_bg(&self) -> String {
        Color::parse(&self.true_color).render_bg()
    }
}

