# Upstream Go File Mapping: `rusty-lipgloss`

Target Upstream Tag: `charmbracelet/lipgloss@v2.0.5`

This mapping accounts for **every** file in the upstream repository (source, tests, examples, docs, and support files). All `.go` files are pinned to upstream tag `v2.0.5`, checked out locally in `upstream-go/` (gitignored).

## Source Files (root package `lipgloss`)

| Upstream Go File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `lipgloss.go` | `src/lib.rs` | Package doc; module facade and re-exports |
| `style.go` | `src/style.rs` | Style struct, `Render` pipeline, padding, margins, hyperlinks, transforms; profile-aware materialization is a Rust extension that delegates SGR fallback to `src/writer.rs` |
| `set.go` | `src/style.rs` | All style setters (Bold, Italic, Padding, Margin, Border, Width, Height, etc.) |
| `get.go` | `src/style.rs` | All style getters (GetBold, GetPadding, GetBorderSizes, GetFrameSize, etc.) |
| `unset.go` | `src/style.rs` | All style unsetters (UnsetBold, UnsetPadding, UnsetBorderForeground, etc.) |
| `color.go` | `src/color.rs` | Color parsing, RGBColor, ANSI 16/256 palettes, LightDark, Complete, Alpha, Complementary, Darken, Lighten, isDarkColor |
| `align.go` | `src/align.rs` | `align_text_horizontal`, `align_text_vertical`, `get_lines` |
| `position.go` | `src/align.rs` + `src/position.rs` | Float-based `Position`, constants, `place`/`place_horizontal`/`place_vertical` |
| `join.go` | `src/join.rs` | `join_horizontal`, `join_vertical` |
| `size.go` | `src/size.rs` | `width`, `height`, `size` (ANSI-aware cell metrics) |
| `borders.go` | `src/border.rs` | `Border` struct, all 10 presets, `BorderBlend`, edge sizing, `render_horizontal_edge` |
| `blending.go` | `src/blending.rs` | CIELAB `blend_1d`, `blend_2d` |
| `whitespace.go` | `src/whitespace.rs` | `Whitespace` renderer, `with_whitespace_chars` |
| `wrap.go` | `src/wrap.rs` + `src/ansi.rs` | `wrap`, `WrapWriter` (ANSI/hyperlink-preserving) |
| `ranges.go` | `src/ranges.rs` | `style_ranges`, `new_range`, `Range` |
| `runes.go` | `src/runes.rs` | `style_runes` |
| `canvas.go` | `src/canvas.rs` | `Canvas` cell buffer (ultraviolet `Screen`/`Drawable` model) |
| `layer.go` | `src/layer.rs` | `Layer`, `LayerHit`, `Compositor`, `Rectangle` |
| `writer.go` | `src/writer.rs` | Print functions with color-profile downsampling (`Writer`, `print*`, `sprint*`) |
| `query.go` | `src/query.rs` | `background_color`, `has_dark_background` (OSC 11 query + env fallback) |
| `terminal.go` | `src/query.rs` | `query_background_color` / `query_terminal` internal helpers |
| `ansi_unix.go` | `src/platform.rs` | `enable_legacy_windows_ansi` no-op on Unix |
| `ansi_windows.go` | `src/platform.rs` | `enable_legacy_windows_ansi` (safe no-op; native Win32 console VT mode remains deferred while the crate denies unsafe Rust) |

## Sub-package `compat`

| Upstream Go File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `compat/doc.go` | `src/compat.rs` | Package docs for the compatibility layer |
| `compat/color.go` | `src/compat.rs` | `AdaptiveColor`, `CompleteColor`, `CompleteAdaptiveColor`, `HasDarkBackground`, `Profile` |

## Sub-package `list`

| Upstream Go File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `list/list.go` | `src/list/list.rs` | `List` component built on the tree renderer |
| `list/enumerator.go` | `src/list/enumerator.rs` | `alphabet`, `arabic`, `roman`, `bullet`, `asterisk`, `dash` |

## Sub-package `table`

| Upstream Go File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `table/table.go` | `src/table/table.rs` | `Table` rendering, borders, headers, rows, style funcs |
| `table/rows.go` | `src/table/rows.rs` | `Data`, `StringData`, `Filter`, `data_to_matrix` |
| `table/resizing.go` | `src/table/resizing.rs` | Automatic column-width / row-height resizer |
| `table/util.go` | `src/table/util.rs` | `btoi`, `bton`, `sum`, `median` |

## Sub-package `tree`

| Upstream Go File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `tree/tree.go` | `src/tree/tree.rs` | `Tree`, `Leaf`, `Node`, child parenting / auto-nesting |
| `tree/children.go` | `src/tree/children.rs` | `Children`, `NodeChildren`, `Filter`, `new_string_data` |
| `tree/enumerator.go` | `src/tree/enumerator.rs` | `default_enumerator`, `rounded_enumerator`, `default_indenter` |
| `tree/renderer.go` | `src/tree/renderer.rs` | `Renderer`, `TreeStyle`, `EffectiveChildren`, render walk |

## Dependency `charmbracelet/x/ansi` (used by upstream)

| Upstream Dependency | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `ansi.Style` (SGR) | `src/ansi.rs` | SGR sequence builder matching upstream byte-for-byte |
| `ansi.Underline` | `src/ansi.rs` | Underline styles (None/Single/Double/Curly/Dotted/Dashed) |
| `ansi.StringWidth` / `Strip` / `Cut` / `Truncate` / `TruncateLeft` | `src/size.rs` + `src/ansi.rs` | Cell-width metrics and ANSI-aware string slicing |
| `ansi.Wrap` / `WrapWriter` | `src/ansi.rs` + `src/wrap.rs` | ANSI/hyperlink-preserving wrapping |
| `ansi.SetHyperlink` / `ResetHyperlink` | `src/ansi.rs` | OSC 8 hyperlink sequences |

## Test Files (`*_test.go` -> `tests/`)

| Upstream Go Test File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `align_test.go` | `tests/position_test.rs` (align cases in `src/align.rs` unit tests) | Vertical alignment cases ported |
| `blending_test.go` | `tests/blending_test.rs` | Blend1D/Blend2D suites |
| `borders_test.go` | `tests/border_test.rs` | Presets, rune helpers, edge widths |
| `canvas_test.go` | `tests/canvas_test.rs` | Canvas/Layer/Compositor behavior |
| `color_test.go` | `tests/color_test.rs` | Hex parsing, RGBA, Alpha, Complementary, Darken, Lighten |
| `join_test.go` | `tests/join_test.rs` | JoinHorizontal / JoinVertical exact outputs |
| `ranges_test.go` | `tests/ranges_test.rs` | StyleRanges |
| `runes_test.go` | `tests/runes_test.rs` | StyleRunes |
| `size_test.go` | `src/size.rs` unit tests | Width/Height metrics |
| `style_test.go` | `tests/style_test.rs` | Render, underline, tabs, margins, hyperlinks, inherit, unset |
| `whitespace_test.go` | `tests/whitespace_test.rs` | Whitespace rendering incl. tab/zero-width progress |
| `wrap_test.go` | `tests/wrap_test.rs` | Wrapping incl. ANSI preservation |
| `list/list_test.go` | `tests/list_test.rs` | List rendering and enumerators |
| `table/table_test.go` | `tests/table_test.rs` | Table rendering, borders, widths, wrapping |
| `tree/example_test.go` | `tests/tree_test.rs` | Tree rendering examples |
| `tree/tree_test.go` | `tests/tree_test.rs` | Tree structure, hiding, offsets, styles |

Golden testdata files (`*/testdata/*.golden`) are accounted for by the corresponding `tests/*_test.rs` assertions (values verified against upstream output).

## Golden Testdata Files (`testdata/*.golden`)

Every `.golden` file below is accounted for by the corresponding Rust
integration test, with output values verified against the upstream golden
fixtures:

- `list/testdata/` — all `*.golden` files (TestList, TestListItems, TestSublist, TestSublistItems, TestSubListItems2, TestListIntegers, TestMultiline, TestComplexSublist, TestEnumerators/{alphabet,arabic,asterisk,bullet,dash,roman}, TestEnumeratorsAlign, TestEnumeratorsTransform/{alphabet_lower,arabic),bullet_is_dash,roman_within_()})
- `table/testdata/` — all `*.golden` files (TestTable*, TestBorderStyles/*, TestContentWrapping*, TestTableHeightShrink/*, TestTableWidth*, TestTableRowSeparators/*, TestStyleFunc/*, TestFilter*, TestTableYOffset, TestTableWithYOffset, TestTableShrinkWithYOffset/*, TestWrapPreStyledContent, TestWrapStyleFuncContent, TestBorderColumnsWithExtraRows, TestBorderedCells, TestCarriageReturn, TestExtraPaddingHeading, TestExtraPaddingHeadingLong, TestInnerBordersOnly, TestMoreCellsThanHeaders, TestMoreCellsThanHeadersExtra, TestNoFinalEmptyRowWhenOverflow, and all other table fixtures)
- `tree/testdata/` — all `*.golden` files (TestTree*, TestAddItemWithAndWithoutRoot/*, TestEmbedListWithinTree, TestFilter, TestMultilinePrefix*, TestRootStyle, TestTreeStyleAt, TestTreeStyleNilFuncs, TestTreeSubTreeWithCustomEnumerator, TestTreeTable, TestTypes)

## Support & Meta Files

| Upstream File | Rust Equivalent / Status |
| :--- | :--- |
| `.editorconfig` / `.gitattributes` / `.gitignore` | `.gitignore` (editor/git config conventions) |
| `.github/ISSUE_TEMPLATE/*` | Process templates; not applicable to the Rust crate |
| `.github/dependabot.yml` | Process config; not applicable to the Rust crate |
| `.github/workflows/*` | `.github/workflows/publish.yml` (CI/CD) |
| `examples/go.mod` / `examples/go.sum` | `Cargo.toml` (dependency manifest) |
| `examples/table/demo.tape` | VHS recording asset; not applicable to the Rust crate |

## Example Applications (`examples/*` -> `examples/*`)

| Upstream Go Example | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `examples/color/standalone/main.go` | `examples/color_standalone/main.rs` | Standalone background detection + adaptive colors |
| `examples/color/bubbletea/main.go` | Documented in mapping; Bubble Tea program | Interactive Bubble Tea example; requires `rusty-bubbletea` runtime, see that repo |
| `examples/layout/main.go` | `examples/layout/main.rs` | Full layout demo: tabs, title, dialog, lists, status bar |
| `examples/list/simple/main.go` | `examples/list_simple/main.rs` | Nested list with Roman enumerator |
| `examples/list/grocery/main.go` | `examples/list_grocery/main.rs` | Nested grocery list |
| `examples/list/sublist/main.go` | `examples/list_simple/main.rs` (same pattern) | Sublist rendering; pattern covered by grocery/simple ports |
| `examples/list/roman/main.go` | `examples/list_simple/main.rs` (Roman enumerator) | Roman-enumerated list covered |
| `examples/list/glow/main.go` | `examples/list_simple/main.rs` (Glow-style nesting) | Nesting pattern covered |
| `examples/list/duckduckgoose/main.go` | `examples/list_simple/main.rs` (nested list pattern) | Covered by grocery/simple ports |
| `examples/table/languages/main.go` | `examples/table_languages/main.rs` | Styled language table |
| `examples/table/ansi/main.go` | `examples/table_ansi/main.rs` | Minimal ANSI-styled table |
| `examples/table/chess/main.go` | `examples/table_ansi/main.rs` (grid table pattern) | Board table; pattern covered |
| `examples/table/pokemon/main.go` | `examples/table_languages/main.rs` (styled rows pattern) | Covered by languages port |
| `examples/table/mindy/main.go` | `examples/table_languages/main.rs` (style func pattern) | Covered by languages port |
| `examples/tree/simple/main.go` | `examples/tree_simple/main.rs` | OS tree |
| `examples/tree/files/main.go` | `examples/tree_simple/main.rs` (file tree pattern) | Covered by simple port |
| `examples/tree/background/main.go` | `examples/tree_simple/main.rs` (styled tree pattern) | Covered |
| `examples/tree/makeup/main.go` | `examples/tree_simple/main.rs` (styled tree pattern) | Covered |
| `examples/tree/rounded/main.go` | `tests/tree_test.rs` (`rounded_enumerator`) | Rounded enumerator covered by tests |
| `examples/tree/selection/main.go` | `examples/tree_simple/main.rs` (style func pattern) | Covered |
| `examples/tree/styles/main.go` | `examples/tree_simple/main.rs` (item styles) | Covered |
| `examples/tree/toggle/main.go` | `tests/tree_test.rs` (hide tests) | Hidden-node toggle behavior covered |
| `examples/brightness/main.go` | `examples/brightness/main.rs` | Lighten/Darken gradients |
| `examples/canvas/main.go` | `examples/canvas/main.rs` | Layer composition via Compositor |
| `examples/blending/linear-1d/standalone/main.go` | `tests/blending_test.rs` | Blend1D covered by tests |
| `examples/blending/linear-2d/standalone/main.go` | `examples/blend_2d/main.rs` | Blend2D gradient box |
| `examples/blending/linear-1d/bubbletea/main.go` | Documented in mapping; Bubble Tea program | Requires `rusty-bubbletea` runtime |
| `examples/blending/linear-2d/bubbletea/main.go` | Documented in mapping; Bubble Tea program | Requires `rusty-bubbletea` runtime |
| `examples/blending/border-blend-rotation/bubbletea/main.go` | Documented in mapping; Bubble Tea program | Requires `rusty-bubbletea` runtime |
| `examples/compat/standalone/main.go` | `src/compat.rs` (unit tests) | Compat colors covered by compat module |
| `examples/compat/bubbletea/main.go` | Documented in mapping; Bubble Tea program | Requires `rusty-bubbletea` runtime |
| `examples/ssh/main.go` | Documented in mapping; SSH server program | Requires a TUI SSH server runtime (e.g. `rusty-wish`), out of scope for this crate |

## Documentation & Support Files

| Upstream File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `README.md` | `README.md` | Documented Rust port header with graphics & links |
| `LICENSE` | `LICENSE` | MIT License (matching upstream copyright) |
| `UPGRADE_GUIDE_V2.md` | `README.md` (notes) | v1 -> v2 migration guidance summarized in README |
| `go.mod` / `go.sum` | `Cargo.toml` | Dependency manifest (Go modules -> Cargo crates) |
| `Taskfile.yaml` / `.goreleaser.yml` / `.golangci.yml` | `.github/workflows/publish.yml` | Build/lint/release config -> CI workflow |
| `.github/workflows/*` | `.github/workflows/publish.yml` | CI/CD -> Rust publish workflow |

## Feature Parity Notes

- All style properties in `style.go`/`set.go`/`get.go`/`unset.go` are implemented: text attributes (bold, italic, underline+styles, strikethrough, reverse, blink, faint), colors (fg/bg/underline, margins, border sides), underline/strikethrough spaces, width/height, alignment, padding, margins, borders (10 presets + custom), border blend + offset, inline, max width/height, tab width, transforms, hyperlinks, inherit.
- All color functions are implemented: `Color()` parsing (hex/RGB/ANSI16/ANSI256/int), `LightDark`, `Complete` (profile), `Alpha`, `Complementary`, `Darken`, `Lighten`, `is_dark_color`, plus the full xterm 16/256 palettes for RGBA conversion.
- `Position` is a float `0.0..1.0` type exactly as upstream; `place*`, `join*`, and `align*` semantics match upstream (verified against upstream golden outputs).
- Table includes the full resizer (expand/shrink to median, fixed widths, y-offset, manual heights, visible-row computation, overflow rows).
- Tree includes offsets, hiding, auto-nesting of root-less children, custom enumerators/indenters/style funcs, and the exact renderer dance for multiline prefixes.
- Canvas/Layer/Compositor implement the ultraviolet `Screen`/`Drawable` model: cell buffers, z-ordering, hit testing, ID indexing, and render.
- Writer implements profile detection (`NO_COLOR`/`COLORTERM`/`TERM`) and SGR downsampling (TrueColor -> ANSI256 -> ANSI16).
