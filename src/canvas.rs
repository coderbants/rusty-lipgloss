//! Cleanroom Rust port of upstream Go source file: `canvas.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! A cell-buffer canvas that can be used to compose and draw layers.
//! Composed drawables are drawn onto the canvas in the order they were
//! composed, meaning later drawables will appear "on top" of earlier ones.
//! </public-docs>

use super::layer::Rectangle;
use crate::ansi;

/// A single cell of the canvas.
#[derive(Debug, Clone, Default)]
pub struct Cell {
    /// The character content of the cell.
    pub content: char,
    /// The style of the cell.
    pub style: ansi::Style,
}

/// A screen is a target that drawables can be drawn onto.
pub trait Screen {
    /// Returns the bounds of the screen.
    fn bounds(&self) -> Rectangle;
    /// Returns the width of the screen.
    fn width(&self) -> usize;
    /// Returns the height of the screen.
    fn height(&self) -> usize;
    /// Returns a reference to the cell at the given position.
    fn cell_at(&self, x: usize, y: usize) -> Option<&Cell>;
    /// Returns a mutable reference to the cell at the given position.
    fn cell_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell>;
}

/// <upstream-comment>Canvas is a cell-buffer that can be used to compose and draw drawables
/// like layers.
///
/// A canvas can read, modify, and render its cell contents.</upstream-comment>
#[derive(Debug, Clone)]
pub struct Canvas {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

/// <upstream-comment>NewCanvas creates a new [Canvas] with the given size.</upstream-comment>
pub fn new_canvas(width: usize, height: usize) -> Canvas {
    let mut c = Canvas {
        cells: Vec::with_capacity(width * height),
        width,
        height,
    };
    c.fill_blank();
    c
}

impl Canvas {
    /// Returns a new Canvas with the given size.
    pub fn new(width: usize, height: usize) -> Canvas {
        new_canvas(width, height)
    }

    fn fill_blank(&mut self) {
        self.cells.clear();
        self.cells.resize(
            self.width * self.height,
            Cell {
                content: ' ',
                style: ansi::Style::default(),
            },
        );
    }

    /// <upstream-comment>Resize resizes the canvas to the given width and height.</upstream-comment>
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.fill_blank();
    }

    /// <upstream-comment>Clear clears the canvas.</upstream-comment>
    pub fn clear(&mut self) {
        self.fill_blank();
    }

    /// <upstream-comment>Bounds returns the bounds of the canvas.</upstream-comment>
    pub fn bounds(&self) -> Rectangle {
        Rectangle {
            min: (0, 0),
            max: (self.width, self.height),
        }
    }

    /// <upstream-comment>Width returns the width of the canvas.</upstream-comment>
    pub fn width(&self) -> usize {
        self.width
    }

    /// <upstream-comment>Height returns the height of the canvas.</upstream-comment>
    pub fn height(&self) -> usize {
        self.height
    }

    /// <upstream-comment>CellAt returns the cell at the given position.</upstream-comment>
    pub fn cell_at(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get(y * self.width + x)
    }

    /// Returns a mutable reference to the cell at the given position.
    pub fn cell_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get_mut(y * self.width + x)
    }

    /// <upstream-comment>SetCell sets the cell at the given position.</upstream-comment>
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if let Some(c) = self.cell_at_mut(x, y) {
            *c = cell;
        }
    }

    /// <upstream-comment>Compose composes a layer (or any drawable) onto the [Canvas].</upstream-comment>
    pub fn compose(&mut self, drawer: &dyn Drawable) -> &mut Canvas {
        let bounds = self.bounds();
        drawer.draw(self, bounds);
        self
    }

    /// <upstream-comment>Draw draws the [Canvas] onto the given screen within the specified
    /// area.</upstream-comment>
    pub fn draw(&self, scr: &mut dyn Screen, area: Rectangle) {
        let (sx, sy) = area.min;
        let (mx, my) = area.max;
        for y in sy..my {
            for x in sx..mx {
                if let Some(cell) = self.cell_at(x - sx, y - sy) {
                    if let Some(target) = scr.cell_at_mut(x, y) {
                        *target = cell.clone();
                    }
                }
            }
        }
    }

    /// <upstream-comment>Render renders the canvas into a styled string.</upstream-comment>
    pub fn render(&self) -> String {
        let mut out = String::new();
        let mut current_style = ansi::Style::default();
        for y in 0..self.height {
            // Find the last non-blank cell in this row so trailing blank
            // cells are trimmed (matching the upstream screen buffer).
            let mut last = 0usize;
            for x in 0..self.width {
                let cell = &self.cells[y * self.width + x];
                if cell.content != ' ' || !cell.style.is_zero() {
                    last = x + 1;
                }
            }
            for x in 0..last {
                let cell = &self.cells[y * self.width + x];
                if cell.style != current_style {
                    out.push_str(&cell.style.string());
                    current_style = cell.style.clone();
                }
                out.push(cell.content);
            }
            if y < self.height - 1 {
                out.push('\n');
            }
        }
        if !current_style.is_zero() {
            out.push_str(ansi::RESET_STYLE);
        }
        out
    }
}

impl Screen for Canvas {
    fn bounds(&self) -> Rectangle {
        self.bounds()
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn cell_at(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cell_at(x, y)
    }
    fn cell_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cell_at_mut(x, y)
    }
}

/// A drawable can draw itself onto a screen.
pub trait Drawable {
    /// Draws this drawable onto the given screen within the specified area.
    fn draw(&self, scr: &mut dyn Screen, area: Rectangle);
}

/// Parses a styled string and draws its cells onto the screen within the area.
/// ANSI SGR sequences are parsed to set the cell style.
pub(crate) fn draw_styled(scr: &mut dyn Screen, content: &str, area: Rectangle) {
    let (sx, sy) = area.min;
    let (mx, my) = area.max;
    let mut style = ansi::Style::default();
    let mut x = sx;
    let mut y = sy;

    let bytes = content.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b == 0x1b {
            // Parse an escape sequence.
            if i + 1 < bytes.len() && bytes[i + 1] == b'[' {
                let mut j = i + 2;
                let mut seq = String::new();
                while j < bytes.len() && !(0x40..=0x7e).contains(&bytes[j]) {
                    seq.push(bytes[j] as char);
                    j += 1;
                }
                if j < bytes.len() {
                    let cmd = bytes[j];
                    if cmd == b'm' {
                        style = parse_sgr(&seq, style);
                    }
                    i = j + 1;
                    continue;
                }
            }
            i += 1;
            continue;
        }
        if b == b'\n' {
            y += 1;
            x = sx;
            i += 1;
            continue;
        }
        let ch = content[i..].chars().next().unwrap();
        if x < mx && y < my {
            if let Some(cell) = scr.cell_at_mut(x, y) {
                cell.content = ch;
                cell.style = style.clone();
            }
        }
        x += 1;
        i += ch.len_utf8();
    }
    let _ = (sx, sy);
}

fn parse_sgr(params: &str, mut style: ansi::Style) -> ansi::Style {
    let parts: Vec<&str> = params.split(';').collect();
    let mut i = 0usize;
    while i < parts.len() {
        match parts[i] {
            "" | "0" => style = ansi::Style::default(),
            "1" => style.bold = true,
            "2" => style.faint = true,
            "3" => style.italic = true,
            "4" => {
                style.underline = true;
                // Possibly 4:<style> in the same param.
                if let Some(rest) = parts[i].strip_prefix("4:") {
                    if let Some(v) = rest.parse::<u8>().ok() {
                        style.underline_style = match v {
                            3 => ansi::Underline::Curly,
                            4 => ansi::Underline::Dotted,
                            5 => ansi::Underline::Dashed,
                            _ => ansi::Underline::Single,
                        };
                    }
                }
            }
            "5" => style.blink = true,
            "7" => style.reverse = true,
            "9" => style.strikethrough = true,
            "21" => {
                style.underline = true;
                style.underline_style = ansi::Underline::Double;
            }
            "38" | "48" | "58" => {
                let prefix = parts[i];
                let mode = parts.get(i + 1).copied();
                match mode {
                    Some("5") => {
                        if let Some(v) = parts.get(i + 2).and_then(|v| v.parse::<u8>().ok()) {
                            let c = crate::color::Color::Ansi256(v);
                            match prefix {
                                "38" => style.fg_color = Some(c),
                                "48" => style.bg_color = Some(c),
                                _ => style.ul_color = Some(c),
                            }
                        }
                        i += 2;
                    }
                    Some("2") => {
                        let r = parts.get(i + 2).and_then(|v| v.parse::<u8>().ok());
                        let g = parts.get(i + 3).and_then(|v| v.parse::<u8>().ok());
                        let b = parts.get(i + 4).and_then(|v| v.parse::<u8>().ok());
                        if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                            let c = crate::color::Color::TrueColor { r, g, b };
                            match prefix {
                                "38" => style.fg_color = Some(c),
                                "48" => style.bg_color = Some(c),
                                _ => style.ul_color = Some(c),
                            }
                        }
                        i += 4;
                    }
                    _ => {}
                }
            }
            v => {
                if let Ok(n) = v.parse::<u8>() {
                    match n {
                        30..=37 => {
                            style.fg_color = Some(crate::color::Color::Ansi16(n - 30));
                        }
                        90..=97 => {
                            style.fg_color = Some(crate::color::Color::Ansi16(8 + n - 90));
                        }
                        40..=47 => {
                            style.bg_color = Some(crate::color::Color::Ansi16(n - 40));
                        }
                        100..=107 => {
                            style.bg_color = Some(crate::color::Color::Ansi16(8 + n - 100));
                        }
                        _ => {}
                    }
                }
            }
        }
        i += 1;
    }
    style
}
