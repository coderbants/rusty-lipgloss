import assert from "node:assert/strict";
import test from "node:test";
import { validateCandidateWorkflow } from "./verify_ci_candidate_boundary.mjs";

const SHA = "a".repeat(40);
const DIRECT = "${{ github.event.pull_request.head.sha || github.sha }}";

function checkout(reference = DIRECT, options = "") {
  return `      - uses: actions/checkout@${SHA}
        with:
${options}          ref: ${reference}`;
}

function workflow({ gateRef = DIRECT, gateCheckout = true, actionRef = SHA, extra = "" } = {}) {
  const gate = gateCheckout
    ? checkout(gateRef)
    : `      - name: No candidate checkout
        run: true`;
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
  windows-safe-fallback:
    steps:
${checkout()}
  coverage:
    steps:
${checkout(DIRECT, "          persist-credentials: false\n")}
${extra}`;
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
