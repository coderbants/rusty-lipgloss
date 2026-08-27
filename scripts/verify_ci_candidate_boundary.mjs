#!/usr/bin/env node

/**
 * Fail closed when pull-request CI stops testing an immutable, unprivileged
 * candidate. This check protects the repository workflow from accidentally
 * restoring synthetic merge refs, mutable sibling inputs, or persisted write
 * credentials in candidate-controlled coverage execution.
 */

import { readFileSync } from "node:fs";

const workflow = readFileSync(new URL("../.github/workflows/ci.yml", import.meta.url), "utf8");
const directHeadRef = "ref: ${{ github.event.pull_request.head.sha || github.sha }}";
const directHeadCount = workflow.split(directHeadRef).length - 1;

if (directHeadCount !== 4) {
  throw new Error(`CI requires four direct candidate-head checkouts; found ${directHeadCount}`);
}
if (workflow.includes("contents: write")) {
  throw new Error("pull-request CI must not request write-capable repository contents permission");
}
if (workflow.includes("ref: dev")) {
  throw new Error("pull-request CI sibling dependencies must not use mutable dev refs");
}

const siblingRefs = [...workflow.matchAll(/repository:\s+coderbants\/[^\n]+\n\s+path:[^\n]+\n(?:\s+#[^\n]+\n)?\s+ref:\s+([^\s]+)/gu)]
  .map((match) => match[1]);
if (siblingRefs.length !== 15 || siblingRefs.some((reference) => !/^[a-f0-9]{40}$/u.test(reference))) {
  throw new Error("every CI sibling checkout must use an immutable full commit SHA");
}

const coverageStart = workflow.indexOf("\n  coverage:\n");
if (coverageStart < 0) throw new Error("CI coverage job is missing");
const coverage = workflow.slice(coverageStart);
if (!coverage.includes("persist-credentials: false")) {
  throw new Error("candidate-controlled coverage must not receive a persisted checkout credential");
}
if (coverage.includes("git push")) {
  throw new Error("pull-request coverage must not mutate repository state");
}
