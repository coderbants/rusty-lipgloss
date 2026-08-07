# Charming Lip Gloss (`charming-lipgloss`)

> [!NOTE]  
> This library is a cleanroom Rust port of Charmbracelet's upstream Go [Lip Gloss (`charmbracelet/lipgloss`)](https://github.com/charmbracelet/lipgloss) terminal styling library.

<p>
    <img src="https://stuff.charm.sh/lipgloss/lipgloss-title-treatment.png" width="313" alt="Lip Gloss Title Treatment"><br>
    <a href="https://github.com/charmbracelet/lipgloss/releases"><img src="https://img.shields.io/github/release/charmbracelet/lipgloss.svg" alt="Latest Release"></a>
    <a href="https://pkg.go.dev/github.com/charmbracelet/lipgloss?tab=doc"><img src="https://godoc.org/github.com/charmbracelet/lipgloss?status.svg" alt="GoDoc"></a>
    <a href="https://github.com/charmbracelet/lipgloss/actions"><img src="https://github.com/charmbracelet/lipgloss/actions/workflows/build.yml/badge.svg" alt="Build Status"></a>
</p>

Style, format and layout tools for terminal applications. Built for Rust based on upstream [charmbracelet/lipgloss](https://github.com/charmbracelet/lipgloss).

<p>
    <img src="https://stuff.charm.sh/lipgloss/lipgloss-example.gif" width="100%" alt="Lip Gloss Example">
</p>

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

## License

[MIT](LICENSE)
