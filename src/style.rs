//! Cleanroom Rust port of upstream Go source files: `style.go`, `set.go`, `get.go`, `unset.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! A fluent style builder and rendering pipeline matching upstream
//! `lipgloss.Style`: text attributes, colors, width/height, alignment, padding,
//! margins, borders, wrapping, truncation, hyperlinks, and transforms.
//! </public-docs>

use crate::align::{align_text_horizontal, align_text_vertical, get_lines, Position, TOP};
use crate::ansi::{self, Underline};
use crate::border::{self, Border};
use crate::color::Color;

/// <upstream-comment>NBSP is the non-breaking space rune.</upstream-comment>
pub const NBSP: char = '\u{00A0}';
/// The default tab width in cells.
pub const TAB_WIDTH_DEFAULT: usize = 4;

/// <upstream-comment>NoTabConversion can be passed to [Style.TabWidth] to disable the replacement
/// of tabs with spaces at render time.</upstream-comment>
pub const NO_TAB_CONVERSION: isize = -1;

type Props = u128;

// Property keys.
const BOLD_KEY: Props = 1 << 0;
const ITALIC_KEY: Props = 1 << 1;
const STRIKETHROUGH_KEY: Props = 1 << 2;
const REVERSE_KEY: Props = 1 << 3;
const BLINK_KEY: Props = 1 << 4;
const FAINT_KEY: Props = 1 << 5;
const UNDERLINE_SPACES_KEY: Props = 1 << 6;
const STRIKETHROUGH_SPACES_KEY: Props = 1 << 7;
const COLOR_WHITESPACE_KEY: Props = 1 << 8;
const UNDERLINE_KEY: Props = 1 << 9;
const FOREGROUND_KEY: Props = 1 << 10;
const BACKGROUND_KEY: Props = 1 << 11;
const UNDERLINE_COLOR_KEY: Props = 1 << 12;
const WIDTH_KEY: Props = 1 << 13;
const HEIGHT_KEY: Props = 1 << 14;
const ALIGN_HORIZONTAL_KEY: Props = 1 << 15;
const ALIGN_VERTICAL_KEY: Props = 1 << 16;
const PADDING_TOP_KEY: Props = 1 << 17;
const PADDING_RIGHT_KEY: Props = 1 << 18;
const PADDING_BOTTOM_KEY: Props = 1 << 19;
const PADDING_LEFT_KEY: Props = 1 << 20;
const PADDING_CHAR_KEY: Props = 1 << 21;
const MARGIN_TOP_KEY: Props = 1 << 22;
const MARGIN_RIGHT_KEY: Props = 1 << 23;
const MARGIN_BOTTOM_KEY: Props = 1 << 24;
const MARGIN_LEFT_KEY: Props = 1 << 25;
const MARGIN_BACKGROUND_KEY: Props = 1 << 26;
const MARGIN_CHAR_KEY: Props = 1 << 27;
const BORDER_STYLE_KEY: Props = 1 << 28;
const BORDER_TOP_KEY: Props = 1 << 29;
const BORDER_RIGHT_KEY: Props = 1 << 30;
const BORDER_BOTTOM_KEY: Props = 1 << 31;
const BORDER_LEFT_KEY: Props = 1 << 32;
const BORDER_TOP_FOREGROUND_KEY: Props = 1 << 33;
const BORDER_RIGHT_FOREGROUND_KEY: Props = 1 << 34;
const BORDER_BOTTOM_FOREGROUND_KEY: Props = 1 << 35;
const BORDER_LEFT_FOREGROUND_KEY: Props = 1 << 36;
const BORDER_FOREGROUND_BLEND_KEY: Props = 1 << 37;
const BORDER_FOREGROUND_BLEND_OFFSET_KEY: Props = 1 << 38;
const BORDER_TOP_BACKGROUND_KEY: Props = 1 << 39;
const BORDER_RIGHT_BACKGROUND_KEY: Props = 1 << 40;
const BORDER_BOTTOM_BACKGROUND_KEY: Props = 1 << 41;
const BORDER_LEFT_BACKGROUND_KEY: Props = 1 << 42;
const INLINE_KEY: Props = 1 << 43;
const MAX_WIDTH_KEY: Props = 1 << 44;
const MAX_HEIGHT_KEY: Props = 1 << 45;
const TAB_WIDTH_KEY: Props = 1 << 46;
const TRANSFORM_KEY: Props = 1 << 47;
const LINK_KEY: Props = 1 << 48;
const LINK_PARAMS_KEY: Props = 1 << 49;

/// A transform applied to a string at render time.
pub type Transform = fn(&str) -> String;

/// <upstream-comment>Style contains a set of rules that comprise a style as a whole.</upstream-comment>
#[derive(Debug, Clone, Default)]
pub struct Style {
    props: Props,
    value: String,

    // hyperlink
    link: String,
    link_params: String,

    // we store bool props values here
    attrs: Props,

    // props that have values
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    ul_color: Option<Color>,

    ul: Underline,

    width: usize,
    height: usize,

    align_horizontal: Position,
    align_vertical: Position,

    padding_top: usize,
    padding_right: usize,
    padding_bottom: usize,
    padding_left: usize,
    padding_char: char,

    margin_top: usize,
    margin_right: usize,
    margin_bottom: usize,
    margin_left: usize,
    margin_bg_color: Option<Color>,
    margin_char: char,

    border_style: Border,
    border_top_fg_color: Option<Color>,
    border_right_fg_color: Option<Color>,
    border_bottom_fg_color: Option<Color>,
    border_left_fg_color: Option<Color>,
    border_blend_fg_color: Vec<Color>,
    border_foreground_blend_offset: isize,
    border_top_bg_color: Option<Color>,
    border_right_bg_color: Option<Color>,
    border_bottom_bg_color: Option<Color>,
    border_left_bg_color: Option<Color>,

    max_width: usize,
    max_height: usize,
    tab_width: isize,

    transform: Option<Transform>,
}

fn join_string(strs: &[&str]) -> String {
    strs.join(" ")
}

/// <upstream-comment>String implements stringer for a Style, returning the rendered result based
/// on the rules in this style. An underlying string value must be set with
/// Style.SetString prior to using this method.</upstream-comment>
impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render(""))
    }
}

impl Style {
    /// <upstream-comment>NewStyle returns a new, empty Style. While it's syntactic sugar for the
    /// `Style{}` primitive, it's recommended to use this function for creating styles
    /// in case the underlying implementation changes.</upstream-comment>
    pub fn new() -> Style {
        Style::default()
    }

    /// A style that uses the given transform at render time.
    pub fn with_transform(f: Transform) -> Style {
        Style::new().transform(f)
    }

    // ------------------------------------------------------------------
    // Setters
    // ------------------------------------------------------------------

    /// <upstream-comment>SetString sets the underlying string value for this style. To render once
    /// the underlying string is set, use the [Style.String]. This method is
    /// a convenience for cases when having a stringer implementation is handy, such
    /// as when using fmt.Sprintf. You can also simply define a style and render out
    /// strings directly with [Style.Render].</upstream-comment>
    pub fn set_string(mut self, strs: &[&str]) -> Style {
        self.value = join_string(strs);
        self
    }

    /// <upstream-comment>Value returns the raw, unformatted, underlying string value for this style.</upstream-comment>
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Renders the underlying string value.
    pub fn string(&self) -> String {
        self.render("")
    }

    /// <upstream-comment>Inherit overlays the style in the argument onto this style by copying each explicitly
    /// set value from the argument style onto this style if it is not already explicitly set.
    /// Existing set values are kept intact and not overwritten.
    ///
    /// Margins, padding, and underlying string values are not inherited.</upstream-comment>
    pub fn inherit(mut self, i: &Style) -> Style {
        let keys: [Props; 50] = [
            BOLD_KEY,
            ITALIC_KEY,
            STRIKETHROUGH_KEY,
            REVERSE_KEY,
            BLINK_KEY,
            FAINT_KEY,
            UNDERLINE_SPACES_KEY,
            STRIKETHROUGH_SPACES_KEY,
            COLOR_WHITESPACE_KEY,
            UNDERLINE_KEY,
            FOREGROUND_KEY,
            BACKGROUND_KEY,
            UNDERLINE_COLOR_KEY,
            WIDTH_KEY,
            HEIGHT_KEY,
            ALIGN_HORIZONTAL_KEY,
            ALIGN_VERTICAL_KEY,
            PADDING_TOP_KEY,
            PADDING_RIGHT_KEY,
            PADDING_BOTTOM_KEY,
            PADDING_LEFT_KEY,
            PADDING_CHAR_KEY,
            MARGIN_TOP_KEY,
            MARGIN_RIGHT_KEY,
            MARGIN_BOTTOM_KEY,
            MARGIN_LEFT_KEY,
            MARGIN_BACKGROUND_KEY,
            MARGIN_CHAR_KEY,
            BORDER_STYLE_KEY,
            BORDER_TOP_KEY,
            BORDER_RIGHT_KEY,
            BORDER_BOTTOM_KEY,
            BORDER_LEFT_KEY,
            BORDER_TOP_FOREGROUND_KEY,
            BORDER_RIGHT_FOREGROUND_KEY,
            BORDER_BOTTOM_FOREGROUND_KEY,
            BORDER_LEFT_FOREGROUND_KEY,
            BORDER_FOREGROUND_BLEND_KEY,
            BORDER_FOREGROUND_BLEND_OFFSET_KEY,
            BORDER_TOP_BACKGROUND_KEY,
            BORDER_RIGHT_BACKGROUND_KEY,
            BORDER_BOTTOM_BACKGROUND_KEY,
            BORDER_LEFT_BACKGROUND_KEY,
            INLINE_KEY,
            MAX_WIDTH_KEY,
            MAX_HEIGHT_KEY,
            TAB_WIDTH_KEY,
            TRANSFORM_KEY,
            LINK_KEY,
            LINK_PARAMS_KEY,
        ];
        for k in keys {
            if !i.is_set(k) {
                continue;
            }
            match k {
                MARGIN_TOP_KEY | MARGIN_RIGHT_KEY | MARGIN_BOTTOM_KEY | MARGIN_LEFT_KEY => {
                    // Margins are not inherited.
                    continue;
                }
                PADDING_TOP_KEY | PADDING_RIGHT_KEY | PADDING_BOTTOM_KEY | PADDING_LEFT_KEY => {
                    // Padding is not inherited.
                    continue;
                }
                BACKGROUND_KEY
                    if !self.is_set(MARGIN_BACKGROUND_KEY) && !i.is_set(MARGIN_BACKGROUND_KEY) =>
                {
                    // The margins also inherit the background color.
                    self.set(MARGIN_BACKGROUND_KEY, Value::Color(i.bg_color.clone()));
                }
                _ => {}
            }
            if self.is_set(k) {
                continue;
            }
            self.set_from(k, i);
        }
        self
    }

    // ------------------------------------------------------------------
    // Rendering
    // ------------------------------------------------------------------

    /// <upstream-comment>Render applies the defined style formatting to a given string.</upstream-comment>
    pub fn render(&self, strs: &str) -> String {
        let mut strs_vec: Vec<&str> = Vec::new();
        if !self.value.is_empty() {
            strs_vec.push(&self.value);
        }
        if !strs.is_empty() {
            strs_vec.push(strs);
        }
        let mut str = join_string(&strs_vec);

        let mut te = ansi::Style::default();
        let mut te_space = ansi::Style::default();
        let mut te_whitespace = ansi::Style::default();

        let bold = self.get_as_bool(BOLD_KEY, false);
        let italic = self.get_as_bool(ITALIC_KEY, false);
        let strikethrough = self.get_as_bool(STRIKETHROUGH_KEY, false);
        let reverse = self.get_as_bool(REVERSE_KEY, false);
        let blink = self.get_as_bool(BLINK_KEY, false);
        let faint = self.get_as_bool(FAINT_KEY, false);

        let fg = self.get_as_color(FOREGROUND_KEY);
        let bg = self.get_as_color(BACKGROUND_KEY);
        let ul = self.get_as_color(UNDERLINE_COLOR_KEY);

        let underline = self.ul != Underline::None;
        let width = self.get_as_int(WIDTH_KEY);
        let height = self.get_as_int(HEIGHT_KEY);
        let horizontal_align = self.get_as_position(ALIGN_HORIZONTAL_KEY);
        let vertical_align = self.get_as_position(ALIGN_VERTICAL_KEY);

        let top_padding = self.get_as_int(PADDING_TOP_KEY);
        let right_padding = self.get_as_int(PADDING_RIGHT_KEY);
        let bottom_padding = self.get_as_int(PADDING_BOTTOM_KEY);
        let left_padding = self.get_as_int(PADDING_LEFT_KEY);

        let horizontal_border_size = self.get_horizontal_border_size();
        let vertical_border_size = self.get_vertical_border_size();

        let color_whitespace = self.get_as_bool(COLOR_WHITESPACE_KEY, true);
        let inline = self.get_as_bool(INLINE_KEY, false);
        let max_width = self.get_as_int(MAX_WIDTH_KEY);
        let max_height = self.get_as_int(MAX_HEIGHT_KEY);

        let underline_spaces = self.get_as_bool(UNDERLINE_SPACES_KEY, false)
            || (underline && self.get_as_bool(UNDERLINE_SPACES_KEY, true));
        let strikethrough_spaces = self.get_as_bool(STRIKETHROUGH_SPACES_KEY, false)
            || (strikethrough && self.get_as_bool(STRIKETHROUGH_SPACES_KEY, true));

        // Do we need to style whitespace (padding and space outside paragraphs) separately?
        let style_whitespace = reverse;

        // Do we need to style spaces separately?
        let use_space_styler =
            underline || underline_spaces || strikethrough || strikethrough_spaces;

        let transform = self.get_as_transform(TRANSFORM_KEY);
        let (link, link_params) = self.get_hyperlink();

        if let Some(t) = transform {
            str = t(&str);
        }

        if self.props == 0 {
            return self.maybe_convert_tabs(&str);
        }

        if bold {
            te.bold = true;
        }
        if italic {
            te.italic = true;
        }
        if underline {
            te.underline = true;
            te.underline_style = self.ul;
        }
        if reverse {
            te_whitespace.reverse = true;
            te.reverse = true;
        }
        if blink {
            te.blink = true;
        }
        if faint {
            te.faint = true;
        }

        if !fg.is_no_color() {
            te.fg_color = Some(fg.clone());
            if style_whitespace {
                te_whitespace.fg_color = Some(fg.clone());
            }
            if use_space_styler {
                te_space.fg_color = Some(fg.clone());
            }
        }

        if !bg.is_no_color() {
            te.bg_color = Some(bg.clone());
            if color_whitespace {
                te_whitespace.bg_color = Some(bg.clone());
            }
            if use_space_styler {
                te_space.bg_color = Some(bg.clone());
            }
        }

        if !ul.is_no_color() {
            te.ul_color = Some(ul.clone());
            if color_whitespace {
                te_whitespace.ul_color = Some(ul.clone());
            }
            if use_space_styler {
                te_space.ul_color = Some(ul.clone());
            }
        }

        if underline {
            te.underline = true;
            te.underline_style = self.ul;
        }
        if strikethrough {
            te.strikethrough = true;
        }

        if underline_spaces {
            te_space.underline = true;
        }
        if strikethrough_spaces {
            te_space.strikethrough = true;
        }

        // Potentially convert tabs to spaces.
        str = self.maybe_convert_tabs(&str);
        // Carriage returns can cause strange behaviour when rendering.
        str = str.replace("\r\n", "\n");

        // Strip newlines in single line mode.
        if inline {
            str = str.replace('\n', "");
        }

        // Include borders in block size.
        let width = width.saturating_sub(horizontal_border_size);
        let height = height.saturating_sub(vertical_border_size);

        // Word wrap.
        if !inline && width > 0 {
            let wrap_at = width.saturating_sub(left_padding + right_padding);
            str = ansi::wrap(&str, wrap_at, "");
        }

        // Render core text.
        {
            let mut b = String::new();
            let mut is_first = true;
            for line in str.split('\n') {
                if !is_first {
                    b.push('\n');
                }
                is_first = false;
                if use_space_styler {
                    // Look for spaces and apply a different styler.
                    for r in line.chars() {
                        if r.is_whitespace() {
                            // Upstream renders spaces with a `teSpace` style
                            // whose params put the underline color before the
                            // underline flag; mirror that ordering.
                            b.push_str(&te_space.styled_whitespace(&r.to_string()));
                        } else {
                            b.push_str(&te.styled(&r.to_string()));
                        }
                    }
                } else {
                    b.push_str(&te.styled(line));
                }
            }
            str = b;
            if !link.is_empty() {
                str = format!(
                    "{}{}{}",
                    ansi::set_hyperlink(&link, &link_params),
                    str,
                    ansi::reset_hyperlink()
                );
            }
        }

        // Padding.
        if !inline {
            let pad_char = if self.padding_char == '\0' {
                ' '
            } else {
                self.padding_char
            };
            if left_padding > 0 {
                let st = if color_whitespace || style_whitespace {
                    Some(&te_whitespace)
                } else {
                    None
                };
                str = pad_left(&str, left_padding, st, pad_char);
            }
            if right_padding > 0 {
                let st = if color_whitespace || style_whitespace {
                    Some(&te_whitespace)
                } else {
                    None
                };
                str = pad_right(&str, right_padding, st, pad_char);
            }
            if top_padding > 0 {
                str = format!("{}{}", "\n".repeat(top_padding), str);
            }
            if bottom_padding > 0 {
                str = format!("{}{}", str, "\n".repeat(bottom_padding));
            }
        }

        // Height.
        if height > 0 {
            str = align_text_vertical(&str, vertical_align, height);
        }

        // Set alignment. This will also pad short lines with spaces so that all
        // lines are the same length.
        {
            let num_lines = str.matches('\n').count();
            if num_lines != 0 || width != 0 {
                let st = if color_whitespace || style_whitespace {
                    Some(&te_whitespace)
                } else {
                    None
                };
                str = align_text_horizontal(&str, horizontal_align, width, st);
            }
        }

        if !inline {
            str = self.apply_border(&str);
            str = self.apply_margins(&str, inline);
        }

        // Truncate according to MaxWidth.
        if max_width > 0 {
            let mut lines: Vec<String> = str.split('\n').map(|l| l.to_string()).collect();
            for line in &mut lines {
                *line = ansi::truncate(line, max_width, "");
            }
            str = lines.join("\n");
        }

        // Truncate according to MaxHeight.
        if max_height > 0 {
            let lines: Vec<&str> = str.split('\n').collect();
            let h = max_height.min(lines.len());
            if !lines.is_empty() {
                str = lines[..h].join("\n");
            }
        }

        str
    }

    fn maybe_convert_tabs(&self, str: &str) -> String {
        let tw = if self.is_set(TAB_WIDTH_KEY) {
            self.tab_width
        } else {
            TAB_WIDTH_DEFAULT as isize
        };
        match tw {
            -1 => str.to_string(),
            0 => str.replace('\t', ""),
            n => str.replace('\t', &" ".repeat(n as usize)),
        }
    }

    fn apply_margins(&self, str: &str, inline: bool) -> String {
        let top_margin = self.get_as_int(MARGIN_TOP_KEY);
        let right_margin = self.get_as_int(MARGIN_RIGHT_KEY);
        let bottom_margin = self.get_as_int(MARGIN_BOTTOM_KEY);
        let left_margin = self.get_as_int(MARGIN_LEFT_KEY);

        let mut style = ansi::Style::default();
        let bgc = self.get_as_color(MARGIN_BACKGROUND_KEY);
        if !bgc.is_no_color() {
            style.bg_color = Some(bgc);
        }

        let margin_char = if self.margin_char == '\0' {
            ' '
        } else {
            self.margin_char
        };

        let mut str = pad_left(str, left_margin, Some(&style), margin_char);
        str = pad_right(&str, right_margin, Some(&style), margin_char);

        // Top/bottom margin.
        if !inline {
            let (_, width) = get_lines(&str);
            let spaces = " ".repeat(width);
            if top_margin > 0 {
                let block = format!("{}\n", spaces).repeat(top_margin);
                str = format!("{}{}", style.styled(&block), str);
            }
            if bottom_margin > 0 {
                let block = format!("\n{}", spaces).repeat(bottom_margin);
                str = format!("{}{}", str, style.styled(&block));
            }
        }

        str
    }

    // ------------------------------------------------------------------
    // Style setters (matching `set.go`)
    // ------------------------------------------------------------------

    /// <upstream-comment>Bold sets a bold formatting rule.</upstream-comment>
    pub fn bold(mut self, v: bool) -> Style {
        self.set(BOLD_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>Italic sets an italic formatting rule. In some terminal emulators this will
    /// render with "reverse" coloring if not italic font variant is available.</upstream-comment>
    pub fn italic(mut self, v: bool) -> Style {
        self.set(ITALIC_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>Underline sets an underline rule. By default, underlines will not be drawn on
    /// whitespace like margins and padding. To change this behavior set
    /// [Style.UnderlineSpaces].</upstream-comment>
    pub fn underline(self, v: bool) -> Style {
        if v {
            return self.underline_style(Underline::Single);
        }
        self.underline_style(Underline::None)
    }

    /// <upstream-comment>UnderlineStyle sets the underline style. This can be used to set the underline
    /// to be a single, double, curly, dotted, or dashed line.</upstream-comment>
    pub fn underline_style(mut self, u: Underline) -> Style {
        self.set(UNDERLINE_KEY, Value::Underline(u));
        self
    }

    /// <upstream-comment>UnderlineColor sets the color of the underline. By default, the underline
    /// will be the same color as the foreground.</upstream-comment>
    pub fn underline_color(mut self, c: Color) -> Style {
        self.set(UNDERLINE_COLOR_KEY, Value::Color(Some(c)));
        self
    }

    /// <upstream-comment>Strikethrough sets a strikethrough rule. By default, strikes will not be
    /// drawn on whitespace like margins and padding. To change this behavior set
    /// StrikethroughSpaces.</upstream-comment>
    pub fn strikethrough(mut self, v: bool) -> Style {
        self.set(STRIKETHROUGH_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>Reverse sets a rule for inverting foreground and background colors.</upstream-comment>
    pub fn reverse(mut self, v: bool) -> Style {
        self.set(REVERSE_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>Blink sets a rule for blinking foreground text.</upstream-comment>
    pub fn blink(mut self, v: bool) -> Style {
        self.set(BLINK_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>Faint sets a rule for rendering the foreground color in a dimmer shade.</upstream-comment>
    pub fn faint(mut self, v: bool) -> Style {
        self.set(FAINT_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>Foreground sets a foreground color.</upstream-comment>
    pub fn foreground(mut self, c: &str) -> Style {
        self.set(FOREGROUND_KEY, Value::Color(Some(Color::parse(c))));
        self
    }

    /// Sets the foreground color directly from a `Color` value.
    pub fn foreground_color(mut self, c: Color) -> Style {
        self.set(FOREGROUND_KEY, Value::Color(Some(c)));
        self
    }

    /// <upstream-comment>Background sets a background color.</upstream-comment>
    pub fn background(mut self, c: &str) -> Style {
        self.set(BACKGROUND_KEY, Value::Color(Some(Color::parse(c))));
        self
    }

    /// Sets the background color directly from a `Color` value.
    pub fn background_color(mut self, c: Color) -> Style {
        self.set(BACKGROUND_KEY, Value::Color(Some(c)));
        self
    }

    /// <upstream-comment>Width sets the width of the block before applying margins. This means your
    /// styled content will exactly equal the size set here. Text will wrap based on
    /// Padding and Borders set on the style.</upstream-comment>
    pub fn width(mut self, i: usize) -> Style {
        self.set(WIDTH_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>Height sets the height of the block before applying padding (or not), the
    /// block will be set to this height.</upstream-comment>
    pub fn height(mut self, i: usize) -> Style {
        self.set(HEIGHT_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>Align is a shorthand method for setting horizontal and vertical alignment.
    ///
    /// With one argument, the position value is applied to the horizontal alignment.
    ///
    /// With two arguments, the value is applied to the horizontal and vertical
    /// alignments, in that order.</upstream-comment>
    pub fn align(mut self, p: &[Position]) -> Style {
        if let Some(&first) = p.first() {
            self.set(ALIGN_HORIZONTAL_KEY, Value::Position(first));
        }
        if let Some(&second) = p.get(1) {
            self.set(ALIGN_VERTICAL_KEY, Value::Position(second));
        }
        self
    }

    /// <upstream-comment>AlignHorizontal sets a horizontal text alignment rule.</upstream-comment>
    pub fn align_horizontal(mut self, p: Position) -> Style {
        self.set(ALIGN_HORIZONTAL_KEY, Value::Position(p));
        self
    }

    /// <upstream-comment>AlignVertical sets a vertical text alignment rule.</upstream-comment>
    pub fn align_vertical(mut self, p: Position) -> Style {
        self.set(ALIGN_VERTICAL_KEY, Value::Position(p));
        self
    }

    /// <upstream-comment>Padding is a shorthand method for setting padding on all sides at once.</upstream-comment>
    pub fn padding(mut self, i: &[usize]) -> Style {
        if let Some((top, right, bottom, left)) = which_sides_int(i) {
            self.set(PADDING_TOP_KEY, Value::Int(top));
            self.set(PADDING_RIGHT_KEY, Value::Int(right));
            self.set(PADDING_BOTTOM_KEY, Value::Int(bottom));
            self.set(PADDING_LEFT_KEY, Value::Int(left));
        }
        self
    }

    /// <upstream-comment>PaddingLeft adds padding on the left.</upstream-comment>
    pub fn padding_left(mut self, i: usize) -> Style {
        self.set(PADDING_LEFT_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>PaddingRight adds padding on the right.</upstream-comment>
    pub fn padding_right(mut self, i: usize) -> Style {
        self.set(PADDING_RIGHT_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>PaddingTop adds padding to the top of the block.</upstream-comment>
    pub fn padding_top(mut self, i: usize) -> Style {
        self.set(PADDING_TOP_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>PaddingBottom adds padding to the bottom of the block.</upstream-comment>
    pub fn padding_bottom(mut self, i: usize) -> Style {
        self.set(PADDING_BOTTOM_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>PaddingChar sets the character used for padding. This is useful for
    /// rendering blocks with a specific character, such as a space or a dot.</upstream-comment>
    pub fn padding_char(mut self, r: char) -> Style {
        self.set(PADDING_CHAR_KEY, Value::Char(r));
        self
    }

    /// <upstream-comment>ColorWhitespace determines whether or not the background color should be
    /// applied to the padding. This is true by default as it's more than likely the
    /// desired and expected behavior, but it can be disabled for certain graphic
    /// effects.</upstream-comment>
    pub fn color_whitespace(mut self, v: bool) -> Style {
        self.set(COLOR_WHITESPACE_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>Margin is a shorthand method for setting margins on all sides at once.</upstream-comment>
    pub fn margin(mut self, i: &[usize]) -> Style {
        if let Some((top, right, bottom, left)) = which_sides_int(i) {
            self.set(MARGIN_TOP_KEY, Value::Int(top));
            self.set(MARGIN_RIGHT_KEY, Value::Int(right));
            self.set(MARGIN_BOTTOM_KEY, Value::Int(bottom));
            self.set(MARGIN_LEFT_KEY, Value::Int(left));
        }
        self
    }

    /// <upstream-comment>MarginLeft sets the value of the left margin.</upstream-comment>
    pub fn margin_left(mut self, i: usize) -> Style {
        self.set(MARGIN_LEFT_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>MarginRight sets the value of the right margin.</upstream-comment>
    pub fn margin_right(mut self, i: usize) -> Style {
        self.set(MARGIN_RIGHT_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>MarginTop sets the value of the top margin.</upstream-comment>
    pub fn margin_top(mut self, i: usize) -> Style {
        self.set(MARGIN_TOP_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>MarginBottom sets the value of the bottom margin.</upstream-comment>
    pub fn margin_bottom(mut self, i: usize) -> Style {
        self.set(MARGIN_BOTTOM_KEY, Value::Int(i));
        self
    }

    /// <upstream-comment>MarginBackground sets the background color of the margin.</upstream-comment>
    pub fn margin_background(mut self, c: &str) -> Style {
        self.set(MARGIN_BACKGROUND_KEY, Value::Color(Some(Color::parse(c))));
        self
    }

    /// <upstream-comment>MarginChar sets the character used for the margin.</upstream-comment>
    pub fn margin_char(mut self, r: char) -> Style {
        self.set(MARGIN_CHAR_KEY, Value::Char(r));
        self
    }

    /// <upstream-comment>Border is shorthand for setting the border style and which sides should
    /// have a border at once.</upstream-comment>
    pub fn border(mut self, b: Border, sides: &[bool]) -> Style {
        self.set(BORDER_STYLE_KEY, Value::Border(Box::new(b)));
        let (top, right, bottom, left, ok) = which_sides_bool(sides);
        let (top, right, bottom, left) = if ok {
            (top, right, bottom, left)
        } else {
            (true, true, true, true)
        };
        self.set(BORDER_TOP_KEY, Value::Bool(top));
        self.set(BORDER_RIGHT_KEY, Value::Bool(right));
        self.set(BORDER_BOTTOM_KEY, Value::Bool(bottom));
        self.set(BORDER_LEFT_KEY, Value::Bool(left));
        self
    }

    /// <upstream-comment>BorderStyle defines the Border on a style.</upstream-comment>
    pub fn border_style(mut self, b: Border) -> Style {
        self.set(BORDER_STYLE_KEY, Value::Border(Box::new(b)));
        self
    }

    /// <upstream-comment>BorderTop determines whether or not to draw a top border.</upstream-comment>
    pub fn border_top(mut self, v: bool) -> Style {
        self.set(BORDER_TOP_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>BorderRight determines whether or not to draw a right border.</upstream-comment>
    pub fn border_right(mut self, v: bool) -> Style {
        self.set(BORDER_RIGHT_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>BorderBottom determines whether or not to draw a bottom border.</upstream-comment>
    pub fn border_bottom(mut self, v: bool) -> Style {
        self.set(BORDER_BOTTOM_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>BorderLeft determines whether or not to draw a left border.</upstream-comment>
    pub fn border_left(mut self, v: bool) -> Style {
        self.set(BORDER_LEFT_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>BorderForeground is a shorthand function for setting all of the
    /// foreground colors of the borders at once.</upstream-comment>
    pub fn border_foreground(mut self, c: &[&str]) -> Style {
        if c.is_empty() {
            return self;
        }
        let colors: Vec<Color> = c.iter().map(|s| Color::parse(s)).collect();
        if let Some((top, right, bottom, left)) = which_sides_color(&colors) {
            self.set(BORDER_TOP_FOREGROUND_KEY, Value::Color(Some(top)));
            self.set(BORDER_RIGHT_FOREGROUND_KEY, Value::Color(Some(right)));
            self.set(BORDER_BOTTOM_FOREGROUND_KEY, Value::Color(Some(bottom)));
            self.set(BORDER_LEFT_FOREGROUND_KEY, Value::Color(Some(left)));
        }
        self
    }

    /// <upstream-comment>BorderTopForeground set the foreground color for the top of the border.</upstream-comment>
    pub fn border_top_foreground(mut self, c: &str) -> Style {
        self.set(
            BORDER_TOP_FOREGROUND_KEY,
            Value::Color(Some(Color::parse(c))),
        );
        self
    }

    /// <upstream-comment>BorderRightForeground sets the foreground color for the right side of the
    /// border.</upstream-comment>
    pub fn border_right_foreground(mut self, c: &str) -> Style {
        self.set(
            BORDER_RIGHT_FOREGROUND_KEY,
            Value::Color(Some(Color::parse(c))),
        );
        self
    }

    /// <upstream-comment>BorderBottomForeground sets the foreground color for the bottom of the
    /// border.</upstream-comment>
    pub fn border_bottom_foreground(mut self, c: &str) -> Style {
        self.set(
            BORDER_BOTTOM_FOREGROUND_KEY,
            Value::Color(Some(Color::parse(c))),
        );
        self
    }

    /// <upstream-comment>BorderLeftForeground sets the foreground color for the left side of the
    /// border.</upstream-comment>
    pub fn border_left_foreground(mut self, c: &str) -> Style {
        self.set(
            BORDER_LEFT_FOREGROUND_KEY,
            Value::Color(Some(Color::parse(c))),
        );
        self
    }

    /// <upstream-comment>BorderForegroundBlend sets the foreground colors for the border blend. At least
    /// 2 colors are required to use blending, otherwise this will no-op with 0 colors,
    /// and pass to BorderForeground with 1 color. This will override all other border
    /// foreground colors when used.</upstream-comment>
    pub fn border_foreground_blend(mut self, c: &[&str]) -> Style {
        if c.is_empty() {
            return self;
        }
        if c.len() == 1 {
            return self.border_foreground(&[c[0]]);
        }
        self.set(
            BORDER_FOREGROUND_BLEND_KEY,
            Value::Colors(c.iter().map(|s| Color::parse(s)).collect()),
        );
        self
    }

    /// <upstream-comment>BorderForegroundBlendOffset sets the border blend offset cells, starting from
    /// the top left corner. Value can be positive or negative, and does not need to
    /// equal the dimensions of the border region.</upstream-comment>
    pub fn border_foreground_blend_offset(mut self, v: isize) -> Style {
        self.set(BORDER_FOREGROUND_BLEND_OFFSET_KEY, Value::Int(v as usize));
        self
    }

    /// <upstream-comment>BorderBackground is a shorthand function for setting all of the
    /// background colors of the borders at once.</upstream-comment>
    pub fn border_background(mut self, c: &[&str]) -> Style {
        if c.is_empty() {
            return self;
        }
        let colors: Vec<Color> = c.iter().map(|s| Color::parse(s)).collect();
        if let Some((top, right, bottom, left)) = which_sides_color(&colors) {
            self.set(BORDER_TOP_BACKGROUND_KEY, Value::Color(Some(top)));
            self.set(BORDER_RIGHT_BACKGROUND_KEY, Value::Color(Some(right)));
            self.set(BORDER_BOTTOM_BACKGROUND_KEY, Value::Color(Some(bottom)));
            self.set(BORDER_LEFT_BACKGROUND_KEY, Value::Color(Some(left)));
        }
        self
    }

    /// <upstream-comment>BorderTopBackground sets the background color of the top of the border.</upstream-comment>
    pub fn border_top_background(mut self, c: &str) -> Style {
        self.set(
            BORDER_TOP_BACKGROUND_KEY,
            Value::Color(Some(Color::parse(c))),
        );
        self
    }

    /// <upstream-comment>BorderRightBackground sets the background color of right side the border.</upstream-comment>
    pub fn border_right_background(mut self, c: &str) -> Style {
        self.set(
            BORDER_RIGHT_BACKGROUND_KEY,
            Value::Color(Some(Color::parse(c))),
        );
        self
    }

    /// <upstream-comment>BorderBottomBackground sets the background color of the bottom of the
    /// border.</upstream-comment>
    pub fn border_bottom_background(mut self, c: &str) -> Style {
        self.set(
            BORDER_BOTTOM_BACKGROUND_KEY,
            Value::Color(Some(Color::parse(c))),
        );
        self
    }

    /// <upstream-comment>BorderLeftBackground set the background color of the left side of the
    /// border.</upstream-comment>
    pub fn border_left_background(mut self, c: &str) -> Style {
        self.set(
            BORDER_LEFT_BACKGROUND_KEY,
            Value::Color(Some(Color::parse(c))),
        );
        self
    }

    /// <upstream-comment>Inline makes rendering output one line and disables the rendering of
    /// margins, padding and borders. This is useful when you need a style to apply
    /// only to font rendering and don't want it to change any physical dimensions.</upstream-comment>
    pub fn inline(mut self, v: bool) -> Style {
        self.set(INLINE_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>MaxWidth applies a max width to a given style. This is useful in enforcing
    /// a certain width at render time, particularly with arbitrary strings and
    /// styles.</upstream-comment>
    pub fn max_width(mut self, n: usize) -> Style {
        self.set(MAX_WIDTH_KEY, Value::Int(n));
        self
    }

    /// <upstream-comment>MaxHeight applies a max height to a given style.</upstream-comment>
    pub fn max_height(mut self, n: usize) -> Style {
        self.set(MAX_HEIGHT_KEY, Value::Int(n));
        self
    }

    /// <upstream-comment>TabWidth sets the number of spaces that a tab (/t) should be rendered as.
    /// When set to 0, tabs will be removed. To disable the replacement of tabs with
    /// spaces entirely, set this to [NoTabConversion].
    ///
    /// By default, tabs will be replaced with 4 spaces.</upstream-comment>
    pub fn tab_width(mut self, n: isize) -> Style {
        let n = if n <= -1 { -1 } else { n };
        self.set(TAB_WIDTH_KEY, Value::Int(n as usize));
        self
    }

    /// <upstream-comment>UnderlineSpaces determines whether to underline spaces between words. By
    /// default, this is true. Spaces can also be underlined without underlining the
    /// text itself.</upstream-comment>
    pub fn underline_spaces(mut self, v: bool) -> Style {
        self.set(UNDERLINE_SPACES_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>StrikethroughSpaces determines whether to apply strikethroughs to spaces
    /// between words. By default, this is true. Spaces can also be struck without
    /// underlining the text itself.</upstream-comment>
    pub fn strikethrough_spaces(mut self, v: bool) -> Style {
        self.set(STRIKETHROUGH_SPACES_KEY, Value::Bool(v));
        self
    }

    /// <upstream-comment>Transform applies a given function to a string at render time, allowing for
    /// the string being rendered to be manipulated.</upstream-comment>
    pub fn transform(mut self, f: Transform) -> Style {
        self.set(TRANSFORM_KEY, Value::Transform(f));
        self
    }

    /// <upstream-comment>Hyperlink sets a hyperlink on a style. This is useful for rendering text that
    /// can be clicked on in a terminal emulator that supports hyperlinks.</upstream-comment>
    pub fn hyperlink(mut self, link: &str, params: &[&str]) -> Style {
        self.set(LINK_KEY, Value::String(link.to_string()));
        if !params.is_empty() {
            self.set(LINK_PARAMS_KEY, Value::String(params.join(":")));
        }
        self
    }

    // ------------------------------------------------------------------
    // Getters (matching `get.go`)
    // ------------------------------------------------------------------

    /// <upstream-comment>GetBold returns the style's bold value. If no value is set false is returned.</upstream-comment>
    pub fn get_bold(&self) -> bool {
        self.get_as_bool(BOLD_KEY, false)
    }

    /// <upstream-comment>GetItalic returns the style's italic value. If no value is set false is returned.</upstream-comment>
    pub fn get_italic(&self) -> bool {
        self.get_as_bool(ITALIC_KEY, false)
    }

    /// <upstream-comment>GetUnderline returns the style's underline value. If no value is set false is returned.</upstream-comment>
    pub fn get_underline(&self) -> bool {
        self.ul != Underline::None
    }

    /// <upstream-comment>GetUnderlineStyle returns the style's underline style. If no value is set
    /// UnderlineNone is returned.</upstream-comment>
    pub fn get_underline_style(&self) -> Underline {
        self.ul
    }

    /// <upstream-comment>GetUnderlineColor returns the style's underline color. If no value is set
    /// NoColor{} is returned.</upstream-comment>
    pub fn get_underline_color(&self) -> Color {
        self.get_as_color(UNDERLINE_COLOR_KEY)
    }

    /// <upstream-comment>GetStrikethrough returns the style's strikethrough value. If no value is set false is returned.</upstream-comment>
    pub fn get_strikethrough(&self) -> bool {
        self.get_as_bool(STRIKETHROUGH_KEY, false)
    }

    /// <upstream-comment>GetReverse returns the style's reverse value. If no value is set false is returned.</upstream-comment>
    pub fn get_reverse(&self) -> bool {
        self.get_as_bool(REVERSE_KEY, false)
    }

    /// <upstream-comment>GetBlink returns the style's blink value. If no value is set false is returned.</upstream-comment>
    pub fn get_blink(&self) -> bool {
        self.get_as_bool(BLINK_KEY, false)
    }

    /// <upstream-comment>GetFaint returns the style's faint value. If no value is set false is returned.</upstream-comment>
    pub fn get_faint(&self) -> bool {
        self.get_as_bool(FAINT_KEY, false)
    }

    /// <upstream-comment>GetForeground returns the style's foreground color. If no value is set
    /// NoColor{} is returned.</upstream-comment>
    pub fn get_foreground(&self) -> Color {
        self.get_as_color(FOREGROUND_KEY)
    }

    /// <upstream-comment>GetBackground returns the style's background color. If no value is set
    /// NoColor{} is returned.</upstream-comment>
    pub fn get_background(&self) -> Color {
        self.get_as_color(BACKGROUND_KEY)
    }

    /// <upstream-comment>GetWidth returns the style's width setting. If no width is set 0 is returned.</upstream-comment>
    pub fn get_width(&self) -> usize {
        self.get_as_int(WIDTH_KEY)
    }

    /// <upstream-comment>GetHeight returns the style's height setting. If no height is set 0 is returned.</upstream-comment>
    pub fn get_height(&self) -> usize {
        self.get_as_int(HEIGHT_KEY)
    }

    /// <upstream-comment>GetAlign returns the style's implicit horizontal alignment setting.
    /// If no alignment is set Position.Left is returned.</upstream-comment>
    pub fn get_align(&self) -> Position {
        let v = self.get_as_position(ALIGN_HORIZONTAL_KEY);
        if v == Position(0.0) {
            crate::align::LEFT
        } else {
            v
        }
    }

    /// <upstream-comment>GetAlignHorizontal returns the style's implicit horizontal alignment setting.</upstream-comment>
    pub fn get_align_horizontal(&self) -> Position {
        let v = self.get_as_position(ALIGN_HORIZONTAL_KEY);
        if v == Position(0.0) {
            crate::align::LEFT
        } else {
            v
        }
    }

    /// <upstream-comment>GetAlignVertical returns the style's implicit vertical alignment setting.
    /// If no alignment is set Position.Top is returned.</upstream-comment>
    pub fn get_align_vertical(&self) -> Position {
        let v = self.get_as_position(ALIGN_VERTICAL_KEY);
        if v == Position(0.0) {
            TOP
        } else {
            v
        }
    }

    /// <upstream-comment>GetPadding returns the style's top, right, bottom, and left padding values,
    /// in that order. 0 is returned for unset values.</upstream-comment>
    pub fn get_padding(&self) -> (usize, usize, usize, usize) {
        (
            self.get_as_int(PADDING_TOP_KEY),
            self.get_as_int(PADDING_RIGHT_KEY),
            self.get_as_int(PADDING_BOTTOM_KEY),
            self.get_as_int(PADDING_LEFT_KEY),
        )
    }

    /// <upstream-comment>GetPaddingTop returns the style's top padding. If no value is set 0 is returned.</upstream-comment>
    pub fn get_padding_top(&self) -> usize {
        self.get_as_int(PADDING_TOP_KEY)
    }

    /// <upstream-comment>GetPaddingRight returns the style's right padding. If no value is set 0 is returned.</upstream-comment>
    pub fn get_padding_right(&self) -> usize {
        self.get_as_int(PADDING_RIGHT_KEY)
    }

    /// <upstream-comment>GetPaddingBottom returns the style's bottom padding. If no value is set 0 is returned.</upstream-comment>
    pub fn get_padding_bottom(&self) -> usize {
        self.get_as_int(PADDING_BOTTOM_KEY)
    }

    /// <upstream-comment>GetPaddingLeft returns the style's left padding. If no value is set 0 is returned.</upstream-comment>
    pub fn get_padding_left(&self) -> usize {
        self.get_as_int(PADDING_LEFT_KEY)
    }

    /// <upstream-comment>GetPaddingChar returns the style's padding character. If no value is set a
    /// space is returned.</upstream-comment>
    pub fn get_padding_char(&self) -> char {
        if !self.is_set(PADDING_CHAR_KEY) {
            return ' ';
        }
        if self.padding_char == '\0' {
            ' '
        } else {
            self.padding_char
        }
    }

    /// <upstream-comment>GetHorizontalPadding returns the style's left and right padding. Unset
    /// values are measured as 0.</upstream-comment>
    pub fn get_horizontal_padding(&self) -> usize {
        self.get_as_int(PADDING_LEFT_KEY) + self.get_as_int(PADDING_RIGHT_KEY)
    }

    /// <upstream-comment>GetVerticalPadding returns the style's top and bottom padding. Unset values
    /// are measured as 0.</upstream-comment>
    pub fn get_vertical_padding(&self) -> usize {
        self.get_as_int(PADDING_TOP_KEY) + self.get_as_int(PADDING_BOTTOM_KEY)
    }

    /// <upstream-comment>GetColorWhitespace returns the style's whitespace coloring setting. If no
    /// value is set false is returned.</upstream-comment>
    pub fn get_color_whitespace(&self) -> bool {
        self.get_as_bool(COLOR_WHITESPACE_KEY, false)
    }

    /// <upstream-comment>GetMargin returns the style's top, right, bottom, and left margins, in that
    /// order. 0 is returned for unset values.</upstream-comment>
    pub fn get_margin(&self) -> (usize, usize, usize, usize) {
        (
            self.get_as_int(MARGIN_TOP_KEY),
            self.get_as_int(MARGIN_RIGHT_KEY),
            self.get_as_int(MARGIN_BOTTOM_KEY),
            self.get_as_int(MARGIN_LEFT_KEY),
        )
    }

    /// <upstream-comment>GetMarginTop returns the style's top margin. If no value is set 0 is returned.</upstream-comment>
    pub fn get_margin_top(&self) -> usize {
        self.get_as_int(MARGIN_TOP_KEY)
    }

    /// <upstream-comment>GetMarginRight returns the style's right margin. If no value is set 0 is returned.</upstream-comment>
    pub fn get_margin_right(&self) -> usize {
        self.get_as_int(MARGIN_RIGHT_KEY)
    }

    /// <upstream-comment>GetMarginBottom returns the style's bottom margin. If no value is set 0 is returned.</upstream-comment>
    pub fn get_margin_bottom(&self) -> usize {
        self.get_as_int(MARGIN_BOTTOM_KEY)
    }

    /// <upstream-comment>GetMarginLeft returns the style's left margin. If no value is set 0 is returned.</upstream-comment>
    pub fn get_margin_left(&self) -> usize {
        self.get_as_int(MARGIN_LEFT_KEY)
    }

    /// <upstream-comment>GetMarginChar returns the style's padding character. If no value is set a
    /// space is returned.</upstream-comment>
    pub fn get_margin_char(&self) -> char {
        if self.margin_char == '\0' {
            ' '
        } else {
            self.margin_char
        }
    }

    /// <upstream-comment>GetHorizontalMargins returns the style's left and right margins. Unset
    /// values are measured as 0.</upstream-comment>
    pub fn get_horizontal_margins(&self) -> usize {
        self.get_as_int(MARGIN_LEFT_KEY) + self.get_as_int(MARGIN_RIGHT_KEY)
    }

    /// <upstream-comment>GetVerticalMargins returns the style's top and bottom margins. Unset values
    /// are measured as 0.</upstream-comment>
    pub fn get_vertical_margins(&self) -> usize {
        self.get_as_int(MARGIN_TOP_KEY) + self.get_as_int(MARGIN_BOTTOM_KEY)
    }

    /// <upstream-comment>GetBorder returns the style's border style (type Border) and value for the
    /// top, right, bottom, and left in that order.</upstream-comment>
    pub fn get_border(&self) -> (Border, bool, bool, bool, bool) {
        (
            self.get_border_style(),
            self.get_as_bool(BORDER_TOP_KEY, false),
            self.get_as_bool(BORDER_RIGHT_KEY, false),
            self.get_as_bool(BORDER_BOTTOM_KEY, false),
            self.get_as_bool(BORDER_LEFT_KEY, false),
        )
    }

    /// <upstream-comment>GetBorderStyle returns the style's border style (type Border). If no value
    /// is set Border{} is returned.</upstream-comment>
    pub fn get_border_style(&self) -> Border {
        if !self.is_set(BORDER_STYLE_KEY) {
            Border::default()
        } else {
            self.border_style.clone()
        }
    }

    /// <upstream-comment>GetBorderTop returns the style's top border setting. If no value is set
    /// false is returned.</upstream-comment>
    pub fn get_border_top(&self) -> bool {
        self.get_as_bool(BORDER_TOP_KEY, false)
    }

    /// <upstream-comment>GetBorderRight returns the style's right border setting. If no value is set
    /// false is returned.</upstream-comment>
    pub fn get_border_right(&self) -> bool {
        self.get_as_bool(BORDER_RIGHT_KEY, false)
    }

    /// <upstream-comment>GetBorderBottom returns the style's bottom border setting. If no value is set
    /// false is returned.</upstream-comment>
    pub fn get_border_bottom(&self) -> bool {
        self.get_as_bool(BORDER_BOTTOM_KEY, false)
    }

    /// <upstream-comment>GetBorderLeft returns the style's left border setting. If no value is set
    /// false is returned.</upstream-comment>
    pub fn get_border_left(&self) -> bool {
        self.get_as_bool(BORDER_LEFT_KEY, false)
    }

    /// <upstream-comment>GetBorderTopForeground returns the style's border top foreground color.</upstream-comment>
    pub fn get_border_top_foreground(&self) -> Color {
        self.get_as_color(BORDER_TOP_FOREGROUND_KEY)
    }

    /// <upstream-comment>GetBorderRightForeground returns the style's border right foreground color.</upstream-comment>
    pub fn get_border_right_foreground(&self) -> Color {
        self.get_as_color(BORDER_RIGHT_FOREGROUND_KEY)
    }

    /// <upstream-comment>GetBorderBottomForeground returns the style's border bottom foreground color.</upstream-comment>
    pub fn get_border_bottom_foreground(&self) -> Color {
        self.get_as_color(BORDER_BOTTOM_FOREGROUND_KEY)
    }

    /// <upstream-comment>GetBorderLeftForeground returns the style's border left foreground color.</upstream-comment>
    pub fn get_border_left_foreground(&self) -> Color {
        self.get_as_color(BORDER_LEFT_FOREGROUND_KEY)
    }

    /// <upstream-comment>GetBorderForegroundBlend returns the style's border blend foreground colors.</upstream-comment>
    pub fn get_border_foreground_blend(&self) -> Vec<Color> {
        if self.is_set(BORDER_FOREGROUND_BLEND_KEY) {
            self.border_blend_fg_color.clone()
        } else {
            Vec::new()
        }
    }

    /// <upstream-comment>GetBorderForegroundBlendOffset returns the style's border blend offset.</upstream-comment>
    pub fn get_border_foreground_blend_offset(&self) -> isize {
        self.border_foreground_blend_offset
    }

    /// <upstream-comment>GetBorderTopBackground returns the style's border top background color.</upstream-comment>
    pub fn get_border_top_background(&self) -> Color {
        self.get_as_color(BORDER_TOP_BACKGROUND_KEY)
    }

    /// <upstream-comment>GetBorderRightBackground returns the style's border right background color.</upstream-comment>
    pub fn get_border_right_background(&self) -> Color {
        self.get_as_color(BORDER_RIGHT_BACKGROUND_KEY)
    }

    /// <upstream-comment>GetBorderBottomBackground returns the style's border bottom background color.</upstream-comment>
    pub fn get_border_bottom_background(&self) -> Color {
        self.get_as_color(BORDER_BOTTOM_BACKGROUND_KEY)
    }

    /// <upstream-comment>GetBorderLeftBackground returns the style's border left background color.</upstream-comment>
    pub fn get_border_left_background(&self) -> Color {
        self.get_as_color(BORDER_LEFT_BACKGROUND_KEY)
    }

    /// <upstream-comment>GetBorderTopSize returns the width of the top border.</upstream-comment>
    pub fn get_border_top_size(&self) -> usize {
        if self.is_border_style_set_without_sides() {
            return 1;
        }
        if !self.get_as_bool(BORDER_TOP_KEY, false) {
            return 0;
        }
        self.get_border_style().get_top_size()
    }

    /// <upstream-comment>GetBorderLeftSize returns the width of the left border.</upstream-comment>
    pub fn get_border_left_size(&self) -> usize {
        if self.is_border_style_set_without_sides() {
            return 1;
        }
        if !self.get_as_bool(BORDER_LEFT_KEY, false) {
            return 0;
        }
        self.get_border_style().get_left_size()
    }

    /// <upstream-comment>GetBorderBottomSize returns the width of the bottom border.</upstream-comment>
    pub fn get_border_bottom_size(&self) -> usize {
        if self.is_border_style_set_without_sides() {
            return 1;
        }
        if !self.get_as_bool(BORDER_BOTTOM_KEY, false) {
            return 0;
        }
        self.get_border_style().get_bottom_size()
    }

    /// <upstream-comment>GetBorderRightSize returns the width of the right border.</upstream-comment>
    pub fn get_border_right_size(&self) -> usize {
        if self.is_border_style_set_without_sides() {
            return 1;
        }
        if !self.get_as_bool(BORDER_RIGHT_KEY, false) {
            return 0;
        }
        self.get_border_style().get_right_size()
    }

    /// <upstream-comment>GetHorizontalBorderSize returns the width of the horizontal borders.</upstream-comment>
    pub fn get_horizontal_border_size(&self) -> usize {
        self.get_border_left_size() + self.get_border_right_size()
    }

    /// <upstream-comment>GetVerticalBorderSize returns the width of the vertical borders.</upstream-comment>
    pub fn get_vertical_border_size(&self) -> usize {
        self.get_border_top_size() + self.get_border_bottom_size()
    }

    /// <upstream-comment>GetInline returns the style's inline setting. If no value is set false is returned.</upstream-comment>
    pub fn get_inline(&self) -> bool {
        self.get_as_bool(INLINE_KEY, false)
    }

    /// <upstream-comment>GetMaxWidth returns the style's max width setting. If no value is set 0 is returned.</upstream-comment>
    pub fn get_max_width(&self) -> usize {
        self.get_as_int(MAX_WIDTH_KEY)
    }

    /// <upstream-comment>GetMaxHeight returns the style's max height setting. If no value is set 0 is returned.</upstream-comment>
    pub fn get_max_height(&self) -> usize {
        self.get_as_int(MAX_HEIGHT_KEY)
    }

    /// <upstream-comment>GetTabWidth returns the style's tab width setting. If no value is set 4 is
    /// returned which is the implicit default.</upstream-comment>
    ///
    /// NOTE: upstream's `getAsInt` returns 0 when unset (the `4` implicit
    /// default is applied only at render time in `maybeConvertTabs`), so this
    /// mirrors that: unset returns 0.
    pub fn get_tab_width(&self) -> isize {
        if self.is_set(TAB_WIDTH_KEY) {
            self.tab_width
        } else {
            0
        }
    }

    /// <upstream-comment>GetUnderlineSpaces returns whether or not the style is set to underline spaces.</upstream-comment>
    pub fn get_underline_spaces(&self) -> bool {
        self.get_as_bool(UNDERLINE_SPACES_KEY, false)
    }

    /// <upstream-comment>GetStrikethroughSpaces returns whether or not the style is set to strikethrough spaces.</upstream-comment>
    pub fn get_strikethrough_spaces(&self) -> bool {
        self.get_as_bool(STRIKETHROUGH_SPACES_KEY, false)
    }

    /// <upstream-comment>GetHorizontalFrameSize returns the sum of the style's horizontal margins, padding
    /// and border widths.</upstream-comment>
    pub fn get_horizontal_frame_size(&self) -> usize {
        self.get_horizontal_margins()
            + self.get_horizontal_padding()
            + self.get_horizontal_border_size()
    }

    /// <upstream-comment>GetVerticalFrameSize returns the sum of the style's vertical margins, padding
    /// and border widths.</upstream-comment>
    pub fn get_vertical_frame_size(&self) -> usize {
        self.get_vertical_margins() + self.get_vertical_padding() + self.get_vertical_border_size()
    }

    /// <upstream-comment>GetFrameSize returns the sum of the margins, padding and border width for
    /// both the horizontal and vertical margins.</upstream-comment>
    pub fn get_frame_size(&self) -> (usize, usize) {
        (
            self.get_horizontal_frame_size(),
            self.get_vertical_frame_size(),
        )
    }

    /// <upstream-comment>GetTransform returns the transform set on the style. If no transform is set
    /// nil is returned.</upstream-comment>
    pub fn get_transform(&self) -> Option<Transform> {
        if !self.is_set(TRANSFORM_KEY) {
            return None;
        }
        self.transform
    }

    /// <upstream-comment>GetHyperlink returns the hyperlink along with its parameters.</upstream-comment>
    pub fn get_hyperlink(&self) -> (String, String) {
        let mut link = String::new();
        let mut params = String::new();
        if self.is_set(LINK_KEY) {
            link = self.link.clone();
        }
        if self.is_set(LINK_PARAMS_KEY) {
            params = self.link_params.clone();
        }
        (link, params)
    }

    // ------------------------------------------------------------------
    // Unsets (matching `unset.go`)
    // ------------------------------------------------------------------

    /// <upstream-comment>UnsetBold removes the bold style rule, if set.</upstream-comment>
    pub fn unset_bold(mut self) -> Style {
        self.unset(BOLD_KEY);
        self
    }

    /// <upstream-comment>UnsetItalic removes the italic style rule, if set.</upstream-comment>
    pub fn unset_italic(mut self) -> Style {
        self.unset(ITALIC_KEY);
        self
    }

    /// <upstream-comment>UnsetUnderline removes the underline style rule, if set.</upstream-comment>
    pub fn unset_underline(self) -> Style {
        self.underline(false)
    }

    /// <upstream-comment>UnsetStrikethrough removes the strikethrough style rule, if set.</upstream-comment>
    pub fn unset_strikethrough(mut self) -> Style {
        self.unset(STRIKETHROUGH_KEY);
        self
    }

    /// <upstream-comment>UnsetReverse removes the reverse style rule, if set.</upstream-comment>
    pub fn unset_reverse(mut self) -> Style {
        self.unset(REVERSE_KEY);
        self
    }

    /// <upstream-comment>UnsetBlink removes the blink style rule, if set.</upstream-comment>
    pub fn unset_blink(mut self) -> Style {
        self.unset(BLINK_KEY);
        self
    }

    /// <upstream-comment>UnsetFaint removes the faint style rule, if set.</upstream-comment>
    pub fn unset_faint(mut self) -> Style {
        self.unset(FAINT_KEY);
        self
    }

    /// <upstream-comment>UnsetForeground removes the foreground style rule, if set.</upstream-comment>
    pub fn unset_foreground(mut self) -> Style {
        self.unset(FOREGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBackground removes the background style rule, if set.</upstream-comment>
    pub fn unset_background(mut self) -> Style {
        self.unset(BACKGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetWidth removes the width style rule, if set.</upstream-comment>
    pub fn unset_width(mut self) -> Style {
        self.unset(WIDTH_KEY);
        self
    }

    /// <upstream-comment>UnsetHeight removes the height style rule, if set.</upstream-comment>
    pub fn unset_height(mut self) -> Style {
        self.unset(HEIGHT_KEY);
        self
    }

    /// <upstream-comment>UnsetAlign removes the horizontal and vertical text alignment style rule, if set.</upstream-comment>
    pub fn unset_align(mut self) -> Style {
        self.unset(ALIGN_HORIZONTAL_KEY);
        self.unset(ALIGN_VERTICAL_KEY);
        self
    }

    /// <upstream-comment>UnsetAlignHorizontal removes the horizontal text alignment style rule, if set.</upstream-comment>
    pub fn unset_align_horizontal(mut self) -> Style {
        self.unset(ALIGN_HORIZONTAL_KEY);
        self
    }

    /// <upstream-comment>UnsetAlignVertical removes the vertical text alignment style rule, if set.</upstream-comment>
    pub fn unset_align_vertical(mut self) -> Style {
        self.unset(ALIGN_VERTICAL_KEY);
        self
    }

    /// <upstream-comment>UnsetPadding removes all padding style rules.</upstream-comment>
    pub fn unset_padding(mut self) -> Style {
        self.unset(PADDING_LEFT_KEY);
        self.unset(PADDING_RIGHT_KEY);
        self.unset(PADDING_TOP_KEY);
        self.unset(PADDING_BOTTOM_KEY);
        self.unset(PADDING_CHAR_KEY);
        self
    }

    /// <upstream-comment>UnsetPaddingChar removes the padding character style rule, if set.</upstream-comment>
    pub fn unset_padding_char(mut self) -> Style {
        self.unset(PADDING_CHAR_KEY);
        self
    }

    /// <upstream-comment>UnsetPaddingLeft removes the left padding style rule, if set.</upstream-comment>
    pub fn unset_padding_left(mut self) -> Style {
        self.unset(PADDING_LEFT_KEY);
        self
    }

    /// <upstream-comment>UnsetPaddingRight removes the right padding style rule, if set.</upstream-comment>
    pub fn unset_padding_right(mut self) -> Style {
        self.unset(PADDING_RIGHT_KEY);
        self
    }

    /// <upstream-comment>UnsetPaddingTop removes the top padding style rule, if set.</upstream-comment>
    pub fn unset_padding_top(mut self) -> Style {
        self.unset(PADDING_TOP_KEY);
        self
    }

    /// <upstream-comment>UnsetPaddingBottom removes the bottom padding style rule, if set.</upstream-comment>
    pub fn unset_padding_bottom(mut self) -> Style {
        self.unset(PADDING_BOTTOM_KEY);
        self
    }

    /// <upstream-comment>UnsetColorWhitespace removes the rule for coloring padding, if set.</upstream-comment>
    pub fn unset_color_whitespace(mut self) -> Style {
        self.unset(COLOR_WHITESPACE_KEY);
        self
    }

    /// <upstream-comment>UnsetMargins removes all margin style rules.</upstream-comment>
    pub fn unset_margins(mut self) -> Style {
        self.unset(MARGIN_LEFT_KEY);
        self.unset(MARGIN_RIGHT_KEY);
        self.unset(MARGIN_TOP_KEY);
        self.unset(MARGIN_BOTTOM_KEY);
        self
    }

    /// <upstream-comment>UnsetMarginLeft removes the left margin style rule, if set.</upstream-comment>
    pub fn unset_margin_left(mut self) -> Style {
        self.unset(MARGIN_LEFT_KEY);
        self
    }

    /// <upstream-comment>UnsetMarginRight removes the right margin style rule, if set.</upstream-comment>
    pub fn unset_margin_right(mut self) -> Style {
        self.unset(MARGIN_RIGHT_KEY);
        self
    }

    /// <upstream-comment>UnsetMarginTop removes the top margin style rule, if set.</upstream-comment>
    pub fn unset_margin_top(mut self) -> Style {
        self.unset(MARGIN_TOP_KEY);
        self
    }

    /// <upstream-comment>UnsetMarginBottom removes the bottom margin style rule, if set.</upstream-comment>
    pub fn unset_margin_bottom(mut self) -> Style {
        self.unset(MARGIN_BOTTOM_KEY);
        self
    }

    /// <upstream-comment>UnsetMarginBackground removes the margin's background color.</upstream-comment>
    pub fn unset_margin_background(mut self) -> Style {
        self.unset(MARGIN_BACKGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderStyle removes the border style rule, if set.</upstream-comment>
    pub fn unset_border_style(mut self) -> Style {
        self.unset(BORDER_STYLE_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderTop removes the border top style rule, if set.</upstream-comment>
    pub fn unset_border_top(mut self) -> Style {
        self.unset(BORDER_TOP_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderRight removes the border right style rule, if set.</upstream-comment>
    pub fn unset_border_right(mut self) -> Style {
        self.unset(BORDER_RIGHT_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderBottom removes the border bottom style rule, if set.</upstream-comment>
    pub fn unset_border_bottom(mut self) -> Style {
        self.unset(BORDER_BOTTOM_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderLeft removes the border left style rule, if set.</upstream-comment>
    pub fn unset_border_left(mut self) -> Style {
        self.unset(BORDER_LEFT_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderForeground removes all border foreground color styles, if set.</upstream-comment>
    pub fn unset_border_foreground(mut self) -> Style {
        self.unset(BORDER_TOP_FOREGROUND_KEY);
        self.unset(BORDER_RIGHT_FOREGROUND_KEY);
        self.unset(BORDER_BOTTOM_FOREGROUND_KEY);
        self.unset(BORDER_LEFT_FOREGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderTopForeground removes the top border foreground color rule, if set.</upstream-comment>
    pub fn unset_border_top_foreground(mut self) -> Style {
        self.unset(BORDER_TOP_FOREGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderRightForeground removes the right border foreground color rule, if set.</upstream-comment>
    pub fn unset_border_right_foreground(mut self) -> Style {
        self.unset(BORDER_RIGHT_FOREGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderBottomForeground removes the bottom border foreground color rule, if set.</upstream-comment>
    pub fn unset_border_bottom_foreground(mut self) -> Style {
        self.unset(BORDER_BOTTOM_FOREGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderLeftForeground removes the left border foreground color rule, if set.</upstream-comment>
    pub fn unset_border_left_foreground(mut self) -> Style {
        self.unset(BORDER_LEFT_FOREGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderForegroundBlend removes the border blend foreground color rules, if set.</upstream-comment>
    pub fn unset_border_foreground_blend(mut self) -> Style {
        self.unset(BORDER_FOREGROUND_BLEND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderForegroundBlendOffset removes the border blend offset style rule, if set.</upstream-comment>
    pub fn unset_border_foreground_blend_offset(mut self) -> Style {
        self.unset(BORDER_FOREGROUND_BLEND_OFFSET_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderBackground removes all border background color styles, if set.</upstream-comment>
    pub fn unset_border_background(mut self) -> Style {
        self.unset(BORDER_TOP_BACKGROUND_KEY);
        self.unset(BORDER_RIGHT_BACKGROUND_KEY);
        self.unset(BORDER_BOTTOM_BACKGROUND_KEY);
        self.unset(BORDER_LEFT_BACKGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderTopBackground removes the top border background color rule, if set.</upstream-comment>
    pub fn unset_border_top_background(mut self) -> Style {
        self.unset(BORDER_TOP_BACKGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderRightBackground removes the right border background color rule, if set.</upstream-comment>
    pub fn unset_border_right_background(mut self) -> Style {
        self.unset(BORDER_RIGHT_BACKGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderBottomBackground removes the bottom border background color rule, if set.</upstream-comment>
    pub fn unset_border_bottom_background(mut self) -> Style {
        self.unset(BORDER_BOTTOM_BACKGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetBorderLeftBackground removes the left border color rule, if set.</upstream-comment>
    pub fn unset_border_left_background(mut self) -> Style {
        self.unset(BORDER_LEFT_BACKGROUND_KEY);
        self
    }

    /// <upstream-comment>UnsetInline removes the inline style rule, if set.</upstream-comment>
    pub fn unset_inline(mut self) -> Style {
        self.unset(INLINE_KEY);
        self
    }

    /// <upstream-comment>UnsetMaxWidth removes the max width style rule, if set.</upstream-comment>
    pub fn unset_max_width(mut self) -> Style {
        self.unset(MAX_WIDTH_KEY);
        self
    }

    /// <upstream-comment>UnsetMaxHeight removes the max height style rule, if set.</upstream-comment>
    pub fn unset_max_height(mut self) -> Style {
        self.unset(MAX_HEIGHT_KEY);
        self
    }

    /// <upstream-comment>UnsetTabWidth removes the tab width style rule, if set.</upstream-comment>
    pub fn unset_tab_width(mut self) -> Style {
        self.unset(TAB_WIDTH_KEY);
        self
    }

    /// <upstream-comment>UnsetUnderlineSpaces removes the value set by UnderlineSpaces.</upstream-comment>
    pub fn unset_underline_spaces(mut self) -> Style {
        self.unset(UNDERLINE_SPACES_KEY);
        self
    }

    /// <upstream-comment>UnsetStrikethroughSpaces removes the value set by StrikethroughSpaces.</upstream-comment>
    pub fn unset_strikethrough_spaces(mut self) -> Style {
        self.unset(STRIKETHROUGH_SPACES_KEY);
        self
    }

    /// <upstream-comment>UnsetTransform removes the value set by Transform.</upstream-comment>
    pub fn unset_transform(mut self) -> Style {
        self.unset(TRANSFORM_KEY);
        self
    }

    /// <upstream-comment>UnsetHyperlink removes the value set by Hyperlink.</upstream-comment>
    pub fn unset_hyperlink(mut self) -> Style {
        self.unset(LINK_KEY);
        self.unset(LINK_PARAMS_KEY);
        self.link = String::new();
        self.link_params = String::new();
        self
    }

    /// <upstream-comment>UnsetString sets the underlying string value to the empty string.</upstream-comment>
    pub fn unset_string(mut self) -> Style {
        self.value = String::new();
        self
    }

    // ------------------------------------------------------------------
    // Internal prop plumbing
    // ------------------------------------------------------------------

    fn is_set(&self, k: Props) -> bool {
        self.props & k != 0
    }

    fn set(&mut self, key: Props, value: Value) {
        match key {
            FOREGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.fg_color = c;
                }
            }
            BACKGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.bg_color = c;
                }
            }
            UNDERLINE_COLOR_KEY => {
                if let Value::Color(c) = value {
                    self.ul_color = c;
                }
            }
            UNDERLINE_KEY => {
                if let Value::Underline(u) = value {
                    self.ul = u;
                }
            }
            WIDTH_KEY => {
                if let Value::Int(i) = value {
                    self.width = i;
                }
            }
            HEIGHT_KEY => {
                if let Value::Int(i) = value {
                    self.height = i;
                }
            }
            ALIGN_HORIZONTAL_KEY => {
                if let Value::Position(p) = value {
                    self.align_horizontal = p;
                }
            }
            ALIGN_VERTICAL_KEY => {
                if let Value::Position(p) = value {
                    self.align_vertical = p;
                }
            }
            PADDING_TOP_KEY => {
                if let Value::Int(i) = value {
                    self.padding_top = i;
                }
            }
            PADDING_RIGHT_KEY => {
                if let Value::Int(i) = value {
                    self.padding_right = i;
                }
            }
            PADDING_BOTTOM_KEY => {
                if let Value::Int(i) = value {
                    self.padding_bottom = i;
                }
            }
            PADDING_LEFT_KEY => {
                if let Value::Int(i) = value {
                    self.padding_left = i;
                }
            }
            PADDING_CHAR_KEY => {
                if let Value::Char(c) = value {
                    self.padding_char = c;
                }
            }
            MARGIN_TOP_KEY => {
                if let Value::Int(i) = value {
                    self.margin_top = i;
                }
            }
            MARGIN_RIGHT_KEY => {
                if let Value::Int(i) = value {
                    self.margin_right = i;
                }
            }
            MARGIN_BOTTOM_KEY => {
                if let Value::Int(i) = value {
                    self.margin_bottom = i;
                }
            }
            MARGIN_LEFT_KEY => {
                if let Value::Int(i) = value {
                    self.margin_left = i;
                }
            }
            MARGIN_BACKGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.margin_bg_color = c;
                }
            }
            MARGIN_CHAR_KEY => {
                if let Value::Char(c) = value {
                    self.margin_char = c;
                }
            }
            BORDER_STYLE_KEY => {
                if let Value::Border(b) = value {
                    self.border_style = *b;
                }
            }
            BORDER_TOP_FOREGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.border_top_fg_color = c;
                }
            }
            BORDER_RIGHT_FOREGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.border_right_fg_color = c;
                }
            }
            BORDER_BOTTOM_FOREGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.border_bottom_fg_color = c;
                }
            }
            BORDER_LEFT_FOREGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.border_left_fg_color = c;
                }
            }
            BORDER_FOREGROUND_BLEND_KEY => {
                if let Value::Colors(c) = value {
                    self.border_blend_fg_color = c;
                }
            }
            BORDER_FOREGROUND_BLEND_OFFSET_KEY => {
                if let Value::Int(i) = value {
                    self.border_foreground_blend_offset = i as isize;
                }
            }
            BORDER_TOP_BACKGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.border_top_bg_color = c;
                }
            }
            BORDER_RIGHT_BACKGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.border_right_bg_color = c;
                }
            }
            BORDER_BOTTOM_BACKGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.border_bottom_bg_color = c;
                }
            }
            BORDER_LEFT_BACKGROUND_KEY => {
                if let Value::Color(c) = value {
                    self.border_left_bg_color = c;
                }
            }
            MAX_WIDTH_KEY => {
                if let Value::Int(i) = value {
                    self.max_width = i;
                }
            }
            MAX_HEIGHT_KEY => {
                if let Value::Int(i) = value {
                    self.max_height = i;
                }
            }
            TAB_WIDTH_KEY => {
                if let Value::Int(i) = value {
                    self.tab_width = i as isize;
                }
            }
            TRANSFORM_KEY => {
                if let Value::Transform(f) = value {
                    self.transform = Some(f);
                }
            }
            LINK_KEY => {
                if let Value::String(s) = value {
                    self.link = s;
                }
            }
            LINK_PARAMS_KEY => {
                if let Value::String(s) = value {
                    self.link_params = s;
                }
            }
            _ => {
                // Boolean props.
                match value {
                    Value::Bool(v) => {
                        if v {
                            self.attrs |= key;
                        } else {
                            self.attrs &= !key;
                        }
                    }
                    Value::Attrs(a) => {
                        if a & key != 0 {
                            self.attrs |= key;
                        } else {
                            self.attrs &= !key;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Set the prop on.
        self.props |= key;
    }

    fn set_from(&mut self, key: Props, i: &Style) {
        match key {
            FOREGROUND_KEY => {
                self.set(FOREGROUND_KEY, Value::Color(i.fg_color.clone()));
            }
            BACKGROUND_KEY => {
                self.set(BACKGROUND_KEY, Value::Color(i.bg_color.clone()));
            }
            UNDERLINE_COLOR_KEY => {
                self.set(UNDERLINE_COLOR_KEY, Value::Color(i.ul_color.clone()));
            }
            UNDERLINE_KEY => {
                self.set(UNDERLINE_KEY, Value::Underline(i.ul));
            }
            WIDTH_KEY => {
                self.set(WIDTH_KEY, Value::Int(i.width));
            }
            HEIGHT_KEY => {
                self.set(HEIGHT_KEY, Value::Int(i.height));
            }
            ALIGN_HORIZONTAL_KEY => {
                self.set(ALIGN_HORIZONTAL_KEY, Value::Position(i.align_horizontal));
            }
            ALIGN_VERTICAL_KEY => {
                self.set(ALIGN_VERTICAL_KEY, Value::Position(i.align_vertical));
            }
            PADDING_TOP_KEY => {
                self.set(PADDING_TOP_KEY, Value::Int(i.padding_top));
            }
            PADDING_RIGHT_KEY => {
                self.set(PADDING_RIGHT_KEY, Value::Int(i.padding_right));
            }
            PADDING_BOTTOM_KEY => {
                self.set(PADDING_BOTTOM_KEY, Value::Int(i.padding_bottom));
            }
            PADDING_LEFT_KEY => {
                self.set(PADDING_LEFT_KEY, Value::Int(i.padding_left));
            }
            PADDING_CHAR_KEY => {
                self.set(PADDING_CHAR_KEY, Value::Char(i.padding_char));
            }
            MARGIN_TOP_KEY => {
                self.set(MARGIN_TOP_KEY, Value::Int(i.margin_top));
            }
            MARGIN_RIGHT_KEY => {
                self.set(MARGIN_RIGHT_KEY, Value::Int(i.margin_right));
            }
            MARGIN_BOTTOM_KEY => {
                self.set(MARGIN_BOTTOM_KEY, Value::Int(i.margin_bottom));
            }
            MARGIN_LEFT_KEY => {
                self.set(MARGIN_LEFT_KEY, Value::Int(i.margin_left));
            }
            MARGIN_BACKGROUND_KEY => {
                self.set(
                    MARGIN_BACKGROUND_KEY,
                    Value::Color(i.margin_bg_color.clone()),
                );
            }
            MARGIN_CHAR_KEY => {
                self.set(MARGIN_CHAR_KEY, Value::Char(i.margin_char));
            }
            BORDER_STYLE_KEY => {
                self.set(
                    BORDER_STYLE_KEY,
                    Value::Border(Box::new(i.border_style.clone())),
                );
            }
            BORDER_TOP_FOREGROUND_KEY => {
                self.set(
                    BORDER_TOP_FOREGROUND_KEY,
                    Value::Color(i.border_top_fg_color.clone()),
                );
            }
            BORDER_RIGHT_FOREGROUND_KEY => {
                self.set(
                    BORDER_RIGHT_FOREGROUND_KEY,
                    Value::Color(i.border_right_fg_color.clone()),
                );
            }
            BORDER_BOTTOM_FOREGROUND_KEY => {
                self.set(
                    BORDER_BOTTOM_FOREGROUND_KEY,
                    Value::Color(i.border_bottom_fg_color.clone()),
                );
            }
            BORDER_LEFT_FOREGROUND_KEY => {
                self.set(
                    BORDER_LEFT_FOREGROUND_KEY,
                    Value::Color(i.border_left_fg_color.clone()),
                );
            }
            BORDER_FOREGROUND_BLEND_KEY => {
                self.set(
                    BORDER_FOREGROUND_BLEND_KEY,
                    Value::Colors(i.border_blend_fg_color.clone()),
                );
            }
            BORDER_FOREGROUND_BLEND_OFFSET_KEY => {
                self.set(
                    BORDER_FOREGROUND_BLEND_OFFSET_KEY,
                    Value::Int(i.border_foreground_blend_offset as usize),
                );
            }
            BORDER_TOP_BACKGROUND_KEY => {
                self.set(
                    BORDER_TOP_BACKGROUND_KEY,
                    Value::Color(i.border_top_bg_color.clone()),
                );
            }
            BORDER_RIGHT_BACKGROUND_KEY => {
                self.set(
                    BORDER_RIGHT_BACKGROUND_KEY,
                    Value::Color(i.border_right_bg_color.clone()),
                );
            }
            BORDER_BOTTOM_BACKGROUND_KEY => {
                self.set(
                    BORDER_BOTTOM_BACKGROUND_KEY,
                    Value::Color(i.border_bottom_bg_color.clone()),
                );
            }
            BORDER_LEFT_BACKGROUND_KEY => {
                self.set(
                    BORDER_LEFT_BACKGROUND_KEY,
                    Value::Color(i.border_left_bg_color.clone()),
                );
            }
            MAX_WIDTH_KEY => {
                self.set(MAX_WIDTH_KEY, Value::Int(i.max_width));
            }
            MAX_HEIGHT_KEY => {
                self.set(MAX_HEIGHT_KEY, Value::Int(i.max_height));
            }
            TAB_WIDTH_KEY => {
                self.set(TAB_WIDTH_KEY, Value::Int(i.tab_width as usize));
            }
            TRANSFORM_KEY => {
                self.set(
                    TRANSFORM_KEY,
                    Value::Transform(i.transform.unwrap_or(|s| s.to_string())),
                );
            }
            _ => {
                // Set attributes for set bool properties.
                self.set(key, Value::Attrs(i.attrs));
            }
        }
    }

    fn unset(&mut self, key: Props) {
        self.props &= !key;
    }

    fn get_as_bool(&self, k: Props, default_val: bool) -> bool {
        if !self.is_set(k) {
            return default_val;
        }
        self.attrs & k != 0
    }

    fn get_as_int(&self, k: Props) -> usize {
        if !self.is_set(k) {
            return 0;
        }
        match k {
            WIDTH_KEY => self.width,
            HEIGHT_KEY => self.height,
            PADDING_TOP_KEY => self.padding_top,
            PADDING_RIGHT_KEY => self.padding_right,
            PADDING_BOTTOM_KEY => self.padding_bottom,
            PADDING_LEFT_KEY => self.padding_left,
            MARGIN_TOP_KEY => self.margin_top,
            MARGIN_RIGHT_KEY => self.margin_right,
            MARGIN_BOTTOM_KEY => self.margin_bottom,
            MARGIN_LEFT_KEY => self.margin_left,
            BORDER_FOREGROUND_BLEND_OFFSET_KEY => self.border_foreground_blend_offset as usize,
            MAX_WIDTH_KEY => self.max_width,
            MAX_HEIGHT_KEY => self.max_height,
            TAB_WIDTH_KEY => self.tab_width as usize,
            _ => 0,
        }
    }

    fn get_as_color(&self, k: Props) -> Color {
        if !self.is_set(k) {
            return Color::NoColor;
        }
        let c: Option<Color> = match k {
            FOREGROUND_KEY => self.fg_color.clone(),
            BACKGROUND_KEY => self.bg_color.clone(),
            MARGIN_BACKGROUND_KEY => self.margin_bg_color.clone(),
            BORDER_TOP_FOREGROUND_KEY => self.border_top_fg_color.clone(),
            BORDER_RIGHT_FOREGROUND_KEY => self.border_right_fg_color.clone(),
            BORDER_BOTTOM_FOREGROUND_KEY => self.border_bottom_fg_color.clone(),
            BORDER_LEFT_FOREGROUND_KEY => self.border_left_fg_color.clone(),
            BORDER_TOP_BACKGROUND_KEY => self.border_top_bg_color.clone(),
            BORDER_RIGHT_BACKGROUND_KEY => self.border_right_bg_color.clone(),
            BORDER_BOTTOM_BACKGROUND_KEY => self.border_bottom_bg_color.clone(),
            BORDER_LEFT_BACKGROUND_KEY => self.border_left_bg_color.clone(),
            UNDERLINE_COLOR_KEY => self.ul_color.clone(),
            _ => None,
        };
        c.unwrap_or(Color::NoColor)
    }

    fn get_as_position(&self, k: Props) -> Position {
        if !self.is_set(k) {
            return Position(0.0);
        }
        match k {
            ALIGN_HORIZONTAL_KEY => self.align_horizontal,
            ALIGN_VERTICAL_KEY => self.align_vertical,
            _ => Position(0.0),
        }
    }

    fn get_as_transform(&self, k: Props) -> Option<Transform> {
        if !self.is_set(k) {
            return None;
        }
        self.transform
    }

    fn get_border_style_impl(&self) -> Border {
        if !self.is_set(BORDER_STYLE_KEY) {
            Border::default()
        } else {
            self.border_style.clone()
        }
    }

    fn is_border_style_set_without_sides(&self) -> bool {
        let border = self.get_border_style_impl();
        let top_set = self.is_set(BORDER_TOP_KEY);
        let right_set = self.is_set(BORDER_RIGHT_KEY);
        let bottom_set = self.is_set(BORDER_BOTTOM_KEY);
        let left_set = self.is_set(BORDER_LEFT_KEY);
        border != Border::default() && !(top_set || right_set || bottom_set || left_set)
    }

    // ------------------------------------------------------------------
    // Border rendering (matching `borders.go` applyBorder)
    // ------------------------------------------------------------------

    fn apply_border(&self, str: &str) -> String {
        let mut border = self.get_border_style_impl();
        let mut has_top = self.get_as_bool(BORDER_TOP_KEY, false);
        let mut has_right = self.get_as_bool(BORDER_RIGHT_KEY, false);
        let mut has_bottom = self.get_as_bool(BORDER_BOTTOM_KEY, false);
        let mut has_left = self.get_as_bool(BORDER_LEFT_KEY, false);

        // If a border is set and no sides have been specifically turned on or off
        // render borders on all sides.
        if self.is_border_style_set_without_sides() {
            has_top = true;
            has_right = true;
            has_bottom = true;
            has_left = true;
        }

        // If no border is set or all borders have been disabled, abort.
        if border == Border::default() || (!has_top && !has_right && !has_bottom && !has_left) {
            return str.to_string();
        }

        let (lines, mut width) = get_lines(str);

        if has_left {
            if border.left.is_empty() {
                border.left = " ".to_string();
            }
            width += border::max_rune_width(&border.left);
        }

        if has_right {
            if border.right.is_empty() {
                border.right = " ".to_string();
            }
            width += border::max_rune_width(&border.right);
        }

        // If corners should be rendered but are set with the empty string, fill them
        // with a single space.
        if has_top && has_left && border.top_left.is_empty() {
            border.top_left = " ".to_string();
        }
        if has_top && has_right && border.top_right.is_empty() {
            border.top_right = " ".to_string();
        }
        if has_bottom && has_left && border.bottom_left.is_empty() {
            border.bottom_left = " ".to_string();
        }
        if has_bottom && has_right && border.bottom_right.is_empty() {
            border.bottom_right = " ".to_string();
        }

        // Figure out which corners we should actually be using based on which
        // sides are set to show.
        if has_top {
            match (has_left, has_right) {
                (false, false) => {
                    border.top_left = String::new();
                    border.top_right = String::new();
                }
                (false, true) => {
                    border.top_left = String::new();
                }
                (true, false) => {
                    border.top_right = String::new();
                }
                _ => {}
            }
        }
        if has_bottom {
            match (has_left, has_right) {
                (false, false) => {
                    border.bottom_left = String::new();
                    border.bottom_right = String::new();
                }
                (false, true) => {
                    border.bottom_left = String::new();
                }
                (true, false) => {
                    border.bottom_right = String::new();
                }
                _ => {}
            }
        }

        // For now, limit corners to one rune.
        border.top_left = border::get_first_rune_as_string(&border.top_left).to_string();
        border.top_right = border::get_first_rune_as_string(&border.top_right).to_string();
        border.bottom_right = border::get_first_rune_as_string(&border.bottom_right).to_string();
        border.bottom_left = border::get_first_rune_as_string(&border.bottom_left).to_string();

        let blend_fg = self.get_border_foreground_blend();
        let top_bg = self.get_as_color(BORDER_TOP_BACKGROUND_KEY);
        let right_bg = self.get_as_color(BORDER_RIGHT_BACKGROUND_KEY);
        let bottom_bg = self.get_as_color(BORDER_BOTTOM_BACKGROUND_KEY);
        let left_bg = self.get_as_color(BORDER_LEFT_BACKGROUND_KEY);

        let blend: Option<crate::border::BorderBlend> = if !blend_fg.is_empty() {
            Some(self.border_blend(width, lines.len(), &blend_fg))
        } else {
            None
        };

        let top_fg = self.get_as_color(BORDER_TOP_FOREGROUND_KEY);
        let right_fg = self.get_as_color(BORDER_RIGHT_FOREGROUND_KEY);
        let bottom_fg = self.get_as_color(BORDER_BOTTOM_FOREGROUND_KEY);
        let left_fg = self.get_as_color(BORDER_LEFT_FOREGROUND_KEY);

        let mut out = String::new();

        // Render top.
        if has_top {
            let top = border::render_horizontal_edge(
                &border.top_left,
                &border.top,
                &border.top_right,
                width,
            );
            if let Some(ref b) = blend {
                out.push_str(&style_border_blend(&top, &b.top_gradient, &top_bg));
            } else {
                out.push_str(&style_border(&top, &top_fg, &top_bg));
            }
            out.push('\n');
        }

        let left_runes: Vec<char> = border.left.chars().collect();
        let mut left_index = 0usize;
        let right_runes: Vec<char> = border.right.chars().collect();
        let mut right_index = 0usize;

        // Render sides.
        for (i, l) in lines.iter().enumerate() {
            if has_left {
                let r = left_runes[left_index].to_string();
                left_index += 1;
                if left_index >= left_runes.len() {
                    left_index = 0;
                }
                if let Some(ref b) = blend {
                    out.push_str(&style_border(&r, &b.left_gradient[i], &left_bg));
                } else {
                    out.push_str(&style_border(&r, &left_fg, &left_bg));
                }
            }
            out.push_str(l);
            if has_right {
                let r = right_runes[right_index].to_string();
                right_index += 1;
                if right_index >= right_runes.len() {
                    right_index = 0;
                }
                if let Some(ref b) = blend {
                    out.push_str(&style_border(&r, &b.right_gradient[i], &right_bg));
                } else {
                    out.push_str(&style_border(&r, &right_fg, &right_bg));
                }
            }
            if i < lines.len() - 1 {
                out.push('\n');
            }
        }

        // Render bottom.
        if has_bottom {
            let bottom = border::render_horizontal_edge(
                &border.bottom_left,
                &border.bottom,
                &border.bottom_right,
                width,
            );
            out.push('\n');
            if let Some(ref b) = blend {
                out.push_str(&style_border_blend(&bottom, &b.bottom_gradient, &bottom_bg));
            } else {
                out.push_str(&style_border(&bottom, &bottom_fg, &bottom_bg));
            }
        }

        out
    }

    fn border_blend(
        &self,
        width: usize,
        height: usize,
        colors: &[Color],
    ) -> crate::border::BorderBlend {
        crate::border::BorderBlend::new(width, height, colors, self.border_foreground_blend_offset)
    }
}

/// Value is an internal enum used to carry set values into `Style::set`.
enum Value {
    Bool(bool),
    Int(usize),
    Char(char),
    Color(Option<Color>),
    Colors(Vec<Color>),
    Border(Box<Border>),
    Position(Position),
    Underline(Underline),
    String(String),
    Transform(Transform),
    Attrs(Props),
}

fn which_sides_int(i: &[usize]) -> Option<(usize, usize, usize, usize)> {
    match i.len() {
        1 => Some((i[0], i[0], i[0], i[0])),
        2 => Some((i[0], i[1], i[0], i[1])),
        3 => Some((i[0], i[1], i[2], i[1])),
        4 => Some((i[0], i[1], i[2], i[3])),
        _ => None,
    }
}

fn which_sides_bool(i: &[bool]) -> (bool, bool, bool, bool, bool) {
    match i.len() {
        1 => (i[0], i[0], i[0], i[0], true),
        2 => (i[0], i[1], i[0], i[1], true),
        3 => (i[0], i[1], i[2], i[1], true),
        4 => (i[0], i[1], i[2], i[3], true),
        _ => (false, false, false, false, false),
    }
}

fn which_sides_color(i: &[Color]) -> Option<(Color, Color, Color, Color)> {
    match i.len() {
        1 => Some((i[0].clone(), i[0].clone(), i[0].clone(), i[0].clone())),
        2 => Some((i[0].clone(), i[1].clone(), i[0].clone(), i[1].clone())),
        3 => Some((i[0].clone(), i[1].clone(), i[2].clone(), i[1].clone())),
        4 => Some((i[0].clone(), i[1].clone(), i[2].clone(), i[3].clone())),
        _ => None,
    }
}

// Apply left padding.
fn pad_left(str: &str, n: usize, style: Option<&ansi::Style>, r: char) -> String {
    pad(str, -(n as isize), style, r)
}

// Apply right padding.
fn pad_right(str: &str, n: usize, style: Option<&ansi::Style>, r: char) -> String {
    pad(str, n as isize, style, r)
}

/// pad adds padding to either the left or right side of a string.
/// Positive values add to the right side while negative values
/// add to the left side.
/// r is the rune to use for padding. We use " " for margins and
/// "\u00A0" for padding so that the padding is preserved when the
/// string is copied and pasted.
fn pad(str: &str, n: isize, style: Option<&ansi::Style>, r: char) -> String {
    if n == 0 {
        return str.to_string();
    }

    let sp = match style {
        Some(s) => s.styled(&r.to_string().repeat(n.unsigned_abs())),
        None => r.to_string().repeat(n.unsigned_abs()),
    };

    let mut b = String::new();
    let mut is_first = true;
    for line in str.split('\n') {
        if !is_first {
            b.push('\n');
        }
        is_first = false;
        if n > 0 {
            // pad right
            b.push_str(line);
            b.push_str(&sp);
        } else {
            // pad left
            b.push_str(&sp);
            b.push_str(line);
        }
    }

    b
}

/// style_border applies foreground and background styling to a border.
fn style_border(border: &str, fg: &Color, bg: &Color) -> String {
    if fg.is_no_color() && bg.is_no_color() {
        return border.to_string();
    }
    let mut style = ansi::Style::default();
    if !fg.is_no_color() {
        style.fg_color = Some(fg.clone());
    }
    if !bg.is_no_color() {
        style.bg_color = Some(bg.clone());
    }
    style.styled(border)
}

/// style_border_blend applies foreground and background styling to a border,
/// using blending.
fn style_border_blend(border: &str, fg: &[Color], bg: &Color) -> String {
    let mut out = String::new();
    for (i, g) in unicode_segmentation::UnicodeSegmentation::graphemes(border, true).enumerate() {
        let mut style = ansi::Style::default();
        if !fg[i].is_no_color() {
            style.fg_color = Some(fg[i].clone());
        }
        if !bg.is_no_color() {
            style.bg_color = Some(bg.clone());
        }
        out.push_str(&style.string());
        out.push_str(g);
    }
    out.push_str(ansi::RESET_STYLE);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold_render() {
        let s = Style::new().bold(true);
        assert_eq!(s.render("hello"), "\x1b[1mhello\x1b[m");
    }

    #[test]
    fn test_italic_render() {
        let s = Style::new().italic(true);
        assert_eq!(s.render("hello"), "\x1b[3mhello\x1b[m");
    }

    #[test]
    fn test_fg_render() {
        let s = Style::new().foreground("#5A56E0");
        assert_eq!(s.render("hello"), "\x1b[38;2;90;86;224mhello\x1b[m");
    }

    #[test]
    fn test_underline_render() {
        let s = Style::new().underline(true);
        assert_eq!(
            s.render("hello"),
            "\x1b[4;4mh\x1b[m\x1b[4;4me\x1b[m\x1b[4;4ml\x1b[m\x1b[4;4ml\x1b[m\x1b[4;4mo\x1b[m"
        );
    }

    #[test]
    fn test_blink_faint() {
        assert_eq!(
            Style::new().blink(true).render("hello"),
            "\x1b[5mhello\x1b[m"
        );
        assert_eq!(
            Style::new().faint(true).render("hello"),
            "\x1b[2mhello\x1b[m"
        );
    }

    #[test]
    fn test_tab_conversion() {
        assert_eq!(Style::new().render("[\t]"), "[    ]");
        assert_eq!(Style::new().tab_width(2).render("[\t]"), "[  ]");
        assert_eq!(Style::new().tab_width(0).render("[\t]"), "[]");
        assert_eq!(Style::new().tab_width(-1).render("[\t]"), "[\t]");
    }

    #[test]
    fn test_custom_padding_char() {
        let s = Style::new().padding(&[0, 3]).padding_char('x');
        assert_eq!(s.render("TEST"), "xxxTESTxxx");
    }

    #[test]
    fn test_margin() {
        let s = Style::new().margin(&[0, 1]);
        assert_eq!(s.render("foo"), " foo ");
    }

    #[test]
    fn test_hyperlink() {
        let s = Style::new()
            .hyperlink("https://example.com", &[])
            .set_string(&["https://example.com"]);
        assert_eq!(
            s.render(""),
            "\x1b]8;;https://example.com\x07https://example.com\x1b]8;;\x07"
        );
    }

    #[test]
    fn test_transform() {
        let s = Style::new().bold(true).transform(|x| x.to_uppercase());
        assert_eq!(s.render("raow"), "\x1b[1mRAOW\x1b[m");
    }

    #[test]
    fn test_width_with_border() {
        let s = Style::new().width(10).border(Border::normal(), &[]);
        let out = s.render("hi");
        assert_eq!(crate::size::width(&out), 10);
    }

    #[test]
    fn test_underline_spaces() {
        let s = Style::new().underline_spaces(true).set_string(&["ab c"]);
        assert_eq!(s.render(""), "ab\x1b[4m \x1b[mc");
    }
}
