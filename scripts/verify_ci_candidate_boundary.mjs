#!/usr/bin/env node

/**
 * Fail closed when pull-request CI stops testing an immutable, unprivileged
 * candidate. This check protects the repository workflows from accidentally
 * restoring synthetic merge refs, mutable inputs, persisted credentials, or
 * release mutation before exact-tag validation succeeds.
 */

import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const DIRECT_HEAD_REF = "ref: ${{ github.event.pull_request.head.sha || github.sha }}";
const RELEASE_HEAD_REF = "ref: ${{ github.sha }}";
const RELEASE_IF = "if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')";
const UPSTREAM_COMMIT = "5bd778d050f0a5a130e7cf041917927496dbe722";
const CANDIDATE_JOBS = ["version-bump", "gate", "windows-safe-fallback", "coverage"];
const PUBLICATION_SIBLINGS = [
  "coderbants/rusty-colorprofile",
  "coderbants/rusty-ultraviolet",
  "coderbants/rusty-x-ansi",
];
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

function checkoutBlocks(job) {
  const checkouts = [...job.matchAll(/^      - uses:\s+actions\/checkout@([a-f0-9]{40})\s*$/gmu)];
  return checkouts.map((checkout) => {
    const nextStep = job.indexOf("\n      - ", checkout.index + checkout[0].length);
    return job.slice(checkout.index, nextStep < 0 ? job.length : nextStep);
  });
}

function ownCheckout(job, name) {
  const checkout = checkoutBlocks(job)[0];
  if (checkout === undefined) throw new Error(`${name} requires an immutable candidate checkout step`);
  return checkout;
}

function siblingCheckouts(workflow) {
  const lines = workflow.split("\n");
  const checkouts = [];
  for (const [index, line] of lines.entries()) {
    const repository = /^(\s*)repository:\s+(coderbants\/[^\s]+)\s*$/u.exec(line);
    if (repository === null) continue;
    const indent = repository[1].length;
    const refs = [];
    for (let next = index + 1; next < lines.length; next += 1) {
      const candidate = lines[next];
      if (candidate.trim().length === 0) continue;
      const candidateIndent = /^\s*/u.exec(candidate)[0].length;
      if (candidateIndent < indent) break;
      const reference = /^\s*ref:\s+([^\s#]+)/u.exec(candidate);
      if (candidateIndent === indent && reference !== null) refs.push(reference[1]);
    }
    checkouts.push({ repository: repository[2], refs });
  }
  return checkouts;
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

  for (const sibling of siblingCheckouts(workflow)) {
    if (sibling.refs.length !== 1 || !SHA.test(sibling.refs[0])) {
      throw new Error(`${sibling.repository} checkout must declare exactly one immutable full commit SHA ref`);
    }
  }

  const jobs = jobBlocks(workflow);
  for (const name of CANDIDATE_JOBS) {
    const job = jobs.get(name);
    if (job === undefined) throw new Error(`CI candidate job ${name} is missing`);
    if (!ownCheckout(job, name).includes(DIRECT_HEAD_REF)) {
      throw new Error(`${name} must check out the direct candidate head`);
    }
    if (checkoutBlocks(job).some((checkout) => !checkout.includes("persist-credentials: false"))) {
      throw new Error(`${name} must disable persisted credentials on every checkout`);
    }
  }

  const coverage = jobs.get("coverage");
  if (coverage.includes("git push")) {
    throw new Error("pull-request coverage must not mutate repository state");
  }
}

/** Validate exact-tag release admission and credential isolation. */
export function validateReleaseWorkflow(workflow) {
  if (workflow.includes("workflow_dispatch")) {
    throw new Error("release publication must not accept manual dispatch");
  }
  if (!workflow.includes("tags:\n      - 'v*'")) {
    throw new Error("release publication must be admitted only by v* tag pushes");
  }
  if (!workflow.includes("permissions:\n  contents: read")) {
    throw new Error("release validation must inherit read-only repository permission");
  }

  const actionRefs = [...workflow.matchAll(/\buses:\s+[^@\s]+@([^\s#]+)/gu)].map((match) => match[1]);
  if (actionRefs.length === 0 || actionRefs.some((reference) => !SHA.test(reference))) {
    throw new Error("every release action implementation must use an immutable full commit SHA");
  }

  const jobs = jobBlocks(workflow);
  const validate = jobs.get("validate");
  const publish = jobs.get("publish");
  if (validate === undefined || publish === undefined) {
    throw new Error("release workflow must split credentialless validation from publication");
  }
  if (!validate.includes(RELEASE_IF) || !ownCheckout(validate, "release validation").includes(RELEASE_HEAD_REF)) {
    throw new Error("release validation must use the exact pushed tag commit");
  }
  if (checkoutBlocks(validate).some((checkout) => !checkout.includes("persist-credentials: false"))) {
    throw new Error("release validation must disable persisted credentials on every checkout");
  }
  if (validate.includes("contents: write") || validate.includes("secrets.") || validate.includes("cargo publish")) {
    throw new Error("release validation must not receive mutation authority or release secrets");
  }
  if (!validate.includes("go-version: '1.25.0'") || !validate.includes(UPSTREAM_COMMIT)) {
    throw new Error("release validation must pin the upstream toolchain and source commit");
  }

  if (!publish.includes("needs: validate") || !publish.includes(RELEASE_IF)) {
    throw new Error("publication must depend on exact-tag validation");
  }
  if (!publish.includes("permissions:\n      contents: write") || !publish.includes("environment: crates-io")) {
    throw new Error("publication authority and registry secret must be isolated in the release environment");
  }
  if (!ownCheckout(publish, "publication").includes(RELEASE_HEAD_REF)
      || checkoutBlocks(publish).some((checkout) => !checkout.includes("persist-credentials: false"))) {
    throw new Error("publication checkout must use the exact tag commit without persisted credentials");
  }
  const publicationSiblings = siblingCheckouts(publish);
  const publicationRepositories = publicationSiblings.map((sibling) => sibling.repository).sort();
  if (JSON.stringify(publicationRepositories) !== JSON.stringify([...PUBLICATION_SIBLINGS].sort())) {
    throw new Error("publication must fetch the complete normal dependency closure");
  }
  for (const sibling of publicationSiblings) {
    if (sibling.refs.length !== 1 || !SHA.test(sibling.refs[0])) {
      throw new Error(`${sibling.repository} publication checkout must use one immutable full commit SHA`);
    }
  }
  const packageCheck = publish.indexOf("cargo package --allow-dirty --no-verify");
  const firstMutation = Math.min(
    ...[publish.indexOf("gh release"), publish.indexOf("cargo publish")].filter((index) => index >= 0),
  );
  if (packageCheck < 0 || packageCheck > firstMutation) {
    throw new Error("publication must prove isolated packaging before release mutation");
  }
  if (!publish.includes("cargo publish") || !publish.includes("secrets.CARGO_REGISTRY_TOKEN")) {
    throw new Error("publication job must own the registry mutation explicitly");
  }
  if (publish.includes("scripts/") || publish.includes("cargo test") || publish.includes("cargo build")) {
    throw new Error("write-authorized publication must not execute candidate validation code");
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  validateCandidateWorkflow(readFileSync(new URL("../.github/workflows/ci.yml", import.meta.url), "utf8"));
  validateReleaseWorkflow(readFileSync(new URL("../.github/workflows/publish.yml", import.meta.url), "utf8"));
}
