//! Cleanroom Rust port of upstream Go test file: `join_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::align::{Position, BOTTOM, CENTER, LEFT, RIGHT, TOP};
use rusty_lipgloss::join::{join_horizontal, join_vertical};

#[test]
fn test_join_vertical() {
    assert_eq!(join_vertical(LEFT, &["A", "BBBB"]), "A   \nBBBB");
    assert_eq!(join_vertical(RIGHT, &["A", "BBBB"]), "   A\nBBBB");
    assert_eq!(join_vertical(Position(0.25), &["A", "BBBB"]), " A  \nBBBB");
}

#[test]
fn test_join_horizontal() {
    assert_eq!(join_horizontal(TOP, &["A", "B\nB\nB\nB"]), "AB\n B\n B\n B");
    assert_eq!(
        join_horizontal(BOTTOM, &["A", "B\nB\nB\nB"]),
        " B\n B\n B\nAB"
    );
    assert_eq!(
        join_horizontal(Position(0.25), &["A", "B\nB\nB\nB"]),
        " B\nAB\n B\n B"
    );
    assert_eq!(
        join_horizontal(CENTER, &["A", "B\nB\nB\nB"]),
        " B\n B\nAB\n B"
    );
}

#[test]
fn test_join_single_and_empty() {
    assert_eq!(join_horizontal(TOP, &["A"]), "A");
    assert_eq!(join_vertical(LEFT, &[]), "");
    assert_eq!(join_horizontal(TOP, &[]), "");
}
