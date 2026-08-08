//! Cleanroom Rust port of upstream Go source files: `compat/color.go` and `compat/doc.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! A compatibility layer providing v1-style adaptive and complete color types.
//! It is "impure" because it uses global state derived from the environment and
//! standard I/O streams.
//! </public-docs>

use crate::color::{Color, Profile};
use crate::query;
use crate::writer::detect_profile;

/// HasDarkBackground is true if the terminal has a dark background.
pub fn has_dark_background() -> bool {
    query::has_dark_background()
}

/// Profile is the color profile of the terminal.
pub fn profile() -> Profile {
    detect_profile()
}

/// <upstream-comment>AdaptiveColor provides color options for light and dark backgrounds. The
/// appropriate color will be returned at runtime based on the darkness of the
/// terminal background color.
///
/// ```text
/// color := lipgloss.AdaptiveColor{Light: "#0000ff", Dark: "#000099"}
/// ```</upstream-comment>
#[derive(Debug, Clone, PartialEq)]
pub struct AdaptiveColor {
    /// The color to use on light backgrounds.
    pub light: Color,
    /// The color to use on dark backgrounds.
    pub dark: Color,
}

impl AdaptiveColor {
    /// Returns the appropriate color for the current terminal background.
    pub fn rgba(&self) -> (u32, u32, u32, u32) {
        if has_dark_background() {
            self.dark.rgba()
        } else {
            self.light.rgba()
        }
    }
}

/// <upstream-comment>CompleteColor specifies exact values for truecolor, ANSI256, and ANSI color
/// profiles. Automatic color degradation will not be performed.</upstream-comment>
#[derive(Debug, Clone, PartialEq)]
pub struct CompleteColor {
    /// The TrueColor value.
    pub true_color: Color,
    /// The 256-color ANSI value.
    pub ansi256: Color,
    /// The 16-color ANSI value.
    pub ansi: Color,
}

impl CompleteColor {
    /// Returns the color for the current terminal color profile.
    pub fn rgba(&self) -> (u32, u32, u32, u32) {
        match profile() {
            Profile::TrueColor => self.true_color.rgba(),
            Profile::Ansi256 => self.ansi256.rgba(),
            Profile::Ansi => self.ansi.rgba(),
            _ => (0, 0, 0, 0xFFFF),
        }
    }
}

/// <upstream-comment>CompleteAdaptiveColor specifies exact values for truecolor, ANSI256, and ANSI color
/// profiles, with separate options for light and dark backgrounds. Automatic
/// color degradation will not be performed.</upstream-comment>
#[derive(Debug, Clone, PartialEq)]
pub struct CompleteAdaptiveColor {
    /// The colors to use on light backgrounds.
    pub light: CompleteColor,
    /// The colors to use on dark backgrounds.
    pub dark: CompleteColor,
}

impl CompleteAdaptiveColor {
    /// Returns the color for the current terminal background and profile.
    pub fn rgba(&self) -> (u32, u32, u32, u32) {
        if has_dark_background() {
            self.dark.rgba()
        } else {
            self.light.rgba()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_color() {
        let c = AdaptiveColor {
            light: Color::parse("#000000"),
            dark: Color::parse("#FFFFFF"),
        };
        let _ = c.rgba();
    }

    #[test]
    fn test_complete_color() {
        let c = CompleteColor {
            true_color: Color::parse("#FF0000"),
            ansi256: Color::parse("9"),
            ansi: Color::parse("1"),
        };
        let _ = c.rgba();
    }
}
