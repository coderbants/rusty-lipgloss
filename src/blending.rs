//! Cleanroom Rust port of upstream Go source file: `blending.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Color blending algorithms using the "CIE L*, a*, b*" (CIELAB) color-space.
//! </public-docs>

use crate::color::{lab_from_rgb, rgb_from_lab, Color, Lab};

fn clamp(v: f64, low: f64, high: f64) -> f64 {
    v.clamp(low, high)
}

/// <upstream-comment>Blend1D blends a series of colors together in one linear dimension using multiple
/// stops, into the provided number of steps. Uses the "CIE L*, a*, b*" (CIELAB) color-space.
///
/// Note that if any of the provided colors are completely transparent, we will
/// assume that the alpha value was lost in conversion from RGB -> RGBA, and we
/// will set the alpha to opaque, as it's not possible to blend something completely
/// transparent.</upstream-comment>
pub fn blend_1d(steps: usize, stops: &[Color]) -> Vec<Color> {
    let mut steps = steps as isize;
    if steps < 0 {
        steps = 0;
    }
    let steps = steps as usize;

    if steps <= stops.len() {
        return stops[..steps].to_vec();
    }

    // Ensure they didn't provide any NoColor colors.
    let stops: Vec<&Color> = stops.iter().filter(|c| !c.is_no_color()).collect();

    if stops.is_empty() {
        return Vec::new(); // We can't safely fallback.
    }

    // If they only provided one valid color (or some nil colors), we will just return
    // an array of that color, for the amount of steps they requested.
    if stops.len() == 1 {
        return vec![stops[0].clone(); steps];
    }

    let mut blended: Vec<Color> = Vec::with_capacity(steps);

    // Convert stops to Lab once.
    let cstops: Vec<Lab> = stops
        .iter()
        .map(|k| {
            let (r, g, b, _) = ensure_not_transparent(k).rgba_bytes();
            lab_from_rgb(r, g, b)
        })
        .collect();

    let num_segments = cstops.len() - 1;
    let default_size = steps / num_segments;
    let remaining_steps = steps % num_segments;

    for i in 0..num_segments {
        let from = cstops[i];
        let to = cstops[i + 1];

        // Calculate segment size.
        let mut segment_size = default_size;
        if i < remaining_steps {
            segment_size += 1;
        }

        let divisor = (segment_size - 1) as f64;

        // Generate colors for this segment.
        for j in 0..segment_size {
            let mut blending_factor = 0.0;
            if segment_size > 1 {
                blending_factor = j as f64 / divisor;
            }
            let l = from.l + blending_factor * (to.l - from.l);
            let a = from.a + blending_factor * (to.a - from.a);
            let b = from.b + blending_factor * (to.b - from.b);
            let (r, g, b) = rgb_from_lab(Lab { l, a, b });
            blended.push(Color::TrueColor { r, g, b });
        }
    }

    blended
}

/// <upstream-comment>Blend2D blends a series of colors together in two linear dimensions using
/// multiple stops, into the provided width/height. Uses the "CIE L*, a*, b*" (CIELAB)
/// color-space. The angle parameter controls the rotation of the gradient (0-360°),
/// where 0° is left-to-right, 45° is bottom-left to top-right (diagonal). The function
/// returns colors in a 1D row-major order ([row1, row2, row3, ...]).</upstream-comment>
pub fn blend_2d(width: usize, height: usize, angle: f64, stops: &[Color]) -> Vec<Color> {
    let width = if width < 1 { 1 } else { width };
    let height = if height < 1 { 1 } else { height };

    // Normalize angle to 0-360.
    let mut angle = angle % 360.0;
    if angle < 0.0 {
        angle += 360.0;
    }

    // Ensure they didn't provide any NoColor colors.
    let stops: Vec<&Color> = stops.iter().filter(|c| !c.is_no_color()).collect();

    if stops.is_empty() {
        return Vec::new(); // We can't safely fallback.
    }

    // If they only provided one valid color, we will just return an array of that
    // color, for the amount of pixels they requested.
    if stops.len() == 1 {
        return vec![stops[0].clone(); width * height];
    }

    // For 2D blending, we'll create a gradient along the diagonal and then sample
    // from it based on the angle.
    let diagonal_stops: Vec<Color> = stops.iter().map(|c| (*c).clone()).collect();
    let diagonal_gradient = blend_1d(width.max(height), &diagonal_stops);

    let mut result: Vec<Color> = Vec::with_capacity(width * height);

    // Calculate center point for rotation.
    let center_x = (width - 1) as f64 / 2.0;
    let center_y = (height - 1) as f64 / 2.0;

    let angle_rad = angle * std::f64::consts::PI / 180.0;

    // Pre-calculate sin and cos.
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    // Calculate diagonal length for proper gradient mapping.
    let diagonal_length = ((width * width + height * height) as f64).sqrt();

    // Pre-calculate gradient length for index calculation.
    let gradient_len = (diagonal_gradient.len() - 1) as f64;

    for y in 0..height {
        let dy = y as f64 - center_y;
        for x in 0..width {
            let dx = x as f64 - center_x;
            let rot_x = dx * cos_angle - dy * sin_angle;

            // Map the rotated position to the gradient.
            let gradient_pos = clamp((rot_x + diagonal_length / 2.0) / diagonal_length, 0.0, 1.0);

            let mut gradient_index = (gradient_pos * gradient_len) as usize;
            if gradient_index >= diagonal_gradient.len() {
                gradient_index = diagonal_gradient.len() - 1;
            }

            result.push(diagonal_gradient[gradient_index].clone());
        }
    }

    result
}

fn ensure_not_transparent(c: &Color) -> Color {
    let (_, _, _, a) = c.rgba();
    if a == 0 {
        crate::color::alpha(c.clone(), 1.0)
    } else {
        c.clone()
    }
}

/// <upstream-comment>Blend1D blends two colors linearly along a 1D scale from 0.0 to 1.0.</upstream-comment>
pub fn blend_1d_pair(start: &Color, end: &Color, factor: f64) -> Color {
    let factor = factor.clamp(0.0, 1.0);
    blend_1d(2, &[start.clone(), end.clone()])
        .get(0)
        .cloned()
        .unwrap_or_else(|| {
            // Fallback: linear interpolation for TrueColor.
            match (start, end) {
                (
                    Color::TrueColor {
                        r: r1,
                        g: g1,
                        b: b1,
                    },
                    Color::TrueColor {
                        r: r2,
                        g: g2,
                        b: b2,
                    },
                ) => Color::TrueColor {
                    r: ((*r1 as f64) * (1.0 - factor) + (*r2 as f64) * factor).round() as u8,
                    g: ((*g1 as f64) * (1.0 - factor) + (*g2 as f64) * factor).round() as u8,
                    b: ((*b1 as f64) * (1.0 - factor) + (*b2 as f64) * factor).round() as u8,
                },
                _ => start.clone(),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgba(r: u8, g: u8, b: u8) -> Color {
        Color::TrueColor { r, g, b }
    }

    #[test]
    fn test_blend_1d_two_colors() {
        let stops = vec![rgba(255, 0, 0), rgba(0, 0, 255)];
        let got = blend_1d(10, &stops);
        assert_eq!(got.len(), 10);
        assert_eq!(got[0], rgba(255, 0, 0));
        assert_eq!(got[9], rgba(0, 0, 255));
    }

    #[test]
    fn test_blend_1d_black_white() {
        let stops = vec![rgba(0, 0, 0), rgba(255, 255, 255)];
        let got = blend_1d(5, &stops);
        assert_eq!(got[0], rgba(0, 0, 0));
        assert_eq!(got[4], rgba(255, 255, 255));
        assert_eq!(got[2], rgba(119, 119, 119));
    }

    #[test]
    fn test_blend_1d_single_stop() {
        let stops = vec![rgba(255, 0, 0)];
        let got = blend_1d(3, &stops);
        assert_eq!(got.len(), 3);
        assert_eq!(got[0], rgba(255, 0, 0));
        assert_eq!(got[2], rgba(255, 0, 0));
    }

    #[test]
    fn test_blend_1d_zero_steps() {
        let got = blend_1d(0, &[]);
        assert_eq!(got.len(), 0);
    }

    #[test]
    fn test_blend_2d() {
        let stops = vec![rgba(255, 0, 0), rgba(0, 0, 255)];
        let got = blend_2d(2, 2, 0.0, &stops);
        assert_eq!(got.len(), 4);
        for c in &got {
            assert!(!c.is_no_color());
        }
    }

    #[test]
    fn test_blend_2d_single_color() {
        let stops = vec![rgba(255, 0, 0)];
        let got = blend_2d(2, 2, 0.0, &stops);
        assert_eq!(got.len(), 4);
        for c in &got {
            assert_eq!(*c, rgba(255, 0, 0));
        }
    }
}
