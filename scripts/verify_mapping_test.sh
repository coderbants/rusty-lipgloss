#!/usr/bin/env bash
# Focused regression checks for fail-closed upstream mapping verification.
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
test_root="$(mktemp -d "${TMPDIR:-/tmp}/rusty-lipgloss-mapping.XXXXXX")"

case "$test_root" in
  "${TMPDIR:-/tmp}"/rusty-lipgloss-mapping.*) ;;
  *)
    echo "ERROR: unexpected temporary test root: $test_root" >&2
    exit 1
    ;;
esac

cleanup() {
  rm -rf -- "$test_root"
}
trap cleanup EXIT

prepare_fixture() {
  local name="$1"
  local fixture="$test_root/$name"
  mkdir -p "$fixture/scripts"
  cp "$repo_root/scripts/verify_mapping.sh" "$fixture/scripts/verify_mapping.sh"
  cp "$repo_root/UPSTREAM_MAPPING.md" "$fixture/UPSTREAM_MAPPING.md"
}

prepare_fixture missing
if bash "$test_root/missing/scripts/verify_mapping.sh"; then
  echo "ERROR: missing upstream checkout was accepted" >&2
  exit 1
fi

prepare_fixture invalid
mkdir -p "$test_root/invalid/upstream-go/.git"
if bash "$test_root/invalid/scripts/verify_mapping.sh"; then
  echo "ERROR: invalid upstream checkout was accepted" >&2
  exit 1
fi

prepare_fixture empty
mkdir -p "$test_root/empty/upstream-go"
git -C "$test_root/empty/upstream-go" init --quiet
empty_output=""
if empty_output="$(bash "$test_root/empty/scripts/verify_mapping.sh" 2>&1)"; then
  echo "ERROR: empty upstream checkout was accepted" >&2
  exit 1
fi
case "$empty_output" in
  *"upstream-go checkout has no tracked files"*) ;;
  *)
    echo "ERROR: empty upstream checkout failed for an unexpected reason" >&2
    exit 1
    ;;
esac

prepare_fixture valid
mkdir -p "$test_root/valid/upstream-go"
git -C "$test_root/valid/upstream-go" init --quiet
cp "$repo_root/README.md" "$test_root/valid/upstream-go/README.md"
git -C "$test_root/valid/upstream-go" add README.md
bash "$test_root/valid/scripts/verify_mapping.sh"

prepare_fixture worktree
mkdir -p "$test_root/worktree-source"
git -C "$test_root/worktree-source" init --quiet
cp "$repo_root/README.md" "$test_root/worktree-source/README.md"
git -C "$test_root/worktree-source" add README.md
git -C "$test_root/worktree-source" \
  -c user.name=fixture \
  -c user.email=fixture@example.invalid \
  commit --quiet -m fixture
git -C "$test_root/worktree-source" worktree add \
  --quiet --detach "$test_root/worktree/upstream-go" HEAD
bash "$test_root/worktree/scripts/verify_mapping.sh"

echo "OK: mapping verifier rejects missing, invalid, and empty checkouts and accepts clones and worktrees"
