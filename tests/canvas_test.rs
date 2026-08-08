//! Cleanroom Rust port of upstream Go test file: `canvas_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use charming_lipgloss::canvas::{new_canvas, Canvas};
use charming_lipgloss::layer::{new_compositor, new_layer};
use charming_lipgloss::style::Style;

#[test]
fn test_canvas_new() {
    let c = new_canvas(10, 5);
    assert_eq!(c.width(), 10);
    assert_eq!(c.height(), 5);
    assert_eq!(c.bounds().dx(), 10);
    assert_eq!(c.bounds().dy(), 5);
}

#[test]
fn test_canvas_set_and_get_cell() {
    let mut c = new_canvas(4, 4);
    let cell = charming_lipgloss::canvas::Cell {
        content: 'x',
        style: Default::default(),
    };
    c.set_cell(1, 2, cell);
    let cell = c.cell_at(1, 2).unwrap();
    assert_eq!(cell.content, 'x');
}

#[test]
fn test_canvas_resize_clear() {
    let mut c = new_canvas(4, 4);
    c.resize(8, 2);
    assert_eq!(c.width(), 8);
    assert_eq!(c.height(), 2);
    c.clear();
    assert_eq!(c.cell_at(0, 0).unwrap().content, ' ');
}

#[test]
fn test_canvas_render() {
    let mut c = new_canvas(3, 1);
    let cell = charming_lipgloss::canvas::Cell {
        content: 'a',
        style: Default::default(),
    };
    c.set_cell(0, 0, cell.clone());
    let cell = charming_lipgloss::canvas::Cell {
        content: 'b',
        style: Default::default(),
    };
    c.set_cell(1, 0, cell.clone());
    let cell = charming_lipgloss::canvas::Cell {
        content: 'c',
        style: Default::default(),
    };
    c.set_cell(2, 0, cell);
    assert_eq!(c.render(), "abc");
}

#[test]
fn test_layer_basics() {
    let l = new_layer("hello", &[]);
    assert_eq!(l.get_content(), "hello");
    assert_eq!(l.width(), 5);
    assert_eq!(l.height(), 1);

    let l = new_layer("hello", &[]).id("greeting").x(1).y(2).z(3);
    assert_eq!(l.get_id(), "greeting");
    assert_eq!(l.get_x(), 1);
    assert_eq!(l.get_y(), 2);
    assert_eq!(l.get_z(), 3);
}

#[test]
fn test_layer_get_layer() {
    let mut parent = new_layer("parent", &[]);
    let child = new_layer("child", &[]).id("c1");
    parent.add_layers(&[child]);
    assert!(parent.get_layer("c1").is_some());
    assert!(parent.get_layer("nope").is_none());
}

#[test]
fn test_compositor() {
    let l1 = new_layer("AAAA", &[]).id("one").z(1);
    let l2 = new_layer("BBBB", &[]).id("two").z(2);
    let comp = new_compositor(&[l1, l2]);
    assert_eq!(comp.bounds().dx(), 4);
    let hit = comp.hit(0, 0);
    assert_eq!(hit.id(), "two");
    assert!(comp.get_layer("one").is_some());
    assert!(comp.get_layer("missing").is_none());
}

#[test]
fn test_compositor_render() {
    let l1 = new_layer("Hello", &[]).id("a").z(1);
    let l2 = new_layer("World", &[]).id("b").z(2).x(3);
    let comp = new_compositor(&[l1, l2]);
    let out = comp.render();
    assert!(out.contains("World"));
    let _ = Style::new;
    let _ = Canvas::new(0, 0);
}
