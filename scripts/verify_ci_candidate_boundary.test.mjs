import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import test from "node:test";
import { fileURLToPath } from "node:url";
import {
  validateCandidateWorkflow,
  validateReleaseWorkflow,
} from "./verify_ci_candidate_boundary.mjs";

const SHA = "a".repeat(40);
const DIRECT = "${{ github.event.pull_request.head.sha || github.sha }}";
const RELEASE = "${{ github.sha }}";
const REGISTRY_TOKEN = "${{ secrets.CARGO_REGISTRY_TOKEN }}";

function checkout(reference = DIRECT, options = "          persist-credentials: false\n") {
  return `      - uses: actions/checkout@${SHA}
        with:
${options}          ref: ${reference}`;
}

function siblingCheckout(repository) {
  const name = repository.split("/").at(-1);
  return `      - uses: actions/checkout@${SHA}
        with:
          repository: ${repository}
          path: siblings/${name}
          persist-credentials: false
          ref: ${SHA}`;
}

function workflow({ gateRef = DIRECT, gateCheckout = true, gateOptions, actionRef = SHA, siblingRef = SHA, extra = "" } = {}) {
  const gate = gateCheckout
    ? checkout(gateRef, gateOptions)
    : `      - name: No candidate checkout
        run: true`;
  const sibling = siblingRef === null ? "" : `\n          ref: ${siblingRef}`;
  return `name: CI
permissions:
  contents: read
jobs:
  version-bump:
    steps:
${checkout()}
  gate:
    steps:
${gate}
      - uses: actions/setup-go@${actionRef}
      - name: Fetch sibling
        uses: actions/checkout@${SHA}
        with:
          repository: coderbants/rusty-colorprofile
          path: siblings/rusty-colorprofile
          persist-credentials: false${sibling}
  windows-safe-fallback:
    steps:
${checkout()}
  coverage:
    steps:
${checkout()}
${extra}`;
}

function releaseWorkflow({
  dispatch = false,
  upstreamRef = "5bd778d050f0a5a130e7cf041917927496dbe722",
  validateOptions,
  publishSiblings = true,
  packageMode = "before",
} = {}) {
  const siblings = publishSiblings
    ? [
      siblingCheckout("coderbants/rusty-colorprofile"),
      siblingCheckout("coderbants/rusty-ultraviolet"),
      siblingCheckout("coderbants/rusty-x-ansi"),
    ].join("\n")
    : "";
  const packageStep = "      - run: cargo package --allow-dirty --no-verify\n";
  const packageBefore = packageMode === "before" ? packageStep : "";
  const packageAfter = packageMode === "after" ? packageStep : "";
  return `name: Publish
on:
  push:
    tags:
      - 'v*'
${dispatch ? "  workflow_dispatch:\n" : ""}permissions:
  contents: read
jobs:
  validate:
    if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')
    steps:
${checkout(RELEASE, validateOptions)}
      - uses: actions/setup-go@${SHA}
        with:
          go-version: '1.25.0'
      - run: git -C upstream-go checkout --quiet ${upstreamRef}
  publish:
    needs: validate
    if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')
    permissions:
      contents: write
    environment: crates-io
    steps:
${checkout(RELEASE)}
${siblings}
${packageBefore}      - run: gh release create
${packageAfter}      - run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${REGISTRY_TOKEN}`;
}

test("accepts job-scoped direct heads with immutable actions and read-only coverage", () => {
  validateCandidateWorkflow(workflow());
});

test("rejects a wrong-job head even when duplicate text preserves the old raw count", () => {
  const duplicateText = `\n# ref: ${DIRECT}\n`;
  assert.throws(
    () => validateCandidateWorkflow(workflow({ gateRef: "${{ github.sha }}", extra: duplicateText })),
    /gate must check out the direct candidate head/u,
  );
});

test("rejects a missing semantic checkout despite duplicate direct-head text", () => {
  const duplicateText = `\n# ref: ${DIRECT}\n# ref: ${DIRECT}\n`;
  assert.throws(
    () => validateCandidateWorkflow(workflow({ gateCheckout: false, extra: duplicateText })),
    /gate requires an immutable candidate checkout step/u,
  );
});

test("rejects mutable action implementations", () => {
  assert.throws(
    () => validateCandidateWorkflow(workflow({ actionRef: "v5" })),
    /every CI action implementation must use an immutable full commit SHA/u,
  );
});

test("rejects an external sibling checkout with no ref", () => {
  assert.throws(
    () => validateCandidateWorkflow(workflow({ siblingRef: null })),
    /rusty-colorprofile checkout must declare exactly one immutable full commit SHA ref/u,
  );
});

test("rejects a candidate checkout that persists its credential", () => {
  assert.throws(
    () => validateCandidateWorkflow(workflow({ gateOptions: "" })),
    /gate must disable persisted credentials on every checkout/u,
  );
});

test("accepts an exact-tag release with isolated publication authority", () => {
  validateReleaseWorkflow(releaseWorkflow());
});

test("rejects manual release dispatch", () => {
  assert.throws(
    () => validateReleaseWorkflow(releaseWorkflow({ dispatch: true })),
    /release publication must not accept manual dispatch/u,
  );
});

test("rejects persisted credentials during release validation", () => {
  assert.throws(
    () => validateReleaseWorkflow(releaseWorkflow({ validateOptions: "" })),
    /release validation must disable persisted credentials on every checkout/u,
  );
});

test("rejects publication without the complete path-dependency closure", () => {
  assert.throws(
    () => validateReleaseWorkflow(releaseWorkflow({ publishSiblings: false })),
    /publication must fetch the complete normal dependency closure/u,
  );
});

test("rejects publication without an isolated package proof", () => {
  assert.throws(
    () => validateReleaseWorkflow(releaseWorkflow({ packageMode: "missing" })),
    /publication must prove isolated packaging before release mutation/u,
  );
});

test("rejects publication packaging after the first mutation", () => {
  assert.throws(
    () => validateReleaseWorkflow(releaseWorkflow({ packageMode: "after" })),
    /publication must prove isolated packaging before release mutation/u,
  );
});

test("rejects a mutable upstream release reference", () => {
  assert.throws(
    () => validateReleaseWorkflow(releaseWorkflow({ upstreamRef: "v2.0.5" })),
    /release validation must pin the upstream toolchain and source commit/u,
  );
});

test("release version validation rejects a non-tag invocation", () => {
  const repositoryRoot = fileURLToPath(new URL("..", import.meta.url));
  const result = spawnSync("bash", ["scripts/verify_upstream_version.sh", "dev"], {
    cwd: repositoryRoot,
    encoding: "utf8",
  });
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /is not an immutable v\* tag/u);
});
