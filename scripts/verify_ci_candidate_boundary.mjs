#!/usr/bin/env node

/**
 * Fail closed when pull-request CI stops testing an immutable, unprivileged
 * candidate. This check protects the repository workflow from accidentally
 * restoring synthetic merge refs, mutable sibling inputs, or persisted write
 * credentials in candidate-controlled coverage execution.
 */

import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const DIRECT_HEAD_REF = "ref: ${{ github.event.pull_request.head.sha || github.sha }}";
const CANDIDATE_JOBS = ["version-bump", "gate", "windows-safe-fallback", "coverage"];
const SHA = /^[a-f0-9]{40}$/u;

function jobBlocks(workflow) {
  const start = workflow.indexOf("\njobs:\n");
  if (start < 0) throw new Error("CI jobs section is missing");
  const section = workflow.slice(start + "\njobs:\n".length);
  const headers = [...section.matchAll(/^  ([A-Za-z0-9_-]+):\s*$/gmu)];
  return new Map(headers.map((header, index) => {
    const end = headers[index + 1]?.index ?? section.length;
    return [header[1], section.slice(header.index, end)];
  }));
}

function ownCheckout(job, name) {
  const checkout = /^      - uses:\s+actions\/checkout@([a-f0-9]{40})\s*$/mu.exec(job);
  if (checkout === null) throw new Error(`${name} requires an immutable candidate checkout step`);
  const nextStep = job.indexOf("\n      - ", checkout.index + checkout[0].length);
  return job.slice(checkout.index, nextStep < 0 ? job.length : nextStep);
}

/** Validate candidate CI identity, dependency immutability, and permissions. */
export function validateCandidateWorkflow(workflow) {
  if (workflow.includes("contents: write")) {
    throw new Error("pull-request CI must not request write-capable repository contents permission");
  }
  if (workflow.includes("ref: dev")) {
    throw new Error("pull-request CI sibling dependencies must not use mutable dev refs");
  }

  const actionRefs = [...workflow.matchAll(/\buses:\s+[^@\s]+@([^\s#]+)/gu)].map((match) => match[1]);
  if (actionRefs.length === 0 || actionRefs.some((reference) => !SHA.test(reference))) {
    throw new Error("every CI action implementation must use an immutable full commit SHA");
  }

  const siblingRefs = [...workflow.matchAll(/repository:\s+coderbants\/[^\n]+\n\s+path:[^\n]+\n(?:\s+#[^\n]+\n)?\s+ref:\s+([^\s]+)/gu)]
    .map((match) => match[1]);
  if (siblingRefs.some((reference) => !SHA.test(reference))) {
    throw new Error("every CI sibling checkout must use an immutable full commit SHA");
  }

  const jobs = jobBlocks(workflow);
  for (const name of CANDIDATE_JOBS) {
    const job = jobs.get(name);
    if (job === undefined) throw new Error(`CI candidate job ${name} is missing`);
    if (!ownCheckout(job, name).includes(DIRECT_HEAD_REF)) {
      throw new Error(`${name} must check out the direct candidate head`);
    }
  }

  const coverage = jobs.get("coverage");
  if (!ownCheckout(coverage, "coverage").includes("persist-credentials: false")) {
    throw new Error("candidate-controlled coverage must not receive a persisted checkout credential");
  }
  if (coverage.includes("git push")) {
    throw new Error("pull-request coverage must not mutate repository state");
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  validateCandidateWorkflow(readFileSync(new URL("../.github/workflows/ci.yml", import.meta.url), "utf8"));
}
