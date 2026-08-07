//! String metrics calculations based on grapheme clusters.

use unicode_segmentation::UnicodeSegmentation;

/// Calculates the visual width of a string in terminal cells using graphemes.
pub fn width(s: &str) -> usize {
    s.lines()
        .map(|line| line.graphemes(true).count())
        .max()
        .unwrap_or(0)
}

/// Calculates the height (number of lines) of a string.
pub fn height(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        s.lines().count()
    }
}
