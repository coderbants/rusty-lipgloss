#!/usr/bin/env bash
# Example equivalence verification for charming-lipgloss.
#
# Compiles the upstream Go lipgloss examples (from upstream-go/, pinned at the
# ported tag), captures their verbatim output, runs the corresponding Rust
# examples with the same environment, and diffs each pair. Fails if any pair
# differs.
#
# Requirements: go (1.21+), cargo. Run from the repository root.
set -u

cd "$(dirname "$0")/.."
ROOT="$PWD"
UPSTREAM="$ROOT/upstream-go/examples"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

# Controlled environment: colors are downsampled to the ANSI256 profile by both
# upstream (colorprofile) and the Rust writer; non-TTY output is stripped.
export TERM=xterm-256color
export LANG=C
export LC_ALL=C
unset NO_COLOR COLORTERM COLORFGBG CLICOLOR CLICOLOR_FORCE 2>/dev/null || true

# 1. Build the upstream examples (Go module with a replace to the local source).
(cd "$UPSTREAM" && go mod tidy >/dev/null 2>&1)
if ! (cd "$UPSTREAM" && go build ./... >/dev/null 2>&1); then
  echo "ERROR: upstream Go examples failed to build" >&2
  exit 1
fi

# 2. Pairs to compare: upstream example dir -> Rust example name -> mode.
# Modes: "byte" (verbatim diff) or "sortblocks" (blocks separated by blank
# lines are sorted before diffing; needed when the upstream example's output
# order is non-deterministic, e.g. Go map iteration in the brightness example).
PAIRS="
blending/linear-2d/standalone:blend_2d:byte
brightness:brightness:sortblocks
canvas:canvas:byte
color/standalone:color_standalone:byte
layout:layout:byte
list/grocery:list_grocery:byte
list/simple:list_simple:byte
table/ansi:table_ansi:byte
table/languages:table_languages:byte
tree/simple:tree_simple:byte
"

# Sort blocks (blank-line separated) within the output.
sortblocks() {
  python3 -c "
import sys
blocks = sys.stdin.read().split('\n\n')
print('\n\n'.join(sorted(blocks)))
"
}

fails=0
for pair in $PAIRS; do
  go_dir="${pair%%:*}"
  rest="${pair#*:}"
  rs_ex="${rest%%:*}"
  mode="${rest##*:}"
  go_out="$TMP/go_$(echo "$go_dir" | tr '/' '_').out"
  rs_out="$TMP/rs_${rs_ex}.out"

  # Capture the upstream Go example output (non-TTY).
  (cd "$UPSTREAM/$go_dir" && go run . </dev/null >"$go_out" 2>/dev/null) || {
    echo "ERROR: upstream example $go_dir failed to run" >&2
    fails=1
    continue
  }
  # Capture the Rust example output with the same environment.
  cargo run --quiet --example "$rs_ex" </dev/null >"$rs_out" 2>/dev/null || {
    echo "ERROR: Rust example $rs_ex failed to run" >&2
    fails=1
    continue
  }

  if [ "$mode" = "sortblocks" ]; then
    ok=0
    diff <(sortblocks <"$go_out") <(sortblocks <"$rs_out") >/dev/null 2>&1 && ok=1
  else
    ok=0
    diff -q "$go_out" "$rs_out" >/dev/null 2>&1 && ok=1
  fi

  if [ "$ok" -eq 1 ]; then
    echo "IDENTICAL: $go_dir"
  else
    echo "DIFFERS:   $go_dir"
    fails=1
  fi
done

if [ "$fails" -ne 0 ]; then
  echo "ERROR: example parity check failed" >&2
  exit 1
fi
echo "OK: all examples match upstream Go output byte-for-byte"
