<p align="center">
    <a href="https://raw.githubusercontent.com/coderbants/rusty-lipgloss/dev/rusty-lipgloss.png"><img src="https://raw.githubusercontent.com/coderbants/rusty-lipgloss/dev/rusty-lipgloss.png" width="313" alt="Rusty Lip Gloss"></a><br>
    <a href="https://crates.io/crates/rusty-lipgloss"><img src="https://img.shields.io/crates/v/rusty-lipgloss.svg" alt="crates.io"></a>
    <a href="https://github.com/coderbants/rusty-lipgloss/actions"><img src="https://github.com/coderbants/rusty-lipgloss/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
    <a href="https://raw.githubusercontent.com/coderbants/rusty-lipgloss/dev/coverage.svg"><img src="https://raw.githubusercontent.com/coderbants/rusty-lipgloss/dev/coverage.svg" alt="coverage"></a>

</p>

# Rusty Lip Gloss (`rusty-lipgloss`)

**Rusty Lip Gloss** is a complete, from-scratch Rust port of [Lip Gloss](https://github.com/charmbracelet/lipgloss), the styling library that powers Charmbracelet's terminal apps. It tracks upstream Go releases on a rolling basis under the family's [porting policies](./POLICIES.md): versions mirror upstream exactly, never ahead or behind, with a hard goal of **1:1 behavioural, visual and license parity**, favouring fidelity to upstream semantics over Rust-native rewrites.

It's part of the Rusty port family of the Bubble Tea ecosystem and builds on [rusty-ultraviolet](https://github.com/coderbants/rusty-ultraviolet) (terminal renderer & input), [rusty-x-ansi](https://github.com/coderbants/rusty-x-ansi) (ANSI primitives), and [rusty-colorprofile](https://github.com/coderbants/rusty-colorprofile) — with UI components available in [rusty-bubbles](https://github.com/coderbants/rusty-bubbles) and the framework in [rusty-bubbletea](https://github.com/coderbants/rusty-bubbletea).

***About Lip Gloss***

Style, format and layout tools for terminal applications. Built for Rust based on upstream [charmbracelet/lipgloss](https://github.com/charmbracelet/lipgloss).

<p>
    <img src="https://stuff.charm.sh/lipgloss/lipgloss-example.gif" width="100%" alt="Lip Gloss Example">
</p>

## Overview

Rusty Lip Gloss gives you a small, fast, dependency-free toolkit for styling and laying out terminal output:

- **Styles** — colors, bold/italic/underline, backgrounds, borders, margins, padding and alignment, all composed with a fluent builder API.
- **Text layout** — width and height control, word wrapping, truncation, and horizontal/vertical alignment.
- **Composition** — join strings horizontally or vertically, build tables, trees and lists, and render borders around any block.
- **Parity** — every upstream Go test is ported 1:1, and the E2E harness verifies byte-for-byte output against `charmbracelet/lipgloss`.


## Installation

```sh
cargo add rusty-lipgloss
```

Use the crate to style, format and lay out terminal text:

```rust
use rusty_lipgloss::new_style;
let styled = new_style().bold(true).foreground("63").render("hello");
```

`rusty-lipgloss` provides styling, layout, string joining, border rendering, tables, trees, and lists for terminal UI applications in lightweight, zero-unsafe, minimal-dependency Rust.

## Principles

1. **Upstream Version Parity**: Crate versions mirror the upstream Go releases on a 1:1 basis.
2. **Borrowing Discipline**: Strongly prioritizes borrowing (`&str`, `&[T]`) over reference counting (`Arc`, `Rc`).
3. **Zero Unsafe**: Enforced `#![deny(unsafe_code)]`.
4. **100% Test Parity**: Complete 1:1 conversion of all upstream Go test cases to Rust `tests/*.rs`.

## Usage Example

Style text, then compose it into larger layouts:

```rust
use rusty_lipgloss::{
    Border, Position, new_style, join_vertical, BOTTOM, CENTER, LEFT, RIGHT, TOP,
};

// A styled line of text.
let title = new_style()
    .bold(true)
    .foreground("#FF0000")
    .render("Hello Rusty Lipgloss!");

// A block with a border, padded and aligned.
let panel = new_style()
    .border(Border::rounded())
    .width(30)
    .height(5)
    .align(&[CENTER])
    .render("Centered inside a rounded border");

// Join blocks together.
let layout = join_vertical(TOP, &[title, panel]);
println!("{layout}");
```

Styles compose: set a background on one style and inherit it into another with `inherit`, render bordered tables with `rusty_lipgloss::table`, and lay out lists with `rusty_lipgloss::list`. See the [examples](https://github.com/coderbants/rusty-lipgloss/tree/dev/examples) directory for complete programs.

## License

[MIT](LICENSE)
