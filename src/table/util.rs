//! Cleanroom Rust port of upstream Go source file: `table/util.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Small numeric helpers used by the table resizer.
//! </public-docs>

/// btoi converts a boolean to an integer, 1 if true, 0 if false.
pub(crate) fn btoi(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

/// bton converts a boolean to a specific integer, n if true, 0 if false.
pub(crate) fn bton(b: bool, n: usize) -> usize {
    if b {
        n
    } else {
        0
    }
}

/// sum returns the sum of all integers in a slice.
pub(crate) fn sum(n: &[usize]) -> usize {
    n.iter().sum()
}

/// median returns the median of a slice of integers.
pub(crate) fn median(n: &mut [usize]) -> usize {
    n.sort_unstable();
    if n.is_empty() {
        return 0;
    }
    if n.len().is_multiple_of(2) {
        let h = n.len() / 2;
        (n[h - 1] + n[h]) / 2
    } else {
        n[n.len() / 2]
    }
}
