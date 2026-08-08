//! Cleanroom Rust port of upstream Go source file: `canvas.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! A cell-buffer canvas that can be used to compose and draw layers.
//! Composed drawables are drawn onto the canvas in the order they were
//! composed, meaning later drawables will appear "on top" of earlier ones.
//!
//! The canvas wraps a [charming_ultraviolet::ScreenBuffer] and implements
//! the ultraviolet [charming_ultraviolet::Screen] and
//! [charming_ultraviolet::Drawable] interfaces, mirroring the upstream
//! `Canvas` struct.
//! </public-docs>

use charming_ultraviolet::{new_screen_buffer, Drawable, Rectangle, Screen, ScreenBuffer};
use charming_x_ansi::method::WidthMethod;


/// <upstream-comment>Canvas is a cell-buffer that can be used to compose and draw drawables
/// like layers.
///
/// A canvas can read, modify, and render its cell contents.
///
/// It implements [Screen] and [Drawable].</upstream-comment>
#[derive(Debug, Clone)]
pub struct Canvas {
    scr: ScreenBuffer,
}

/// <upstream-comment>NewCanvas creates a new [Canvas] with the given size.</upstream-comment>
pub fn new_canvas(width: usize, height: usize) -> Canvas {
    let mut scr = new_screen_buffer(width, height);
    scr.method = WidthMethod::GraphemeWidth;
    Canvas { scr }
}

impl Canvas {
    /// Returns a new Canvas with the given size.
    pub fn new(width: usize, height: usize) -> Canvas {
        new_canvas(width, height)
    }

    /// <upstream-comment>Resize resizes the canvas to the given width and height.</upstream-comment>
    pub fn resize(&mut self, width: usize, height: usize) {
        self.scr.render_buffer.buffer.resize(width, height);
    }

    /// <upstream-comment>Clear clears the canvas.</upstream-comment>
    pub fn clear(&mut self) {
        self.scr.render_buffer.clear();
    }

    /// <upstream-comment>Bounds returns the bounds of the canvas.</upstream-comment>
    pub fn bounds(&self) -> Rectangle {
        self.scr.bounds()
    }

    /// <upstream-comment>Width returns the width of the canvas.</upstream-comment>
    pub fn width(&self) -> usize {
        self.scr.width()
    }

    /// <upstream-comment>Height returns the height of the canvas.</upstream-comment>
    pub fn height(&self) -> usize {
        self.scr.height()
    }

    /// <upstream-comment>CellAt returns the cell at the given position.</upstream-comment>
    pub fn cell_at(&self, x: usize, y: usize) -> Option<&charming_ultraviolet::Cell> {
        self.scr.cell_at(x, y)
    }

    /// <upstream-comment>SetCell sets the cell at the given position.</upstream-comment>
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Option<&charming_ultraviolet::Cell>) {
        self.scr.set_cell(x, y, cell);
    }

    /// <upstream-comment>Compose composes a layer (or any drawable) onto the [Canvas].</upstream-comment>
    pub fn compose(&mut self, drawer: &mut dyn Drawable) -> &mut Canvas {
        let bounds = self.bounds();
        drawer.draw(self, bounds);
        self
    }

    /// <upstream-comment>Draw draws the [Canvas] onto the given screen within the specified
    /// area.</upstream-comment>
    pub fn draw(&self, scr: &mut dyn Screen, area: Rectangle) {
        self.scr.render_buffer.buffer.draw(scr, area);
    }

    /// <upstream-comment>Render renders the canvas into a styled string.</upstream-comment>
    pub fn render(&self) -> String {
        self.scr.render_buffer.buffer.render()
    }
}

impl Screen for Canvas {
    fn bounds(&self) -> Rectangle {
        self.bounds()
    }
    fn cell_at(&self, x: usize, y: usize) -> Option<&charming_ultraviolet::Cell> {
        self.cell_at(x, y)
    }
    fn set_cell(&mut self, x: usize, y: usize, c: Option<&charming_ultraviolet::Cell>) {
        self.set_cell(x, y, c)
    }
    fn width_method(&self) -> WidthMethod {
        self.scr.width_method()
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

impl Drawable for Canvas {
    fn draw(&mut self, scr: &mut dyn Screen, area: Rectangle) {
        Canvas::draw(self, scr, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_basic() {
        let mut c = new_canvas(5, 3);
        assert_eq!(c.width(), 5);
        assert_eq!(c.height(), 3);
        c.set_cell(
            0,
            0,
            Some(&charming_ultraviolet::Cell::new("x")),
        );
        assert_eq!(c.cell_at(0, 0).unwrap().content, "x");
        let out = c.render();
        // Trailing blank lines are emitted as empty lines (Go-verified
        // renderLine behavior).
        assert_eq!(out, "x\n\n");
    }

    #[test]
    fn test_canvas_clear() {
        let mut c = new_canvas(5, 3);
        c.set_cell(0, 0, Some(&charming_ultraviolet::Cell::new("x")));
        c.clear();
        assert_eq!(c.cell_at(0, 0).unwrap().content, " ");
    }
}
