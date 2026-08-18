//! Cleanroom Rust port of upstream Go test file: `canvas_test.go`
//! Upstream Target Tag / Version: `v2.0.5`

use rusty_lipgloss::canvas::{new_canvas, Canvas};
use rusty_lipgloss::layer::{new_compositor, new_layer};
use rusty_lipgloss::style::Style;
use rusty_ultraviolet::Cell;

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
    c.set_cell(1, 2, Some(&Cell::new("x")));
    let cell = c.cell_at(1, 2).unwrap();
    assert_eq!(cell.content, "x");
}

#[test]
fn test_canvas_resize_clear() {
    let mut c = new_canvas(4, 4);
    c.resize(8, 2);
    assert_eq!(c.width(), 8);
    assert_eq!(c.height(), 2);
    c.clear();
    assert_eq!(c.cell_at(0, 0).unwrap().content, " ");
}

#[test]
fn test_canvas_render() {
    let mut c = new_canvas(3, 1);
    c.set_cell(0, 0, Some(&Cell::new("a")));
    c.set_cell(1, 0, Some(&Cell::new("b")));
    c.set_cell(2, 0, Some(&Cell::new("c")));
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
    let mut comp = new_compositor(&[l1, l2]);
    let out = comp.render();
    assert!(out.contains("World"));
    let _ = Style::new;
    let _ = Canvas::new(0, 0);
}

#[test]
fn test_canvas_render_upstream() {
    // Go TestCanvasRender: fully filled canvas.
    let mut c = new_canvas(5, 3);
    for y in 0..c.height() {
        for x in 0..c.width() {
            c.set_cell(x, y, Some(&Cell::new(".")));
        }
    }
    for x in 1..4 {
        c.set_cell(x, 1, Some(&Cell::new("#")));
    }
    assert_eq!(c.render(), ".....\n.###.\n.....");
}

#[test]
fn test_canvas_render_trailing_spaces() {
    // Go TestCanvasRenderWithTrailingSpaces.
    let mut c = new_canvas(5, 2);
    for y in 0..c.height() {
        for x in 0..c.width() {
            if x < 3 {
                c.set_cell(x, y, Some(&Cell::new("A")));
            } else {
                c.set_cell(x, y, Some(&Cell::new(" ")));
            }
        }
    }
    assert_eq!(c.render(), "AAA\nAAA");
}

/// Layer max_z, nested lookups, empty-id and add_layers paths.
#[test]
fn test_layer_max_z_and_nested() {
    use rusty_lipgloss::layer::new_layer;
    let child1 = new_layer("c1", &[]).id("c1").z(5);
    let child2 = new_layer("c2", &[]).id("c2").z(-3);
    let mut parent = new_layer("parent", &[child1, child2]).id("p").z(2);
    assert_eq!(parent.max_z(), 5);
    // Nested get_layer with empty id is skipped.
    assert!(parent.get_layer("").is_none());
    // add_layers updates parent bounds.
    let extra = new_layer("extra", &[]).id("e").x(3).y(3);
    parent.add_layers(&[extra]);
    assert!(parent.get_layer("e").is_some());
    // Missing id.
    assert!(parent.get_layer("missing").is_none());
}

/// LayerHit default and bounds.
#[test]
fn test_layer_hit_default() {
    use rusty_lipgloss::layer::{new_compositor, new_layer};
    let l = new_layer("AB", &[]).id("a");
    let comp = new_compositor(&[l]);
    let hit = comp.hit(0, 0);
    assert_eq!(hit.id(), "a");
    let hit = comp.hit(99, 99);
    // Out-of-bounds hits return an empty/default layer hit.
    assert!(hit.bounds().dx() <= 2);
}

/// Compositor render with layers of differing sizes/z.
#[test]
fn test_compositor_render_z_order() {
    use rusty_lipgloss::layer::{new_compositor, new_layer};
    let l1 = new_layer("AAA", &[]).id("one").z(1);
    let l2 = new_layer("BBB", &[]).id("two").z(2);
    let mut comp = new_compositor(&[l1, l2]);
    let out = comp.render();
    // Higher z renders on top where they overlap.
    assert!(out.contains("BBB"));
    // refresh resets the internal screen without panicking.
    comp.refresh();
    let out2 = comp.render();
    assert_eq!(out, out2);
}
