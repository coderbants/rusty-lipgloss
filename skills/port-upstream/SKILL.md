# Upstream Porting & Synchronization Guide (`charming-lipgloss`)

This skill provides step-by-step instructions and automated helper scripts for fetching upstream Go [`charmbracelet/lipgloss`](https://github.com/charmbracelet/lipgloss), storing the local Go source copy inside `upstream-go/`, diffing release versions, porting changes into Rust, running test verification, updating crate version tags, and committing back to Git.

---

## 1. Local Upstream Go Source Directory Location

The canonical local upstream Go source code is stored inside the repository at:
```
charming-lipgloss/upstream-go/
```
This folder contains the complete checked-out Go source code of `charmbracelet/lipgloss` matching the currently ported release tag (e.g. `v2.0.5`).

---

## 2. Upstream Synchronization Workflow

### Phase A: Fetch & Update Local Upstream Go Source
To check out or update the local `upstream-go/` directory to a new upstream release tag (e.g. `v2.0.6` or `v2.1.0`):

```bash
# 1. Clone or fetch upstream tags into local upstream-go/
git clone --depth 1 --branch v2.0.5 https://github.com/charmbracelet/lipgloss.git upstream-go/
# Or if upstream-go already exists:
cd upstream-go && git fetch --tags && git checkout v2.1.0 && cd ..
```

---

### Phase B: Diff Analysis Between Release Versions
To inspect exact `.go` source changes introduced upstream between versions (e.g., `v2.0.5` vs `v2.1.0`):

```bash
cd upstream-go
git diff v2.0.5..v2.1.0 -- '*.go'
```

---

### Phase C: Porting Guidelines (Go to Rust)

1. **File & Module Mapping**:
   | Upstream Go File (`upstream-go/`) | Rust Module (`src/`) |
   | --- | --- |
   | `style.go` | `src/style.rs` |
   | `color.go` | `src/color.rs` |
   | `align.go` | `src/align.rs` |
   | `position.go` | `src/position.rs` |
   | `blending.go` | `src/blending.rs` |
   | `join.go` | `src/join.rs` |
   | `border.go` | `src/border.rs` |
   | `size.go` | `src/size.rs` |
   | `whitespace.go` | `src/whitespace.rs` |
   | `table/table.go` | `src/table/table.rs` |
   | `tree/tree.go` | `src/tree/tree.rs` |
   | `list/list.go` | `src/list/list.rs` |

2. **Borrowing Discipline & Upstream Comment Tagging**:
   - Prefer `&str` and `&[T]` for string inputs and slice params.
   - Do NOT introduce `Arc` or `Rc` unless borrowing is demonstrably impossible.
   - **Upstream Comment Tagging**: All comments ported directly from upstream Go declarations MUST be wrapped in `<upstream-comment>...</upstream-comment>` tags within doc comments.

3. **Test Case Parity**:
   - Convert every new `Test*` function in upstream `*_test.go` to a `#[test]` in `tests/*_test.rs`.

---

### Phase D: Version Tagging & Git Commit

Once the Rust porting pass and tests are verified:

1. **Update `Cargo.toml` Version**: Update `version = "X.Y.Z"` in `Cargo.toml` to match the ported upstream version tag.
2. **Run Rust Test Suite**:
   ```bash
   cargo test
   ```
3. **Commit & Tag Release**:
   ```bash
   git add -A
   git commit -m "chore: sync charming-lipgloss to upstream lipgloss vX.Y.Z

   - Update local upstream-go/ source to vX.Y.Z
   - Port upstream diffs to src/
   - Enforce test parity in tests/

   Mutate-Request: 000123-Chrm-charming-lipgloss"

   git tag -a "vX.Y.Z" -m "charming-lipgloss vX.Y.Z matching upstream charmbracelet/lipgloss"
   git push origin main --tags
   ```
