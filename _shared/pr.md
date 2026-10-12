# Pull request style

Every PR created by the workflow uses this shape. Follow the approved
design's minimal PR grouping; multiple implementation slices may share a PR.
A PR covers only its own branch's changes.

## Title

- Imperative mood, ≤72 characters: "Add inbox collapse for stacked PRs".
- Optional scope prefix when the repo already uses one (`auth:`, `fix:`).

## Description

```markdown
## TL;DR

2–3 lines: what this PR does and why, readable without context.

## What changed

- One bullet per meaningful change, behavior-first (what it does, not which
  files moved).
- Under the relevant change, show concrete proof: before → after behavior,
  screenshots, actual test results, request/response output, or measured data.
  Explain what the evidence demonstrates and the conditions it was collected
  under. If a before capture isn't available, show direct evidence that the
  new behavior works and state any meaningful verification limits.

## How to test

- Concrete steps or the test command(s) that prove the behavior.

## Why

One or two lines of self-contained context — why the change is needed and
what behavior or acceptance criteria it delivers. Link the relevant GitHub
issue when one exists; use `Closes #<n>` only when this PR resolves it.
```

## Rules

- PR descriptions must stand alone for GitHub reviewers. Do not link local
  vault folders, local files, or the local design document. Include needed
  context inline; link a design only if it is hosted where reviewers can
  access it.
- Report deviations from design and their justifications to the human in
  the agent session and retain them in `decisions.md`, not in the PR
  description. Still describe the actual shipped behavior and relevant
  limitations clearly.
- Evidence must be real and accessible to reviewers: embed uploaded images,
  include concise output/data inline, or link accessible CI runs or artifacts.
  Local paths and unrun test commands are not proof. If screenshot or upload
  tooling is unavailable, use available test/output evidence and state what
  remains unverified. Do not fabricate captures or results. Exclude secrets
  and private data from published evidence.
- Tests-first evidence stays visible: don't squash away the red commit locally
  before the PR is opened (squash-on-merge is fine).
- No generated noise, no credentials, no unrelated changes in the diff.
