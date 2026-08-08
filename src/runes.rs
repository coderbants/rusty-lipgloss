//! Cleanroom Rust port of upstream Go source file: `runes.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Functions for applying a given style to runes at the given indices in a string.
//! </public-docs>

use std::collections::HashSet;

use crate::style::Style;

/// <upstream-comment>StyleRunes apply a given style to runes at the given indices in the string.
/// Note that you must provide styling options for both matched and unmatched
/// runes. Indices out of bounds will be ignored.</upstream-comment>
pub fn style_runes(str: &str, indices: &[usize], matched: &Style, unmatched: &Style) -> String {
    // Convert slice of indices to a set for easier lookups.
    let m: HashSet<usize> = indices.iter().copied().collect();

    let mut out = String::new();
    let mut group = String::new();

    let runes: Vec<char> = str.chars().collect();

    for (i, r) in runes.iter().enumerate() {
        group.push(*r);

        let matches = m.contains(&i);
        let next_matches = m.contains(&(i + 1));

        if matches != next_matches || i == runes.len() - 1 {
            // Flush.
            if matches {
                out.push_str(&matched.render(&group));
            } else {
                out.push_str(&unmatched.render(&group));
            }
            group.clear();
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_runes() {
        let matched = Style::new().bold(true);
        let unmatched = Style::new().faint(true);
        let out = style_runes("abcde", &[0, 2], &matched, &unmatched);
        assert_eq!(out, "\x1b[1ma\x1b[m\x1b[2mb\x1b[m\x1b[1mc\x1b[m\x1b[2mde\x1b[m");
    }
}
