<p>
    <a href="charming-lipgloss.png"><img src="charming-lipgloss.png" width="313" alt="Charming Lip Gloss"></a><br>
    <a href="https://crates.io/crates/charming-lipgloss"><img src="https://img.shields.io/crates/v/charming-lipgloss.svg" alt="crates.io"></a>
</p>

# Charming Lip Gloss (`charming-lipgloss`)

**Charming Lip Gloss** is a complete, from-scratch Rust port of [Lip Gloss](https://github.com/charmbracelet/lipgloss), the styling library that powers Charmbracelet's terminal apps. It tracks upstream Go releases on a rolling basis — this crate mirrors upstream `v2.0.5` — with a hard goal of **1:1 behavioral and visual parity**: the same styles, rendering output, and layout semantics, favoring fidelity to upstream over Rust-native rewrites whenever the two would diverge.

It's part of the Charming port family of the Bubble Tea ecosystem and builds on [charming-ultraviolet](https://github.com/coderbants/charming-ultraviolet) (terminal renderer & input), [charming-x-ansi](https://github.com/coderbants/charming-x-ansi) (ANSI primitives), and [charming-colorprofile](https://github.com/coderbants/charming-colorprofile) — with UI components available in [charming-bubbles](https://github.com/coderbants/charming-bubbles) and the framework in [charming-bubbletea](https://github.com/coderbants/charming-bubbletea).

Style, format and layout tools for terminal applications. Built for Rust based on upstream [charmbracelet/lipgloss](https://github.com/charmbracelet/lipgloss).

<p>
    <img src="https://stuff.charm.sh/lipgloss/lipgloss-example.gif" width="100%" alt="Lip Gloss Example">
</p>

## Overview

## Installation

```sh
cargo add charming-lipgloss
```

Use the crate to style, format and lay out terminal text:

```rust
use charming_lipgloss::new_style;
let styled = new_style().bold(true).foreground("63").render("hello");
```

`charming-lipgloss` provides styling, layout, string joining, border rendering, tables, trees, and lists for terminal UI applications in lightweight, zero-unsafe, minimal-dependency Rust.

## Principles

1. **Upstream Version Parity**: Direct semver alignment with upstream Go `charmbracelet/lipgloss@v2.0.5`.
2. **Borrowing Discipline**: Strongly prioritizes borrowing (`&str`, `&[T]`) over reference counting (`Arc`, `Rc`).
3. **Zero Unsafe**: Enforced `#![deny(unsafe_code)]`.
4. **100% Test Parity**: Complete 1:1 conversion of all upstream Go test cases to Rust `tests/*.rs`.

## Usage Example

```rust
use charming_lipgloss::{Style, Position, Border};

let style = Style::new()
    .bold(true)
    .foreground("#FF0000")
    .background("#000000");

println!("{}", style.render("Hello Charming Lipgloss!"));
```

## License

[MIT](LICENSE)
