//! Color blending algorithms matching upstream `lipgloss/blending.go`.

use crate::color::Color;

/// <upstream-comment>Blend1D blends two colors linearly along a 1D scale from 0.0 to 1.0.</upstream-comment>
pub fn blend_1d(start: &Color, end: &Color, factor: f64) -> Color {
    let factor = factor.clamp(0.0, 1.0);
    match (start, end) {
        (Color::TrueColor { r: r1, g: g1, b: b1 }, Color::TrueColor { r: r2, g: g2, b: b2 }) => {
            let r = ((*r1 as f64) * (1.0 - factor) + (*r2 as f64) * factor).round() as u8;
            let g = ((*g1 as f64) * (1.0 - factor) + (*g2 as f64) * factor).round() as u8;
            let b = ((*b1 as f64) * (1.0 - factor) + (*b2 as f64) * factor).round() as u8;
            Color::TrueColor { r, g, b }
        }
        _ => start.clone(),
    }
}

/// <upstream-comment>Blend2D blends four colors across a 2D quad (top-left, top-right, bottom-left, bottom-right) at normalized (x, y).</upstream-comment>
pub fn blend_2d(
    top_left: &Color,
    top_right: &Color,
    bottom_left: &Color,
    bottom_right: &Color,
    x: f64,
    y: f64,
) -> Color {
    let top = blend_1d(top_left, top_right, x);
    let bottom = blend_1d(bottom_left, bottom_right, x);
    blend_1d(&top, &bottom, y)
}
