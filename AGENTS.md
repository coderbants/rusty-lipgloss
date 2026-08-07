# Charming Lip Gloss Development Guidelines

Read the parent workspace `AGENTS.md` first for Rust standards, process routing, documentation requirements, and borrowing rules.

## Repository Purpose

`charming-lipgloss` is a 1:1 cleanroom Rust port of Charmbracelet's upstream Go `charmbracelet/lipgloss` library (pinned to release **`v2.0.5`**).

## Key Invariants

1. **Structural Parity**: Keep module and type structure direct 1:1 with upstream Go declarations so that `git diff` porting from Go release tags remains mechanical and straightforward.
2. **Borrowing Discipline**: Strongly favor borrowing (`&str`, `&[T]`) over reference counting (`Arc`, `Rc`). Any introduction of `Arc`/`Rc` requires explicit documented technical justification proving borrowing was impossible.
3. **Zero Unsafe**: `#![deny(unsafe_code)]` is enforced across all modules. Zero `.unwrap()` / `.expect()` in library paths.
4. **100% Test Parity**: All upstream Go test cases in `*_test.go` files must be ported 1:1 to Rust `tests/*.rs`.
