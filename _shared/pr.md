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
  files moved). Keep each bullet to a short sentence.

## Screenshots

Optional: include useful application screenshots captured during verification.
For a changed screen, use a simple comparison:

| Before | After |
| --- | --- |
| ![Before](uploaded-before-url) | ![After](uploaded-after-url) |

Use an after-only image for a new screen or when no before capture exists.
Add a short caption only when needed. Omit this section when there are no
useful, reviewer-accessible screenshots.

## How to test

- Briefly list checks actually run and their outcomes, plus any meaningful
  verification gaps. Add manual steps only when they help review the change.

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
- Keep **What changed** focused on shipped behavior. Put screenshots in
  **Screenshots** and the brief verification summary in **How to test**.
  Avoid raw logs, per-test narratives, and an evidence checklist for every
  change. Include a measurement only when it helps explain the improvement.
- Embed real screenshots using URLs reviewers can access. Local paths do not
  render on GitHub. If images cannot be uploaded, omit them from the PR and
  mention the limitation in the agent session. Do not fabricate captures or
  results, and exclude secrets and private data from published evidence.
- Tests-first evidence stays visible: don't squash away the red commit locally
  before the PR is opened (squash-on-merge is fine).
- No generated noise, no credentials, no unrelated changes in the diff.
