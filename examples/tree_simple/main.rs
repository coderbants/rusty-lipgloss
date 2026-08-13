//! Cleanroom Rust port of upstream Go example: `examples/tree/simple/main.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! A simple operating-systems tree.

use rusty_lipgloss::tree::{self, Child};
use rusty_lipgloss::writer::println;

fn main() {
    let linux = tree::root("Linux")
        .child(Child::Str("NixOS".into()))
        .child(Child::Str("Arch Linux (btw)".into()))
        .child(Child::Str("Void Linux".into()));
    let bsd = tree::root("BSD")
        .child(Child::Str("FreeBSD".into()))
        .child(Child::Str("OpenBSD".into()));

    let t = tree::root(".")
        .child(Child::Str("macOS".into()))
        .child(Child::Tree(Box::new(linux)))
        .child(Child::Tree(Box::new(bsd)));

    println(&t.render()).unwrap();
}
