# Charming Lip Gloss (`charming-lipgloss`)

A 1:1 cleanroom Rust port of Charmbracelet's upstream Go [`lipgloss`](https://github.com/charmbracelet/lipgloss) library (pinned to release **`v2.0.5`**).

## Overview

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
