# Upstream Porting Skill Guide (`charming-lipgloss`)

This skill provides step-by-step instructions for diffing and porting incremental changes from upstream Go [`charmbracelet/lipgloss`](https://github.com/charmbracelet/lipgloss) into `charming-lipgloss`.

## Upstream Synchronization Workflow

### Step 1: Identify Upstream Release Tag
Check the target upstream release tag (e.g. `v2.0.5` or `v2.1.0`):
```bash
git clone https://github.com/charmbracelet/lipgloss.git /tmp/upstream-lipgloss
cd /tmp/upstream-lipgloss
git fetch --tags
```

### Step 2: Extract Diff Between Release Tags
```bash
git diff v2.0.5..v2.1.0 -- '*.go'
```

### Step 3: Mechanical Go-to-Rust Translation Guidelines
1. **Module Alignment**:
   - `style.go` -> `src/style.rs`
   - `color.go` -> `src/color.rs`
   - `align.go` -> `src/align.rs`
   - `join.go` -> `src/join.rs`
   - `border.go` -> `src/border.rs`
   - `table/table.go` -> `src/table/table.rs`
   - `tree/tree.go` -> `src/tree/tree.rs`
   - `list/list.go` -> `src/list/list.rs`

2. **Borrowing Discipline & Comment Tagging Rules**:
   - Prefer `&str` and `&[T]` for string inputs and slice params.
   - Do NOT introduce `Arc` or `Rc` unless borrowing is demonstrably impossible across async/thread boundaries.
   - **Upstream Comment Tagging**: All comments ported directly from upstream Go declarations MUST be wrapped in `<upstream-comment>...</upstream-comment>` tags within doc comments to distinguish them from Rust-specific documentation.


3. **Test Case Parity**:
   - Convert every new `Test*` function in upstream `*_test.go` to a `#[test]` in `tests/*_test.rs`.
