use charming_lipgloss::align::Position;
use charming_lipgloss::join::{join_horizontal, join_vertical};

#[test]
fn test_join_vertical() {
    let res = join_vertical(Position::Left, &["Hello", "World"]);
    assert_eq!(res, "Hello\nWorld");
}

#[test]
fn test_join_horizontal() {
    let res = join_horizontal(Position::Top, &["A", "B"]);
    assert_eq!(res, "A B");
}
