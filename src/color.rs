//! Cleanroom Rust port of upstream Go source file: `color.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Terminal color abstractions for ANSI 16, 256, TrueColor (RGB), AdaptiveColor,
//! and CompleteColor, plus color transforms like alpha, complementary, darken,
//! and lighten.
//! </public-docs>

use std::cmp::min;
use std::fmt;

/// <upstream-comment>4-bit color constants.</upstream-comment>
pub const BLACK: u8 = 0;
/// <upstream-comment>Red ANSI 4-bit color constant.</upstream-comment>
pub const RED: u8 = 1;
/// <upstream-comment>Green ANSI 4-bit color constant.</upstream-comment>
pub const GREEN: u8 = 2;
/// <upstream-comment>Yellow ANSI 4-bit color constant.</upstream-comment>
pub const YELLOW: u8 = 3;
/// <upstream-comment>Blue ANSI 4-bit color constant.</upstream-comment>
pub const BLUE: u8 = 4;
/// <upstream-comment>Magenta ANSI 4-bit color constant.</upstream-comment>
pub const MAGENTA: u8 = 5;
/// <upstream-comment>Cyan ANSI 4-bit color constant.</upstream-comment>
pub const CYAN: u8 = 6;
/// <upstream-comment>White ANSI 4-bit color constant.</upstream-comment>
pub const WHITE: u8 = 7;
/// <upstream-comment>BrightBlack ANSI 4-bit color constant.</upstream-comment>
pub const BRIGHT_BLACK: u8 = 8;
/// <upstream-comment>BrightRed ANSI 4-bit color constant.</upstream-comment>
pub const BRIGHT_RED: u8 = 9;
/// <upstream-comment>BrightGreen ANSI 4-bit color constant.</upstream-comment>
pub const BRIGHT_GREEN: u8 = 10;
/// <upstream-comment>BrightYellow ANSI 4-bit color constant.</upstream-comment>
pub const BRIGHT_YELLOW: u8 = 11;
/// <upstream-comment>BrightBlue ANSI 4-bit color constant.</upstream-comment>
pub const BRIGHT_BLUE: u8 = 12;
/// <upstream-comment>BrightMagenta ANSI 4-bit color constant.</upstream-comment>
pub const BRIGHT_MAGENTA: u8 = 13;
/// <upstream-comment>BrightCyan ANSI 4-bit color constant.</upstream-comment>
pub const BRIGHT_CYAN: u8 = 14;
/// <upstream-comment>BrightWhite ANSI 4-bit color constant.</upstream-comment>
pub const BRIGHT_WHITE: u8 = 15;

/// <upstream-comment>ColorProfile is the color profile of the terminal.</upstream-comment>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Profile {
    /// No terminal support; all ANSI sequences are stripped.
    NoTty,
    /// No color support (colors dropped, text decoration kept).
    Ascii,
    /// 16-color ANSI support.
    Ansi,
    /// 256-color ANSI support.
    Ansi256,
    /// 24-bit TrueColor support.
    TrueColor,
}

/// <upstream-comment>NoColor is used to specify the absence of color styling. When this is active
/// foreground colors will be rendered with the terminal's default text color,
/// and background colors will not be drawn at all.</upstream-comment>
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoColor;

/// A color value that can be applied as a terminal foreground or background.
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// The absence of color.
    NoColor,
    /// 16-color ANSI code (0-15).
    Ansi16(u8),
    /// 256-color ANSI code (0-255).
    Ansi256(u8),
    /// 24-bit TrueColor RGB value.
    TrueColor {
        /// Red component (0-255).
        r: u8,
        /// Green component (0-255).
        g: u8,
        /// Blue component (0-255).
        b: u8,
    },
    /// An adaptive color with light and dark options.
    Adaptive {
        /// The color to use on light terminal backgrounds.
        light: Box<Color>,
        /// The color to use on dark terminal backgrounds.
        dark: Box<Color>,
    },
    /// A complete color with truecolor, 256, and 16 color options.
    Complete {
        /// The TrueColor value.
        true_color: Box<Color>,
        /// The 256-color ANSI value.
        ansi256: Box<Color>,
        /// The 16-color ANSI value.
        ansi: Box<Color>,
    },
}

impl Default for Color {
    fn default() -> Self {
        Color::NoColor
    }
}

impl Color {
    /// <upstream-comment>Color specifies a color by hex or ANSI256 value. For example:
    ///
    /// ```text
    /// ansiColor := lipgloss.Color("1") // The same as lipgloss.Red
    /// ansi256Color := lipgloss.Color("21")
    /// hexColor := lipgloss.Color("#0000ff")
    /// ```</upstream-comment>
    pub fn parse(s: &str) -> Color {
        if let Some(hex) = s.strip_prefix('#') {
            if let Ok(c) = parse_hex(s) {
                return c;
            }
            let _ = hex;
            return Color::NoColor;
        }

        if let Ok(i) = s.parse::<i64>() {
            let i = i.abs();
            if i < 16 {
                return Color::Ansi16(i as u8);
            }
            if i < 256 {
                return Color::Ansi256(i as u8);
            }
            let r = ((i >> 16) & 0xff) as u8;
            let g = ((i >> 8) & 0xff) as u8;
            let b = (i & 0xff) as u8;
            return Color::TrueColor { r, g, b };
        }

        Color::NoColor
    }

    /// Returns the RGBA components of this color, matching Go's `color.Color.RGBA()`.
    pub fn rgba(&self) -> (u32, u32, u32, u32) {
        match self {
            Color::TrueColor { r, g, b } => {
                ((*r as u32) << 8, (*g as u32) << 8, (*b as u32) << 8, 0xFFFF)
            }
            Color::Ansi16(c) => {
                let (r, g, b) = basic_palette(*c);
                (
                    (r as u32) << 8,
                    (g as u32) << 8,
                    (b as u32) << 8,
                    0xFFFF,
                )
            }
            Color::Ansi256(c) => {
                let (r, g, b) = indexed_palette(*c);
                (
                    (r as u32) << 8,
                    (g as u32) << 8,
                    (b as u32) << 8,
                    0xFFFF,
                )
            }
            _ => (0, 0, 0, 0xFFFF),
        }
    }

    /// Returns the RGB bytes of this color (as u8, derived from the 16-bit RGBA).
    pub fn rgba_bytes(&self) -> (u8, u8, u8, u8) {
        let (r, g, b, a) = self.rgba();
        (
            (r >> 8) as u8,
            (g >> 8) as u8,
            (b >> 8) as u8,
            (a >> 8) as u8,
        )
    }

    /// Renders the ANSI escape code sequence prefix for foreground color.
    pub fn render_fg(&self) -> String {
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
            Color::Adaptive { dark, .. } => dark.render_fg(),
            Color::Complete { true_color, .. } => true_color.render_fg(),
            Color::NoColor => String::new(),
        }
    }

    /// Renders the ANSI escape code sequence prefix for background color.
    pub fn render_bg(&self) -> String {
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
            Color::Adaptive { dark, .. } => dark.render_bg(),
            Color::Complete { true_color, .. } => true_color.render_bg(),
            Color::NoColor => String::new(),
        }
    }

    /// Renders the ANSI escape code sequence for an underline color.
    pub fn render_ul(&self) -> String {
        match self {
            Color::Ansi16(c) => format!("\x1b[5{}m", 50 + c),
            Color::Ansi256(c) => format!("\x1b[58;5;{}m", c),
            Color::TrueColor { r, g, b } => format!("\x1b[58;2;{};{};{}m", r, g, b),
            Color::Adaptive { dark, .. } => dark.render_ul(),
            Color::Complete { true_color, .. } => true_color.render_ul(),
            Color::NoColor => String::new(),
        }
    }

    /// Returns whether this color is the absence of color.
    pub fn is_no_color(&self) -> bool {
        matches!(self, Color::NoColor)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Ansi16(c) | Color::Ansi256(c) => write!(f, "{}", c),
            Color::TrueColor { r, g, b } => write!(f, "#{:02X}{:02X}{:02X}", r, g, b),
            Color::Adaptive { .. } => write!(f, "Adaptive"),
            Color::Complete { .. } => write!(f, "Complete"),
            Color::NoColor => write!(f, ""),
        }
    }
}

/// <upstream-comment>AdaptiveColor provides light and dark color options depending on the terminal background.</upstream-comment>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptiveColor {
    /// The color to use when the terminal has a light background.
    pub light: String,
    /// The color to use when the terminal has a dark background.
    pub dark: String,
}

impl fmt::Display for AdaptiveColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Light: {}, Dark: {}", self.light, self.dark)
    }
}

impl AdaptiveColor {
    /// Renders the ANSI escape code sequence prefix for foreground color.
    pub fn render_fg(&self) -> String {
        Color::parse(&self.dark).render_fg()
    }

    /// Renders the ANSI escape code sequence prefix for background color.
    pub fn render_bg(&self) -> String {
        Color::parse(&self.dark).render_bg()
    }
}

/// <upstream-comment>CompleteColor provides explicit true color, 256 color, and 16 color values.</upstream-comment>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteColor {
    /// The TrueColor value.
    pub true_color: String,
    /// The 256-color ANSI value.
    pub ansi256: String,
    /// The 16-color ANSI value.
    pub ansi: String,
}

impl fmt::Display for CompleteColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.true_color)
    }
}

impl CompleteColor {
    /// Renders the ANSI escape code sequence prefix for foreground color.
    pub fn render_fg(&self) -> String {
        Color::parse(&self.true_color).render_fg()
    }

    /// Renders the ANSI escape code sequence prefix for background color.
    pub fn render_bg(&self) -> String {
        Color::parse(&self.true_color).render_bg()
    }
}

/// <upstream-comment>LightDark is a simple helper type that can be used to choose the appropriate
/// color based on whether the terminal has a light or dark background.
///
/// ```text
/// lightDark := lipgloss.LightDark(hasDarkBackground)
/// red, blue := lipgloss.Color("#ff0000"), lipgloss.Color("#0000ff")
/// myHotColor := lightDark(red, blue)
/// ```</upstream-comment>
pub fn light_dark(is_dark: bool) -> impl Fn(Color, Color) -> Color {
    move |light, dark| {
        if is_dark {
            dark
        } else {
            light
        }
    }
}

/// <upstream-comment>Complete returns a function that will return the appropriate color based on
/// the given color profile.
///
/// ```text
/// p := colorprofile.Detect(os.Stderr, os.Environ())
/// complete := lipgloss.Complete(p)
/// color := complete(
///     lipgloss.Color(1),     // ANSI
///     lipgloss.Color(124),   // ANSI256
///     lipgloss.Color("#ff34ac"), // TrueColor
/// )
/// ```</upstream-comment>
pub fn complete(profile: Profile) -> impl Fn(Color, Color, Color) -> Color {
    move |ansi, ansi256, truecolor| match profile {
        Profile::Ansi => ansi,
        Profile::Ansi256 => ansi256,
        Profile::TrueColor => truecolor,
        _ => Color::NoColor,
    }
}

/// ensure_not_transparent ensures that the alpha value of a color is not 0, and if
/// it is, we will set it to opaque. This is useful for when we are converting from
/// RGB -> RGBA, and the alpha value is lost in the conversion for gradient purposes.
fn ensure_not_transparent(c: Color) -> Color {
    let (_, _, _, a) = c.rgba();
    if a == 0 {
        return alpha(c, 1.0);
    }
    c
}

/// <upstream-comment>Alpha adjusts the alpha value of a color using a 0-1 (clamped) float scale
/// 0 = transparent, 1 = opaque.</upstream-comment>
pub fn alpha(c: Color, alpha: f64) -> Color {
    match c {
        Color::TrueColor { r, g, b } => {
            let a = (clamp(alpha, 0.0, 1.0) * 255.0) as u8;
            Color::TrueColor {
                r,
                g,
                b,
            }
            .with_alpha(a)
        }
        _ => c,
    }
}

/// <upstream-comment>Complementary returns the complementary color (180° away on color wheel) of
/// the given color. This is useful for creating a contrasting color.</upstream-comment>
pub fn complementary(c: Color) -> Color {
    if c.is_no_color() {
        return Color::NoColor;
    }
    let (r, g, b, _) = ensure_not_transparent(c).rgba_bytes();
    let mut hsv = rgb_to_hsv(r, g, b);
    hsv.0 += 180.0;
    if hsv.0 >= 360.0 {
        hsv.0 -= 360.0;
    } else if hsv.0 < 0.0 {
        hsv.0 += 360.0;
    }
    let (r, g, b) = hsv_to_rgb(hsv.0, hsv.1, hsv.2);
    Color::TrueColor { r, g, b }
}

/// <upstream-comment>Darken takes a color and makes it darker by a specific percentage (0-1, clamped).</upstream-comment>
pub fn darken(c: Color, percent: f64) -> Color {
    if c.is_no_color() {
        return Color::NoColor;
    }
    let mult = 1.0 - clamp(percent, 0.0, 1.0);
    let (r, g, b, a) = c.rgba_bytes();
    Color::TrueColor {
        r: (r as f64 * mult) as u8,
        g: (g as f64 * mult) as u8,
        b: (b as f64 * mult) as u8,
    }
    .with_alpha(a)
}

/// <upstream-comment>Lighten makes a color lighter by a specific percentage (0-1, clamped).</upstream-comment>
pub fn lighten(c: Color, percent: f64) -> Color {
    if c.is_no_color() {
        return Color::NoColor;
    }
    let add = 255.0 * clamp(percent, 0.0, 1.0);
    let (r, g, b, a) = c.rgba_bytes();
    Color::TrueColor {
        r: min(255, (r as f64 + add) as u8),
        g: min(255, (g as f64 + add) as u8),
        b: min(255, (b as f64 + add) as u8),
    }
    .with_alpha(a)
}

impl Color {
    fn with_alpha(self, _a: u8) -> Color {
        // Alpha is not representable in the terminal color model; keep RGB.
        self
    }
}

/// is_dark_color returns whether the given color is dark (based on the luminance
/// portion of the color as interpreted as HSL).
pub fn is_dark_color(c: &Color) -> bool {
    let (r, g, b, _) = c.rgba_bytes();
    let (_, _, l) = rgb_to_hsl(r, g, b);
    l < 0.5
}

fn clamp(v: f64, low: f64, high: f64) -> f64 {
    let (low, high) = if high < low { (high, low) } else { (low, high) };
    v.clamp(low, high)
}

/// The standard xterm 16-color palette.
fn basic_palette(c: u8) -> (u8, u8, u8) {
    const PALETTE: [(u8, u8, u8); 16] = [
        (0x00, 0x00, 0x00),
        (0x80, 0x00, 0x00),
        (0x00, 0x80, 0x00),
        (0x80, 0x80, 0x00),
        (0x00, 0x00, 0x80),
        (0x80, 0x00, 0x80),
        (0x00, 0x80, 0x80),
        (0xC0, 0xC0, 0xC0),
        (0x80, 0x80, 0x80),
        (0xFF, 0x00, 0x00),
        (0x00, 0xFF, 0x00),
        (0xFF, 0xFF, 0x00),
        (0x00, 0x00, 0xFF),
        (0xFF, 0x00, 0xFF),
        (0x00, 0xFF, 0xFF),
        (0xFF, 0xFF, 0xFF),
    ];
    PALETTE[(c as usize).min(15)]
}

/// The xterm-256 palette: 16 basic colors, a 6x6x6 color cube, and grayscale.
fn indexed_palette(c: u8) -> (u8, u8, u8) {
    let c = c as usize;
    match c {
        0..=15 => basic_palette(c as u8),
        16..=231 => {
            let i = c - 16;
            let r = i / 36;
            let g = (i / 6) % 6;
            let b = i % 6;
            let level = |v: usize| -> u8 {
                if v == 0 {
                    0
                } else {
                    (55 + v * 40) as u8
                }
            };
            (level(r), level(g), level(b))
        }
        _ => {
            let gray = 8 + (c - 232) * 10;
            (gray as u8, gray as u8, gray as u8)
        }
    }
}

/// Returns the RGB value of an xterm-256 palette index.
pub fn indexed_rgb(c: u8) -> (u8, u8, u8) {
    indexed_palette(c)
}

// ---------------------------------------------------------------------------
// RGB/HSL/HSV helpers
// ---------------------------------------------------------------------------

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let (r, g, b) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
    let max_c = r.max(g).max(b);
    let min_c = r.min(g).min(b);
    let delta = max_c - min_c;
    let mut h = 0.0;
    if delta != 0.0 {
        h = if max_c == r {
            ((g - b) / delta).rem_euclid(6.0)
        } else if max_c == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };
        h *= 60.0;
    }
    let s = if max_c == 0.0 { 0.0 } else { delta / max_c };
    (h, s, max_c)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let h = h.rem_euclid(360.0) / 60.0;
    let c = v * s;
    let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let (r, g, b) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
    let max_c = r.max(g).max(b);
    let min_c = r.min(g).min(b);
    let delta = max_c - min_c;
    let l = (max_c + min_c) / 2.0;
    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };
    let h = if delta == 0.0 {
        0.0
    } else if max_c == r {
        ((g - b) / delta).rem_euclid(6.0) * 60.0
    } else if max_c == g {
        ((b - r) / delta + 2.0) * 60.0
    } else {
        ((r - g) / delta + 4.0) * 60.0
    };
    (h, s, l)
}

// ---------------------------------------------------------------------------
// CIELAB color space helpers (used for blending)
// ---------------------------------------------------------------------------

/// A color in CIE L*a*b* space.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

const EPSILON: f64 = 216.0 / 24389.0;
const KAPPA: f64 = 24389.0 / 27.0;

pub(crate) fn srgb_to_xyz(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let lr = linearize(r);
    let lg = linearize(g);
    let lb = linearize(b);
    (
        lr * 0.4124 + lg * 0.3576 + lb * 0.1805,
        lr * 0.2126 + lg * 0.7152 + lb * 0.0722,
        lr * 0.0193 + lg * 0.1192 + lb * 0.9505,
    )
}

pub(crate) fn xyz_to_srgb(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    (
        linearize_rgb(x * 3.2406 + y * -1.5372 + z * -0.4986),
        linearize_rgb(x * -0.9689 + y * 1.8758 + z * 0.0415),
        linearize_rgb(x * 0.0557 + y * -0.2040 + z * 1.0570),
    )
}

fn linearize(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linearize_rgb(c: f64) -> f64 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

pub(crate) fn lab_from_rgb(r: u8, g: u8, b: u8) -> Lab {
    let (x, y, z) = srgb_to_xyz(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
    // D65 reference white.
    let xr = x / 0.95047;
    let yr = y / 1.00000;
    let zr = z / 1.08883;
    let fx = lab_f(xr);
    let fy = lab_f(yr);
    let fz = lab_f(zr);
    Lab {
        l: 116.0 * fy - 16.0,
        a: 500.0 * (fx - fy),
        b: 200.0 * (fy - fz),
    }
}

fn lab_f(t: f64) -> f64 {
    if t > EPSILON {
        t.cbrt()
    } else {
        (t * KAPPA + 16.0) / 116.0
    }
}

fn lab_f_inv(t: f64) -> f64 {
    let t3 = t * t * t;
    if t3 > EPSILON {
        t3
    } else {
        (116.0 * t - 16.0) / KAPPA
    }
}

pub(crate) fn rgb_from_lab(lab: Lab) -> (u8, u8, u8) {
    let fy = (lab.l + 16.0) / 116.0;
    let fx = fy + lab.a / 500.0;
    let fz = fy - lab.b / 200.0;
    let xr = lab_f_inv(fx);
    let yr = lab_f_inv(fy);
    let zr = lab_f_inv(fz);
    let (r, g, b) = xyz_to_srgb(xr * 0.95047, yr * 1.00000, zr * 1.08883);
    (
        (clamp(r, 0.0, 1.0) * 255.0).round() as u8,
        (clamp(g, 0.0, 1.0) * 255.0).round() as u8,
        (clamp(b, 0.0, 1.0) * 255.0).round() as u8,
    )
}

// ---------------------------------------------------------------------------
// Hex parsing
// ---------------------------------------------------------------------------

/// parse_hex parses a hex color string and returns a Color. The string can be
/// in the format `#RRGGBB` or `#RGB`.
pub fn parse_hex(s: &str) -> Result<Color, &'static str> {
    if s.len() < 1 || !s.starts_with('#') {
        return Err("invalid hex format");
    }
    let hex = &s[1..];
    let hex_to_byte = |b: u8| -> Result<u8, &'static str> {
        match b {
            b'0'..=b'9' => Ok(b - b'0'),
            b'a'..=b'f' => Ok(b - b'a' + 10),
            b'A'..=b'F' => Ok(b - b'A' + 10),
            _ => Err("invalid hex format"),
        }
    };
    match hex.len() {
        6 => {
            let r = hex_to_byte(hex.as_bytes()[0])? << 4 | hex_to_byte(hex.as_bytes()[1])?;
            let g = hex_to_byte(hex.as_bytes()[2])? << 4 | hex_to_byte(hex.as_bytes()[3])?;
            let b = hex_to_byte(hex.as_bytes()[4])? << 4 | hex_to_byte(hex.as_bytes()[5])?;
            Ok(Color::TrueColor { r, g, b })
        }
        3 => {
            let r = hex_to_byte(hex.as_bytes()[0])? * 17;
            let g = hex_to_byte(hex.as_bytes()[1])? * 17;
            let b = hex_to_byte(hex.as_bytes()[2])? * 17;
            Ok(Color::TrueColor { r, g, b })
        }
        _ => Err("invalid hex format"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        assert_eq!(
            Color::parse("#FF0000"),
            Color::TrueColor {
                r: 255,
                g: 0,
                b: 0
            }
        );
        assert_eq!(
            Color::parse("#F00"),
            Color::TrueColor {
                r: 255,
                g: 0,
                b: 0
            }
        );
        assert_eq!(Color::parse("FF0000"), Color::NoColor);
    }

    #[test]
    fn test_rgba_values() {
        // 9 = bright red (#FF0000).
        let (r, g, b, _) = Color::parse("9").rgba_bytes();
        assert_eq!((r, g, b), (0xFF, 0x00, 0x00));
        // 21 = blue.
        let (r, g, b, _) = Color::parse("21").rgba_bytes();
        assert_eq!((r, g, b), (0x00, 0x00, 0xFF));
        // 16711680 = #FF0000.
        let (r, g, b, _) = Color::parse("16711680").rgba_bytes();
        assert_eq!((r, g, b), (0xFF, 0x00, 0x00));
    }

    #[test]
    fn test_complementary() {
        let c = complementary(Color::parse("#FF0000"));
        let (r, g, b, _) = c.rgba_bytes();
        assert_eq!((r, g, b), (0x00, 0xFF, 0xFF));
    }

    #[test]
    fn test_darken_lighten() {
        let c = darken(Color::parse("#FFFFFF"), 0.5);
        let (r, g, b, _) = c.rgba_bytes();
        assert_eq!((r, g, b), (0x7F, 0x7F, 0x7F));
        let c = lighten(Color::parse("#000000"), 0.5);
        let (r, g, b, _) = c.rgba_bytes();
        assert_eq!((r, g, b), (0x7F, 0x7F, 0x7F));
    }
}
