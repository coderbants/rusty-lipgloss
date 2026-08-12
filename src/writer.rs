//! Cleanroom Rust port of upstream Go source file: `writer.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Print functions that write to stdout/stderr, automatically downsampling
//! colors when necessary based on the detected terminal color profile.
//!
//! Profile detection and SGR downsampling mirror `charmbracelet/colorprofile`
//! and `charmbracelet/x/ansi` byte-for-byte: non-TTY output is stripped of all
//! ANSI sequences (NoTTY), NO_COLOR disables colors but keeps text decoration,
//! and ANSI/ANSI256 profiles re-encode SGR colors using the xterm-256 palette
//! with the tmux cube + HSLuv-distance algorithm.
//! </public-docs>

// The TTY check uses a libc FFI call; the unsafe code is isolated here.
#![allow(unsafe_code)]

use std::collections::HashMap;
use std::io::Write;

use crate::color::{Color, Profile};

/// Detects the terminal color profile, mirroring `colorprofile.Detect`.
///
/// Rules (matching upstream):
/// - Non-TTY output always yields `Profile::NoTty` (all ANSI stripped).
/// - `TERM=dumb` or missing `TERM` yields `Profile::NoTty`.
/// - `NO_COLOR` disables colors but keeps text decoration.
/// - `COLORTERM=truecolor|24bit|yes|true` yields TrueColor.
/// - `TERM=*-256color` yields ANSI256; other color terminals yield ANSI.
pub fn detect_profile() -> Profile {
    let env: HashMap<String, String> = std::env::vars().collect();
    let isatty = stdout_is_tty();
    let is_dumb = env.get("TERM").is_none_or(|t| t.is_empty() || t == "dumb");
    let mut p = env_color_profile(&env);
    if !isatty || is_dumb {
        p = Profile::NoTty;
    }

    if env_no_color(&env) && isatty {
        if p > Profile::Ascii {
            p = Profile::Ascii;
        }
        return p;
    }

    if cli_color_forced(&env) {
        if p < Profile::Ansi {
            p = Profile::Ansi;
        }
        let envp = env_color_profile(&env);
        if envp > p {
            p = envp;
        }
        return p;
    }

    if cli_color(&env) && isatty && !is_dumb && p < Profile::Ansi {
        p = Profile::Ansi;
    }

    p
}

fn env_no_color(env: &HashMap<String, String>) -> bool {
    env.get("NO_COLOR")
        .is_some_and(|v| v == "1" || v == "true" || v == "TRUE")
}

fn cli_color(env: &HashMap<String, String>) -> bool {
    env.get("CLICOLOR")
        .is_some_and(|v| v == "1" || v == "true" || v == "TRUE")
}

fn cli_color_forced(env: &HashMap<String, String>) -> bool {
    env.get("CLICOLOR_FORCE")
        .is_some_and(|v| v == "1" || v == "true" || v == "TRUE")
}

fn env_color_profile(env: &HashMap<String, String>) -> Profile {
    let term = env.get("TERM").cloned().unwrap_or_default();
    let mut p = if term.is_empty() || term == "dumb" {
        Profile::NoTty
    } else {
        Profile::Ansi
    };

    for known in [
        "alacritty",
        "contour",
        "foot",
        "ghostty",
        "kitty",
        "rio",
        "st",
        "wezterm",
    ] {
        if term.contains(known) {
            return Profile::TrueColor;
        }
    }
    if (term.starts_with("tmux") || term.starts_with("screen")) && p < Profile::Ansi256 {
        p = Profile::Ansi256;
    }
    if term.starts_with("xterm") && p < Profile::Ansi {
        p = Profile::Ansi;
    }

    if env.get("WT_SESSION").map_or(0, |v| v.len()) > 0 {
        return Profile::TrueColor;
    }

    if let Some(cloud) = env.get("GOOGLE_CLOUD_SHELL") {
        if cloud == "1" || cloud == "true" || cloud == "TRUE" {
            return Profile::TrueColor;
        }
    }

    // GNU Screen doesn't support TrueColor; Tmux doesn't support $COLORTERM.
    if color_term(env) && !term.starts_with("screen") && !term.starts_with("tmux") {
        return Profile::TrueColor;
    }

    if term.ends_with("256color") && p < Profile::Ansi256 {
        p = Profile::Ansi256;
    }

    if term.ends_with("direct") {
        return Profile::TrueColor;
    }

    p
}

fn color_term(env: &HashMap<String, String>) -> bool {
    let ct = env
        .get("COLORTERM")
        .map(|v| v.to_ascii_lowercase())
        .unwrap_or_default();
    ct == "truecolor" || ct == "24bit" || ct == "yes" || ct == "true"
}

/// Returns whether the given output file descriptor is a terminal.
pub fn stdout_is_tty() -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        unsafe { libc::isatty(std::io::stdout().as_raw_fd()) == 1 }
    }
    #[cfg(not(unix))]
    {
        false
    }
}

/// The writer that prints to stdout, automatically downsampling colors when
/// necessary. Mirrors the upstream package-level `Writer`.
pub struct Writer<'a> {
    forward: Box<dyn Write + 'a>,
    profile: Profile,
}

impl<'a> Writer<'a> {
    /// Returns a new writer wrapping the given forward writer.
    pub fn new(w: Box<dyn Write + 'a>) -> Writer<'a> {
        Writer {
            forward: w,
            profile: detect_profile(),
        }
    }

    /// Returns the detected color profile.
    pub fn profile(&self) -> Profile {
        self.profile
    }

    /// Writes the given bytes, downsampling colors to the detected profile.
    pub fn write(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        if self.profile == Profile::TrueColor {
            self.forward.write_all(bytes)?;
            return Ok(());
        }
        let s = downsample_sgr(&String::from_utf8_lossy(bytes), self.profile);
        self.forward.write_all(s.as_bytes())?;
        Ok(())
    }
}

/// Rewrites ANSI SGR sequences in the string, downsampling colors to the given
/// profile. Non-SGR sequences are passed through verbatim. With `NoTty` all
/// ANSI sequences are stripped entirely.
pub fn downsample_sgr(s: &str, profile: Profile) -> String {
    match profile {
        Profile::TrueColor => s.to_string(),
        Profile::NoTty => crate::ansi::strip(s),
        _ => {
            let mut out = String::with_capacity(s.len());
            let mut rest = s;
            while let Some(pos) = rest.find('\x1b') {
                out.push_str(&rest[..pos]);
                rest = &rest[pos..];
                if !rest.starts_with("\x1b[") {
                    out.push_str(rest);
                    break;
                }
                // Find the terminating byte.
                let bytes = rest.as_bytes();
                let mut end = 2usize;
                while end < bytes.len() && !(0x40..=0x7e).contains(&bytes[end]) {
                    end += 1;
                }
                if end >= bytes.len() {
                    out.push_str(rest);
                    break;
                }
                let seq = &rest[..=end];
                if seq.ends_with('m') {
                    out.push_str(&downsample_sgr_seq(seq, profile));
                } else {
                    out.push_str(seq);
                }
                rest = &rest[end + 1..];
            }
            out.push_str(rest);
            out
        }
    }
}

/// Re-encodes a single SGR sequence, mirroring `colorprofile.handleSgr` and
/// `ansi.Style.String`.
fn downsample_sgr_seq(seq: &str, profile: Profile) -> String {
    let inner = &seq[2..seq.len() - 1];
    let parts: Vec<&str> = inner.split(';').collect();
    let mut style: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < parts.len() {
        let param = parts[i];
        let num = param.parse::<i32>().unwrap_or(-1);
        match num {
            0 => {
                // SGR default parameter is 0. We use an empty string to reduce
                // the number of bytes written to the buffer.
                style.push(String::new());
            }
            30..=37 => {
                if profile >= Profile::Ansi {
                    style.push(basic_color_string(convert_basic(num - 30), 3));
                }
            }
            38 => {
                let (n, c) = read_style_color(&parts[i..]);
                i += n.saturating_sub(1);
                if profile >= Profile::Ansi {
                    if let Some(c) = c {
                        style.push(convert_color_string(&c, profile, 3));
                    } else {
                        style.push("39".to_string());
                    }
                }
            }
            39 => {
                if profile >= Profile::Ansi {
                    style.push("39".to_string());
                }
            }
            40..=47 => {
                if profile >= Profile::Ansi {
                    style.push(basic_color_string(convert_basic(num - 40), 4));
                }
            }
            48 => {
                let (n, c) = read_style_color(&parts[i..]);
                i += n.saturating_sub(1);
                if profile >= Profile::Ansi {
                    if let Some(c) = c {
                        style.push(convert_color_string(&c, profile, 4));
                    } else {
                        style.push("49".to_string());
                    }
                }
            }
            49 => {
                if profile >= Profile::Ansi {
                    style.push("49".to_string());
                }
            }
            58 => {
                let (n, c) = read_style_color(&parts[i..]);
                i += n.saturating_sub(1);
                if profile >= Profile::Ansi {
                    if let Some(c) = c {
                        style.push(convert_color_string(&c, profile, 5));
                    } else {
                        style.push("59".to_string());
                    }
                }
            }
            59 => {
                if profile >= Profile::Ansi {
                    style.push("59".to_string());
                }
            }
            90..=97 => {
                if profile >= Profile::Ansi {
                    style.push(basic_color_string(convert_basic(num - 90 + 8), 3));
                }
            }
            100..=107 => {
                if profile >= Profile::Ansi {
                    style.push(basic_color_string(convert_basic(num - 100 + 8), 4));
                }
            }
            _ => {
                // If this is not a color attribute, just append it to the style.
                style.push(param.to_string());
            }
        }
        i += 1;
    }
    if style.is_empty() {
        return crate::ansi::RESET_STYLE.to_string();
    }
    format!("\x1b[{}m", style.join(";"))
}

/// Reads a color from `38/48/58` SGR params, mirroring `ansi.ReadStyleColor`
/// for the common semicolon-separated forms. Returns the number of params
/// consumed and the color (None for ambiguous forms).
fn read_style_color(parts: &[&str]) -> (usize, Option<Color>) {
    if parts.len() < 2 {
        return (0, None);
    }
    let color_type = parts[1];
    match color_type {
        "2" => {
            // Legacy color values separated by semicolons: 38 ; 2 ; r ; g ; b
            if parts.len() < 5 {
                return (0, None);
            }
            let r = parts[2].parse::<u8>().ok();
            let g = parts[3].parse::<u8>().ok();
            let b = parts[4].parse::<u8>().ok();
            match (r, g, b) {
                (Some(r), Some(g), Some(b)) => (5, Some(Color::TrueColor { r, g, b })),
                _ => (0, None),
            }
        }
        "5" => {
            // Extended 256-color: 38 ; 5 ; n
            if parts.len() < 3 {
                return (0, None);
            }
            match parts[2].parse::<u8>().ok() {
                Some(n) => (3, Some(Color::Ansi256(n))),
                None => (0, None),
            }
        }
        _ => (0, None),
    }
}

/// Converts a basic (0-15) color index through the profile, mirroring
/// `colorprofile.Profile.Convert` (basic colors are passed through unchanged
/// for every profile that supports color).
fn convert_basic(c: i32) -> Color {
    Color::Ansi16(c.clamp(0, 15) as u8)
}

/// Renders the SGR color parameter string for the given color at the given
/// profile, mirroring `colorprofile.Profile.Convert` + `colorString`.
fn convert_color_string(c: &Color, profile: Profile, base: u8) -> String {
    match profile {
        Profile::Ansi256 => match c {
            Color::TrueColor { r, g, b } => {
                color_seq(&Color::Ansi256(convert_256(*r, *g, *b)), base)
            }
            other => color_seq(other, base),
        },
        Profile::Ansi => match c {
            Color::Ansi256(n) => color_seq(&Color::Ansi16(ansi256_to_16(*n)), base),
            Color::TrueColor { r, g, b } => {
                let n = convert_256(*r, *g, *b);
                color_seq(&Color::Ansi16(ansi256_to_16(n)), base)
            }
            other => color_seq(other, base),
        },
        _ => color_seq(c, base),
    }
}

/// The SGR color sequence for a color and base (3 = fg, 4 = bg, 5 = underline).
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
        _ => format!("{}", base * 10),
    }
}

/// Basic color SGR string (30-37/90-97 or 40-47/100-107).
fn basic_color_string(c: Color, base: u8) -> String {
    color_seq(&c, base)
}

/// Downsamples a color to the given profile.
pub fn downsample(c: &Color, profile: Profile) -> Color {
    match profile {
        Profile::TrueColor => c.clone(),
        Profile::Ansi256 => match c {
            Color::TrueColor { r, g, b } => Color::Ansi256(convert_256(*r, *g, *b)),
            other => other.clone(),
        },
        Profile::Ansi => match c {
            Color::Ansi256(n) => Color::Ansi16(ansi256_to_16(*n)),
            Color::TrueColor { r, g, b } => {
                let n = convert_256(*r, *g, *b);
                Color::Ansi16(ansi256_to_16(n))
            }
            other => other.clone(),
        },
        Profile::Ascii | Profile::NoTty => Color::NoColor,
    }
}

// ---------------------------------------------------------------------------
// xterm-256 palette conversion (mirrors `ansi.Convert256` / `ansi.Convert16`)
// ---------------------------------------------------------------------------

const Q2C: [i32; 6] = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

fn to6_cube(v: i32) -> usize {
    if v < 48 {
        0
    } else if v < 115 {
        1
    } else {
        ((v - 35) / 40).clamp(0, 5) as usize
    }
}

/// Converts an RGB color to the xterm-256 palette index, mirroring
/// `ansi.Convert256` (tmux cube + HSLuv distance).
pub fn convert_256(r: u8, g: u8, b: u8) -> u8 {
    // colorprofile receives colors via color.Color.RGBA(), so the RGB values
    // pass through MakeColor's 16-bit division before scaling back to 255.
    let r16 = (r as f64 * 65280.0) / 65535.0;
    let g16 = (g as f64 * 65280.0) / 65535.0;
    let b16 = (b as f64 * 65280.0) / 65535.0;

    let qr = to6_cube(r16 as i32);
    let cr = Q2C[qr];
    let qg = to6_cube(g16 as i32);
    let cg = Q2C[qg];
    let qb = to6_cube(b16 as i32);
    let cb = Q2C[qb];

    let ci = 36 * qr + 6 * qg + qb;
    if cr == r16 as i32 && cg == g16 as i32 && cb == b16 as i32 {
        return (16 + ci) as u8;
    }

    // Work out the closest grey (average of RGB).
    let grey_avg = ((r16 + g16 + b16) / 3.0) as i32;
    let grey_idx = if grey_avg > 238 {
        23
    } else {
        ((grey_avg - 3) / 10).clamp(0, 23)
    };
    let grey = 8 + 10 * grey_idx;

    // Prefer the closer color in terms of HSLuv distance.
    let color_dist = hsluv_distance(
        r16 / 255.0,
        g16 / 255.0,
        b16 / 255.0,
        cr as f64 / 255.0,
        cg as f64 / 255.0,
        cb as f64 / 255.0,
    );
    let gray_dist = hsluv_distance(
        r16 / 255.0,
        g16 / 255.0,
        b16 / 255.0,
        grey as f64 / 255.0,
        grey as f64 / 255.0,
        grey as f64 / 255.0,
    );

    if color_dist <= gray_dist {
        (16 + ci) as u8
    } else {
        (232 + grey_idx) as u8
    }
}

/// The `ansi256To16` conversion table from `charmbracelet/x/ansi`.
const ANSI256_TO_16: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 4, 4, 4, 12, 12, 2, 6, 4, 4, 12, 12,
    2, 2, 6, 4, 12, 12, 2, 2, 2, 6, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14, 1, 5,
    4, 4, 12, 12, 3, 8, 4, 4, 12, 12, 2, 2, 6, 4, 12, 12, 2, 2, 2, 6, 12, 12, 10, 10, 10, 10, 14,
    12, 10, 10, 10, 10, 10, 14, 1, 1, 5, 4, 12, 12, 1, 1, 5, 4, 12, 12, 3, 3, 8, 4, 12, 12, 2, 2,
    2, 6, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14, 1, 1, 1, 5, 12, 12, 1, 1, 1, 5,
    12, 12, 1, 1, 1, 5, 12, 12, 3, 3, 3, 7, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14,
    9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 11, 11, 11, 11,
    7, 12, 10, 10, 10, 10, 10, 14, 9, 9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 9, 9,
    9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 11, 11, 11, 11, 11, 15, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 7,
    7, 7, 7, 7, 7, 15, 15, 15, 15, 15, 15,
];

fn ansi256_to_16(n: u8) -> u8 {
    ANSI256_TO_16[n as usize]
}

// ---------------------------------------------------------------------------
// HSLuv distance (mirrors `go-colorful` DistanceHSLuv)
// ---------------------------------------------------------------------------

const HSLUV_D65: [f64; 3] = [0.95045592705167, 1.0, 1.089057750759878];
const KAPPA: f64 = 903.2962962962963;
const EPSILON: f64 = 0.008_856_451_679_035_631;
const M: [[f64; 3]; 3] = [
    [
        3.2409699419045214,
        -1.5373831775700935,
        -0.498_610_760_293_003_3,
    ],
    [
        -0.969_243_636_280_879_8,
        1.8759675015077207,
        0.041_555_057_407_175_61,
    ],
    [
        0.055_630_079_696_993_61,
        -0.20397695888897657,
        1.0569715142428786,
    ],
];

fn hsluv_distance(r1: f64, g1: f64, b1: f64, r2: f64, g2: f64, b2: f64) -> f64 {
    let (h1, s1, l1) = hsluv(r1, g1, b1);
    let (h2, s2, l2) = hsluv(r2, g2, b2);
    let dh = (h1 - h2) / 100.0;
    let ds = s1 - s2;
    let dl = l1 - l2;
    (dh * dh + ds * ds + dl * dl).sqrt()
}

fn hsluv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let (l, c, h) = luv_lch(r, g, b);
    luv_lch_to_hsluv(l, c, h)
}

fn luv_lch(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let (x, y, z) = linear_rgb_to_xyz(linearize(r), linearize(g), linearize(b));
    let (l, u, v) = xyz_to_luv_white_ref(x, y, z, HSLUV_D65);
    luv_to_luv_lch(l, u, v)
}

fn linearize(v: f64) -> f64 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_rgb_to_xyz(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    (
        0.412_390_799_265_959_5 * r + 0.35758433938387796 * g + 0.180_480_788_401_834_3 * b,
        0.21263900587151036 * r + 0.715_168_678_767_755_9 * g + 0.072_192_315_360_733_71 * b,
        0.019_330_818_715_591_85 * r + 0.11919477979462599 * g + 0.950_532_152_249_660_6 * b,
    )
}

fn xyz_to_uv(x: f64, y: f64, z: f64) -> (f64, f64) {
    let denom = x + 15.0 * y + 3.0 * z;
    if denom == 0.0 {
        (0.0, 0.0)
    } else {
        (4.0 * x / denom, 9.0 * y / denom)
    }
}

fn xyz_to_luv_white_ref(x: f64, y: f64, z: f64, wref: [f64; 3]) -> (f64, f64, f64) {
    let l = if y / wref[1] <= 6.0 / 29.0 * 6.0 / 29.0 * 6.0 / 29.0 {
        y / wref[1] * (29.0 / 3.0 * 29.0 / 3.0 * 29.0 / 3.0) / 100.0
    } else {
        1.16 * (y / wref[1]).cbrt() - 0.16
    };
    let (ubis, vbis) = xyz_to_uv(x, y, z);
    let (un, vn) = xyz_to_uv(wref[0], wref[1], wref[2]);
    (l, 13.0 * l * (ubis - un), 13.0 * l * (vbis - vn))
}

fn luv_to_luv_lch(l: f64, u: f64, v: f64) -> (f64, f64, f64) {
    // Oops, floating point workaround necessary if u ~= v and both are very
    // small (i.e. almost zero).
    let h = if (v - u).abs() > 1e-4 && u.abs() > 1e-4 {
        (57.295_779_513_082_32 * v.atan2(u) + 360.0).rem_euclid(360.0)
    } else {
        0.0
    };
    (l, (u * u + v * v).sqrt(), h)
}

fn luv_lch_to_hsluv(l: f64, c: f64, h: f64) -> (f64, f64, f64) {
    // [-1..1] but the code expects it to be [-100..100].
    let c = c * 100.0;
    let l = l * 100.0;

    let s = if !(0.00000001..=99.9999999).contains(&l) {
        0.0
    } else {
        let max = max_chroma_for_lh(l, h);
        c / max * 100.0
    };
    (h, (s / 100.0).clamp(0.0, 1.0), (l / 100.0).clamp(0.0, 1.0))
}

fn max_chroma_for_lh(l: f64, h: f64) -> f64 {
    let h_rad = h / 360.0 * std::f64::consts::PI * 2.0;
    let mut min_length = f64::MAX;
    for line in get_bounds(l) {
        let length = length_of_ray_until_intersect(h_rad, line[0], line[1]);
        if length > 0.0 && length < min_length {
            min_length = length;
        }
    }
    min_length
}

fn get_bounds(l: f64) -> [[f64; 2]; 6] {
    let mut ret = [[0.0; 2]; 6];
    let sub1 = (l + 16.0).powi(3) / 1560896.0;
    let sub2 = if sub1 > EPSILON { sub1 } else { l / KAPPA };
    for (i, mrow) in M.iter().enumerate() {
        for k in 0..2 {
            let top1 = (284517.0 * mrow[0] - 94839.0 * mrow[2]) * sub2;
            let top2 = (838422.0 * mrow[2] + 769860.0 * mrow[1] + 731718.0 * mrow[0]) * l * sub2
                - 769860.0 * k as f64 * l;
            let bottom = (632260.0 * mrow[2] - 126452.0 * mrow[1]) * sub2 + 126452.0 * k as f64;
            ret[i * 2 + k][0] = top1 / bottom;
            ret[i * 2 + k][1] = top2 / bottom;
        }
    }
    ret
}

fn length_of_ray_until_intersect(theta: f64, x: f64, y: f64) -> f64 {
    y / (theta.sin() - x * theta.cos())
}

/// <upstream-comment>Println to stdout, automatically downsampling colors when necessary, ending
/// with a trailing newline.</upstream-comment>
pub fn println(v: &str) -> std::io::Result<()> {
    let mut w = Writer::new(Box::new(std::io::stdout()));
    w.write(v.as_bytes())?;
    w.write(b"\n")
}

/// <upstream-comment>Print to stdout, automatically downsampling colors when necessary.</upstream-comment>
pub fn print(v: &str) -> std::io::Result<()> {
    let mut w = Writer::new(Box::new(std::io::stdout()));
    w.write(v.as_bytes())
}

/// <upstream-comment>Fprint prints to the given writer, automatically downsampling colors when
/// necessary.</upstream-comment>
pub fn fprint(w: &mut dyn Write, v: &str) -> std::io::Result<()> {
    let mut dw = Writer::new(Box::new(w));
    dw.write(v.as_bytes())
}

/// <upstream-comment>Fprintln prints to the given writer, automatically downsampling colors when
/// necessary, and ending with a trailing newline.</upstream-comment>
pub fn fprintln(w: &mut dyn Write, v: &str) -> std::io::Result<()> {
    fprint(w, v)?;
    w.write_all(b"\n")
}

/// <upstream-comment>Sprint returns a string for stdout, automatically downsampling colors when
/// necessary.</upstream-comment>
pub fn sprint(v: &str) -> String {
    downsample_sgr(v, detect_profile())
}

/// <upstream-comment>Sprintln returns a string for stdout, automatically downsampling colors when
/// necessary, and ending with a trailing newline.</upstream-comment>
pub fn sprintln(v: &str) -> String {
    format!("{}\n", sprint(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_256() {
        // Exact cube color matches.
        assert_eq!(convert_256(0, 0, 255), 21);
        assert_eq!(convert_256(255, 0, 0), 196);
        assert_eq!(convert_256(0, 255, 0), 46);
        // Greys map to the grey ramp.
        assert_eq!(convert_256(128, 128, 128), 244);
    }

    #[test]
    fn test_ansi256_to_16() {
        assert_eq!(ansi256_to_16(240), 8);
        assert_eq!(ansi256_to_16(196), 9);
        assert_eq!(ansi256_to_16(21), 12);
    }

    #[test]
    fn test_downsample_truecolor_to_256() {
        let c = Color::parse("#FF0000");
        let down = downsample(&c, Profile::Ansi256);
        assert_eq!(down, Color::Ansi256(196));
    }

    #[test]
    fn test_downsample_truecolor_to_ansi() {
        let c = Color::parse("#FF0000");
        let down = downsample(&c, Profile::Ansi);
        assert_eq!(down, Color::Ansi16(9));
    }

    #[test]
    fn test_downsample_sgr_256() {
        let s = "\x1b[38;2;255;0;0mred\x1b[m";
        let out = downsample_sgr(s, Profile::Ansi256);
        assert_eq!(out, "\x1b[38;5;196mred\x1b[m");
    }

    #[test]
    fn test_downsample_sgr_ansi() {
        let s = "\x1b[38;5;240mgrey\x1b[m";
        let out = downsample_sgr(s, Profile::Ansi);
        assert_eq!(out, "\x1b[90mgrey\x1b[m");
    }

    #[test]
    fn test_downsample_sgr_notty_strips_all() {
        let s = "\x1b[1mbold\x1b[m and \x1b[38;2;255;0;0mred\x1b[m";
        let out = downsample_sgr(s, Profile::NoTty);
        assert_eq!(out, "bold and red");
    }

    #[test]
    fn test_downsample_sgr_ascii_keeps_decoration() {
        let s = "\x1b[1mbold\x1b[m \x1b[38;2;255;0;0mred\x1b[m";
        let out = downsample_sgr(s, Profile::Ascii);
        assert_eq!(out, "\x1b[1mbold\x1b[m \x1b[mred\x1b[m");
    }

    #[test]
    fn test_downsample_sgr_reset_param() {
        let s = "\x1b[0m";
        let out = downsample_sgr(s, Profile::Ansi256);
        assert_eq!(out, "\x1b[m");
    }
}
