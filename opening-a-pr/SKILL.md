---
name: opening-a-pr
description: Write a pull request title and description and open the PR. Use when a branch is ready for review, or when asked to "open a PR", "write the PR description", or "put this up for review". Describes and opens only; branching, commits, and stacking are not covered here.
---

# Opening a PR

A PR description is a one-minute briefing for a reviewer with no context: what changed, why, and how it was checked. Anything that exists to convince yourself rather than the reviewer stays out, and nothing gets committed to the repo so that the description can link to it.

## 1. Check the branch before writing

- Run `~/.agents/skills/deslop/SKILL.md` first if it has not run on this branch. A description cannot excuse local-testing docs, screenshots, or probe scripts in the diff.
- Diff against the base and read every file. Describe the diff that exists, not the work you remember doing.
- Keep the red test commits. They prove tests came first; squash-on-merge later is fine.
- No unrelated changes, generated noise, credentials, or AI attribution trailers.

## 2. Write the title

Imperative mood, at most 72 characters, says what the change does: `Add inbox collapse for stacked PRs`. Use a scope prefix (`auth:`, `fix:`) only when the repo already does.

## 3. Write the description

Drop a section that would be empty; never pad one.

```markdown
## TL;DR

Two or three lines: what this PR does and why, readable with no other context.

## What changed

- One bullet per meaningful behavior change. What it does, not which files moved.

## Screenshots

| Before | After |
| --- | --- |
| ![Before](uploaded-url) | ![After](uploaded-url) |

## How to test

- The checks you ran and their outcomes, plus any gap a reviewer should know about.
- Manual steps only when they help review the change.

## Why

One or two lines: the problem and the behavior this delivers. Link the issue;
`Closes #<n>` only when this PR resolves it.
```

Rules the shape does not make obvious:

- **A screenshot proves a visible change or is omitted.** It must be uploaded where reviewers can see it: the PR's own upload, an issue comment, an image host. Never commit an image, HTML preview, or "previews" doc to the repo to have something to link. No upload path: omit the section and say so in the session.
- **No links a reviewer cannot open.** No vault folders, local paths, or local design docs. Put needed context inline.
- **Deviations from the design go to the human and `decisions.md`, not the PR.** The PR still states the shipped behavior and its limits plainly.
- **No recitals.** No raw logs, per-test narratives, evidence checklists, or accounts of local setup. A measurement belongs only when it explains the improvement.
- **The repo's template wins.** When `AGENTS.md` or `.github/PULL_REQUEST_TEMPLATE.md` requires sections, use them and fit the content above inside.

## 4. Open it

Push and open with the PR tooling your instructions name; otherwise:

```bash
gh pr create --title "<title>" --body-file <file>
```

Follow the design's PR grouping: several slices may share a PR, and no PR exists per session or test step. Record the URL where the calling skill asks; for topic work that is `_state.md`.

## When not to use

- `deslop` cleans the branch. This skill only describes and opens it.
- `pr-review` reviews a PR that is already open.
