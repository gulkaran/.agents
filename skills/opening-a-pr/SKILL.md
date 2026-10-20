---
name: opening-a-pr
description: Write a pull request title and description and open the PR, as a draft or ready for review. Use when a branch is ready for a PR, or when asked to "open a PR", "write the PR description", or "put this up for review". Describes and opens only; branching, commits, and stacking are not covered here.
---

# Opening a PR

A PR description is a one-minute briefing for a reviewer with no context: what changed, why, and how it was checked. Anything that exists to convince yourself rather than the reviewer stays out, and nothing gets committed to the repo so that the description can link to it.

## 1. Read the branch before writing

- Take draft or ready from the caller. Execute's workers pass draft. A human who invokes this skill directly gets ready for review. Marking a draft ready is the human's action with `gh pr ready`; this skill never does it, including when it updates a body with screenshots.
- Diff against the base and read every file. For a stack, the base is the assigned parent branch. Describe the diff that exists, not the work you remember doing.
- Keep red tests, green implementation, deslop, and any human-requested simplification in separate commits throughout PR preparation and review. Never amend, squash, or fixup cleanup into implementation.
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

- **A screenshot proves a visible change or is omitted.** Use the inspected captures and captions from `verify`. Upload them as GitHub attachments through the procedure below. Never commit an image, HTML preview, or "previews" doc to the repo to have something to link. Missing capture is a reported gap, not a fabricated image.
- **No links a reviewer cannot open.** The posted body must contain hosted image URLs, never vault folders, local paths, or local design docs. Local image paths are allowed only in the temporary body file before `--attach` rewrites them.
- **Deviations from the design go to the human and `decisions.md`, not the PR.** The PR still states the shipped behavior and its limits plainly.
- **No recitals.** No raw logs, per-test narratives, evidence checklists, or accounts of local setup. A measurement belongs only when it explains the improvement.
- **The repo's template wins.** When `AGENTS.md` or `.github/PULL_REQUEST_TEMPLATE.md` requires sections, use them and fit the content above inside.

## 4. Open it

Prepare the exact description in a file outside the application repo. For visible
changes, place the inspected before and after images in the screenshot table
using their absolute local paths. Use descriptive captions for the behavior and
viewport. Do not imply a baseline exists when only an after capture is available.

Push and open with the PR tooling your instructions name; otherwise use GitHub
CLI. Pass every referenced image with a separate `--attach`, using the same path
spelling as the body. GitHub CLI uploads the files to GitHub storage and rewrites
the matching references in place. No image enters the repository's Git history.
Add `--draft` when the caller asked for a draft.

```bash
gh pr create --base "<base>" --head "<branch>" --title "<title>" \
  --body-file "<body-file>" \
  --attach "<absolute-before-image>" --attach "<absolute-after-image>"
```

Omit `--attach` when there are no relevant captures. For an existing PR, read its
current body and use `gh pr edit <number> --body-file <file> --attach <image>`.
Preserve unrelated text and existing hosted URLs; attach only new or replaced
images. Keep work evidence on the target repository's GitHub host, not an
unrelated public image host. Inspect captures for secrets and unrelated data
before upload.

If an attachment fails, creation may still have succeeded. Inspect the returned
URL and existing PR for this branch before retrying. Repair that PR with
`gh pr edit`; never create a duplicate. Check the installed command's help if
attachment support is missing. Do not silently omit available screenshots when
an upload fails; report the evidence delivery gap if it cannot be repaired.
Replace any remaining local reference for a failed upload with a plain gap note
so the published description never leaves a broken local image link.

Read back `gh pr view <number> --json body,url,headRefOid`. Confirm each selected
image has a hosted URL and no local image references remain. Inspect the rendered
description when browser access is available; otherwise report that rendering
was not checked. Do not upload screenshots from an earlier head as current proof.

Follow the design's PR grouping and assigned base. Reuse an existing PR for this
branch instead of opening a duplicate. Attach the PR to the current chat when
supported. For topic work, return the URL and upload outcome in the PR handoff.

## When not to use

- `deslop` cleans the branch. This skill only describes and opens it.
- `pr-review` reviews a PR that is already open.
